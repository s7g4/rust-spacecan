use spacecan::primitives::can_frame::CanFrame;
use spacecan::transport::base::BusImpl;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() {
    // Initialize the CAN bus
    let bus = Arc::new(BusImpl::new());
    
    println!("📡 Waiting for incoming CAN frames...");
    
    loop {
        // Attempt to receive a CAN frame
        if let Some(frame) = bus.get_frame() {
            println!("✅ Received CAN Frame: {:?}", frame);
        }
        
        // Sleep briefly to simulate real-time processing
        thread::sleep(Duration::from_millis(500));
    }
}
