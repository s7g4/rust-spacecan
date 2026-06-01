#[cfg(feature = "std")]
use crate::primitives::can_frame::CanFrame;
#[cfg(feature = "std")]
use heapless::Deque;

#[cfg(feature = "std")]
pub struct FrameBuffer {
    buffer: std::sync::Mutex<Deque<CanFrame, 64>>,
}

#[cfg(feature = "std")]
impl Default for FrameBuffer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "std")]
impl FrameBuffer {
    pub fn new() -> Self {
        FrameBuffer {
            buffer: std::sync::Mutex::new(Deque::new()),
        }
    }

    pub fn push(&self, frame: CanFrame) -> Result<(), &'static str> {
        let mut buf = self.buffer.lock().unwrap();
        buf.push_back(frame).map_err(|_| "Buffer full")
    }

    pub fn pop(&self) -> Option<CanFrame> {
        self.buffer.lock().unwrap().pop_front()
    }
}
