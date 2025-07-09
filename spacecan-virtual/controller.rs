#![cfg_attr(not(feature = "async"), allow(dead_code, unused_imports))]

use std::io::{self, Write};
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};
use socketcan::{CanFrame, CanSocket, Socket, StandardId, EmbeddedFrame};
use anyhow::{Result, anyhow};
use std::time::{SystemTime, UNIX_EPOCH};

use spacecan::services::{
    ST01_request_verification,
    ST03_housekeeping,
    ST08_function_management,
    ST17_test,
    ST20_parameter_management,
};

#[cfg(feature = "async")]
#[tokio::main]
async fn main() -> Result<()> {
    // Open CAN socket on vcan0
    let socket: CanSocket = Socket::open("vcan0")?;

    // Channel for sending commands from menu to CAN sender task
    let (cmd_tx, mut cmd_rx) = mpsc::channel::<String>(10);

    // Shared state for sent and received data
    let (sent_tx, mut sent_rx) = mpsc::channel::<String>(100);
    let (recv_tx, mut recv_rx) = mpsc::channel::<String>(100);

    // Spawn task to send heartbeat and sync frames
    let sender_socket: CanSocket = Socket::open("vcan0")?;
    let sent_tx_clone = sent_tx.clone();
    tokio::spawn(async move {
        let mut heartbeat_counter: u32 = 0;
        let mut sync_counter: u32 = 0;

        loop {
            // Send heartbeat frame every 1 second
            let heartbeat_id = StandardId::new(0x700).unwrap();
            let heartbeat_data = heartbeat_counter.to_be_bytes(); // 4 bytes
            let heartbeat_frame = CanFrame::new(heartbeat_id, &heartbeat_data).unwrap();
            if let Err(e) = sender_socket.write_frame(&heartbeat_frame) {
                let _ = sent_tx_clone.send(format!("Error sending heartbeat: {}", e)).await;
            } else {
                let _ = sent_tx_clone.send(format!("Sent Heartbeat: counter={}", heartbeat_counter)).await;
            }
            heartbeat_counter = heartbeat_counter.wrapping_add(1);

            // Every 5 seconds send SYNC frame and time frames
            if sync_counter % 5 == 0 {
                // SYNC frame (ID 0x080, empty payload)
                let sync_id = StandardId::new(0x080).unwrap();
                let sync_frame = CanFrame::new(sync_id, &[]).unwrap();
                if let Err(e) = sender_socket.write_frame(&sync_frame) {
                    let _ = sent_tx_clone.send(format!("Error sending SYNC frame: {}", e)).await;
                } else {
                    let _ = sent_tx_clone.send("Sent SYNC frame".to_string()).await;
                }

                // Send SCET (Spacecraft Event Time) - simulate as UNIX timestamp (u64)
                let scet = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("Time went backwards")
                    .as_secs();
                let scet_bytes = scet.to_be_bytes();
                let scet_id = StandardId::new(0x100).unwrap();
                let scet_frame = CanFrame::new(scet_id, &scet_bytes[0..8]).unwrap();
                if let Err(e) = sender_socket.write_frame(&scet_frame) {
                    let _ = sent_tx_clone.send(format!("Error sending SCET frame: {}", e)).await;
                } else {
                    let _ = sent_tx_clone.send(format!("Sent SCET: {}", scet)).await;
                }

                // Send UTC time as seconds since UNIX_EPOCH too (just a demo)
                let utc = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("Time went backwards")
                    .as_secs();
                let utc_bytes = utc.to_be_bytes();
                let utc_id = StandardId::new(0x101).unwrap();
                let utc_frame = CanFrame::new(utc_id, &utc_bytes[0..8]).unwrap();
                if let Err(e) = sender_socket.write_frame(&utc_frame) {
                    let _ = sent_tx_clone.send(format!("Error sending UTC frame: {}", e)).await;
                } else {
                    let _ = sent_tx_clone.send(format!("Sent UTC: {}", utc)).await;
                }
            }

            sync_counter += 1;

            // Check for commands from menu (non-blocking)
            if let Ok(cmd) = cmd_rx.try_recv() {
                // Handle service functionality commands here
                let _ = sent_tx_clone.send(format!("Received command: {}", cmd)).await;

                // Example: parse command and call ST service functions
                match cmd.as_str() {
                    "st01" => {
                        let mut service = ST01_request_verification::RequestVerificationService::new();
                        service.accept_telecommand(1, 0);
                        let _ = sent_tx_clone.send("ST01 service command executed".to_string()).await;
                    }
                    "st08" => {
                        let mut service = ST08_function_management::FunctionManagementService::new();
                        service.perform_function(1, vec![]);
                        let _ = sent_tx_clone.send("ST08 service command executed".to_string()).await;
                    }
                    "st03" => {
                        let mut service = ST03_housekeeping::HousekeepingService::new();
                        service.create_report(vec![1], 0u32);
                        let _ = sent_tx_clone.send("ST03 service command executed".to_string()).await;
                    }
                    "st17" => {
                        let mut service = ST17_test::TestService::new();
                        service.create_connection_test(1);
                        let _ = sent_tx_clone.send("ST17 service command executed".to_string()).await;
                    }
                    "st20" => {
                        let mut service = ST20_parameter_management::ParameterManagementService::new();
                        service.report_parameter_values(vec![1]);
                        let _ = sent_tx_clone.send("ST20 service command executed".to_string()).await;
                    }
                    _ => {
                        let _ = sent_tx_clone.send(format!("Unknown command: {}", cmd)).await;
                    }
                }
            }

            sleep(Duration::from_secs(1)).await;
        }
    });

    // Spawn task to receive CAN frames
    let recv_socket: CanSocket = Socket::open("vcan0")?;
    let recv_tx_clone = recv_tx.clone();
    tokio::spawn(async move {
        loop {
                    match recv_socket.read_frame() {
                        Ok(frame) => {
                            let id = match frame.id() {
                                socketcan::Id::Standard(sid) => sid.as_raw() as u32,
                                socketcan::Id::Extended(eid) => eid.as_raw(),
                            };
                            let data = frame.data();
                            let msg = format!("Received frame: ID=0x{:X}, Data={:?}", id, data);
                            let _ = recv_tx_clone.send(msg).await;
                        }
                        Err(e) => {
                            let _ = recv_tx_clone.send(format!("Error receiving frame: {}", e)).await;
                        }
                    }
            sleep(Duration::from_millis(100)).await;
        }
    });

    // Menu interface task
    loop {
        // Clear screen
        print!("\x1B[2J\x1B[1;1H");
        println!("SpaceCAN Virtual Controller");
        println!("============================");
        println!("Menu:");
        println!("1. Show sent data");
        println!("2. Show received data");
        println!("3. Service functionality options");
        println!("h. Help");
        println!("q. Quit");
        println!();
        print!("Enter choice: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let choice = input.trim();

        match choice {
            "1" => {
                println!("Sent Data:");
                while let Ok(msg) = sent_rx.try_recv() {
                    println!("{}", msg);
                }
            }
            "2" => {
                println!("Received Data:");
                while let Ok(msg) = recv_rx.try_recv() {
                    println!("{}", msg);
                }
            }
            "3" => {
                println!("Service Functionality Options:");
                println!("Select ST Service:");
                println!("1. ST01 Request Verification");
                println!("2. ST08 Function Management");
                println!("3. ST03 Housekeeping");
                println!("4. ST17 Test");
                println!("5. ST20 Parameter Management");
                println!("b. Back to main menu");
                print!("Enter choice: ");
                io::stdout().flush().unwrap();

                let mut st_input = String::new();
                io::stdin().read_line(&mut st_input).unwrap();
                let st_choice = st_input.trim();

                match st_choice {
                    "1" => {
                        println!("Invoking ST01 Request Verification Service...");
                        // Call a representative function from ST01_request_verification
                        let mut service = ST01_request_verification::RequestVerificationService::new();
                        // Example call to accept_telecommand
                        service.accept_telecommand(1, 0);
                    }
                    "2" => {
                        println!("Invoking ST08 Function Management Service...");
                        let mut service = ST08_function_management::FunctionManagementService::new();
                        // Example call to perform_function
                        service.perform_function(1, vec![]);
                    }
                    "3" => {
                        println!("Invoking ST03 Housekeeping Service...");
                        let mut service = ST03_housekeeping::HousekeepingService::new();
                        // Example call to create_report
                        service.create_report(vec![1], 0u32);
                    }
                    "4" => {
                        println!("Invoking ST17 Test Service...");
                        let mut service = ST17_test::TestService::new();
                        service.create_connection_test(1);
                    }
                    "5" => {
                        println!("Invoking ST20 Parameter Management Service...");
                        let mut service = ST20_parameter_management::ParameterManagementService::new();
                        service.report_parameter_values(vec![1]);
                    }
                    "b" => {
                        println!("Returning to main menu...");
                    }
                    _ => {
                        println!("Invalid choice");
                    }
                }
            }
            "h" => {
                println!("Help:");
                println!("1: Show sent CAN frames and messages");
                println!("2: Show received CAN frames and messages");
                println!("3: Service functionality options menu");
                println!("q: Quit the controller");
            }
            "q" => {
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Invalid choice");
            }
        }

        println!("Press Enter to continue...");
        let mut dummy = String::new();
        io::stdin().read_line(&mut dummy).unwrap();
    }

    Ok(())
}

#[cfg(not(feature = "async"))]
fn main() {
    println!("Async feature disabled. This binary does nothing.");
}
