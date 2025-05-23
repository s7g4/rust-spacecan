extern crate alloc;

use crate::primitives::can_frame::{CanFrame, CanFrameError};
use alloc::vec::Vec;
use alloc::string::ToString;
use core::result::Result;
use alloc::string::String;
use core::option::Option;
use core::option::Option::{Some, None};
use core::result::Result::{Ok, Err};

/// Decodes a CAN frame from a byte slice.
///
/// # Arguments
///
/// * `data` - A byte slice representing the encoded CAN frame.
///
/// # Returns
///
/// * `Result<CanFrame, String>` - Ok with CanFrame on success, Err with error message on failure.
pub fn decode_frame(data: &[u8]) -> Result<CanFrame, String> {
    CanFrame::from_bytes(data).map_err(|e| e.to_string())
}

/// Encodes a CAN frame into a byte vector.
///
/// # Arguments
///
/// * `frame` - A reference to a CanFrame.
///
/// # Returns
///
/// * `Result<Vec<u8>, String>` - Ok with encoded bytes on success, Err with error message on failure.
pub fn encode_frame(frame: &CanFrame) -> Result<Vec<u8>, String> {
    Ok(frame.to_bytes())
}
