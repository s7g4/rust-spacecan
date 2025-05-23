

extern crate alloc;

use alloc::sync::Arc;
#[cfg(feature = "std")]
use std::sync::Mutex;
#[cfg(not(feature = "std"))]
use cortex_m::interrupt::Mutex;

use core::time::Duration;
#[cfg(feature = "std")]
use std::thread;
#[cfg(feature = "std")]
use std::time::Instant;

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
    #[cfg(feature = "std")]
    fn start(&self) {
        let running = Arc::clone(&self.running);
        let callback = Arc::clone(&self.callback);
        let period = self.period;
        *running.lock().unwrap() = true;

        std::thread::spawn(move || {
            while *running.lock().unwrap() {
                std::thread::sleep(period);
                callback();
            }
        });
    }

    #[cfg(not(feature = "std"))]
    fn start(&self) {
        // No-op or alternative implementation for no_std
    }

    /// Stops the timer.
    #[cfg(feature = "std")]
    fn stop(&self) {
        *self.running.lock().unwrap() = false;
    }

    #[cfg(not(feature = "std"))]
    fn stop(&self) {
        // no_std alternative implementation if needed
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
    #[cfg(feature = "std")]
    last_received: Arc<Mutex<Option<Instant>>>,
    #[cfg(not(feature = "std"))]
    last_received: Arc<Mutex<Option<()>>>,
    timeout: Duration,
}

impl HeartbeatConsumer {
    /// Creates a new HeartbeatConsumer with the specified timeout.
    pub fn new(timeout: Duration) -> Self {
        HeartbeatConsumer {
            #[cfg(feature = "std")]
            last_received: Arc::new(Mutex::new(None)),
            #[cfg(not(feature = "std"))]
            last_received: Arc::new(Mutex::new(None)),
            timeout,
        }
    }

    /// Records the receipt of a heartbeat.
    #[cfg(feature = "std")]
    pub fn receive_heartbeat(&self) {
        let mut last_received = self.last_received.lock().unwrap();
        *last_received = Some(Instant::now());
    }

    #[cfg(not(feature = "std"))]
    pub fn receive_heartbeat(&self) {
        // No-op or alternative implementation for no_std
    }

    /// Checks if the heartbeat has timed out.
    #[cfg(feature = "std")]
    pub fn check_timeout(&self) -> bool {
        let last_received = self.last_received.lock().unwrap();
        if let Some(last) = *last_received {
            return last.elapsed() > self.timeout;
        }
        true
    }

    #[cfg(not(feature = "std"))]
    pub fn check_timeout(&self) -> bool {
        // Always timeout in no_std or implement alternative logic
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
    pub fn to_payload(&self) -> alloc::vec::Vec<u8> {
        let mut payload = alloc::vec::Vec::new();
        payload.extend(&self.uptime.to_be_bytes());
        payload.push(self.status);
        payload
    }
}
