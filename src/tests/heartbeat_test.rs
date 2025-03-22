#[cfg(test)]
mod tests {
    use super::*;
    use crate::heartbeat::{HeartbeatProducer, HeartbeatConsumer};
    use std::sync::Arc;
    use std::time::Duration;
    use std::thread;
    
    #[test]
    fn test_heartbeat_producer_send() {
        let producer = HeartbeatProducer::new(Arc::new(MockNetwork::new())).unwrap();
        assert!(producer.send().is_ok());
    }
    
    #[test]
    fn test_heartbeat_consumer_receive() {
        let consumer = HeartbeatConsumer::new(Duration::from_secs(2));
        consumer.receive_heartbeat();
        
        assert!(!consumer.check_timeout()); // Should not timeout immediately
    }
    
    #[test]
    fn test_heartbeat_consumer_timeout() {
        let consumer = HeartbeatConsumer::new(Duration::from_millis(100));
        thread::sleep(Duration::from_millis(200)); // Wait longer than timeout
        
        assert!(consumer.check_timeout()); // Should detect timeout
    }
}
