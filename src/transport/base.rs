use crate::can_frame::{CanFrame, CanFrameError};
use std::sync::{Arc, Mutex};

// Define a trait for Bus operations
pub trait Bus {
    fn flush_frame_buffer(&self);
    fn start_receive(&self);
    fn stop_receive(&self);
    fn send(&self, can_frame: &CanFrame) -> Result<(), CanFrameError>;
    fn get_frame(&self) -> Option<CanFrame>;
}

// Implementation of a basic Bus
pub struct BusImpl {
    buffer: Arc<Mutex<Vec<CanFrame>>>,
}

impl BusImpl {
    pub fn new() -> Self {
        BusImpl {
            buffer: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl Bus for BusImpl {
    fn flush_frame_buffer(&self) {
        let mut buffer = self.buffer.lock().unwrap();
        buffer.clear();
    }
    
    fn start_receive(&self) {
        println!("Bus receiving started.");
    }
    
    fn stop_receive(&self) {
        println!("Bus receiving stopped.");
    }
    
    fn send(&self, can_frame: &CanFrame) -> Result<(), CanFrameError> {
        let mut buffer = self.buffer.lock().unwrap();
        buffer.push(can_frame.clone());
        println!("Sent CAN Frame: {:?}", can_frame);
        Ok(())
    }
    
    fn get_frame(&self) -> Option<CanFrame> {
        let mut buffer = self.buffer.lock().unwrap();
        buffer.pop()
    }
}