extern crate alloc;

#[cfg(not(feature = "embedded"))]
extern crate std;

use crate::primitives::can_frame::{CanFrame, CanFrameError};
use alloc::collections::VecDeque;
#[cfg(feature = "embedded")]
use core::cell::RefCell;
#[cfg(feature = "embedded")]
use cortex_m::interrupt::{Mutex, free as interrupt_free};

#[cfg(feature = "embedded")]
type MutexType<T> = cortex_m::interrupt::Mutex<RefCell<T>>;

#[cfg(not(feature = "embedded"))]
type MutexType<T> = std::sync::Mutex<T>;

/// Fixed-capacity ring buffer for CAN frames.
pub struct FrameBuffer {
    buffer: MutexType<VecDeque<CanFrame>>,
    capacity: usize,
}

impl FrameBuffer {
    pub fn new(capacity: usize) -> Self {
        FrameBuffer {
            #[cfg(feature = "embedded")]
            buffer: cortex_m::interrupt::Mutex::new(RefCell::new(VecDeque::with_capacity(
                capacity,
            ))),
            #[cfg(not(feature = "embedded"))]
            buffer: std::sync::Mutex::new(VecDeque::with_capacity(capacity)),
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
            if let Ok(mut guard) = self.buffer.lock() {
                if guard.len() >= self.capacity {
                    return Err(CanFrameError::SendFailed);
                }
                guard.push_back(frame);
                Ok(())
            } else {
                Err(CanFrameError::SendFailed)
            }
        }
    }

    /// Dequeues a frame. Returns None if empty.
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
            if let Ok(mut guard) = self.buffer.lock() {
                guard.pop_front()
            } else {
                None
            }
        }
    }

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
            if let Ok(guard) = self.buffer.lock() {
                guard.len()
            } else {
                0
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn is_full(&self) -> bool {
        self.len() >= self.capacity
    }

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
            if let Ok(mut guard) = self.buffer.lock() {
                guard.clear();
            }
        }
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }
}
