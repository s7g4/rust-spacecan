use crate::primitives::can_frame::{CanFrame, CanFrameError};
extern crate alloc;

#[cfg(not(feature = "embedded"))]
extern crate std;

use alloc::vec::Vec;
#[cfg(feature = "embedded")]
use core::cell::RefCell;
use core::option::Option;
use core::option::Option::{None, Some};
use core::result::Result;
use core::result::Result::{Err, Ok};
#[cfg(feature = "embedded")]
use cortex_m::interrupt::{Mutex, free as interrupt_free};

#[cfg(not(feature = "embedded"))]
use std::sync::Mutex;

// Define a trait for Bus operations
pub trait Bus {
    fn flush_frame_buffer(&self);
    fn start_receive(&self);
    fn stop_receive(&self);
    fn send(&self, can_frame: &CanFrame) -> Result<(), CanFrameError>;
    fn get_frame(&self) -> Option<CanFrame>;
}

// Basic Bus implementation using platform-specific Mutex
pub struct BusImpl {
    #[cfg(feature = "embedded")]
    buffer: cortex_m::interrupt::Mutex<core::cell::RefCell<Vec<CanFrame>>>,
    #[cfg(not(feature = "embedded"))]
    buffer: std::sync::Mutex<Vec<CanFrame>>,
}

impl BusImpl {
    pub fn new() -> Self {
        BusImpl {
            #[cfg(feature = "embedded")]
            buffer: cortex_m::interrupt::Mutex::new(core::cell::RefCell::new(Vec::new())),
            #[cfg(not(feature = "embedded"))]
            buffer: std::sync::Mutex::new(Vec::new()),
        }
    }
}

impl Bus for BusImpl {
    fn flush_frame_buffer(&self) {
        #[cfg(feature = "embedded")]
        {
            interrupt_free(|cs| {
                self.buffer.borrow(cs).borrow_mut().clear();
            });
        }
        #[cfg(not(feature = "embedded"))]
        {
            if let Ok(mut guard) = self.buffer.lock() {
                guard.clear();
            }
        }
    }

    fn start_receive(&self) {}

    fn stop_receive(&self) {}

    fn send(&self, can_frame: &CanFrame) -> Result<(), CanFrameError> {
        #[cfg(feature = "embedded")]
        {
            interrupt_free(|cs| {
                self.buffer.borrow(cs).borrow_mut().push(can_frame.clone());
            });
        }
        #[cfg(not(feature = "embedded"))]
        {
            if let Ok(mut guard) = self.buffer.lock() {
                guard.push(can_frame.clone());
            }
        }
        Ok(())
    }

    fn get_frame(&self) -> Option<CanFrame> {
        #[cfg(feature = "embedded")]
        {
            interrupt_free(|cs| self.buffer.borrow(cs).borrow_mut().pop())
        }
        #[cfg(not(feature = "embedded"))]
        {
            if let Ok(mut guard) = self.buffer.lock() {
                guard.pop()
            } else {
                None
            }
        }
    }
}
