#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;

use stm32f7xx_hal::{
    pac,
    prelude::*,
    can::Can,
};

use spacecan::{SpaceCANFrame, SpaceCAN};

#[entry]
fn main() -> ! {
    // Get access to the device-specific peripherals
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::peripheral::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(216.mhz()).freeze();

    let gpio = dp.GPIOB.split(); // CAN is on PB8 (RX), PB9 (TX) for F767
    let can_rx = gpio.pb8.into_alternate();
    let can_tx = gpio.pb9.into_alternate();

    let mut can = Can::new(dp.CAN1, (can_tx, can_rx));
    can.modify_filters().enable_bank(0, |bank| {
        bank.enable().set_filter_id(0x123);
    });
    can.enable();

    let mut spacecan = SpaceCAN::new(can);

    // Send a test packet (Command ID: 0x01, 4 bytes payload)
    let frame = SpaceCANFrame::new(0x01, &[1, 2, 3, 4]).unwrap();
    spacecan.send_frame(&frame).unwrap();

    loop {
        if let Ok(frame) = spacecan.receive_frame() {
            // Do something with received SpaceCANFrame
        }
    }
}
