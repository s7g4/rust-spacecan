use space_can::primitives::can_frame::CanFrame;
use space_can::parser::{decode_frame, encode_frame};
use space_can::primitives::heartbeat::Heartbeat;
// Removed import of primitives::sts as it does not exist
use space_can::transport::mock::MockTransport;

fn main() {
    // Initialize mock transport
    let mut transport = MockTransport::new();

    // === 1. Heartbeat Frame ===
    let heartbeat = Heartbeat {
        uptime: 42,
        status: 0b00000001, // example status flag
    };

    let hb_payload = heartbeat.to_payload();
    let hb_frame = CanFrame::new(0x700, Some(hb_payload)).expect("Failed to create CanFrame");

    let hb_encoded = encode_frame(&hb_frame).expect("Encoding heartbeat failed");
    println!("Encoded Heartbeat Frame: {:?}", hb_encoded);

    let hb_decoded = decode_frame(&hb_encoded).expect("Decoding heartbeat failed");
    println!("Decoded Heartbeat Frame: {:?}", hb_decoded);

    transport.send(&hb_encoded);
    let received = transport.receive().expect("No frame received");
    println!("Received Frame from Mock Transport: {:?}", received);

    // === 2. STS Frame (e.g., Ping) ===
    // Since primitives::sts does not exist, this part is commented out or needs implementation
    /*
    let sts = Sts {
        subsystem: 1,
        command: StsType::Ping as u8,
        parameters: vec![],
    };

    let sts_payload = sts.to_payload();
    let sts_frame = CanFrame::new(0x380, Some(sts_payload)).expect("Failed to create CanFrame");

    let sts_encoded = encode_frame(&sts_frame).expect("Encoding STS failed");
    println!("\nEncoded STS Frame: {:?}", sts_encoded);

    let sts_decoded = decode_frame(&sts_encoded).expect("Decoding STS failed");
    println!("Decoded STS Frame: {:?}", sts_decoded);

    transport.send(&sts_encoded);
    let received_sts = transport.receive().expect("No STS frame received");
    println!("Received STS Frame from Mock Transport: {:?}", received_sts);
    */
}
