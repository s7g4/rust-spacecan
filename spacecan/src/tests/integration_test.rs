#[cfg(test)]
mod tests {
    extern crate alloc;
    
    use alloc::boxed::Box;
    use alloc::string::String;
    use alloc::vec;
    use alloc::vec::Vec;
    use crate::services::{
        ServiceManager,
        RequestVerificationService,
        HousekeepingService,
        TestService,
        FunctionManagementService,
        ParameterManagementService,
    };
    use crate::protocol::SpaceCANFrame;
    use crate::constants::*;
    
    #[test]
    fn test_service_manager_creation() {
        let mut service_manager = ServiceManager::new(1);
        
        // Register services
        service_manager.register_service(Box::new(RequestVerificationService::new()));
        service_manager.register_service(Box::new(HousekeepingService::new()));
        service_manager.register_service(Box::new(TestService::new()));
        service_manager.register_service(Box::new(FunctionManagementService::new()));
        service_manager.register_service(Box::new(ParameterManagementService::new()));
        
        let registered_services = service_manager.get_registered_services();
        assert_eq!(registered_services.len(), 5);
        assert!(registered_services.contains(&ST01_REQUEST_VERIFICATION));
        assert!(registered_services.contains(&ST03_HOUSEKEEPING));
        assert!(registered_services.contains(&ST17_TEST));
        assert!(registered_services.contains(&ST08_FUNCTION_MANAGEMENT));
        assert!(registered_services.contains(&ST20_PARAMETER_MANAGEMENT));
    }
    
    #[test]
    fn test_request_verification_service() {
        let mut service_manager = ServiceManager::new(1);
        service_manager.register_service(Box::new(RequestVerificationService::new()));
        
        // Create a request verification frame
        let frame = SpaceCANFrame::new(
            ID_TC | 1,
            ST01_REQUEST_VERIFICATION,
            1, // Acceptance success
            2, // Source node
            vec![0x00, 0x01], // Packet ID
        ).unwrap();
        
        let response = service_manager.process_frame(&frame).unwrap();
        assert!(response.is_some());
        
        let response_frame = response.unwrap();
        assert_eq!(response_frame.service_type, ST01_REQUEST_VERIFICATION);
    }
    
    #[test]
    fn test_housekeeping_service() {
        let mut service_manager = ServiceManager::new(1);
        service_manager.register_service(Box::new(HousekeepingService::new()));
        
        // Create housekeeping report definition
        let mut data = Vec::new();
        data.extend_from_slice(&60u32.to_be_bytes()); // 60 second interval
        data.extend_from_slice(&2u16.to_be_bytes()); // 2 parameters
        data.extend_from_slice(&1u16.to_be_bytes()); // Parameter ID 1
        data.extend_from_slice(&2u16.to_be_bytes()); // Parameter ID 2
        
        let frame = SpaceCANFrame::new(
            ID_TC | 1,
            ST03_HOUSEKEEPING,
            1, // Create report definition
            2, // Source node
            data,
        ).unwrap();
        
        let response = service_manager.process_frame(&frame).unwrap();
        assert!(response.is_some());
        
        let response_frame = response.unwrap();
        assert_eq!(response_frame.service_type, ST03_HOUSEKEEPING);
        assert_eq!(response_frame.data.len(), 2); // Report ID returned
    }
    
    #[test]
    fn test_test_service() {
        let mut service_manager = ServiceManager::new(1);
        service_manager.register_service(Box::new(TestService::new()));
        
        // Create connection test request
        let current_time = 1000u32;
        let frame = SpaceCANFrame::new(
            ID_TC | 1,
            ST17_TEST,
            1, // Connection test request
            2, // Source node
            current_time.to_be_bytes().to_vec(),
        ).unwrap();
        
        let response = service_manager.process_frame(&frame).unwrap();
        assert!(response.is_some());
        
        let response_frame = response.unwrap();
        assert_eq!(response_frame.service_type, ST17_TEST);
        assert!(response_frame.data.len() >= 6); // Test ID + timestamp
    }
    
    #[test]
    fn test_parameter_management_service() {
        let mut service_manager = ServiceManager::new(1);
        let mut param_service = ParameterManagementService::new();
        
        // Register some test parameters
        param_service.register_parameter(String::from("temperature"), vec![0x12, 0x34], false);
        param_service.register_parameter(String::from("voltage"), vec![0x56, 0x78], false);
        
        service_manager.register_service(Box::new(param_service));
        
        // Request parameter values
        let mut data = Vec::new();
        data.extend_from_slice(&2u16.to_be_bytes()); // 2 parameters
        data.extend_from_slice(&1u16.to_be_bytes()); // Parameter ID 1
        data.extend_from_slice(&2u16.to_be_bytes()); // Parameter ID 2
        
        let frame = SpaceCANFrame::new(
            ID_TC | 1,
            ST20_PARAMETER_MANAGEMENT,
            21, // Report parameter values
            2, // Source node
            data,
        ).unwrap();
        
        let response = service_manager.process_frame(&frame).unwrap();
        assert!(response.is_some());
        
        let response_frame = response.unwrap();
        assert_eq!(response_frame.service_type, ST20_PARAMETER_MANAGEMENT);
        assert!(response_frame.data.len() > 2); // Should contain parameter data
    }
    
    #[test]
    fn test_unknown_service() {
        let mut service_manager = ServiceManager::new(1);
        
        let frame = SpaceCANFrame::new(
            ID_TC | 1,
            99, // Unknown service
            1,
            2,
            vec![0x01, 0x02],
        ).unwrap();
        
        let result = service_manager.process_frame(&frame);
        assert!(result.is_err());
    }
}
