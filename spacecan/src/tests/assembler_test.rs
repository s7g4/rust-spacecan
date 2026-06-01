use crate::FrameData;
use crate::primitives::can_frame::CanFrame;
use crate::protocol::PacketAssembler;

#[test]
fn test_assembler_single_frame() {
    let mut assembler = PacketAssembler::new();
    let mut frame_data = FrameData::new();
    // 3 = unsegmented
    // next 11 bits = length (let's say 4)
    // byte 0: flag(2) | len_msb(6) = 0b11_000000 = 0xC0
    // byte 1: len_lsb(5) | padding = 0b00001_000 = 0x08
    let _ = frame_data.push(0xC0);
    let _ = frame_data.push(0x08);
    let _ = frame_data.push(0xFF);
    let _ = frame_data.push(0xEE);

    let frame = CanFrame::new(0x123, Some(frame_data)).unwrap();
    let result = assembler.process_frame(&frame);
    assert!(result.is_some());
    let packet = result.unwrap();
    assert_eq!(packet.data.len(), 3); // 4 bytes total - 1 flag byte
    assert_eq!(packet.data[0], 0x08);
}

#[test]
fn test_assembler_fragmentation() {
    let mut assembler = PacketAssembler::new();

    // First frame (Flag 1)
    let mut data1 = FrameData::new();
    data1.push(0x40).unwrap(); // Flag 1 (01)
    data1.push(0x00).unwrap();
    data1.push(0xAA).unwrap();
    let frame1 = CanFrame::new(0x123, Some(data1)).unwrap();
    assert!(assembler.process_frame(&frame1).is_none());

    // Middle frame (Flag 0)
    let mut data2 = FrameData::new();
    data2.push(0x00).unwrap(); // Flag 0 (00)
    data2.push(0x00).unwrap();
    data2.push(0xBB).unwrap();
    let frame2 = CanFrame::new(0x123, Some(data2)).unwrap();
    assert!(assembler.process_frame(&frame2).is_none());

    // Last frame (Flag 2)
    let mut data3 = FrameData::new();
    data3.push(0x80).unwrap(); // Flag 2 (10)
    data3.push(0x00).unwrap();
    data3.push(0xCC).unwrap();
    let frame3 = CanFrame::new(0x123, Some(data3)).unwrap();

    let result = assembler.process_frame(&frame3);
    assert!(result.is_some());
    let packet = result.unwrap();
    assert_eq!(packet.data.len(), 6);
    assert_eq!(packet.data[0], 0x00);
    assert_eq!(packet.data[0], 0x00);
    assert_eq!(packet.data[1], 0xAA);
    assert_eq!(packet.data[2], 0x00);
    assert_eq!(packet.data[3], 0xBB);
    assert_eq!(packet.data[4], 0x00);
    assert_eq!(packet.data[5], 0xCC);
}
