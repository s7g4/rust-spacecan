#[cfg(test)]
mod tests {
    use super::*;
    use crate::sync::{SyncProducer, SyncConsumer};
    use std::sync::Arc;
    use std::time::Duration;
    use std::thread;
    
    #[test]
    fn test_sync_producer_send() {
        let producer = SyncProducer::new(Arc::new(MockNetwork::new())).unwrap();
        assert!(producer.send().is_ok());
    }
    
    #[test]
    fn test_sync_consumer_receive() {
        let consumer = SyncConsumer::new(Duration::from_secs(2));
        consumer.receive_sync();
        
        assert!(!consumer.check_timeout()); // Should not timeout immediately
    }
    
    #[test]
    fn test_sync_consumer_timeout() {
        let consumer = SyncConsumer::new(Duration::from_millis(100));
        thread::sleep(Duration::from_millis(200)); // Wait longer than timeout
        
        assert!(consumer.check_timeout()); // Should detect timeout
    }
}
