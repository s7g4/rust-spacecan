use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

// Timer struct to handle periodic execution
pub struct Timer {
    period: Duration,
    callback: Box<dyn Fn() + Send + Sync>,
    running: Arc<Mutex<bool>>,
    last_execution: Arc<Mutex<Option<Instant>>>,
}

impl Timer {
    pub fn new(period: Duration, callback: Box<dyn Fn() + Send + Sync>) -> Self {
        Timer {
            period,
            callback,
            running: Arc::new(Mutex::new(false)),
            last_execution: Arc::new(Mutex::new(None)),
        }
    }

    pub fn start(&self) {
        let running = Arc::clone(&self.running);
        let callback = self.callback.clone();
        let last_execution = Arc::clone(&self.last_execution);
        *running.lock().unwrap() = true;

        thread::spawn(move || {
            while *running.lock().unwrap() {
                thread::sleep(self.period);
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
