"The Heartbeat service is needed for the redundancy management, to let the
controller node define the active bus (either nominal or redundant one) for
communication and to let the responder nodes know which is the active bus to
listen to. For this, controller node implements the HeartbeatProducer class
while responder nodes implement the HeartbeatConsumer class."
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

const TIMER_ID: u32 = 1;
const ID_HEARTBEAT: u32 = 0x700; // Assuming this is the same as in your previous code

#[derive(Debug)]
struct CanFrame {
    can_id: u32,
    data: Vec<u8>,
}

impl CanFrame {
    fn new(can_id: u32, data: Option<Vec<u8>>) -> Result<Self, String> {
        if let Some(data) = data {
            if data.len() > 8 {
                return Err("not more than 8 data bytes allowed".to_string());
            }
            Ok(CanFrame { can_id, data })
        } else {
            Ok(CanFrame {
                can_id,
                data: Vec::new(),
            })
        }
    }
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

struct HeartbeatProducer {
    parent: Arc<dyn Network>,
    running: bool,
    timer: Option<Timer>,
    period: Option<Duration>,
    can_frame: CanFrame,
}

impl HeartbeatProducer {
    fn new(parent: Arc<dyn Network>) -> Self {
        HeartbeatProducer {
            parent,
            running: false,
            timer: None,
            period: None,
            can_frame: CanFrame::new(ID_HEARTBEAT, None).unwrap(),
        }
    }

    fn send(&self) {
        self.parent.send(&self.can_frame);
        self.parent.sent_heartbeat();
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
            // In a real application, you might want to use a more sophisticated approach
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

struct HeartbeatConsumer {
    parent: Arc<dyn Network>,
    running: bool,
    timer: Option<Timer>,
    period: Option<Duration>,
    max_miss_heartbeat: Option<u32>,
    max_bus_switch: Option<u32>,
    heartbeats_missed: u32,
    bus_switches: u32,
}

impl HeartbeatConsumer {
    fn new(parent: Arc<dyn Network>) -> Self {
        HeartbeatConsumer {
            parent,
            running: false,
            timer: None,
            period: None,
            max_miss_heartbeat: None,
            max_bus_switch: None,
            heartbeats_missed: 0,
            bus_switches: 0,
        }
    }

    fn start(&mut self, period: Duration, max_miss_heartbeat: Option<u32>, max_bus_switch: Option<u32>) {
        self.period = Some(period);
        self.max_miss_heartbeat = max_miss_heartbeat;
        self.max_bus_switch = max_bus_switch;
        self.heartbeats_missed = 0;
        self.bus_switches = 0;
        self.running = true;

        let timer = Timer::new(period, || {
            // Call the timer expired function
            // This would need to