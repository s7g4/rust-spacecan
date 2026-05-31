extern crate alloc;

use crate::constants::ST01_REQUEST_VERIFICATION;
use crate::services::core::{ServiceError, ServiceHandler};
use alloc::collections::BTreeMap;
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerificationStage {
    Acceptance = 1,
    Start = 3,
    Progress = 5,
    Completion = 7,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerificationResult {
    Success = 1,
    Failure = 2,
}

#[derive(Debug, Clone)]
struct PendingRequest {
    packet_id: u16,
    source_node: u32,
    stage: VerificationStage,
}

/// ST01 Request Verification Service
/// Implements ECSS-E-ST-70-41C Service 1
pub struct RequestVerificationService {
    pending_requests: BTreeMap<u16, PendingRequest>,
    next_packet_id: u16,
}

impl RequestVerificationService {
    pub fn new() -> Self {
        RequestVerificationService {
            pending_requests: BTreeMap::new(),
            next_packet_id: 1,
        }
    }

    fn create_verification_report(
        &self,
        stage: VerificationStage,
        result: VerificationResult,
        packet_id: u16,
        error_code: Option<u16>,
    ) -> Vec<u8> {
        let mut response = Vec::new();

        // Packet identification
        response.extend_from_slice(&packet_id.to_be_bytes());

        // Error code if failure
        if result == VerificationResult::Failure {
            if let Some(code) = error_code {
                response.extend_from_slice(&code.to_be_bytes());
            } else {
                response.extend_from_slice(&[0xFF, 0xFF]); // Generic error
            }
        }

        response
    }

    pub fn accept_telecommand(&mut self, packet_id: u16, source_node: u32) -> Vec<u8> {
        let pending = PendingRequest {
            packet_id,
            source_node,
            stage: VerificationStage::Acceptance,
        };

        self.pending_requests.insert(packet_id, pending);

        self.create_verification_report(
            VerificationStage::Acceptance,
            VerificationResult::Success,
            packet_id,
            None,
        )
    }

    pub fn reject_telecommand(&mut self, packet_id: u16, error_code: u16) -> Vec<u8> {
        self.pending_requests.remove(&packet_id);

        self.create_verification_report(
            VerificationStage::Acceptance,
            VerificationResult::Failure,
            packet_id,
            Some(error_code),
        )
    }

    pub fn report_start(&mut self, packet_id: u16) -> Result<Vec<u8>, ServiceError> {
        if !self.pending_requests.contains_key(&packet_id) {
            return Err(ServiceError::InvalidPacket);
        }

        Ok(self.create_verification_report(
            VerificationStage::Start,
            VerificationResult::Success,
            packet_id,
            None,
        ))
    }

    pub fn report_completion(
        &mut self,
        packet_id: u16,
        success: bool,
        error_code: Option<u16>,
    ) -> Vec<u8> {
        self.pending_requests.remove(&packet_id);

        let result = if success {
            VerificationResult::Success
        } else {
            VerificationResult::Failure
        };

        self.create_verification_report(
            VerificationStage::Completion,
            result,
            packet_id,
            error_code,
        )
    }

    pub fn pending_count(&self) -> usize {
        self.pending_requests.len()
    }
}

impl ServiceHandler for RequestVerificationService {
    fn handle_request(
        &mut self,
        subservice: u8,
        data: &[u8],
        source_node: u32,
    ) -> Result<Option<Vec<u8>>, ServiceError> {
        if data.len() < 2 {
            return Err(ServiceError::InvalidPacket);
        }

        let packet_id = u16::from_be_bytes([data[0], data[1]]);

        match subservice {
            1 => {
                // Acceptance Success Report Request
                let response = self.accept_telecommand(packet_id, source_node);
                Ok(Some(response))
            }
            2 => {
                // Acceptance Failure Report Request
                let error_code = if data.len() >= 4 {
                    u16::from_be_bytes([data[2], data[3]])
                } else {
                    0xFFFF
                };
                let response = self.reject_telecommand(packet_id, error_code);
                Ok(Some(response))
            }
            3 => {
                // Start Success Report Request
                let response = self.report_start(packet_id)?;
                Ok(Some(response))
            }
            7 => {
                // Completion Success Report Request
                let response = self.report_completion(packet_id, true, None);
                Ok(Some(response))
            }
            8 => {
                // Completion Failure Report Request
                let error_code = if data.len() >= 4 {
                    Some(u16::from_be_bytes([data[2], data[3]]))
                } else {
                    Some(0xFFFF)
                };
                let response = self.report_completion(packet_id, false, error_code);
                Ok(Some(response))
            }
            _ => Err(ServiceError::UnknownService),
        }
    }

    fn get_service_type(&self) -> u8 {
        ST01_REQUEST_VERIFICATION
    }
}
