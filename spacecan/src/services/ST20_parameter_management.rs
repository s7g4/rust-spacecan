extern crate alloc;

use alloc::vec::Vec;
use alloc::collections::BTreeMap;
use alloc::string::String;
use crate::services::core::{ServiceHandler, ServiceError};
use crate::constants::ST20_PARAMETER_MANAGEMENT;

#[derive(Debug, Clone)]
pub struct Parameter {
    pub parameter_id: u16,
    pub parameter_name: String,
    pub value: Vec<u8>,
    pub read_only: bool,
    pub last_update: u32,
}

/// ST20 Parameter Management Service
/// Implements ECSS-E-ST-70-41C Service 20
pub struct ParameterManagementService {
    parameters: BTreeMap<u16, Parameter>,
    next_parameter_id: u16,
}

impl ParameterManagementService {
    pub fn new() -> Self {
        ParameterManagementService {
            parameters: BTreeMap::new(),
            next_parameter_id: 1,
        }
    }
    
    /// Report parameter values
    pub fn report_parameter_values(&self, parameter_ids: Vec<u16>) -> Result<Vec<u8>, ServiceError> {
        let mut response = Vec::new();
        
        // Number of parameters
        response.extend_from_slice(&(parameter_ids.len() as u16).to_be_bytes());
        
        for param_id in parameter_ids {
            if let Some(parameter) = self.parameters.get(&param_id) {
                // Parameter ID
                response.extend_from_slice(&param_id.to_be_bytes());
                
                // Parameter value length
                response.push(parameter.value.len() as u8);
                
                // Parameter value
                response.extend_from_slice(&parameter.value);
                
                // Last update timestamp
                response.extend_from_slice(&parameter.last_update.to_be_bytes());
            } else {
                return Err(ServiceError::InvalidPacket);
            }
        }
        
        Ok(response)
    }
    
    /// Set parameter values
    pub fn set_parameter_values(&mut self, updates: Vec<(u16, Vec<u8>)>, current_time: u32) -> Result<Vec<u8>, ServiceError> {
        let mut response = Vec::new();
        let mut success_count = 0u16;
        
        for (param_id, new_value) in updates {
            if let Some(parameter) = self.parameters.get_mut(&param_id) {
                if !parameter.read_only {
                    parameter.value = new_value;
                    parameter.last_update = current_time;
                    success_count += 1;
                }
            }
        }
        
        response.extend_from_slice(&success_count.to_be_bytes());
        Ok(response)
    }
    
    /// Register a parameter
    pub fn register_parameter(&mut self, parameter_name: String, initial_value: Vec<u8>, read_only: bool) -> u16 {
        let parameter_id = self.next_parameter_id;
        self.next_parameter_id = self.next_parameter_id.wrapping_add(1);
        
        let parameter = Parameter {
            parameter_id,
            parameter_name,
            value: initial_value,
            read_only,
            last_update: 0,
        };
        
        self.parameters.insert(parameter_id, parameter);
        parameter_id
    }
    
    /// Get parameter value
    pub fn get_parameter_value(&self, parameter_id: u16) -> Option<&Vec<u8>> {
        self.parameters.get(&parameter_id).map(|p| &p.value)
    }
    
    /// Get all parameters
    pub fn get_parameters(&self) -> Vec<u16> {
        self.parameters.keys().copied().collect()
    }
    
    /// Get parameter info
    pub fn get_parameter_info(&self, parameter_id: u16) -> Option<&Parameter> {
        self.parameters.get(&parameter_id)
    }
}

impl ServiceHandler for ParameterManagementService {
    fn handle_request(&mut self, subservice: u8, data: &[u8], _source_node: u32) -> Result<Option<Vec<u8>>, ServiceError> {
        match subservice {
            21 => {
                // Report parameter values
                if data.len() < 2 {
                    return Err(ServiceError::InvalidPacket);
                }
                
                let param_count = u16::from_be_bytes([data[0], data[1]]);
                
                if data.len() < 2 + (param_count as usize * 2) {
                    return Err(ServiceError::InvalidPacket);
                }
                
                let mut parameter_ids = Vec::new();
                for i in 0..param_count {
                    let offset = 2 + (i as usize * 2);
                    let param_id = u16::from_be_bytes([data[offset], data[offset + 1]]);
                    parameter_ids.push(param_id);
                }
                
                let response = self.report_parameter_values(parameter_ids)?;
                Ok(Some(response))
            }
            23 => {
                // Set parameter values
                if data.len() < 6 {
                    return Err(ServiceError::InvalidPacket);
                }
                
                let current_time = u32::from_be_bytes([data[0], data[1], data[2], data[3]]);
                let param_count = u16::from_be_bytes([data[4], data[5]]);
                
                let mut updates = Vec::new();
                let mut offset = 6;
                
                for _ in 0..param_count {
                    if offset + 3 > data.len() {
                        return Err(ServiceError::InvalidPacket);
                    }
                    
                    let param_id = u16::from_be_bytes([data[offset], data[offset + 1]]);
                    let value_len = data[offset + 2] as usize;
                    offset += 3;
                    
                    if offset + value_len > data.len() {
                        return Err(ServiceError::InvalidPacket);
                    }
                    
                    let value = data[offset..offset + value_len].to_vec();
                    offset += value_len;
                    
                    updates.push((param_id, value));
                }
                
                let response = self.set_parameter_values(updates, current_time)?;
                Ok(Some(response))
            }
            _ => Err(ServiceError::UnknownService),
        }
    }
    
    fn get_service_type(&self) -> u8 {
        ST20_PARAMETER_MANAGEMENT
    }
}
