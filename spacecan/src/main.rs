#![no_std]
#![no_main]

#[cfg(feature = "std")]
extern crate alloc;

#[cfg(all(feature = "std", not(test)))]
use std::println;
#[cfg(feature = "std")]
use spacecan::primitives::can_frame::CanFrame;
#[cfg(feature = "std")]
use spacecan::parser::{decode_frame, encode_frame};
#[cfg(feature = "std")]
use spacecan::primitives::heartbeat::Heartbeat;
#[cfg(feature = "std")]
use core::option::Option::Some;
#[cfg(feature = "std")]
use spacecan::transport::mock::MockTransport;
#[cfg(feature = "std")]
use core::alloc::Layout;
#[cfg(feature = "std")]
use linked_list_allocator::LockedHeap;
#[cfg(feature = "std")]
use core::panic::PanicInfo;

#[cfg(feature = "std")]
#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

#[cfg(not(feature = "std"))]
macro_rules! eprintln {
    ($($arg:tt)*) => {};
}

#[cfg(feature = "std")]
macro_rules! eprintln {
    ($($arg:tt)*) => {
        println!($($arg)*);
    };
}

/// # Safety
/// - `heap_start` must be a valid pointer to a memory region.
/// - The memory region starting at `heap_start` must be at least `heap_size` bytes long.
/// - The memory region must not overlap with other memory regions.
#[cfg(feature = "std")]
fn init_allocator(heap_start: usize, heap_size: usize) {
    unsafe {
        ALLOCATOR.lock().init(heap_start as *mut u8, heap_size);
    }
}

#[cfg(feature = "std")]
/// Entry point of the application demonstrating CAN frame encoding and decoding.
#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    // Initialize the allocator
    let heap_start = 0x2000_0000; // Replace with your actual heap start address
    let heap_size = 1024; // Replace with your desired heap size
    init_allocator(heap_start, heap_size);

    // Initialize a mock transport for sending and receiving CAN frames.
    let mut transport = MockTransport::new();

    // === 1. Heartbeat Frame ===
    let heartbeat = Heartbeat {
        uptime: 42,
        status: 0b00000001, // Example status flag indicating system state.
    };

    let hb_payload = heartbeat.to_payload();
    let hb_frame = CanFrame::new(0x700, Some(hb_payload))
        .expect("Failed to create CanFrame for heartbeat");

    let hb_encoded = encode_frame(&hb_frame).expect("Encoding heartbeat failed");
    eprintln!("Encoded Heartbeat Frame: {:?}", hb_encoded);

    let hb_decoded = decode_frame(&hb_encoded).expect("Decoding heartbeat failed");
    eprintln!("Decoded Heartbeat Frame: {:?}", hb_decoded);

    transport.send(&hb_encoded);
    eprintln!("Sent Heartbeat Frame via Mock Transport");

    let received = transport.receive().expect("No frame received");
    eprintln!("Received Frame from Mock Transport: {:?}", received);

    loop {}
}

#[cfg(not(feature = "std"))]
use core::panic::PanicInfo;

/// This function is called on panic.
#[cfg(not(feature = "std"))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
