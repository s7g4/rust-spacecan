extern crate alloc;

#[cfg(not(feature = "embedded"))]
extern crate std;

use alloc::string::{String, ToString};
use alloc::vec::Vec;
#[cfg(feature = "embedded")]
use core::cell::RefCell;
use core::option::Option;
use core::option::Option::{None, Some};
use core::result::Result;
use core::result::Result::{Err, Ok};
#[cfg(feature = "embedded")]
use cortex_m::interrupt::{Mutex, free as interrupt_free};

#[cfg(feature = "embedded")]
type MutexType<T> = cortex_m::interrupt::Mutex<RefCell<T>>;

#[cfg(not(feature = "embedded"))]
type MutexType<T> = std::sync::Mutex<T>;

pub struct MockTransport {
    last_sent: MutexType<Option<Vec<u8>>>,
}

impl MockTransport {
    pub fn new() -> Self {
        MockTransport {
            #[cfg(feature = "embedded")]
            last_sent: cortex_m::interrupt::Mutex::new(RefCell::new(None)),
            #[cfg(not(feature = "embedded"))]
            last_sent: std::sync::Mutex::new(None),
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
            if let Ok(mut guard) = self.last_sent.lock() {
                *guard = None;
            }
        }
    }

    fn start_receive(&self) {}

    fn stop_receive(&self) {}

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
            if let Ok(mut guard) = self.last_sent.lock() {
                *guard = Some(data.to_vec());
            }
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
            if let Ok(guard) = self.last_sent.lock() {
                if let Some(data) = &*guard {
                    CanFrame::new(0, Some(data.clone())).ok()
                } else {
                    None
                }
            } else {
                None
            }
        }
    }
}
