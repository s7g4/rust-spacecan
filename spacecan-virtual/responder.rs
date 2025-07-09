#![cfg_attr(not(feature = "async"), allow(dead_code, unused_imports))]

use spacecan::services::ST08_function_management;
use spacecan::services::ST01_request_verification;
use spacecan::services::ST20_parameter_management;
use spacecan::services::ST03_housekeeping;
use spacecan::services::ST17_test;
use spacecan::Packet;
use socketcan::EmbeddedFrame;


#[cfg(feature = "async")]
use tokio_stream::StreamExt;
#[cfg(feature = "async")]
use futures_core::stream::Stream;
#[cfg(feature = "async")]
use std::pin::Pin;
#[cfg(feature = "async")]
use std::task::{Context, Poll};
#[cfg(feature = "async")]
use std::convert::TryInto;
#[cfg(feature = "async")]
use std::sync::mpsc::{self, Receiver};
#[cfg(feature = "async")]
use std::thread;

#[cfg(feature = "async")]
struct CanSocketStream {
    receiver: Receiver<Result<socketcan::CanFrame, std::io::Error>>,
}

#[cfg(feature = "async")]
impl Stream for CanSocketStream {
    type Item = Result<socketcan::CanFrame, std::io::Error>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match self.receiver.try_recv() {
            Ok(frame) => Poll::Ready(Some(frame)),
            Err(mpsc::TryRecvError::Empty) => {
                cx.waker().wake_by_ref();
                Poll::Pending
            }
            Err(mpsc::TryRecvError::Disconnected) => Poll::Ready(None),
        }
    }
}

#[cfg(feature = "async")]
impl CanSocketStream {
    fn new(socket: socketcan::CanSocket) -> Self {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            loop {
                match socketcan::Socket::read_frame(&socket) {
                    Ok(frame) => {
                        if tx.send(Ok(frame)).is_err() {
                            break;
                        }
                    }
                    Err(e) => {
                        let _ = tx.send(Err(e));
                        break;
                    }
                }
            }
        });

        CanSocketStream { receiver: rx }
    }
}

#[cfg(feature = "async")]
#[tokio::main]
async fn main() -> anyhow::Result<()> {

    let socket = socketcan::Socket::open("vcan0")?;
    let mut stream = CanSocketStream::new(socket);

    println!("Responder listening on vcan0...");

    // Instantiate services
    let mut function_management = ST08_function_management::FunctionManagementService::new();
    let mut request_verification = ST01_request_verification::RequestVerificationService::new();
    let mut parameter_management = ST20_parameter_management::ParameterManagementService::new();
    let mut housekeeping = ST03_housekeeping::HousekeepingService::new();
    let mut test_service = ST17_test::TestService::new();

    while let Some(frame_result) = stream.next().await {
        match frame_result {
            Ok(frame) => {
                let id = socketcan::EmbeddedFrame::id(&frame);
                let data = socketcan::EmbeddedFrame::data(&frame);

                let raw_id = match id {
                    socketcan::Id::Standard(id_val) => id_val.as_raw(),
                    socketcan::Id::Extended(id_val) => id_val.as_raw().try_into().unwrap(),
                };

                // Example: dispatch frame to services based on raw_id or other criteria
                match raw_id {
                    0x700 => {
                        // Heartbeat frame
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
                        // Dispatch to services here, example:
                        let _ = function_management.perform_function(raw_id as u16, data.to_vec());
                        let _ = request_verification.accept_telecommand(raw_id as u16, raw_id as u32);
                        let _ = parameter_management.report_parameter_values(vec![raw_id as u16]);
                        let _ = housekeeping.create_report(vec![raw_id as u16], 0);
                        let _ = test_service.create_connection_test(raw_id as u32);
                        println!("Other CAN frame received: id=0x{:X} data={:?}", raw_id, data);
                    }
                }

                // Display menu correspondence: print a summary line for each service
                println!("Service status:");
                println!("Function Management: last processed ID 0x{:X}", raw_id);
                println!("Request Verification: last processed ID 0x{:X}", raw_id);
                println!("Parameter Management: last processed ID 0x{:X}", raw_id);
                println!("Housekeeping: last processed ID 0x{:X}", raw_id);
                println!("Test Service: last processed ID 0x{:X}", raw_id);
            }
            Err(e) => {
                eprintln!("Error reading CAN frame: {}", e);
            }
        }
    }

    Ok(())
}

// Remove all the incorrect trait implementations as they don't exist in the actual services

#[cfg(not(feature = "async"))]
fn main() {
    println!("Async feature disabled. This binary does nothing.");
}
