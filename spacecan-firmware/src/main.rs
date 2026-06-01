#![no_std]
#![no_main]

use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
#[cfg(not(test))]
use panic_halt as _;
use stm32f4xx_hal::{can::Can, pac, prelude::*, watchdog::IndependentWatchdog};
use bxcan::{Fifo, Frame, Id, StandardId, filter::Mask32};
use spacecan::{
    primitives::can_frame::{CanFrame, CanFrameError},
    protocol::SpaceCANProtocol,
    transport::base::Bus,
};
use systick_monotonic::{fugit::Duration, Systick};

pub struct HardwareBus {
    can: Mutex<RefCell<bxcan::Can<Can<pac::CAN1>>>>,
}

impl HardwareBus {
    pub fn new(can: bxcan::Can<Can<pac::CAN1>>) -> Self {
        Self {
            can: Mutex::new(RefCell::new(can)),
        }
    }
}

impl Bus for HardwareBus {
    fn flush_frame_buffer(&self) {}
    fn start_receive(&self) {}
    fn stop_receive(&self) {}

    fn send(&self, can_frame: &CanFrame) -> Result<(), CanFrameError> {
        let id = Id::Standard(StandardId::new(can_frame.can_id() as u16).unwrap());
        let data = bxcan::Data::new(can_frame.data()).unwrap();
        let frame = Frame::new_data(id, data);
        cortex_m::interrupt::free(|cs| {
            let mut can = self.can.borrow(cs).borrow_mut();
            can.transmit(&frame).map_err(|_| CanFrameError::SendFailed)?;
            Ok(())
        })
    }

    fn get_frame(&self) -> Option<CanFrame> {
        cortex_m::interrupt::free(|cs| {
            let mut can = self.can.borrow(cs).borrow_mut();
            match can.receive() {
                Ok(frame) => {
                    if let Some(data) = frame.data() {
                        let id = match frame.id() {
                            Id::Standard(sid) => sid.as_raw() as u32,
                            Id::Extended(eid) => eid.as_raw(),
                        };
                        CanFrame::new(id, Some(spacecan::FrameData::from_slice(data.as_ref()).unwrap())).ok()
                    } else {
                        None
                    }
                }
                Err(_) => None,
            }
        })
    }
}

#[rtic::app(device = stm32f4xx_hal::pac, peripherals = true, dispatchers = [EXTI0])]
mod app {
    use super::*;

    #[shared]
    struct Shared {
        protocol: SpaceCANProtocol<HardwareBus>,
    }

    #[local]
    struct Local {
        watchdog: IndependentWatchdog,
    }

    #[monotonic(binds = SysTick, default = true)]
    type MonoTimer = Systick<1000>; // 1 kHz / 1ms

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        let dp = cx.device;
        let rcc = dp.RCC.constrain();
        let _clocks = rcc
            .cfgr
            .sysclk(168.MHz())
            .pclk1(42.MHz())
            .pclk2(84.MHz())
            .freeze();

        let mut watchdog = IndependentWatchdog::new(dp.IWDG);
        watchdog.start(2000u32.millis());

        let gpiob = dp.GPIOB.split();
        let rx = gpiob.pb8.into_alternate::<9>();
        let tx = gpiob.pb9.into_alternate::<9>();

        let mut can = bxcan::Can::builder(Can::new(dp.CAN1, (tx, rx)))
            .set_bit_timing(0x001c_0003)
            .enable();

        let mut filters = can.modify_filters();
        filters.enable_bank(0, Fifo::Fifo0, Mask32::accept_all());
        drop(filters);

        can.enable_interrupts(
            bxcan::Interrupts::FIFO0_MESSAGE_PENDING | bxcan::Interrupts::FIFO1_MESSAGE_PENDING,
        );

        let bus = HardwareBus::new(can);
        let protocol = SpaceCANProtocol::new(bus, 1);

        let mono = Systick::new(cx.core.SYST, 168_000_000);

        pet_watchdog::spawn().unwrap();

        (Shared { protocol }, Local { watchdog }, init::Monotonics(mono))
    }

    #[task(binds = CAN1_RX0, shared = [protocol])]
    fn can_rx(mut cx: can_rx::Context) {
        cx.shared.protocol.lock(|p| {
            while let Ok(Some(frame)) = p.receive_frame() {
                let _ = p.send_frame(&frame);
            }
        });
    }

    #[task(local = [watchdog])]
    fn pet_watchdog(cx: pet_watchdog::Context) {
        cx.local.watchdog.feed();
        pet_watchdog::spawn_after(Duration::<u64, 1, 1000>::from_ticks(1000)).unwrap();
    }
}
