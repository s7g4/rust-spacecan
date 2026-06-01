#[cfg(feature = "std")]
use crate::primitives::can_frame::{CanFrame, CanFrameError};
#[cfg(feature = "std")]
use crate::transport::base::Bus;
#[cfg(feature = "std")]
use heapless::Deque;

#[cfg(feature = "std")]
pub struct MockBus {
    pub sent_frames: std::sync::Mutex<heapless::Vec<CanFrame, 32>>,
    pub receive_queue: std::sync::Mutex<Deque<CanFrame, 32>>,
}

#[cfg(feature = "std")]
impl Default for MockBus {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "std")]
impl MockBus {
    pub fn new() -> Self {
        MockBus {
            sent_frames: std::sync::Mutex::new(heapless::Vec::new()),
            receive_queue: std::sync::Mutex::new(Deque::new()),
        }
    }

    pub fn push_receive_frame(&self, frame: CanFrame) {
        let _ = self.receive_queue.lock().unwrap().push_back(frame);
    }
}

#[cfg(feature = "std")]
impl Bus for MockBus {
    fn send(&self, frame: &CanFrame) -> Result<(), CanFrameError> {
        let _ = self.sent_frames.lock().unwrap().push(frame.clone());
        Ok(())
    }

    fn get_frame(&self) -> Option<CanFrame> {
        self.receive_queue.lock().unwrap().pop_front()
    }

    fn start_receive(&self) {}
    fn stop_receive(&self) {}
    fn flush_frame_buffer(&self) {
        self.receive_queue.lock().unwrap().clear();
    }
}
