use space_can::primitives::can_frame::CanFrame;
use space_can::parser::{decode_frame, encode_frame};
use space_can::primitives::heartbeat::Heartbeat;
// Removed import of primitives::sts as it does not exist
use space_can::transport::mock::MockTransport;

/// Entry point of the application demonstrating CAN frame encoding and decoding.
fn main() {
    // Initialize a mock transport for sending and receiving CAN frames.
    let mut transport = MockTransport::new();

    // === 1. Heartbeat Frame ===
    // Create a heartbeat frame with example uptime and status.
    let heartbeat = Heartbeat {
        uptime: 42,
        status: 0b00000001, // Example status flag indicating system state.
    };

    // Convert heartbeat to payload and create a CAN frame.
    let hb_payload = heartbeat.to_payload();
    let hb_frame = CanFrame::new(0x700, Some(hb_payload))
        .expect("Failed to create CanFrame for heartbeat");

    // Encode the CAN frame into bytes.
    let hb_encoded = encode_frame(&hb_frame).expect("Encoding heartbeat failed");
    println!("Encoded Heartbeat Frame: {:?}", hb_encoded);

    // Decode the encoded bytes back into a CAN frame.
    let hb_decoded = decode_frame(&hb_encoded).expect("Decoding heartbeat failed");
    println!("Decoded Heartbeat Frame: {:?}", hb_decoded);

    // Send the encoded heartbeat frame via the mock transport.
    transport.send(&hb_encoded);

    // Receive a frame from the mock transport.
    let received = transport.receive().expect("No frame received");
    println!("Received Frame from Mock Transport: {:?}", received);

    // === 2. STS Frame (e.g., Ping) ===
    // The STS frame code is commented out because primitives::sts does not exist.
    /*
    let sts = Sts {
        subsystem: 1,
        command: StsType::Ping as u8,
        parameters: vec![],
    };

    let sts_payload = sts.to_payload();
    let sts_frame = CanFrame::new(0x380, Some(sts_payload))
        .expect("Failed to create CanFrame for STS");

    let sts_encoded = encode_frame(&sts_frame).expect("Encoding STS failed");
    println!("\nEncoded STS Frame: {:?}", sts_encoded);

    let sts_decoded = decode_frame(&sts_encoded).expect("Decoding STS failed");
    println!("Decoded STS Frame: {:?}", sts_decoded);

    transport.send(&sts_encoded);
    let received_sts = transport.receive().expect("No STS frame received");
    println!("Received STS Frame from Mock Transport: {:?}", received_sts);
    */
}
