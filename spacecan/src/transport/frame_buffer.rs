
extern crate alloc;

#[cfg(feature = "std")]
use std::thread;
#[cfg(feature = "std")]
use std::time::Duration;

use alloc::sync::{Arc};
#[cfg(feature = "std")]
use std::sync::Mutex;
#[cfg(not(feature = "std"))]
use cortex_m::interrupt::Mutex;

use alloc::vec;
