#![no_std]
#![no_main]

extern crate alloc;

use spacecan::{
    SpaceCANProtocol, SpaceCANFrame,
    transport::base::BusImpl,
    constants::*,
    init_allocator,
};
use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    // Initialize the allocator
    init_allocator();
    
    // Create a bus implementation
    let bus = BusImpl::new();
    
    // Create a SpaceCAN protocol instance
    let mut protocol = SpaceCANProtocol::new(bus, 1); // Node ID 1
    
    // Start receiving
    protocol.start_receive();
    
    // Example: Send a test heartbeat
    let test_frame = SpaceCANFrame::new(
        ID_HEARTBEAT | 1, // Heartbeat from node 1
        ST01_REQUEST_VERIFICATION,
        1, // subservice
        1, // node_id
        alloc::vec![0x01, 0x02, 0x03, 0x04],
    ).unwrap();
    
    let _ = protocol.send_frame(&test_frame);
    
    // Main loop
    loop {
        // Process incoming frames
        if let Ok(Some(_frame)) = protocol.receive_frame() {
            // Handle received frame
            // In a real implementation, you would process the frame
            // based on its service type and subservice
        }
        
        // In a real embedded system, you might yield to other tasks here
        // or enter a low-power mode
    }
}

/// This function is called on panic.
/// Only compiled for no_std environments (excluded during tests)
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
