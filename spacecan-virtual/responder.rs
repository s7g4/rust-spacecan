#![cfg_attr(not(feature = "async"), allow(dead_code, unused_imports))]

#[cfg(feature = "async")]
use tokio::task;
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

    while let Some(frame_result) = stream.next().await {
        match frame_result {
            Ok(frame) => {
                let id = socketcan::EmbeddedFrame::id(&frame);
                let data = socketcan::EmbeddedFrame::data(&frame);

                let raw_id = match id {
                    socketcan::Id::Standard(id_val) => id_val.as_raw(),
                    socketcan::Id::Extended(id_val) => id_val.as_raw().try_into().unwrap(),
                };

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

#[cfg(not(feature = "async"))]
fn main() {
    println!("Async feature disabled. This binary does nothing.");
}
