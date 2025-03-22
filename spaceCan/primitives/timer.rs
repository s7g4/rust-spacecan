use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

struct Timer {
    period: Duration,
    callback: Arc<dyn Fn() + Send + Sync>, // Callback function
    running: Arc<Mutex<bool>>, // Shared state to control the timer
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
        
        // Stop the timer if it's already running
        if *running.lock().unwrap() {
            self.stop();
        }

        // Set the running state to true
        *running.lock().unwrap() = true;

        // Spawn a new thread to handle the timer
        thread::spawn(move || {
            while *running.lock().unwrap() {
                thread::sleep(self.period);
                if *running.lock().unwrap() {
                    callback(); // Call the callback function
                }
            }
        });
    }

    fn stop(&self) {
        *self.running.lock().unwrap() = false; // Set running state to false
    }
}

fn main() {
    // Example usage of the Timer
    let timer = Timer::new(Duration::from_secs(1), Arc::new(|| {
        println!("Timer expired!");
    }));

    timer.start();

    // Let the timer run for a few seconds
    thread::sleep(Duration::from_secs(5));

    timer.stop();
    println!("Timer stopped.");
}