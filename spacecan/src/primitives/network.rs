
extern crate alloc;

use alloc::sync::Arc;
#[cfg(feature = "std")]
use std::sync::Mutex;
#[cfg(not(feature = "std"))]
use cortex_m::interrupt::Mutex;

use super::can_frame::{CanFrame, CanFrameError}; // Import improved CanFrame
use core::ptr;

// Define a trait for Bus
pub trait Bus {
    fn flush_frame_buffer(&self);
    fn start_receive(&self);
    fn stop_receive(&self);
    fn send(&self, can_frame: &CanFrame) -> Result<(), CanFrameError>;
    fn get_frame(&self) -> Option<CanFrame>;
}

// Network struct with thread-safe bus switching

pub struct Network<T: Bus> {
    parent: Arc<dyn Parent>, // Assuming Parent is a trait that has the method received_frame
    node_id: u32,
    bus_a: T,
    bus_b: T,
    #[cfg(feature = "std")]
    selected_bus: Mutex<T>, // Ensures safe concurrent access
    #[cfg(not(feature = "std"))]
    selected_bus: cortex_m::interrupt::Mutex<core::cell::RefCell<T>>,
}

impl<T: Bus + Clone> Network<T> {
    pub fn new(parent: Arc<dyn Parent>, node_id: u32, bus_a: T, bus_b: T) -> Self {
        Network {
            parent,
            node_id,
            bus_a: bus_a.clone(),
            bus_b: bus_b.clone(),
            #[cfg(feature = "std")]
            selected_bus: Mutex::new(bus_a.clone()), // Start with bus_a clone
            #[cfg(not(feature = "std"))]
            selected_bus: cortex_m::interrupt::Mutex::new(core::cell::RefCell::new(bus_a.clone())),
        }
    }

    pub fn start(&self) {
        #[cfg(feature = "std")]
        {
            let mut bus = self.selected_bus.lock().unwrap();
            bus.flush_frame_buffer();
            bus.start_receive();
        }
        #[cfg(not(feature = "std"))]
        {
            cortex_m::interrupt::free(|cs| {
                let bus = self.selected_bus.borrow(cs).borrow_mut();
                bus.flush_frame_buffer();
                bus.start_receive();
            });
        }
    }

    pub fn stop(&self) {
        #[cfg(feature = "std")]
        {
            let mut bus = self.selected_bus.lock().unwrap();
            bus.flush_frame_buffer();
            bus.stop_receive();
        }
        #[cfg(not(feature = "std"))]
        {
            cortex_m::interrupt::free(|cs| {
                let bus = self.selected_bus.borrow(cs).borrow_mut();
                bus.flush_frame_buffer();
                bus.stop_receive();
            });
        }
    }

    // Process frames from the bus
    pub fn process(&self) {
        #[cfg(feature = "std")]
        {
            let mut bus = self.selected_bus.lock().unwrap();
            if let Some(can_frame) = bus.get_frame() {
                self.parent.received_frame(can_frame);
            }
        }
        #[cfg(not(feature = "std"))]
        {
            cortex_m::interrupt::free(|cs| {
                let bus = self.selected_bus.borrow(cs).borrow_mut();
                if let Some(can_frame) = bus.get_frame() {
                    self.parent.received_frame(can_frame);
                }
            });
        }
    }

    pub fn send(&self, can_frame: &CanFrame) -> Result<(), CanFrameError> {
        #[cfg(feature = "std")]
        {
            let mut bus = self.selected_bus.lock().unwrap();
            bus.send(can_frame)
        }
        #[cfg(not(feature = "std"))]
        {
            let mut result = Err(CanFrameError::SendFailed);
            cortex_m::interrupt::free(|cs| {
                let bus = self.selected_bus.borrow(cs).borrow_mut();
                result = bus.send(can_frame);
            });
            result
        }
    }

    // Thread-safe bus switching
    pub fn switch_bus(&self) {
        #[cfg(feature = "std")]
        {
            let mut selected = self.selected_bus.lock().unwrap();
            if ptr::eq(&*selected, &self.bus_a) {
                *selected = self.bus_b.clone();
            } else {
                *selected = self.bus_a.clone();
            }
        }
        #[cfg(not(feature = "std"))]
        {
            cortex_m::interrupt::free(|cs| {
                let mut selected = self.selected_bus.borrow(cs).borrow_mut();
                if ptr::eq(&*selected, &self.bus_a) {
                    *selected = self.bus_b.clone();
                } else {
                    *selected = self.bus_a.clone();
                }
            });
        }
    }
}

// Assuming a Parent trait is defined somewhere
pub trait Parent: Send + Sync {
    fn received_frame(&self, can_frame: CanFrame);
    fn send(&self, can_frame: &CanFrame) -> Result<(), CanFrameError>;
}
