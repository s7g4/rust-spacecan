#![no_std]

extern crate alloc;

use linked_list_allocator::LockedHeap;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

// Core modules
pub mod primitives;
pub mod protocol;
pub mod transport;

// Service modules
pub mod services;

// Test modules
#[cfg(test)]
mod tests {
    pub mod integration_test;
}

// Re-export key types
pub use primitives::{
    can_frame::{CanFrame, CanFrameError},
    packet::{Packet, PacketAssembler},
    heartbeat::HeartbeatManager,
    sync::SyncManager,
    network::NetworkManager,
    timer::Timer,
};

pub use protocol::{SpaceCANProtocol, SpaceCANFrame, SpaceCANError};

pub use transport::{
    base::{Bus, BusImpl},
    frame_buffer::FrameBuffer,
    mock::MockTransport,
};

// Initialize allocator for no_std environments
pub fn init_allocator() {
    use core::mem::MaybeUninit;
    
    const HEAP_SIZE: usize = 8192;
    static mut HEAP: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
    
    unsafe { 
        let heap_ptr = core::ptr::addr_of_mut!(HEAP) as *mut u8;
        ALLOCATOR.lock().init(heap_ptr, HEAP_SIZE);
    }
}

// Constants for SpaceCAN protocol
pub mod constants {
    // CAN ID definitions for SpaceCAN
    pub const ID_SYNC: u32 = 0x080;
    pub const ID_HEARTBEAT: u32 = 0x700;
    pub const ID_SCET: u32 = 0x180;      // Spacecraft Elapsed Time
    pub const ID_UTC: u32 = 0x200;       // UTC Time
    pub const ID_TC: u32 = 0x280;        // Telecommand
    pub const ID_TM: u32 = 0x300;        // Telemetry
    pub const ID_MESSAGE: u32 = 0x380;   // General Message
    
    // Service type definitions (ECSS-E-ST-70-41C)
    pub const ST01_REQUEST_VERIFICATION: u8 = 1;
    pub const ST03_HOUSEKEEPING: u8 = 3;
    pub const ST08_FUNCTION_MANAGEMENT: u8 = 8;
    pub const ST17_TEST: u8 = 17;
    pub const ST20_PARAMETER_MANAGEMENT: u8 = 20;
    
    // Protocol constants
    pub const MAX_CAN_DATA_LENGTH: usize = 8;
    pub const MAX_PACKET_DATA_LENGTH: usize = 6; // 2 bytes for fragmentation header
    pub const CAN_ID_MASK: u32 = 0x7FF;
    pub const NODE_ID_MASK: u32 = 0x07F;
    pub const FUNCTION_ID_MASK: u32 = 0x780;
}
