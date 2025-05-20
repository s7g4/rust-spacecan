use spacecan::primitives::can_frame::CanFrame;
use spacecan::transport::base::BusImpl;
use std::sync::Arc;

fn main() {
    // Initialize the CAN bus
    let bus = Arc::new(BusImpl::new());
    
    // Create a CAN frame with an 11-bit ID and sample payload
    let frame = CanFrame::new(0x100, Some(vec![0x12, 0x34, 0x56, 0x78]))
        .expect("Failed to create CAN frame");
    
    // Send the frame over the bus
    match bus.send(&frame) {
        Ok(_) => println!("✅ CAN Frame sent successfully: {:?}", frame),
        Err(e) => eprintln!("❌ Error sending CAN Frame: {:?}", e),
    }
}
