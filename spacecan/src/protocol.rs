extern crate alloc;

use core::fmt;
use alloc::vec::Vec;
use crate::primitives::can_frame::{CanFrame, CanFrameError};
use crate::primitives::packet::{Packet, PacketAssembler};
use crate::transport::base::Bus;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpaceCANError {
    InvalidFrame,
    TransportError,
    PacketAssemblyError,
    InvalidData,
    BufferFull,
}

impl fmt::Display for SpaceCANError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SpaceCANError::InvalidFrame => write!(f, "Invalid SpaceCAN frame"),
            SpaceCANError::TransportError => write!(f, "Transport layer error"),
            SpaceCANError::PacketAssemblyError => write!(f, "Packet assembly error"),
            SpaceCANError::InvalidData => write!(f, "Invalid data"),
            SpaceCANError::BufferFull => write!(f, "Buffer full"),
        }
    }
}

/// Represents a SpaceCAN frame with protocol-specific fields
#[cfg(all(feature = "defmt", not(test)))]
use defmt::{Format, Formatter};

#[derive(Debug, Clone)]
pub struct SpaceCANFrame {
    pub can_id: u32,
    pub service_type: u8,
    pub subservice_type: u8,
    pub node_id: u32,
    pub data: Vec<u8>,
}

#[cfg(all(feature = "defmt", not(test)))]
impl Format for SpaceCANFrame {
    fn format(&self, fmt: Formatter) {
        defmt::write!(fmt, "SpaceCANFrame {{ can_id: {}, service_type: {}, subservice_type: {}, node_id: {}, data: [", 
            self.can_id, self.service_type, self.subservice_type, self.node_id);
        for byte in &self.data {
            defmt::write!(fmt, "{}, ", byte);
        }
        defmt::write!(fmt, "] }}");
    }
}

impl SpaceCANFrame {
    /// Creates a new SpaceCAN frame
    pub fn new(can_id: u32, service_type: u8, subservice_type: u8, node_id: u32, data: Vec<u8>) -> Result<Self, SpaceCANError> {
        if can_id > crate::constants::CAN_ID_MASK {
            return Err(SpaceCANError::InvalidFrame);
        }
        
        if node_id > crate::constants::NODE_ID_MASK {
            return Err(SpaceCANError::InvalidFrame);
        }
        
        Ok(SpaceCANFrame {
            can_id,
            service_type,
            subservice_type,
            node_id,
            data,
        })
    }
    
    /// Creates a SpaceCAN frame from a CAN frame
    pub fn from_can_frame(can_frame: CanFrame) -> Result<Self, SpaceCANError> {
        let data = can_frame.data();
        if data.len() < 2 {
            return Err(SpaceCANError::InvalidFrame);
        }
        
        let service_type = data[0];
        let subservice_type = data[1];
        let payload = data[2..].to_vec();
        let node_id = can_frame.get_node_id();
        
        Ok(SpaceCANFrame {
            can_id: can_frame.can_id(),
            service_type,
            subservice_type,
            node_id,
            data: payload,
        })
    }
    
    /// Converts to a CAN frame
    pub fn to_can_frame(&self) -> Result<CanFrame, SpaceCANError> {
        let mut frame_data = Vec::with_capacity(2 + self.data.len());
        frame_data.push(self.service_type);
        frame_data.push(self.subservice_type);
        frame_data.extend_from_slice(&self.data);
        
        CanFrame::new(self.can_id, Some(frame_data))
            .map_err(|_| SpaceCANError::InvalidFrame)
    }
    
    /// Get the function ID from the CAN ID
    pub fn get_function_id(&self) -> u32 {
        (self.can_id & crate::constants::FUNCTION_ID_MASK) >> 7
    }
}

/// Main SpaceCAN protocol handler
pub struct SpaceCANProtocol<T: Bus> {
    transport: T,
    packet_assembler: PacketAssembler,
    node_id: u32,
}

impl<T: Bus> SpaceCANProtocol<T> {
    /// Creates a new SpaceCAN protocol instance
    pub fn new(transport: T, node_id: u32) -> Self {
        SpaceCANProtocol {
            transport,
            packet_assembler: PacketAssembler::new(),
            node_id,
        }
    }
    
    /// Send a SpaceCAN frame
    pub fn send_frame(&self, frame: &SpaceCANFrame) -> Result<(), SpaceCANError> {
        let can_frame = frame.to_can_frame()?;
        self.transport.send(&can_frame)
            .map_err(|_| SpaceCANError::TransportError)
    }
    
    /// Send a large packet by fragmenting it across multiple CAN frames
    pub fn send_packet(&self, service_type: u8, subservice_type: u8, target_node: u32, data: Vec<u8>) -> Result<(), SpaceCANError> {
        let packet = Packet::new(Some(data));
        let fragments = packet.split();
        
        for fragment in fragments {
            let can_id = crate::constants::ID_TC | target_node;
            let frame = SpaceCANFrame::new(can_id, service_type, subservice_type, self.node_id, fragment)?;
            self.send_frame(&frame)?;
        }
        
        Ok(())
    }
    
    /// Receive and process a CAN frame
    pub fn receive_frame(&mut self) -> Result<Option<SpaceCANFrame>, SpaceCANError> {
        if let Some(can_frame) = self.transport.get_frame() {
            // Try to assemble packet if this is a fragmented frame
            if let Some(packet) = self.packet_assembler.process_frame(can_frame.clone()) {
                // Reconstruct the SpaceCAN frame from the complete packet
                let data = packet.data();
                if data.len() >= 2 {
                    let service_type = data[0];
                    let subservice_type = data[1];
                    let payload = data[2..].to_vec();
                    
                    let frame = SpaceCANFrame::new(
                        can_frame.can_id(),
                        service_type,
                        subservice_type,
                        can_frame.get_node_id(),
                        payload,
                    )?;
                    return Ok(Some(frame));
                }
            } else {
                // Single frame, not fragmented
                let frame = SpaceCANFrame::from_can_frame(can_frame)?;
                return Ok(Some(frame));
            }
        }
        
        Ok(None)
    }
    
    /// Get the node ID
    pub fn node_id(&self) -> u32 {
        self.node_id
    }
    
    /// Start receiving frames
    pub fn start_receive(&self) {
        self.transport.start_receive();
    }
    
    /// Stop receiving frames
    pub fn stop_receive(&self) {
        self.transport.stop_receive();
    }
    
    /// Flush the frame buffer
    pub fn flush_buffer(&self) {
        self.transport.flush_frame_buffer();
    }
}
