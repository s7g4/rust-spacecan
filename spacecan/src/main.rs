#![no_std]
#![no_main]

extern crate alloc;

use spacecan::primitives::can_frame::CanFrame;
use spacecan::parser::{decode_frame, encode_frame};
use spacecan::primitives::heartbeat::Heartbeat;
use core::option::Option::Some;
use spacecan::transport::mock::MockTransport;
use core::alloc::Layout;
use linked_list_allocator::LockedHeap;
use core::panic::PanicInfo;

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

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    // Log the panic information if possible (e.g., send it over UART).
    if let Some(location) = info.location() {
        // Example: Log the file and line number of the panic.
        // Replace this with your logging mechanism.
        eprintln!(
            "Panic occurred in file '{}' at line {}",
            location.file(),
            location.line()
        );
    } else {
        eprintln!("Panic occurred but can't get location information.");
    }

    loop {}
}

/// # Safety
/// - `heap_start` must be a valid pointer to a memory region.
/// - The memory region starting at `heap_start` must be at least `heap_size` bytes long.
/// - The memory region must not overlap with other memory regions.
fn init_allocator(heap_start: usize, heap_size: usize) {
    unsafe {
        ALLOCATOR.lock().init(heap_start as *mut u8, heap_size);
    }
}

#[cfg(feature = "std")]
macro_rules! eprintln {
    ($($arg:tt)*) => {
        println!($($arg)*);
    };
}

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


    // === 2. STS Frame (e.g., Ping) ===
    // The STS frame code is commented out because primitives::sts does not exist.
    /*
    let sts = Sts {
        subsystem: 1,
        command: StsType::Ping as u8,
        parameters: vec![],
    };

    let sts_payload = sts.to_payload();
    let sts_frame = CanFrame::new(0x380, Some(sts_payload))
        .expect("Failed to create CanFrame for STS");

    let sts_encoded = encode_frame(&sts_frame).expect("Encoding STS failed");
    println!("\nEncoded STS Frame: {:?}", sts_encoded);

    let sts_decoded = decode_frame(&sts_encoded).expect("Decoding STS failed");
    println!("Decoded STS Frame: {:?}", sts_decoded);

    transport.send(&sts_encoded);
    let received_sts = transport.receive().expect("No STS frame received");
    println!("Received STS Frame from Mock Transport: {:?}", received_sts);
    */

    loop {}
}