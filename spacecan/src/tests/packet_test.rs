#[cfg(test)]
mod tests {
    use crate::primitives::packet::{Packet, PacketAssembler};
    use crate::protocol::SpaceCANFrame;
    use alloc::vec;
    use alloc::vec::Vec;

    #[test]
    fn test_packet_split() {
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let packet = Packet::new(Some(data));
        let frames = packet.split();

        // 10 bytes / 4 bytes per chunk = 3 fragments
        assert_eq!(frames.len(), 3);

        // Each fragment starts with [remaining_frames, index]
        assert_eq!(frames[0][0], 2); // total_frames - 1
        assert_eq!(frames[0][1], 0); // index 0
        assert_eq!(&frames[0][2..], &[1, 2, 3, 4]);

        assert_eq!(frames[1][0], 2);
        assert_eq!(frames[1][1], 1);
        assert_eq!(&frames[1][2..], &[5, 6, 7, 8]);

        assert_eq!(frames[2][0], 2);
        assert_eq!(frames[2][1], 2);
        assert_eq!(&frames[2][2..], &[9, 10]);
    }

    #[test]
    fn test_packet_reassembly() {
        let mut assembler = PacketAssembler::new();

        // Simulate 3 fragments for CAN ID 0x100, total_frames - 1 = 2
        let f0 = SpaceCANFrame::new(0x100, 0xFF, 0x00, 1, vec![2, 0, 1, 2, 3, 4]).unwrap();
        let f1 = SpaceCANFrame::new(0x100, 0xFF, 0x00, 1, vec![2, 1, 5, 6, 7, 8]).unwrap();
        let f2 = SpaceCANFrame::new(0x100, 0xFF, 0x00, 1, vec![2, 2, 9, 10]).unwrap();

        assert!(assembler.process_fragment(&f0).is_none());
        assert!(assembler.process_fragment(&f1).is_none());

        let packet = assembler.process_fragment(&f2).unwrap();
        assert_eq!(packet.data(), &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn test_incomplete_packet() {
        let mut assembler = PacketAssembler::new();

        // Only send 2 of 3 expected fragments
        let f0 = SpaceCANFrame::new(0x100, 0xFF, 0x00, 1, vec![2, 0, 1, 2, 3, 4]).unwrap();
        let f1 = SpaceCANFrame::new(0x100, 0xFF, 0x00, 1, vec![2, 1, 5, 6, 7, 8]).unwrap();

        assert!(assembler.process_fragment(&f0).is_none());
        assert!(assembler.process_fragment(&f1).is_none());
    }

    #[test]
    fn test_single_fragment() {
        let mut assembler = PacketAssembler::new();

        // A payload that fits in a single fragment: total_frames - 1 = 0
        let f = SpaceCANFrame::new(0x200, 0xFF, 0x00, 2, vec![0, 0, 0xAA, 0xBB]).unwrap();
        let packet = assembler.process_fragment(&f).unwrap();
        assert_eq!(packet.data(), &[0xAA, 0xBB]);
    }
}
