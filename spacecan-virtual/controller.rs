#![allow(dead_code, unused_imports, unused_variables)]

use anyhow::Result;
use std::io::{self, Write};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::mpsc;
use tokio::time::{Duration, sleep};

use spacecan::services::{
    ST01_request_verification, ST03_housekeeping, ST08_function_management, ST17_test,
    ST20_parameter_management,
};

#[path = "network.rs"]
mod network;

#[cfg(feature = "async")]
#[tokio::main]
async fn main() -> Result<()> {
    // Channel for sending commands from menu to sender task
    let (cmd_tx, mut cmd_rx) = mpsc::channel::<String>(10);

    // Shared state for sent and received data
    let (sent_tx, mut sent_rx) = mpsc::channel::<String>(100);
    let (recv_tx, mut recv_rx) = mpsc::channel::<String>(100);

    let sender_socket = network::create_multicast_socket()?;
    let sent_tx_clone = sent_tx.clone();

    tokio::spawn(async move {
        let mut heartbeat_counter: u32 = 0;
        let mut sync_counter: u32 = 0;

        loop {
            // Send heartbeat frame every 1 second
            let heartbeat_frame = network::UdpCanFrame {
                id: 0x700,
                data: heartbeat_counter.to_be_bytes().to_vec(),
            };

            if let Err(e) = network::send_multicast(&sender_socket, &heartbeat_frame).await {
                let _ = sent_tx_clone
                    .send(format!("Error sending heartbeat: {}", e))
                    .await;
            } else {
                let _ = sent_tx_clone
                    .send(format!("Sent Heartbeat: counter={}", heartbeat_counter))
                    .await;
            }
            heartbeat_counter = heartbeat_counter.wrapping_add(1);

            // Every 5 seconds send SYNC frame and time frames
            if sync_counter % 5 == 0 {
                let sync_frame = network::UdpCanFrame {
                    id: 0x080,
                    data: vec![],
                };
                if let Err(e) = network::send_multicast(&sender_socket, &sync_frame).await {
                    let _ = sent_tx_clone
                        .send(format!("Error sending SYNC frame: {}", e))
                        .await;
                } else {
                    let _ = sent_tx_clone.send("Sent SYNC frame".to_string()).await;
                }

                let scet = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                let scet_frame = network::UdpCanFrame {
                    id: 0x100,
                    data: scet.to_be_bytes().to_vec(),
                };
                if let Err(e) = network::send_multicast(&sender_socket, &scet_frame).await {
                    let _ = sent_tx_clone
                        .send(format!("Error sending SCET frame: {}", e))
                        .await;
                } else {
                    let _ = sent_tx_clone.send(format!("Sent SCET: {}", scet)).await;
                }

                let utc = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                let utc_frame = network::UdpCanFrame {
                    id: 0x101,
                    data: utc.to_be_bytes().to_vec(),
                };
                if let Err(e) = network::send_multicast(&sender_socket, &utc_frame).await {
                    let _ = sent_tx_clone
                        .send(format!("Error sending UTC frame: {}", e))
                        .await;
                } else {
                    let _ = sent_tx_clone.send(format!("Sent UTC: {}", utc)).await;
                }
            }

            sync_counter += 1;

            if let Ok(cmd) = cmd_rx.try_recv() {
                let _ = sent_tx_clone
                    .send(format!("Received command: {}", cmd))
                    .await;

                match cmd.as_str() {
                    "st01" => {
                        let mut service =
                            ST01_request_verification::RequestVerificationService::new();
                        service.accept_telecommand(1, 0);
                        let _ = sent_tx_clone
                            .send("ST01 service command executed".to_string())
                            .await;
                    }
                    "st08" => {
                        let mut service =
                            ST08_function_management::FunctionManagementService::new();
                        let _ = service.perform_function(1, spacecan::PacketData::new());
                        let _ = sent_tx_clone
                            .send("ST08 service command executed".to_string())
                            .await;
                    }
                    "st03" => {
                        let mut service = ST03_housekeeping::HousekeepingService::new();
                        service.create_report(spacecan::ParamList::from_slice(&[1]).unwrap(), 0u32);
                        let _ = sent_tx_clone
                            .send("ST03 service command executed".to_string())
                            .await;
                    }
                    "st17" => {
                        let mut service = ST17_test::TestService::new();
                        service.create_connection_test(1);
                        let _ = sent_tx_clone
                            .send("ST17 service command executed".to_string())
                            .await;
                    }
                    "st20" => {
                        let service = ST20_parameter_management::ParameterManagementService::new();
                        let _ = service.report_parameter_values(spacecan::ParamList::from_slice(&[1]).unwrap());
                        let _ = sent_tx_clone
                            .send("ST20 service command executed".to_string())
                            .await;
                    }
                    _ => {
                        let _ = sent_tx_clone
                            .send(format!("Unknown command: {}", cmd))
                            .await;
                    }
                }
            }

            sleep(Duration::from_secs(1)).await;
        }
    });

    let recv_socket = network::create_multicast_socket()?;
    let recv_tx_clone = recv_tx.clone();
    tokio::spawn(async move {
        let mut buf = vec![0u8; 1024];
        loop {
            match recv_socket.recv_from(&mut buf).await {
                Ok((len, _addr)) => {
                    if let Ok(frame) = serde_json::from_slice::<network::UdpCanFrame>(&buf[..len]) {
                        let msg =
                            format!("Received frame: ID=0x{:X}, Data={:?}", frame.id, frame.data);
                        let _ = recv_tx_clone.send(msg).await;
                    }
                }
                Err(e) => {
                    let _ = recv_tx_clone
                        .send(format!("Error receiving frame: {}", e))
                        .await;
                }
            }
        }
    });

    loop {
        print!("\x1B[2J\x1B[1;1H");
        println!("SpaceCAN Virtual Controller (UDP Multicast)");
        println!("===========================================");
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
                        let _ = cmd_tx.send("st01".to_string()).await;
                    }
                    "2" => {
                        let _ = cmd_tx.send("st08".to_string()).await;
                    }
                    "3" => {
                        let _ = cmd_tx.send("st03".to_string()).await;
                    }
                    "4" => {
                        let _ = cmd_tx.send("st17".to_string()).await;
                    }
                    "5" => {
                        let _ = cmd_tx.send("st20".to_string()).await;
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
