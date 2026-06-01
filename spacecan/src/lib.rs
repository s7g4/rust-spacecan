#![no_std]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_macros)]
#![allow(clippy::new_without_default)]
#![allow(clippy::get_first)]
#![allow(clippy::unwrap_or_default)]
#![allow(clippy::len_without_is_empty)]
#![allow(clippy::manual_div_ceil)]
#![allow(clippy::collapsible_if)]

extern crate alloc;

pub mod primitives;
pub mod protocol;
pub mod transport;

pub mod services;

#[cfg(test)]
mod tests {
    pub mod integration_test;
    pub mod packet_test;
}

#[cfg(all(feature = "defmt", not(target_os = "none")))]
#[path = "tests/defmt_mock.rs"]
mod defmt_mock;

pub use primitives::{
    can_frame::{CanFrame, CanFrameError},
    heartbeat::HeartbeatManager,
    network::NetworkManager,
    packet::{Packet, PacketAssembler},
    sync::SyncManager,
    timer::Timer,
};

pub use protocol::{SpaceCANError, SpaceCANFrame, SpaceCANProtocol};

pub use transport::{
    base::{Bus, BusImpl},
    frame_buffer::FrameBuffer,
    mock::MockTransport,
};

pub mod constants {
    pub const ID_SYNC: u32 = 0x080;
    pub const ID_HEARTBEAT: u32 = 0x700;
    pub const ID_SCET: u32 = 0x180; // Spacecraft Elapsed Time
    pub const ID_UTC: u32 = 0x200; // UTC Time
    pub const ID_TC: u32 = 0x280; // Telecommand
    pub const ID_TM: u32 = 0x300; // Telemetry
    pub const ID_MESSAGE: u32 = 0x380; // General Message

    pub const ST01_REQUEST_VERIFICATION: u8 = 1;
    pub const ST03_HOUSEKEEPING: u8 = 3;
    pub const ST08_FUNCTION_MANAGEMENT: u8 = 8;
    pub const ST17_TEST: u8 = 17;
    pub const ST20_PARAMETER_MANAGEMENT: u8 = 20;

    pub const MAX_CAN_DATA_LENGTH: usize = 8;
    pub const ST_FRAGMENTED: u8 = 0xFF;
    pub const MAX_PACKET_DATA_LENGTH: usize = 4; // 2 bytes for fragmentation header
    pub const CAN_ID_MASK: u32 = 0x7FF;
    pub const NODE_ID_MASK: u32 = 0x07F;
    pub const FUNCTION_ID_MASK: u32 = 0x780;
}
