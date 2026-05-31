#![no_std]
#![no_main]

use cortex_m_rt::entry;
#[cfg(not(test))]
use panic_halt as _;

#[entry]
#[allow(clippy::empty_loop)]
fn main() -> ! {
    // TODO: init bxCAN, register ISR, enter event loop
    loop {}
}
