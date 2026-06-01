use crate::FrameData;
use crate::primitives::can_frame::CanFrame;
use crate::primitives::packet::SpaceCANPacket;

#[test]
fn test_can_frame_bounds() {
    // Valid frame: 8 bytes
    let mut valid_data = FrameData::new();
    for i in 0..8 {
        let _ = valid_data.push(i);
    }
    let frame = CanFrame::new(0x123, Some(valid_data.clone()));
    assert!(frame.is_ok());

    // SpaceCANPacket initialization
    let mut oversized_packet_data = crate::PacketData::new();
    for _ in 0..1024 {
        let _ = oversized_packet_data.push(0xFF);
    }
    let packet = SpaceCANPacket::new(1, 0, oversized_packet_data);
    assert!(packet.is_ok());
    assert_eq!(packet.unwrap().data.len(), 1024);
}
