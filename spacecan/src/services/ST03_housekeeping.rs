extern crate alloc;

use crate::constants::ST03_HOUSEKEEPING;
use crate::services::core::{ServiceError, ServiceHandler};
use alloc::collections::BTreeMap;
use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub struct HousekeepingParameter {
    pub parameter_id: u16,
    pub value: Vec<u8>,
    pub timestamp: u32,
}

#[derive(Debug, Clone)]
pub struct HousekeepingReport {
    pub report_id: u16,
    pub parameters: Vec<HousekeepingParameter>,
    pub generation_time: u32,
    pub enabled: bool,
    pub collection_interval: u32, // in seconds
}

/// ST03 Housekeeping Service
/// Implements ECSS-E-ST-70-41C Service 3
pub struct HousekeepingService {
    reports: BTreeMap<u16, HousekeepingReport>,
    parameters: BTreeMap<u16, Vec<u8>>, // parameter_id -> current_value
    next_report_id: u16,
}

impl HousekeepingService {
    pub fn new() -> Self {
        HousekeepingService {
            reports: BTreeMap::new(),
            parameters: BTreeMap::new(),
            next_report_id: 1,
        }
    }

    pub fn create_report(&mut self, parameter_ids: Vec<u16>, collection_interval: u32) -> u16 {
        let report_id = self.next_report_id;
        self.next_report_id = self.next_report_id.wrapping_add(1);

        let report = HousekeepingReport {
            report_id,
            parameters: parameter_ids
                .into_iter()
                .map(|id| HousekeepingParameter {
                    parameter_id: id,
                    value: Vec::new(),
                    timestamp: 0,
                })
                .collect(),
            generation_time: 0,
            enabled: false,
            collection_interval,
        };

        self.reports.insert(report_id, report);
        report_id
    }

    pub fn delete_report(&mut self, report_id: u16) -> Result<(), ServiceError> {
        if self.reports.remove(&report_id).is_some() {
            Ok(())
        } else {
            Err(ServiceError::InvalidPacket)
        }
    }

    pub fn enable_report(&mut self, report_id: u16) -> Result<(), ServiceError> {
        if let Some(report) = self.reports.get_mut(&report_id) {
            report.enabled = true;
            Ok(())
        } else {
            Err(ServiceError::InvalidPacket)
        }
    }

    pub fn disable_report(&mut self, report_id: u16) -> Result<(), ServiceError> {
        if let Some(report) = self.reports.get_mut(&report_id) {
            report.enabled = false;
            Ok(())
        } else {
            Err(ServiceError::InvalidPacket)
        }
    }

    pub fn generate_report(
        &mut self,
        report_id: u16,
        current_time: u32,
    ) -> Result<Vec<u8>, ServiceError> {
        if let Some(report) = self.reports.get_mut(&report_id) {
            let mut response = Vec::new();

            // Report ID
            response.extend_from_slice(&report_id.to_be_bytes());

            // Generation time
            response.extend_from_slice(&current_time.to_be_bytes());

            // Number of parameters
            response.extend_from_slice(&(report.parameters.len() as u16).to_be_bytes());

            // Parameters
            for param in &mut report.parameters {
                // Parameter ID
                response.extend_from_slice(&param.parameter_id.to_be_bytes());

                // Get current value from parameters map
                if let Some(value) = self.parameters.get(&param.parameter_id) {
                    param.value = value.clone();
                    param.timestamp = current_time;
                }

                // Parameter length
                response.push(param.value.len() as u8);

                // Parameter value
                response.extend_from_slice(&param.value);

                // Timestamp
                response.extend_from_slice(&param.timestamp.to_be_bytes());
            }

            report.generation_time = current_time;
            Ok(response)
        } else {
            Err(ServiceError::InvalidPacket)
        }
    }

    pub fn update_parameter(&mut self, parameter_id: u16, value: Vec<u8>) {
        self.parameters.insert(parameter_id, value);
    }

    pub fn get_report_status(&self, report_id: u16) -> Option<bool> {
        self.reports.get(&report_id).map(|r| r.enabled)
    }

    pub fn get_reports(&self) -> Vec<u16> {
        self.reports.keys().copied().collect()
    }
}

impl ServiceHandler for HousekeepingService {
    fn handle_request(
        &mut self,
        subservice: u8,
        data: &[u8],
        _source_node: u32,
    ) -> Result<Option<Vec<u8>>, ServiceError> {
        match subservice {
            1 => {
                // Create housekeeping report definition
                if data.len() < 6 {
                    return Err(ServiceError::InvalidPacket);
                }

                let collection_interval = u32::from_be_bytes([data[0], data[1], data[2], data[3]]);
                let param_count = u16::from_be_bytes([data[4], data[5]]);

                if data.len() < 6 + (param_count as usize * 2) {
                    return Err(ServiceError::InvalidPacket);
                }

                let mut parameter_ids = Vec::new();
                for i in 0..param_count {
                    let offset = 6 + (i as usize * 2);
                    let param_id = u16::from_be_bytes([data[offset], data[offset + 1]]);
                    parameter_ids.push(param_id);
                }

                let report_id = self.create_report(parameter_ids, collection_interval);
                Ok(Some(report_id.to_be_bytes().to_vec()))
            }
            2 => {
                // Delete housekeeping report definition
                if data.len() < 2 {
                    return Err(ServiceError::InvalidPacket);
                }

                let report_id = u16::from_be_bytes([data[0], data[1]]);
                self.delete_report(report_id)?;
                Ok(None)
            }
            3 => {
                // Enable housekeeping report generation
                if data.len() < 2 {
                    return Err(ServiceError::InvalidPacket);
                }

                let report_id = u16::from_be_bytes([data[0], data[1]]);
                self.enable_report(report_id)?;
                Ok(None)
            }
            4 => {
                // Disable housekeeping report generation
                if data.len() < 2 {
                    return Err(ServiceError::InvalidPacket);
                }

                let report_id = u16::from_be_bytes([data[0], data[1]]);
                self.disable_report(report_id)?;
                Ok(None)
            }
            25 => {
                // Generate housekeeping report
                if data.len() < 6 {
                    return Err(ServiceError::InvalidPacket);
                }

                let report_id = u16::from_be_bytes([data[0], data[1]]);
                let current_time = u32::from_be_bytes([data[2], data[3], data[4], data[5]]);

                let response = self.generate_report(report_id, current_time)?;
                Ok(Some(response))
            }
            _ => Err(ServiceError::UnknownService),
        }
    }

    fn get_service_type(&self) -> u8 {
        ST03_HOUSEKEEPING
    }
}
