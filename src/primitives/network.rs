use std::sync::{Arc, Mutex};
use crate::can_frame::{CanFrame, CanFrameError}; // Import improved CanFrame

// Define a trait for Bus
pub trait Bus {
    fn flush_frame_buffer(&self);
    fn start_receive(&self);
    fn stop_receive(&self);
    fn send(&self, can_frame: &CanFrame) -> Result<(), CanFrameError>;
    fn get_frame(&self) -> Option<CanFrame>;
}

// Network struct with thread-safe bus switching
pub struct Network<T: Bus> {
    parent: Arc<dyn Parent>, // Assuming Parent is a trait that has the method received_frame
    node_id: u32,
    bus_a: T,
    bus_b: T,
    selected_bus: Arc<Mutex<T>>, // Ensures safe concurrent access
}

impl<T: Bus> Network<T> {
    pub fn new(parent: Arc<dyn Parent>, node_id: u32, bus_a: T, bus_b: T) -> Self {
        Network {
            parent,
            node_id,
            bus_a,
            bus_b,
            selected_bus: Arc::new(Mutex::new(bus_a)), // Start with bus_a
        }
    }

    pub fn start(&self) {
        let mut bus = self.selected_bus.lock().unwrap();
        bus.flush_frame_buffer();
        bus.start_receive();
    }

    pub fn stop(&self) {
        let mut bus = self.selected_bus.lock().unwrap();
        bus.flush_frame_buffer();
        bus.stop_receive();
    }

    // Process frames from the bus
    pub fn process(&self) {
        let mut bus = self.selected_bus.lock().unwrap();
        if let Some(can_frame) = bus.get_frame() {
            self.parent.received_frame(can_frame);
        }
    }

    pub fn send(&self, can_frame: &CanFrame) -> Result<(), CanFrameError> {
        let mut bus = self.selected_bus.lock().unwrap();
        bus.send(can_frame)
    }

    // Thread-safe bus switching
    pub fn switch_bus(&self) {
        let mut selected = self.selected_bus.lock().unwrap();
        if std::ptr::eq(&*selected, &self.bus_a) {
            *selected = self.bus_b;
        } else {
            *selected = self.bus_a;
        }
    }
}

// Assuming a Parent trait is defined somewhere
pub trait Parent {
    fn received_frame(&self, can_frame: CanFrame);
}
