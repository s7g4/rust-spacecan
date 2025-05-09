use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use super::can_frame::{CanFrame, CanFrameError}; // Import improved CanFrame
use crate::primitives::network::Parent;

const ID_HEARTBEAT: u32 = 0x700;

/// Timer struct to handle periodic heartbeat signals.
struct Timer {
    period: Duration,
    callback: Arc<dyn Fn() + Send + Sync>,
    running: Arc<Mutex<bool>>,
}

impl Timer {
    /// Creates a new timer with the specified period and callback.
    fn new(period: Duration, callback: Arc<dyn Fn() + Send + Sync>) -> Self {
        Timer {
            period,
            callback,
            running: Arc::new(Mutex::new(false)),
        }
    }

    /// Starts the timer, invoking the callback periodically.
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

    /// Stops the timer.
    fn stop(&self) {
        *self.running.lock().unwrap() = false;
    }
}

/// HeartbeatProducer struct to send periodic heartbeats.
pub struct HeartbeatProducer {
    parent: Arc<dyn Parent>,
    running: bool,
    timer: Option<Timer>,
    period: Option<Duration>,
    can_frame: CanFrame,
}

impl HeartbeatProducer {
    /// Creates a new HeartbeatProducer.
    pub fn new(parent: Arc<dyn Parent>) -> Result<Self, CanFrameError> {
        let can_frame = CanFrame::new(ID_HEARTBEAT, None)?;
        Ok(HeartbeatProducer {
            parent,
            running: false,
            timer: None,
            period: None,
            can_frame,
        })
    }

    /// Sends a heartbeat CAN frame.
    pub fn send(&self) -> Result<(), CanFrameError> {
        self.parent.send(&self.can_frame)
    }

    /// Starts sending heartbeats periodically with the given period.
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

    /// Stops sending heartbeats.
    pub fn stop(&mut self) {
        self.running = false;
        if let Some(timer) = self.timer.take() {
            timer.stop();
        }
    }
}

/// HeartbeatConsumer struct to monitor received heartbeats.
pub struct HeartbeatConsumer {
    last_received: Arc<Mutex<Option<std::time::Instant>>>,
    timeout: Duration,
}

impl HeartbeatConsumer {
    /// Creates a new HeartbeatConsumer with the specified timeout.
    pub fn new(timeout: Duration) -> Self {
        HeartbeatConsumer {
            last_received: Arc::new(Mutex::new(None)),
            timeout,
        }
    }

    /// Records the receipt of a heartbeat.
    pub fn receive_heartbeat(&self) {
        let mut last_received = self.last_received.lock().unwrap();
        *last_received = Some(std::time::Instant::now());
    }

    /// Checks if the heartbeat has timed out.
    pub fn check_timeout(&self) -> bool {
        let last_received = self.last_received.lock().unwrap();
        if let Some(last) = *last_received {
            return last.elapsed() > self.timeout;
        }
        true
    }
}

/// Heartbeat struct for main.rs usage.
pub struct Heartbeat {
    pub uptime: u32,
    pub status: u8,
}

impl Heartbeat {
    /// Converts the Heartbeat struct to a payload byte vector.
    pub fn to_payload(&self) -> Vec<u8> {
        let mut payload = Vec::new();
        payload.extend(&self.uptime.to_be_bytes());
        payload.push(self.status);
        payload
    }
}
