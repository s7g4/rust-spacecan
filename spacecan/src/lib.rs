#![no_std]

#[cfg(feature = "std")]
extern crate std;

pub mod primitives;
pub mod services;
pub mod transport;
pub mod parser;
pub mod protocol;

#[cfg(not(feature = "std"))]
pub mod panic_handler;
