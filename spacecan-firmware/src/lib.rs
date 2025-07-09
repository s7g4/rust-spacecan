#![no_std]
#![no_main]

use cortex_m_rt::entry;
use stm32g0xx_hal::{
    pac,
    prelude::*,
    rcc::RccExt, // for sysclk()
    time::U32Ext, // for mhz()
    gpio::GpioExt, // for split()
};
use spacecan::protocol::{SpaceCANProtocol, SpaceCANFrame};

pub mod panic_handler;

#[entry]
fn main() -> ! {
    // Get access to the device-specific peripherals
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::peripheral::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(64.mhz()).freeze();

    let gpio = dp.GPIOB.split(&mut rcc);
    let can_rx = gpio.pb8.into_alternate::<9>();
    let can_tx = gpio.pb9.into_alternate::<9>();

    // Initialize the CAN peripheral using bxcan directly with raw CAN peripheral
    // Note: STM32G0 might not have CAN, using a mock for now
    // let can_peripheral = dp.CAN;

    // Enable the CAN clock - handled by bxcan or HAL internally if needed

    // For STM32G0 which doesn't have CAN, we'll use a mock transport
    // In a real implementation, you'd use the actual CAN peripheral
    use spacecan::transport::mock::MockTransport;
    
    let mut transport = MockTransport::new();
    let mut spacecan = SpaceCANProtocol::new(transport, 1); // node_id = 1

    // Send a test packet (Command ID: 0x01, 4 bytes payload)
    let frame = SpaceCANFrame::new(0x01, 1, 1, 1, [1, 2, 3, 4].to_vec()).unwrap();
    spacecan.send_frame(&frame).unwrap();

    loop {
        if let Ok(Some(_frame)) = spacecan.receive_frame() {
            // Do something with received SpaceCANFrame
            defmt::println!("Received frame: {:?}", _frame);
        }
    }
}
