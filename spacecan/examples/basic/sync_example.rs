use spacecan::primitives::sync::SyncProducer;
use spacecan::primitives::sync::SyncConsumer;
use std::sync::Arc;
use std::time::Duration;
use std::thread;

fn main() {
    // Initialize Sync Producer and Consumer
    let sync_producer = Arc::new(SyncProducer::new().expect("Failed to create SyncProducer"));
    let sync_consumer = Arc::new(SyncConsumer::new(Duration::from_secs(2)));

    // Start Sync Transmission in a separate thread
    let producer_clone = Arc::clone(&sync_producer);
    thread::spawn(move || {
        loop {
            if producer_clone.send().is_ok() {
                println!("📡 Sent Sync Signal");
            }
            thread::sleep(Duration::from_secs(1));
        }
    });

    println!("🛰️ Listening for Sync Signals...");
    loop {
        sync_consumer.receive_sync();
        if sync_consumer.check_timeout() {
            println!("⚠️ Sync Timeout Detected!");
        } else {
            println!("✅ Sync Signal Received");
        }
        thread::sleep(Duration::from_secs(1));
    }
}
