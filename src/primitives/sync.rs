use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use super::can_frame::{CanFrame, CanFrameError};
use crate::primitives::network::Parent;

const ID_SYNC: u32 = 0x080;

// Timer struct for periodic sync frame transmission
struct Timer {
    period: Duration,
    callback: Arc<dyn Fn() + Send + Sync>,
    running: Arc<Mutex<bool>>,
}

impl Timer {
    fn new(period: Duration, callback: Arc<dyn Fn() + Send + Sync>) -> Self {
        Timer {
            period,
            callback,
            running: Arc::new(Mutex::new(false)),
        }
    }

    fn start(&self) {
        let running = Arc::clone(&self.running);
        let callback = Arc::clone(&self.callback);
        let period = self.period;
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

// SyncProducer struct to send periodic synchronization frames
pub struct SyncProducer {
    parent: Arc<dyn Parent>,
    running: bool,
    timer: Option<Timer>,
    period: Option<Duration>,
    can_frame: CanFrame,
}

impl SyncProducer {
    pub fn new(parent: Arc<dyn Parent>) -> Result<Self, CanFrameError> {
        let can_frame = CanFrame::new(ID_SYNC, None)?;
        Ok(SyncProducer {
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

        let timer = Timer::new(period, Arc::new(move || {
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

// SyncConsumer struct to track received sync frames
pub struct SyncConsumer {
    last_received: Arc<Mutex<Option<Instant>>>,
    timeout: Duration,
}

impl SyncConsumer {
    pub fn new(timeout: Duration) -> Self {
        SyncConsumer {
            last_received: Arc::new(Mutex::new(None)),
            timeout,
        }
    }

    pub fn receive_sync(&self) {
        let mut last_received = self.last_received.lock().unwrap();
        *last_received = Some(Instant::now());
    }

    pub fn check_timeout(&self) -> bool {
        let last_received = self.last_received.lock().unwrap();
        if let Some(last) = *last_received {
            return last.elapsed() > self.timeout;
        }
        true
    }
}