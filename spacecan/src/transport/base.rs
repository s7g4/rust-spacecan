use crate::primitives::can_frame::CanFrame;
use heapless::Vec;

pub trait Bus {
    fn send(&self, frame: &CanFrame) -> Result<(), crate::primitives::can_frame::CanFrameError>;
    fn get_frame(&self) -> Option<CanFrame>;
    fn start_receive(&self);
    fn stop_receive(&self);
    fn flush_frame_buffer(&self);
}

#[cfg(feature = "std")]
pub struct BusImpl {
    buffer: std::sync::Mutex<Vec<CanFrame, 32>>,
}

#[cfg(feature = "std")]
impl BusImpl {
    pub fn new() -> Self {
        BusImpl {
            buffer: std::sync::Mutex::new(Vec::new()),
        }
    }
}

#[cfg(feature = "std")]
impl Bus for BusImpl {
    fn send(&self, frame: &CanFrame) -> Result<(), crate::primitives::can_frame::CanFrameError> {
        let mut buf = self.buffer.lock().unwrap();
        let _ = buf.push(frame.clone());
        Ok(())
    }
    fn get_frame(&self) -> Option<CanFrame> {
        let mut buf = self.buffer.lock().unwrap();
        if buf.is_empty() { None } else { Some(buf.remove(0)) }
    }
    fn start_receive(&self) {}
    fn stop_receive(&self) {}
    fn flush_frame_buffer(&self) {
        self.buffer.lock().unwrap().clear();
    }
}
