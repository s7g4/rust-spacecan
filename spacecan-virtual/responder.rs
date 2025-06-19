#![cfg_attr(not(feature = "async"), allow(dead_code, unused_imports))]

use spacecan::services::ST08_function_management;
use spacecan::services::ST01_request_verification;
use spacecan::services::ST20_parameter_management;
use spacecan::services::ST03_housekeeping;


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
    use spacecan::services::ST17_test;

    let socket = socketcan::Socket::open("vcan0")?;
    let mut stream = CanSocketStream::new(socket);

    println!("Responder listening on vcan0...");

    // Instantiate services
    let function_management = ST08_function_management::FunctionManagementService::new(&MyParent {});
    let request_verification = ST01_request_verification::RequestVerificationServiceController::new(std::sync::Arc::new(MyParent {}));
    let parameter_management = ST20_parameter_management::ParameterManagementServiceController::new(Box::new(MyParent {}));
    let housekeeping = ST03_housekeeping::HousekeepingServiceController::new(std::sync::Arc::new(MyParent {}));
    let test_service = ST17_test::TestServiceController::new(Box::new(MyParent {}));

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
                        function_management.process(0, raw_id as u8, data.to_vec(), 0);
                        request_verification.process(0, raw_id as u8, data.to_vec(), 0);
                        parameter_management.process(0, raw_id as u8, data.to_vec(), 0);
                        housekeeping.process(0, raw_id as u8, data.to_vec(), 0);
                        test_service.process(0, raw_id as u8, data.to_vec(), 0);
                        println!("Other CAN frame received: id=0x{:X} data={:?}", raw_id, data);
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading CAN frame: {}", e);
            }
        }
    }

    Ok(())
}

struct MyParent;

impl ST08_function_management::Parent for MyParent {
    fn send(&self, _packet: ST08_function_management::Packet) {
        // Implement send logic here
    }
}

use spacecan::services::ST17_test::RequestVerification as ST17RequestVerification;

trait RequestVerification {
    fn send_success_acceptance_report(&self, args: &[u8]);
    fn send_success_completion_report(&self, args: &[u8]);
    fn send_fail_acceptance_report(&self, args: &[u8]);
    fn send_fail_completion_report(&self, args: &[u8]);
}

trait ParentST01 {
    fn send(&self, packet: Vec<u8>);
    fn request_verification(&self) -> &dyn RequestVerification;
}

struct MyRequestVerification;

impl RequestVerification for MyRequestVerification {
    fn send_success_acceptance_report(&self, _args: &[u8]) {
        // Implement actual logic or leave empty
    }
    fn send_success_completion_report(&self, _args: &[u8]) {
        // Implement actual logic or leave empty
    }
    fn send_fail_acceptance_report(&self, _args: &[u8]) {
        // Implement actual logic or leave empty
    }
    fn send_fail_completion_report(&self, _args: &[u8]) {
        // Implement actual logic or leave empty
    }
}

impl ParentST01 for MyParent {
    fn send(&self, _packet: Vec<u8>) {
        // Implement send logic here
    }
    fn request_verification(&self) -> &dyn RequestVerification {
        &MyRequestVerification
    }
}

trait ParentST20 {
    fn send(&self, packet: Vec<u8>, node_id: u32);
    fn request_verification(&self) -> &dyn RequestVerification;
}

impl ParentST20 for MyParent {
    fn send(&self, _packet: Vec<u8>, _node_id: u32) {
        // Implement send logic here
    }
    fn request_verification(&self) -> &dyn RequestVerification {
        &MyRequestVerification
    }
}

trait ParentST03 {
    fn send(&self, packet: Vec<u8>);
    fn get_parameter(&self, parameter_id: (u32, u32)) -> Vec<u8>;
}

impl ParentST03 for MyParent {
    fn send(&self, _packet: Vec<u8>) {
        // Implement send logic here
    }
    fn get_parameter(&self, _parameter_id: (u32, u32)) -> Vec<u8> {
        // Implement get_parameter logic here or return dummy
        vec![]
    }
}

#[cfg(not(feature = "async"))]
fn main() {
    println!("Async feature disabled. This binary does nothing.");
}
