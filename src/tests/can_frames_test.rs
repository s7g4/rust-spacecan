#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_can_frame_creation_valid() {
        let frame = CanFrame::new(0x100, Some(vec![1, 2, 3, 4])).unwrap();
        assert_eq!(frame.get_node_id(), 0x100 & 0x07F);
        assert_eq!(frame.get_func_id(), (0x100 & 0x780) >> 7);
        assert_eq!(frame.len(), 4);
    }
    
    #[test]
    fn test_can_frame_creation_invalid_id() {
        let frame = CanFrame::new(0x800, Some(vec![1, 2]));
        assert!(frame.is_err());
    }
    
    #[test]
    fn test_can_frame_creation_too_long() {
        let frame = CanFrame::new(0x100, Some(vec![0; 9]));
        assert!(frame.is_err());
    }
    
    #[test]
    fn test_can_frame_to_bytes() {
        let frame = CanFrame::new(0x100, Some(vec![1, 2, 3])).unwrap();
        let bytes = frame.to_bytes();
        assert_eq!(bytes.len(), 5);
    }
    
    #[test]
    fn test_can_frame_from_bytes() {
        let bytes = vec![0x20, 0x00, 1, 2, 3];
        let frame = CanFrame::from_bytes(&bytes).unwrap();
        assert_eq!(frame.get_node_id(), 0x100 & 0x07F);
        assert_eq!(frame.get_func_id(), (0x100 & 0x780) >> 7);
        assert_eq!(frame.len(), 3);
    }
}
