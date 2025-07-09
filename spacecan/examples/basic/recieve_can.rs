use stm32g0xx_hal::{
    pac,
    prelude::*,
    gpio::GpioExt,
    rcc::RccExt,
};
use bxcan::{Can, Frame, Fifo};

fn main() {
    let dp = pac::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.constrain();
    let mut gpio = dp.GPIOB.split(&mut rcc);

    let clocks = rcc.cfgr.sysclk(64.mhz()).freeze();

    let can_rx = gpio.pb8.into_alternate::<6>();
    let can_tx = gpio.pb9.into_alternate::<6>();

    dp.RCC.apbenr1.modify(|_, w| w.canen().set_bit());

    let can_peripheral = dp.CAN;
    let mut can = bxcan::Can::builder(can_peripheral)
        .set_bit_timing(0x001c_0000)
        .leave_disabled();

    can.modify_filters().enable_bank(
        0,
        Fifo::Fifo0,
        bxcan::filter::BankConfig::Mask32(bxcan::filter::Mask32::accept_all()),
    );

    can.enable_interrupts();
    can.enable();

    loop {
        if let Ok(frame) = can.receive() {
            match frame.id() {
                bxcan::Id::Standard(id) => {
                    println!("Received frame with ID: {}", id.as_raw());
                }
                bxcan::Id::Extended(id) => {
                    println!("Received frame with extended ID: {}", id.as_raw());
                }
            }
        }
    }
}
