#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;

use stm32f7xx_hal::{
    pac,
    prelude::*,
    gpio::{Alternate},
    can::Can as HalCan,
};
use bxcan::{Can, filter::{Mask32, BankConfig}, Fifo};
use fugit::HertzU32;

use spacecan::protocol::{SpaceCAN, SpaceCANFrame};

#[entry]
fn main() -> ! {
    // Get access to the device-specific peripherals
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::peripheral::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.constrain(); // Make rcc mutable
    let clocks = rcc.cfgr.sysclk(HertzU32::from_raw(216_000_000)).freeze();

    let gpio = dp.GPIOB.split(); // CAN is on PB8 (RX), PB9 (TX) for F767
    let can_rx = gpio.pb8.into_alternate::<9>();
    let can_tx = gpio.pb9.into_alternate::<9>();

    // Initialize the CAN peripheral using stm32f7xx-hal
    let mut hal_can = HalCan::new(dp.CAN1, &mut rcc.apb1, (can_tx, can_rx));

    // Enable the CAN clock
    let mut can = bxcan::Can::builder(hal_can)
        .set_bit_timing(0x001c_0000) // Example bit timing configuration
        .enable();

    // Configure CAN filters
    can.modify_filters().enable_bank(
        0,
        Fifo::Fifo0,
        BankConfig::Mask32(Mask32::accept_all()),
    );

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
