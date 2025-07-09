extern crate alloc;

use alloc::vec::Vec;
use alloc::string::{String, ToString};
use core::option::Option;
use core::option::Option::{Some, None};
use core::result::Result;
use core::result::Result::{Ok, Err};
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

pub struct MockTransport {
    last_sent: MutexType<Option<Vec<u8>>>,
}

impl MockTransport {
    pub fn new() -> Self {
        MockTransport {
            #[cfg(feature = "embedded")]
            last_sent: cortex_m::interrupt::Mutex::new(RefCell::new(None)),
            #[cfg(not(feature = "embedded"))]
            last_sent: RefCell::new(None),
        }
    }
}

use crate::primitives::can_frame::{CanFrame, CanFrameError};
use crate::transport::base::Bus;

impl Bus for MockTransport {
    fn flush_frame_buffer(&self) {
        #[cfg(feature = "embedded")]
        {
            interrupt_free(|cs| {
                let mut last_sent = self.last_sent.borrow(cs).borrow_mut();
                *last_sent = None;
            });
        }
        #[cfg(not(feature = "embedded"))]
        {
            interrupt_free(|| {
                let mut last_sent = self.last_sent.borrow_mut();
                *last_sent = None;
            });
        }
    }

    fn start_receive(&self) {
        // No operation needed for mock
    }

    fn stop_receive(&self) {
        // No operation needed for mock
    }

    fn send(&self, can_frame: &CanFrame) -> Result<(), CanFrameError> {
        let data = can_frame.data();
        #[cfg(feature = "embedded")]
        {
            interrupt_free(|cs| {
                let mut last_sent = self.last_sent.borrow(cs).borrow_mut();
                *last_sent = Some(data.to_vec());
            });
        }
        #[cfg(not(feature = "embedded"))]
        {
            interrupt_free(|| {
                let mut last_sent = self.last_sent.borrow_mut();
                *last_sent = Some(data.to_vec());
            });
        }
        Ok(())
    }

    fn get_frame(&self) -> Option<CanFrame> {
        #[cfg(feature = "embedded")]
        {
            interrupt_free(|cs| {
                let last_sent = self.last_sent.borrow(cs).borrow();
                if let Some(data) = &*last_sent {
                    CanFrame::new(0, Some(data.clone())).ok()
                } else {
                    None
                }
            })
        }
        #[cfg(not(feature = "embedded"))]
        {
            interrupt_free(|| {
                let last_sent = self.last_sent.borrow();
                if let Some(data) = &*last_sent {
                    CanFrame::new(0, Some(data.clone())).ok()
                } else {
                    None
                }
            })
        }
    }
}