let heartbeat_producer = Arc::new(HeartbeatProducer::new().expect("Failed to create HeartbeatProducer"));
let heartbeat_consumer = Arc::new(HeartbeatConsumer::new(Duration::from_secs(3)));

// Start Heartbeat Transmission in a separate thread
let producer_clone = Arc::clone(&heartbeat_producer);
thread::spawn(move || {
    loop {
        if let Err(e) = producer_clone.send() {
            eprintln!("Error sending heartbeat signal: {}", e);
        } else {
            println!("❤️ Sent Heartbeat Signal");
        }
        thread::sleep(Duration::from_secs(2));
    }
});

println!("📡 Listening for Heartbeat Signals...");
loop {
    if let Err(e) = heartbeat_consumer.receive_heartbeat() {
        eprintln!("Error receiving heartbeat signal: {}", e);
    }
    if heartbeat_consumer.check_timeout() {
        println!("⚠️ Heartbeat Timeout Detected!");
    } else {
        println!("✅ Heartbeat Signal Received");
    }
    thread::sleep(Duration::from_secs(2));
}use spacecan::primitives::heartbeat::{HeartbeatProducer, HeartbeatConsumer};
use std::sync::Arc;
use std::time::Duration;
use std::thread;

fn main() {
    // Initialize Heartbeat Producer and Consumer
    let heartbeat_producer = Arc::new(HeartbeatProducer::new().expect("Failed to create HeartbeatProducer"));
    let heartbeat_consumer = Arc::new(HeartbeatConsumer::new(Duration::from_secs(3)));

    // Start Heartbeat Transmission in a separate thread
    let producer_clone = Arc::clone(&heartbeat_producer);
    thread::spawn(move || {
        loop {
            if producer_clone.send().is_ok() {
                println!("❤️ Sent Heartbeat Signal");
            }
            thread::sleep(Duration::from_secs(2));
        }
    });

    println!("📡 Listening for Heartbeat Signals...");
    loop {
        heartbeat_consumer.receive_heartbeat();
        if heartbeat_consumer.check_timeout() {
            println!("⚠️ Heartbeat Timeout Detected!");
        } else {
            println!("✅ Heartbeat Signal Received");
        }
        thread::sleep(Duration::from_secs(2));
    }
}
