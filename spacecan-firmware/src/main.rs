#![no_std]
#![no_main]

extern crate alloc;

use bxcan::{Fifo, Frame, Id, StandardId, filter::Mask32};
use core::cell::RefCell;
use cortex_m::interrupt::{Mutex, free};
use cortex_m_rt::entry;
use linked_list_allocator::LockedHeap;
#[cfg(not(test))]
use panic_halt as _;
use spacecan::{
    primitives::can_frame::{CanFrame, CanFrameError},
    protocol::SpaceCANProtocol,
    transport::base::Bus,
};
use stm32f4xx_hal::{can::Can, pac, prelude::*};

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

const HEAP_SIZE: usize = 1024 * 32; // 32 KB

// Hardware Bus Wrapper for SpaceCAN
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
        free(|cs| {
            let mut can = self.can.borrow(cs).borrow_mut();
            can.transmit(&frame)
                .map_err(|_| CanFrameError::SendFailed)?;
            Ok(())
        })
    }

    fn get_frame(&self) -> Option<CanFrame> {
        free(|cs| {
            let mut can = self.can.borrow(cs).borrow_mut();
            match can.receive() {
                Ok(frame) => {
                    if let Some(data) = frame.data() {
                        let id = match frame.id() {
                            Id::Standard(sid) => sid.as_raw() as u32,
                            Id::Extended(eid) => eid.as_raw(),
                        };
                        CanFrame::new(id, Some(data.to_vec())).ok()
                    } else {
                        None
                    }
                }
                Err(_) => None, // WouldBlock
            }
        })
    }
}

#[entry]
fn main() -> ! {
    // Initialize heap allocator
    unsafe {
        static mut HEAP: [core::mem::MaybeUninit<u8>; HEAP_SIZE] =
            [core::mem::MaybeUninit::uninit(); HEAP_SIZE];
        let heap_ptr = core::ptr::addr_of_mut!(HEAP) as *mut u8;
        ALLOCATOR.lock().init(heap_ptr, HEAP_SIZE);
    }

    let dp = pac::Peripherals::take().unwrap();
    let _cp = cortex_m::Peripherals::take().unwrap();

    // Configure clocks
    let rcc = dp.RCC.constrain();
    let _clocks = rcc
        .cfgr
        .sysclk(168.MHz())
        .pclk1(42.MHz())
        .pclk2(84.MHz())
        .freeze();

    // Configure CAN GPIOs
    let gpiob = dp.GPIOB.split();
    // PB8: CAN1_RX, PB9: CAN1_TX
    let rx = gpiob.pb8.into_alternate::<9>();
    let tx = gpiob.pb9.into_alternate::<9>();

    // Initialize bxCAN
    let mut can = bxcan::Can::builder(Can::new(dp.CAN1, (tx, rx)))
        .set_bit_timing(0x001c_0003) // 500kbps @ 42MHz PCLK1
        .enable();

    // Set up hardware filter to accept all standard frames
    let mut filters = can.modify_filters();
    filters.enable_bank(0, Fifo::Fifo0, Mask32::accept_all());
    drop(filters);

    let bus = HardwareBus::new(can);
    let mut protocol = SpaceCANProtocol::new(bus, 1); // Node ID = 1

    loop {
        if let Ok(Some(frame)) = protocol.receive_frame() {
            // Echo frame back or route to ServiceManager
            let _ = protocol.send_frame(&frame);
        }
    }
}
