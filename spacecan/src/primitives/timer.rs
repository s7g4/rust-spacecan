
#[cfg(feature = "std")]
use std::sync::{Arc, Mutex};
#[cfg(feature = "std")]
use std::time::{Duration, Instant};
#[cfg(feature = "std")]
use std::thread;

#[cfg(not(feature = "std"))]
use core::time::Duration;

#[cfg(feature = "std")]
pub struct Timer {
    period: Duration,
    callback: Arc<dyn Fn() + Send + Sync>,
    running: Arc<Mutex<bool>>,
    last_execution: Arc<Mutex<Option<Instant>>>,
}

#[cfg(feature = "std")]
impl Timer {
    pub fn new(period: Duration, callback: Arc<dyn Fn() + Send + Sync>) -> Self {
        Timer {
            period,
            callback,
            running: Arc::new(Mutex::new(false)),
            last_execution: Arc::new(Mutex::new(None)),
        }
    }

    pub fn start(&self) {
        let running = Arc::clone(&self.running);
        let callback = Arc::clone(&self.callback);
        let last_execution = Arc::clone(&self.last_execution);
        let period = self.period;
        *running.lock().unwrap() = true;

        thread::spawn(move || {
            while *running.lock().unwrap() {
                thread::sleep(period);
                callback();
                *last_execution.lock().unwrap() = Some(Instant::now());
            }
        });
    }

    pub fn stop(&self) {
        *self.running.lock().unwrap() = false;
    }

    pub fn is_running(&self) -> bool {
        *self.running.lock().unwrap()
    }

    pub fn last_execution_time(&self) -> Option<Instant> {
        *self.last_execution.lock().unwrap()
    }
}

#[cfg(not(feature = "std"))]
pub struct Timer {
    period: Duration,
    callback: fn(),
    running: bool,
    last_execution: Option<u64>, // placeholder for timestamp in millis or ticks
}

#[cfg(not(feature = "std"))]
impl Timer {
    pub fn new(period: Duration, callback: fn()) -> Self {
        Timer {
            period,
            callback,
            running: false,
            last_execution: None,
        }
    }

    pub fn start(&mut self) {
        self.running = true;
        // In no_std, no thread or sleep, user must call poll() periodically
    }

    pub fn stop(&mut self) {
        self.running = false;
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn last_execution_time(&self) -> Option<u64> {
        self.last_execution
    }

    // User must call this periodically to check timer and call callback
    pub fn poll(&mut self, current_time: u64) {
        if self.running {
            if let Some(last) = self.last_execution {
                if current_time - last >= self.period.as_millis() as u64 {
                    (self.callback)();
                    self.last_execution = Some(current_time);
                }
            } else {
                (self.callback)();
                self.last_execution = Some(current_time);
            }
        }
    }
}
