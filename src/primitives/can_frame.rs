/// # SpaceCAN - CAN Frame Module
/// 
/// This module defines the CAN frame structure used in the SpaceCAN protocol.
/// It includes functions for creating, encoding, and decoding CAN frames.
use thiserror::Error;

/// The maximum length of a CAN frame's data payload (8 bytes).
const FULL_MASK: u32 = 0x7FF; // 11-bit CAN frame ID
const FUNCTION_MASK: u32 = 0x780; // Function ID bits
const NODE_MASK: u32 = 0x07F; // Node ID bits
const MAX_DATA_LENGTH: usize = 8; // Max bytes allowed in CAN frame

// Mapping of CANopen COB-IDs
const ID_SYNC: u32 = 0x080;
const ID_HEARTBEAT: u32 = 0x700;
const ID_SCET: u32 = 0x180;
const ID_UTC: u32 = 0x200;
const ID_TC: u32 = 0x280;
const ID_TM: u32 = 0x300;
const ID_MESSAGE: u32 = 0x380;

/// Represents possible errors in CAN frame handling.
#[derive(Error, Debug)]
pub enum CanFrameError {
    /// Error when the data length exceeds 8 bytes.
    #[error("Data length exceeds 8 bytes")]
    DataTooLong,
    /// Error when the CAN ID is invalid.
    #[error("Invalid CAN ID: {0}")]
    InvalidCanId(u32),
}
/// Represents a CAN frame used in SpaceCAN communication.
#[derive(Debug, Clone)]
pub struct CanFrame {
    /// The 11-bit CAN identifier.
    can_id: u32,
    /// The payload data of the CAN frame (max 8 bytes).
    data: Vec<u8>,
}

impl CanFrame {
    /// Creates a new CAN frame with validation.
    /// 
    /// # Arguments
    /// * `can_id` - The identifier for the CAN frame (must be 11-bit).
    /// * `data` - An optional payload (max 8 bytes).
    /// 
    /// # Returns
    /// * `Ok(CanFrame)` if valid, otherwise `CanFrameError`.
    // Constructor for CanFrame with validation
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

    // Get the length of the data
    pub fn len(&self) -> usize {
        self.data.len()
    }

    // Get the node ID
    pub fn get_node_id(&self) -> u32 {
        self.can_id & NODE_MASK
    }

    // Get the function ID
    pub fn get_func_id(&self) -> u32 {
        (self.can_id & FUNCTION_MASK) >> 7 // Shift right to get meaningful value
    }
    
    // Convert to byte representation
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut frame_bytes = vec![(self.can_id >> 3) as u8, (self.can_id & 0x07) as u8];
        frame_bytes.extend_from_slice(&self.data);
        frame_bytes
    }
    
    // Create CanFrame from bytes
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, CanFrameError> {
        if bytes.len() < 2 {
            return Err(CanFrameError::InvalidCanId(0));
        }
        
        let can_id = ((bytes[0] as u32) << 3) | (bytes[1] as u32 & 0x07);
        let data = bytes[2..].to_vec();
        
        CanFrame::new(can_id, Some(data))
    }

    // Public getter for can_id
    pub fn can_id(&self) -> u32 {
        self.can_id
    }

    // Public getter for data
    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }
}

// Example usage
fn main() {
    match CanFrame::new(ID_SYNC, Some(vec![1, 2, 3, 4, 5])) {
        Ok(can_frame) => {
            println!("{:?}", can_frame);
            println!("Node ID: {:#X}", can_frame.get_node_id());
            println!("Function ID: {:#X}", can_frame.get_func_id());
            println!("Data length: {}", can_frame.len());
            println!("Bytes: {:?}", can_frame.to_bytes());
        }
        Err(e) => println!("Error creating CAN frame: {}", e),
    }
}
