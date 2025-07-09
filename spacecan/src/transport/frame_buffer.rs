extern crate alloc;

use crate::primitives::can_frame::{CanFrame, CanFrameError};
use alloc::collections::VecDeque;
use core::cell::RefCell;

#[cfg(feature = "embedded")]
use cortex_m::interrupt::{Mutex, free as interrupt_free};

#[cfg(not(feature = "embedded"))]
fn interrupt_free<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    f()
}

#[cfg(feature = "embedded")]
type MutexType<T> = cortex_m::interrupt::Mutex<RefCell<T>>;

#[cfg(not(feature = "embedded"))]
type MutexType<T> = RefCell<T>;

/// Frame buffer for storing CAN frames with fixed size.
pub struct FrameBuffer {
    buffer: MutexType<VecDeque<CanFrame>>,
    capacity: usize,
}

impl FrameBuffer {
    /// Creates a new frame buffer with the given capacity.
    pub fn new(capacity: usize) -> Self {
        FrameBuffer {
            #[cfg(feature = "embedded")]
            buffer: cortex_m::interrupt::Mutex::new(RefCell::new(VecDeque::with_capacity(capacity))),
            #[cfg(not(feature = "embedded"))]
            buffer: RefCell::new(VecDeque::with_capacity(capacity)),
            capacity,
        }
    }

    /// Adds a frame to the buffer. Returns an error if the buffer is full.
    pub fn push(&self, frame: CanFrame) -> Result<(), CanFrameError> {
        #[cfg(feature = "embedded")]
        {
            interrupt_free(|cs| {
                let mut buffer = self.buffer.borrow(cs).borrow_mut();
                if buffer.len() >= self.capacity {
                    return Err(CanFrameError::SendFailed);
                }
                buffer.push_back(frame);
                Ok(())
            })
        }
        #[cfg(not(feature = "embedded"))]
        {
            interrupt_free(|| {
                let mut buffer = self.buffer.borrow_mut();
                if buffer.len() >= self.capacity {
                    return Err(CanFrameError::SendFailed);
                }
                buffer.push_back(frame);
                Ok(())
            })
        }
    }

    /// Removes and returns a frame from the buffer. Returns None if buffer is empty.
    pub fn pop(&self) -> Option<CanFrame> {
        #[cfg(feature = "embedded")]
        {
            interrupt_free(|cs| {
                let mut buffer = self.buffer.borrow(cs).borrow_mut();
                buffer.pop_front()
            })
        }
        #[cfg(not(feature = "embedded"))]
        {
            interrupt_free(|| {
                let mut buffer = self.buffer.borrow_mut();
                buffer.pop_front()
            })
        }
    }

    /// Returns the number of frames currently in the buffer.
    pub fn len(&self) -> usize {
        #[cfg(feature = "embedded")]
        {
            interrupt_free(|cs| {
                let buffer = self.buffer.borrow(cs).borrow();
                buffer.len()
            })
        }
        #[cfg(not(feature = "embedded"))]
        {
            interrupt_free(|| {
                let buffer = self.buffer.borrow();
                buffer.len()
            })
        }
    }

    /// Returns true if the buffer is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns true if the buffer is full.
    pub fn is_full(&self) -> bool {
        self.len() >= self.capacity
    }

    /// Clears all frames from the buffer.
    pub fn clear(&self) {
        #[cfg(feature = "embedded")]
        {
            interrupt_free(|cs| {
                let mut buffer = self.buffer.borrow(cs).borrow_mut();
                buffer.clear();
            })
        }
        #[cfg(not(feature = "embedded"))]
        {
            interrupt_free(|| {
                let mut buffer = self.buffer.borrow_mut();
                buffer.clear();
            })
        }
    }

    /// Returns the capacity of the buffer.
    pub fn capacity(&self) -> usize {
        self.capacity
    }
}
