extern crate alloc;

use crate::constants::ST08_FUNCTION_MANAGEMENT;
use crate::services::core::{ServiceError, ServiceHandler};
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub struct Function {
    pub function_id: u16,
    pub function_name: String,
    pub enabled: bool,
    pub arguments: Vec<u8>,
}

/// ST08 Function Management Service
/// Implements ECSS-E-ST-70-41C Service 8
pub struct FunctionManagementService {
    functions: BTreeMap<u16, Function>,
    next_function_id: u16,
}

impl FunctionManagementService {
    pub fn new() -> Self {
        FunctionManagementService {
            functions: BTreeMap::new(),
            next_function_id: 1,
        }
    }

    pub fn perform_function(
        &mut self,
        function_id: u16,
        arguments: Vec<u8>,
    ) -> Result<Vec<u8>, ServiceError> {
        if let Some(function) = self.functions.get(&function_id) {
            if function.enabled {
                // Simulate function execution
                let mut response = Vec::new();
                response.extend_from_slice(&function_id.to_be_bytes());
                response.push(0); // Success status
                response.extend_from_slice(&arguments); // Echo arguments as result
                Ok(response)
            } else {
                Err(ServiceError::ProcessingFailed)
            }
        } else {
            Err(ServiceError::InvalidPacket)
        }
    }

    pub fn register_function(&mut self, function_name: String) -> u16 {
        let function_id = self.next_function_id;
        self.next_function_id = self.next_function_id.wrapping_add(1);

        let function = Function {
            function_id,
            function_name,
            enabled: true,
            arguments: Vec::new(),
        };

        self.functions.insert(function_id, function);
        function_id
    }

    pub fn enable_function(&mut self, function_id: u16) -> Result<(), ServiceError> {
        if let Some(function) = self.functions.get_mut(&function_id) {
            function.enabled = true;
            Ok(())
        } else {
            Err(ServiceError::InvalidPacket)
        }
    }

    pub fn disable_function(&mut self, function_id: u16) -> Result<(), ServiceError> {
        if let Some(function) = self.functions.get_mut(&function_id) {
            function.enabled = false;
            Ok(())
        } else {
            Err(ServiceError::InvalidPacket)
        }
    }

    pub fn get_function_status(&self, function_id: u16) -> Option<bool> {
        self.functions.get(&function_id).map(|f| f.enabled)
    }

    pub fn get_functions(&self) -> Vec<u16> {
        self.functions.keys().copied().collect()
    }
}

impl ServiceHandler for FunctionManagementService {
    fn handle_request(
        &mut self,
        subservice: u8,
        data: &[u8],
        _source_node: u32,
    ) -> Result<Option<Vec<u8>>, ServiceError> {
        match subservice {
            1 => {
                // Perform function
                if data.len() < 2 {
                    return Err(ServiceError::InvalidPacket);
                }

                let function_id = u16::from_be_bytes([data[0], data[1]]);
                let arguments = if data.len() > 2 {
                    data[2..].to_vec()
                } else {
                    Vec::new()
                };

                let response = self.perform_function(function_id, arguments)?;
                Ok(Some(response))
            }
            _ => Err(ServiceError::UnknownService),
        }
    }

    fn get_service_type(&self) -> u8 {
        ST08_FUNCTION_MANAGEMENT
    }
}
