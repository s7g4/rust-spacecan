#![allow(dead_code, unused_imports, unused_variables)]

use anyhow::Result;
use spacecan::Packet;
use spacecan::services::ST01_request_verification;
use spacecan::services::ST03_housekeeping;
use spacecan::services::ST08_function_management;
use spacecan::services::ST17_test;
use spacecan::services::ST20_parameter_management;

#[path = "network.rs"]
mod network;

#[cfg(feature = "async")]
#[tokio::main]
async fn main() -> Result<()> {
    let socket = network::create_multicast_socket()?;
    println!(
        "Responder listening on UDP Multicast {}...",
        network::MULTICAST_ADDR
    );

    let mut buf = vec![0u8; 1024];

    // Instantiate services
    let mut function_management = ST08_function_management::FunctionManagementService::new();
    let mut request_verification = ST01_request_verification::RequestVerificationService::new();
    let parameter_management = ST20_parameter_management::ParameterManagementService::new();
    let mut housekeeping = ST03_housekeeping::HousekeepingService::new();
    let mut test_service = ST17_test::TestService::new();

    loop {
        match socket.recv_from(&mut buf).await {
            Ok((len, _addr)) => {
                let frame: network::UdpCanFrame = match serde_json::from_slice(&buf[..len]) {
                    Ok(f) => f,
                    Err(_) => continue, // ignore non-JSON junk on this port
                };

                let raw_id = frame.id;
                let data = frame.data;

                match raw_id {
                    0x700 => {
                        println!("Heartbeat received: counter={:?}", data);
                    }
                    0x080 => {
                        println!("SYNC frame received");
                    }
                    0x100 => {
                        if data.len() >= 8 {
                            let scet = u64::from_be_bytes(data[0..8].try_into().unwrap());
                            println!("SCET frame received: {}", scet);
                        }
                    }
                    0x101 => {
                        if data.len() >= 8 {
                            let utc = u64::from_be_bytes(data[0..8].try_into().unwrap());
                            println!("UTC frame received: {}", utc);
                        }
                    }
                    _ => {
                        let _ = function_management.perform_function(raw_id as u16, data.to_vec());
                        let _ =
                            request_verification.accept_telecommand(raw_id as u16, raw_id as u32);
                        let _ = parameter_management.report_parameter_values(vec![raw_id as u16]);
                        let _ = housekeeping.create_report(vec![raw_id as u16], 0);
                        let _ = test_service.create_connection_test(raw_id as u32);

                        println!(
                            "Other CAN frame received: id=0x{:X} data={:?}",
                            raw_id, data
                        );
                    }
                }

                println!("Service status:");
                println!("Function Management: last processed ID 0x{:X}", raw_id);
                println!("Request Verification: last processed ID 0x{:X}", raw_id);
                println!("Parameter Management: last processed ID 0x{:X}", raw_id);
                println!("Housekeeping: last processed ID 0x{:X}", raw_id);
                println!("Test Service: last processed ID 0x{:X}", raw_id);
            }
            Err(e) => {
                eprintln!("Error reading UDP frame: {}", e);
            }
        }
    }
}

#[cfg(not(feature = "async"))]
fn main() {
    println!("Async feature disabled. This binary does nothing.");
}
