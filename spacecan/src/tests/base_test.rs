#[cfg(test)]
mod tests {
    use super::*;
    use crate::can_frame::CanFrame;
    use crate::base::{Bus, BusImpl};
    
    #[test]
    fn test_bus_send_and_receive() {
        let bus = BusImpl::new();
        let frame = CanFrame::new(0x100, Some(vec![1, 2, 3, 4])).unwrap();
        
        assert!(bus.send(&frame).is_ok());
        let received = bus.get_frame().unwrap();
        assert_eq!(received.to_bytes(), frame.to_bytes());
    }
    
    #[test]
    fn test_bus_flush_frame_buffer() {
        let bus = BusImpl::new();
        let frame = CanFrame::new(0x200, Some(vec![5, 6, 7])).unwrap();
        
        bus.send(&frame).unwrap();
        bus.flush_frame_buffer();
        
        assert!(bus.get_frame().is_none());
    }
    
    #[test]
    fn test_bus_start_and_stop_receive() {
        let bus = BusImpl::new();
        bus.start_receive();
        bus.stop_receive();
        
        // Just checking if no panics occur since start/stop are currently print statements
        assert!(true);
    }
}
