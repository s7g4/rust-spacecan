use stm32g0xx_hal::{
    pac,
    prelude::*,
    gpio::GpioExt,
    rcc::RccExt,
};
use bxcan::{Can, Frame, Id, StandardId};
use fugit::ExtU32;

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
        bxcan::Fifo::Fifo0,
        bxcan::filter::BankConfig::Mask32(bxcan::filter::Mask32::accept_all()),
    );

    can.enable_interrupts();
    can.enable();

    let frame = Frame::new_data(
        Id::Standard(StandardId::new(0x700).unwrap()),
        &[0x01, 0x02, 0x03, 0x04],
    );

    can.transmit(&frame).unwrap();
}
