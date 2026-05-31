extern crate alloc;

use crate::constants::ST17_TEST;
use crate::services::core::{ServiceError, ServiceHandler};
use alloc::collections::BTreeMap;
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TestType {
    Connection = 1,
    ApplicationConnection = 17,
}

#[derive(Debug, Clone)]
pub struct TestRequest {
    pub test_id: u16,
    pub test_type: TestType,
    pub application_id: Option<u16>,
    pub timestamp: u32,
}

/// ST17 Test Service
/// Implements ECSS-E-ST-70-41C Service 17
pub struct TestService {
    pending_tests: BTreeMap<u16, TestRequest>,
    next_test_id: u16,
}

impl TestService {
    pub fn new() -> Self {
        TestService {
            pending_tests: BTreeMap::new(),
            next_test_id: 1,
        }
    }

    pub fn create_connection_test(&mut self, current_time: u32) -> (u16, Vec<u8>) {
        let test_id = self.next_test_id;
        self.next_test_id = self.next_test_id.wrapping_add(1);

        let test_request = TestRequest {
            test_id,
            test_type: TestType::Connection,
            application_id: None,
            timestamp: current_time,
        };

        self.pending_tests.insert(test_id, test_request);

        let mut response = Vec::new();
        response.extend_from_slice(&test_id.to_be_bytes());
        response.extend_from_slice(&current_time.to_be_bytes());

        (test_id, response)
    }

    pub fn create_application_connection_test(
        &mut self,
        application_id: u16,
        current_time: u32,
    ) -> (u16, Vec<u8>) {
        let test_id = self.next_test_id;
        self.next_test_id = self.next_test_id.wrapping_add(1);

        let test_request = TestRequest {
            test_id,
            test_type: TestType::ApplicationConnection,
            application_id: Some(application_id),
            timestamp: current_time,
        };

        self.pending_tests.insert(test_id, test_request);

        let mut response = Vec::new();
        response.extend_from_slice(&test_id.to_be_bytes());
        response.extend_from_slice(&application_id.to_be_bytes());
        response.extend_from_slice(&current_time.to_be_bytes());

        (test_id, response)
    }

    pub fn process_connection_test_report(
        &mut self,
        test_id: u16,
    ) -> Result<Vec<u8>, ServiceError> {
        if let Some(test_request) = self.pending_tests.remove(&test_id) {
            let mut response = Vec::new();
            response.extend_from_slice(&test_id.to_be_bytes());
            response.extend_from_slice(&test_request.timestamp.to_be_bytes());
            response.push(1); // Success status
            Ok(response)
        } else {
            Err(ServiceError::InvalidPacket)
        }
    }

    pub fn process_application_connection_test_report(
        &mut self,
        test_id: u16,
        application_id: u16,
    ) -> Result<Vec<u8>, ServiceError> {
        if let Some(test_request) = self.pending_tests.get(&test_id) {
            if test_request.application_id == Some(application_id) {
                let test_request = self.pending_tests.remove(&test_id).unwrap();
                let mut response = Vec::new();
                response.extend_from_slice(&test_id.to_be_bytes());
                response.extend_from_slice(&application_id.to_be_bytes());
                response.extend_from_slice(&test_request.timestamp.to_be_bytes());
                response.push(1); // Success status
                Ok(response)
            } else {
                Err(ServiceError::InvalidPacket)
            }
        } else {
            Err(ServiceError::InvalidPacket)
        }
    }

    pub fn pending_count(&self) -> usize {
        self.pending_tests.len()
    }

    pub fn get_pending_tests(&self) -> Vec<u16> {
        self.pending_tests.keys().copied().collect()
    }
}

impl ServiceHandler for TestService {
    fn handle_request(
        &mut self,
        subservice: u8,
        data: &[u8],
        _source_node: u32,
    ) -> Result<Option<Vec<u8>>, ServiceError> {
        match subservice {
            1 => {
                // Connection test request
                if data.len() < 4 {
                    return Err(ServiceError::InvalidPacket);
                }

                let current_time = u32::from_be_bytes([data[0], data[1], data[2], data[3]]);
                let (_test_id, response) = self.create_connection_test(current_time);
                Ok(Some(response))
            }
            2 => {
                // Connection test report
                if data.len() < 2 {
                    return Err(ServiceError::InvalidPacket);
                }

                let test_id = u16::from_be_bytes([data[0], data[1]]);
                let response = self.process_connection_test_report(test_id)?;
                Ok(Some(response))
            }
            17 => {
                // Application connection test request
                if data.len() < 6 {
                    return Err(ServiceError::InvalidPacket);
                }

                let application_id = u16::from_be_bytes([data[0], data[1]]);
                let current_time = u32::from_be_bytes([data[2], data[3], data[4], data[5]]);

                let (_test_id, response) =
                    self.create_application_connection_test(application_id, current_time);
                Ok(Some(response))
            }
            18 => {
                // Application connection test report
                if data.len() < 4 {
                    return Err(ServiceError::InvalidPacket);
                }

                let test_id = u16::from_be_bytes([data[0], data[1]]);
                let application_id = u16::from_be_bytes([data[2], data[3]]);

                let response =
                    self.process_application_connection_test_report(test_id, application_id)?;
                Ok(Some(response))
            }
            _ => Err(ServiceError::UnknownService),
        }
    }

    fn get_service_type(&self) -> u8 {
        ST17_TEST
    }
}
