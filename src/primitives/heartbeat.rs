use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use crate::can_frame::{CanFrame, CanFrameError}; // Import improved CanFrame
use crate::network::Network;

const ID_HEARTBEAT: u32 = 0x700;

// Timer struct to handle periodic heartbeat signals
struct Timer {
    period: Duration,
    callback: Box<dyn Fn() + Send + Sync>,
    running: Arc<Mutex<bool>>,
}

impl Timer {
    fn new(period: Duration, callback: Box<dyn Fn() + Send + Sync>) -> Self {
        Timer {
            period,
            callback,
            running: Arc::new(Mutex::new(false)),
        }
    }

    fn start(&self) {
        let running = Arc::clone(&self.running);
        let callback = self.callback.clone();
        *running.lock().unwrap() = true;

        thread::spawn(move || {
            while *running.lock().unwrap() {
                thread::sleep(period);
                callback();
            }
        });
    }

    fn stop(&self) {
        *self.running.lock().unwrap() = false;
    }
}

// HeartbeatProducer struct to send periodic heartbeats
pub struct HeartbeatProducer {
    parent: Arc<dyn Network>,
    running: bool,
    timer: Option<Timer>,
    period: Option<Duration>,
    can_frame: CanFrame,
}

impl HeartbeatProducer {
    pub fn new(parent: Arc<dyn Network>) -> Result<Self, CanFrameError> {
        let can_frame = CanFrame::new(ID_HEARTBEAT, None)?;
        Ok(HeartbeatProducer {
            parent,
            running: false,
            timer: None,
            period: None,
            can_frame,
        })
    }

    pub fn send(&self) -> Result<(), CanFrameError> {
        self.parent.send(&self.can_frame)
    }

    pub fn start(&mut self, period: Duration) {
        self.period = Some(period);
        if self.running {
            self.stop();
        }
        self.running = true;
        let parent_clone = Arc::clone(&self.parent);
        let frame_clone = self.can_frame.clone();

        let timer = Timer::new(period, Box::new(move || {
            let _ = parent_clone.send(&frame_clone);
        }));
        self.timer = Some(timer);
        self.timer.as_ref().unwrap().start();
    }

    pub fn stop(&mut self) {
        self.running = false;
        if let Some(timer) = self.timer.take() {
            timer.stop();
        }
    }
}

// HeartbeatConsumer struct to monitor received heartbeats
pub struct HeartbeatConsumer {
    last_received: Arc<Mutex<Option<std::time::Instant>>>,
    timeout: Duration,
}

impl HeartbeatConsumer {
    pub fn new(timeout: Duration) -> Self {
        HeartbeatConsumer {
            last_received: Arc::new(Mutex::new(None)),
            timeout,
        }
    }

    pub fn receive_heartbeat(&self) {
        let mut last_received = self.last_received.lock().unwrap();
        *last_received = Some(std::time::Instant::now());
    }

    pub fn check_timeout(&self) -> bool {
        let last_received = self.last_received.lock().unwrap();
        if let Some(last) = *last_received {
            return last.elapsed() > self.timeout;
        }
        true
    }
}
