extern crate alloc;

use super::can_frame::{CanFrame, CanFrameError};
use crate::constants::ID_SYNC;
use alloc::vec::Vec;
use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyncError {
    InvalidData,
    SendFailed,
}

impl fmt::Display for SyncError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SyncError::InvalidData => write!(f, "Invalid sync data"),
            SyncError::SendFailed => write!(f, "Failed to send sync"),
        }
    }
}

#[derive(Debug)]
pub struct SyncData {
    pub sync_counter: u32,
    pub timestamp: u32,
}

impl SyncData {
    pub fn new(sync_counter: u32, timestamp: u32) -> Self {
        SyncData {
            sync_counter,
            timestamp,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(8);
        bytes.extend_from_slice(&self.sync_counter.to_be_bytes());
        bytes.extend_from_slice(&self.timestamp.to_be_bytes());
        bytes
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, SyncError> {
        if bytes.len() < 8 {
            return Err(SyncError::InvalidData);
        }

        let sync_counter = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        let timestamp = u32::from_be_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);

        Ok(SyncData {
            sync_counter,
            timestamp,
        })
    }
}

pub struct SyncManager {
    sync_counter: u32,
}

impl SyncManager {
    pub fn new() -> Self {
        SyncManager { sync_counter: 0 }
    }

    pub fn create_sync(&mut self, timestamp: u32) -> Result<CanFrame, SyncError> {
        self.sync_counter = self.sync_counter.wrapping_add(1);
        let sync_data = SyncData::new(self.sync_counter, timestamp);

        CanFrame::new(ID_SYNC, Some(sync_data.to_bytes())).map_err(|_| SyncError::SendFailed)
    }

    pub fn parse_sync(&self, frame: &CanFrame) -> Result<SyncData, SyncError> {
        if frame.can_id() != ID_SYNC {
            return Err(SyncError::InvalidData);
        }

        SyncData::from_bytes(frame.data())
    }

    pub fn get_sync_counter(&self) -> u32 {
        self.sync_counter
    }
}
