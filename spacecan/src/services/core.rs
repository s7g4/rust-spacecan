extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use alloc::boxed::Box;
use core::fmt;
use crate::protocol::SpaceCANFrame;
use crate::constants::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceError {
    UnknownService,
    InvalidPacket,
    ProcessingFailed,
    ServiceNotRegistered,
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServiceError::UnknownService => write!(f, "Unknown service type"),
            ServiceError::InvalidPacket => write!(f, "Invalid packet format"),
            ServiceError::ProcessingFailed => write!(f, "Service processing failed"),
            ServiceError::ServiceNotRegistered => write!(f, "Service not registered"),
        }
    }
}

/// Trait for service handlers
pub trait ServiceHandler {
    fn handle_request(&mut self, subservice: u8, data: &[u8], source_node: u32) -> Result<Option<Vec<u8>>, ServiceError>;
    fn get_service_type(&self) -> u8;
}

/// Main service manager for handling ECSS services
pub struct ServiceManager {
    services: BTreeMap<u8, Box<dyn ServiceHandler>>,
    node_id: u32,
}

impl ServiceManager {
    pub fn new(node_id: u32) -> Self {
        ServiceManager {
            services: BTreeMap::new(),
            node_id,
        }
    }
    
    /// Register a service handler
    pub fn register_service(&mut self, service: Box<dyn ServiceHandler>) {
        let service_type = service.get_service_type();
        self.services.insert(service_type, service);
    }
    
    /// Process an incoming SpaceCAN frame
    pub fn process_frame(&mut self, frame: &SpaceCANFrame) -> Result<Option<SpaceCANFrame>, ServiceError> {
        let service_type = frame.service_type;
        let subservice = frame.subservice_type;
        let source_node = frame.node_id;
        
        if let Some(service) = self.services.get_mut(&service_type) {
            match service.handle_request(subservice, &frame.data, source_node) {
                Ok(Some(response_data)) => {
                    // Create response frame
                    let response_frame = SpaceCANFrame::new(
                        frame.can_id, // Use same CAN ID
                        service_type,
                        subservice,
                        self.node_id,
                        response_data,
                    ).map_err(|_| ServiceError::ProcessingFailed)?;
                    
                    Ok(Some(response_frame))
                }
                Ok(None) => Ok(None), // No response needed
                Err(e) => Err(e),
            }
        } else {
            Err(ServiceError::ServiceNotRegistered)
        }
    }
    
    /// Get list of registered service types
    pub fn get_registered_services(&self) -> Vec<u8> {
        self.services.keys().copied().collect()
    }
    
    /// Get node ID
    pub fn get_node_id(&self) -> u32 {
        self.node_id
    }
}
