extern crate alloc;

use cortex_m::interrupt::{Mutex, free as interrupt_free};
use alloc::vec::{self, Vec};
use alloc::string::{String, ToString};
use core::option::Option;
use core::option::Option::{Some, None};
use core::result::Result;
use core::result::Result::{Ok, Err};
use core::fmt::Write;
use core::borrow::Borrow;
use core::borrow::BorrowMut;
pub struct MockTransport {
    last_sent: Mutex<Option<Vec<u8>>>,
}

impl MockTransport {
    pub fn new() -> Self {
        MockTransport {
            last_sent: Mutex::new(None),
        }
    }

    pub fn send(&self, data: &[u8]) {
        // Mock send implementation
        interrupt_free(|cs| {
            let mut binding = self.last_sent.borrow(cs);
            let mut last_sent = binding.borrow_mut();
            *last_sent = &Some(data.to_vec());
        });
    }

    pub fn receive(&self) -> Result<Vec<u8>, String> {
        // Mock receive implementation returning last sent data
        interrupt_free(|cs| {
            let last_sent = self.last_sent.borrow(cs);
            match &*last_sent {
                Some(data) => Ok(data.clone()),
                None => Err("No data available".to_string()),
            }
        })
    }
}
