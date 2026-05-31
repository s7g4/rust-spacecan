extern crate alloc;

use super::can_frame::{CanFrame, CanFrameError};
use crate::constants::ID_HEARTBEAT;
use alloc::vec::Vec;
use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeartbeatError {
    InvalidData,
    SendFailed,
}

impl fmt::Display for HeartbeatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HeartbeatError::InvalidData => write!(f, "Invalid heartbeat data"),
            HeartbeatError::SendFailed => write!(f, "Failed to send heartbeat"),
        }
    }
}

#[derive(Debug)]
pub struct HeartbeatData {
    pub node_id: u32,
    pub status: u8,
    pub timestamp: u32,
}

impl HeartbeatData {
    pub fn new(node_id: u32, status: u8, timestamp: u32) -> Self {
        HeartbeatData {
            node_id,
            status,
            timestamp,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(5);
        bytes.push(self.status);
        bytes.extend_from_slice(&self.timestamp.to_be_bytes());
        bytes
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, HeartbeatError> {
        if bytes.len() < 5 {
            return Err(HeartbeatError::InvalidData);
        }

        let status = bytes[0];
        let timestamp = u32::from_be_bytes([bytes[1], bytes[2], bytes[3], bytes[4]]);

        Ok(HeartbeatData {
            node_id: 0, // Will be set from CAN frame
            status,
            timestamp,
        })
    }
}

pub struct HeartbeatManager {
    node_id: u32,
    status: u8,
}

impl HeartbeatManager {
    pub fn new(node_id: u32) -> Self {
        HeartbeatManager { node_id, status: 0 }
    }

    pub fn set_status(&mut self, status: u8) {
        self.status = status;
    }

    pub fn create_heartbeat(&self, timestamp: u32) -> Result<CanFrame, HeartbeatError> {
        let heartbeat_data = HeartbeatData::new(self.node_id, self.status, timestamp);
        let can_id = ID_HEARTBEAT | self.node_id;

        CanFrame::new(can_id, Some(heartbeat_data.to_bytes()))
            .map_err(|_| HeartbeatError::SendFailed)
    }

    pub fn parse_heartbeat(&self, frame: &CanFrame) -> Result<HeartbeatData, HeartbeatError> {
        if (frame.can_id() & !crate::constants::NODE_ID_MASK) != ID_HEARTBEAT {
            return Err(HeartbeatError::InvalidData);
        }

        let mut heartbeat_data = HeartbeatData::from_bytes(frame.data())?;
        heartbeat_data.node_id = frame.get_node_id();

        Ok(heartbeat_data)
    }
}
