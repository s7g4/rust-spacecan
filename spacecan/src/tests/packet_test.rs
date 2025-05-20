#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::{Packet, PacketAssembler};
    use crate::can_frame::CanFrame;
    
    #[test]
    fn test_packet_split() {
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let packet = Packet::new(Some(data.clone()));
        let frames = packet.split();
        
        assert!(!frames.is_empty());
        assert_eq!(frames.concat(), data);
    }
    
    #[test]
    fn test_packet_reassembly() {
        let mut assembler = PacketAssembler::new();
        let can_frames = vec![
            CanFrame::new(0x100, Some(vec![2, 0, 1, 2, 3])).unwrap(),
            CanFrame::new(0x100, Some(vec![2, 1, 4, 5, 6])).unwrap(),
            CanFrame::new(0x100, Some(vec![2, 2, 7, 8, 9, 10])).unwrap(),
        ];
        
        for frame in &can_frames {
            assembler.process_frame(frame.clone());
        }
        
        let assembled_packet = assembler.process_frame(can_frames[2].clone()).unwrap();
        assert_eq!(assembled_packet.data, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }
    
    #[test]
    fn test_incomplete_packet() {
        let mut assembler = PacketAssembler::new();
        let can_frames = vec![
            CanFrame::new(0x100, Some(vec![2, 0, 1, 2, 3])).unwrap(),
            CanFrame::new(0x100, Some(vec![2, 1, 4, 5, 6])).unwrap(),
        ];
        
        for frame in &can_frames {
            assembler.process_frame(frame.clone());
        }
        
        assert!(assembler.process_frame(can_frames[1].clone()).is_none());
    }
}
