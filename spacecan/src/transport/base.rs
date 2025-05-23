use crate::primitives::can_frame::{CanFrame, CanFrameError};
extern crate alloc;

use alloc::sync::Arc;
use cortex_m::interrupt::{Mutex, free as interrupt_free};
use alloc::vec::Vec;
use core::option::Option;
use core::option::Option::{Some, None};
use core::result::Result;
use core::result::Result::{Ok, Err};
use core::cell::UnsafeCell;
use core::borrow::Borrow;

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
    buffer: UnsafeCell<Vec<CanFrame>>
}

impl BusImpl {
    pub fn new() -> Self {
        BusImpl {
            buffer: UnsafeCell::new(Vec::new()),
        }
    }
}

impl Bus for BusImpl {
    fn flush_frame_buffer(&self) {
        interrupt_free(|_cs| {
            let buffer = unsafe { &mut *self.buffer.get() }; // Use UnsafeCell::get
            buffer.clear();
        });
    }
    
    fn start_receive(&self) {
        // Removed println for no_std compatibility
    }
    
    fn stop_receive(&self) {
        // Removed println for no_std compatibility
    }
    
    fn send(&self, can_frame: &CanFrame) -> Result<(), CanFrameError> {
        interrupt_free(|_cs| {
            let buffer = unsafe { &mut *self.buffer.get() }; // Use UnsafeCell::get
            buffer.push(can_frame.clone());
        });
        Ok(())
    }
    
    fn get_frame(&self) -> Option<CanFrame> {
        let mut frame = None;
        interrupt_free(|_cs| {
            let buffer = unsafe { &mut *self.buffer.get() }; // Use UnsafeCell::get
            frame = buffer.pop();
        });
        frame
    }
}
