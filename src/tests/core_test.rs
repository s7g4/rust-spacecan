#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{PacketUtilizationService, PacketUtilizationServiceController};
    use crate::packet::Packet;
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_packet_service_initialization() {
        let service = PacketUtilizationService::new();
        assert!(service.lock().is_ok());
    }
    
    #[test]
    fn test_packet_routing() {
        let service = PacketUtilizationService::new();
        let controller = PacketUtilizationServiceController::new(service.clone());
        
        let packet = Packet::new(Some(vec![1, 2, 3, 4]));
        controller.send(packet.clone(), 1);
        
        assert!(service.lock().is_ok()); // Ensuring no crashes
    }
    
    #[test]
    fn test_received_packet_handling() {
        let service = PacketUtilizationService::new();
        let controller = PacketUtilizationServiceController::new(service.clone());
        
        let packet = Packet::new(Some(vec![5, 6, 7, 8]));
        controller.received_packet(packet.clone(), 2);
        
        thread::sleep(Duration::from_millis(100)); // Allow processing time
        assert!(service.lock().is_ok());
    }
}
