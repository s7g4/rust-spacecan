extern crate alloc;

#[cfg(feature = "std")]
use std::println;
#[cfg(not(feature = "std"))]
macro_rules! println {
    ($($arg:tt)*) => {};
}

use core::fmt;
use alloc::vec;
use alloc::vec::Vec;
use alloc::string::{String, ToString};
use alloc::format;

const FULL_MASK: u32 = 0x7FF;
const FUNCTION_MASK: u32 = 0x780;
const NODE_MASK: u32 = 0x07F;
const MAX_DATA_LENGTH: usize = 8;

const ID_SYNC: u32 = 0x080;
const ID_HEARTBEAT: u32 = 0x700;
const ID_SCET: u32 = 0x180;
const ID_UTC: u32 = 0x200;
const ID_TC: u32 = 0x280;
const ID_TM: u32 = 0x300;
const ID_MESSAGE: u32 = 0x380;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanFrameError {
    DataTooLong,
    InvalidCanId(u32),
    SendFailed,
}

impl fmt::Display for CanFrameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CanFrameError::DataTooLong => write!(f, "Data length exceeds 8 bytes"),
            CanFrameError::InvalidCanId(id) => write!(f, "Invalid CAN ID: {}", id),
            CanFrameError::SendFailed => write!(f, "CAN frame send failed"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CanFrame {
    can_id: u32,
    data: Vec<u8>,
}

impl CanFrame {
    pub fn new(can_id: u32, data: Option<Vec<u8>>) -> Result<Self, CanFrameError> {
        if can_id > FULL_MASK {
            return Err(CanFrameError::InvalidCanId(can_id));
        }
        let data = data.unwrap_or_else(Vec::new);
        if data.len() > MAX_DATA_LENGTH {
            return Err(CanFrameError::DataTooLong);
        }
        Ok(CanFrame { can_id, data })
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn get_node_id(&self) -> u32 {
        self.can_id & NODE_MASK
    }

    pub fn get_func_id(&self) -> u32 {
        (self.can_id & FUNCTION_MASK) >> 7
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut frame_bytes = vec![(self.can_id >> 3) as u8, (self.can_id & 0x07) as u8];
        frame_bytes.extend_from_slice(&self.data);
        frame_bytes
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, CanFrameError> {
        if bytes.len() < 2 {
            return Err(CanFrameError::InvalidCanId(0));
        }
        let can_id = ((bytes[0] as u32) << 3) | (bytes[1] as u32 & 0x07);
        let data = bytes[2..].to_vec();
        CanFrame::new(can_id, Some(data))
    }

    pub fn can_id(&self) -> u32 {
        self.can_id
    }

    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }
}
