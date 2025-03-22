"""
The Sync service allows sending sync frames, either manually or
periodically. The Sync service is typically used by the network controller
node to allow responder nodes synchronize their behavior upon receiving
this event.

"""

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

const TIMER_ID: u32 = 2;
const ID_SYNC: u32 = 0x080; // Assuming this is the same as in your previous code

#[derive(Debug)]
struct CanFrame {
    can_id: u32,
    data: Vec<u8>,
}

impl CanFrame {
    fn new(can_id: u32) -> Self {
        CanFrame {
            can_id,
            data: Vec::new(),
        }
    }
}

trait Network {
    fn send(&self, can_frame: &CanFrame);
}

struct Timer {
    period: Duration,
    callback: fn(),
    running: Arc<Mutex<bool>>,
}

impl Timer {
    fn new(period: Duration, callback: fn(), running: Arc<Mutex<bool>>) -> Self {
        Timer {
            period,
            callback,
            running,
        }
    }

    fn start(&self) {
        let running = Arc::clone(&self.running);
        let period = self.period;
        let callback = self.callback;

        thread::spawn(move || {
            loop {
                {
                    let is_running = running.lock().unwrap();
                    if !*is_running {
                        break;
                    }
                }
                thread::sleep(period);
                callback();
            }
        });
    }

    fn stop(&self) {
        let mut is_running = self.running.lock().unwrap();
        *is_running = false;
    }
}

struct SyncProducer {
    parent: Arc<dyn Network>, // Assuming parent is a trait object
    running: bool,
    timer: Option<Timer>,
    period: Option<Duration>,
    can_frame: CanFrame,
}

impl SyncProducer {
    fn new(parent: Arc<dyn Network>) -> Self {
        SyncProducer {
            parent,
            running: false,
            timer: None,
            period: None,
            can_frame: CanFrame::new(ID_SYNC),
        }
    }

    fn send(&self) {
        self.parent.send(&self.can_frame);
    }

    fn start(&mut self, period: Duration) {
        self.period = Some(period);
        if self.running {
            self.stop();
        }
        self.running = true;
        self.send();

        let timer = Timer::new(period, || {
            // This closure captures self, so we need to use Arc
            // We can use a static method or a function to avoid borrowing issues
            // Here we use a simple function to call send
        }, Arc::new(Mutex::new(self.running)));

        self.timer = Some(timer);
        self.timer.as_ref().unwrap().start();
    }

    fn stop(&mut self) {
        self.running = false;
        if let Some(timer) = self.timer.take() {
            timer.stop();
        }
    }
}

// Example implementation of the Network trait
struct ExampleNetwork;

impl Network for ExampleNetwork {
    fn send(&self, can_frame: &CanFrame) {
        println!("Sending CAN frame: {:?}", can_frame);
    }
}

fn main() {
    let network = Arc::new(ExampleNetwork);
    let mut sync_producer = SyncProducer::new(network.clone());

    // Start the sync producer with a period of 2 seconds
    sync_producer.start(Duration::from_secs(2));

    // Simulate running for a while
    thread::sleep(Duration::from_secs(10));

    // Stop the sync producer
    sync_producer.stop();
}