#![cfg_attr(not(feature = "async"), allow(dead_code, unused_imports))]

#[cfg(feature = "async")]
use socketcan::{CanFrame, CanSocket, Socket, StandardId, EmbeddedFrame};
#[cfg(feature = "async")]
use tokio::time::{sleep, Duration};
#[cfg(feature = "async")]
use std::time::{SystemTime, UNIX_EPOCH};
#[cfg(feature = "async")]
use anyhow::{Result, anyhow};

#[cfg(feature = "async")]
#[tokio::main]
async fn main() -> Result<()> {
    // Open CAN socket on vcan0
    let socket: CanSocket = Socket::open("vcan0")?;

    let mut heartbeat_counter: u32 = 0;
    let mut sync_counter: u32 = 0;

    loop {
        // Send heartbeat frame every 1 second
        let heartbeat_id = StandardId::new(0x700).ok_or(anyhow!("Invalid StandardId"))?;
        let heartbeat_data = heartbeat_counter.to_be_bytes(); // 4 bytes
        let heartbeat_frame = CanFrame::new(heartbeat_id, &heartbeat_data)
            .ok_or(anyhow!("Failed to create heartbeat frame"))?;
        socket.write_frame(&heartbeat_frame)?;
        println!("Sent Heartbeat: counter={}", heartbeat_counter);
        heartbeat_counter = heartbeat_counter.wrapping_add(1);

        // Every 5 seconds send SYNC frame and time frames
        if sync_counter % 5 == 0 {
            // SYNC frame (ID 0x080, empty payload)
            let sync_id = StandardId::new(0x080).ok_or(anyhow!("Invalid StandardId"))?;
            let sync_frame = CanFrame::new(sync_id, &[])
                .ok_or(anyhow!("Failed to create sync frame"))?;
            socket.write_frame(&sync_frame)?;
            println!("Sent SYNC frame");

            // Send SCET (Spacecraft Event Time) - simulate as UNIX timestamp (u64)
            let scet = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs();
            let scet_bytes = scet.to_be_bytes();
            let scet_id = StandardId::new(0x100).ok_or(anyhow!("Invalid StandardId"))?;
            let scet_frame = CanFrame::new(scet_id, &scet_bytes[0..8])
                .ok_or(anyhow!("Failed to create SCET frame"))?;
            socket.write_frame(&scet_frame)?;
            println!("Sent SCET: {}", scet);

            // Send UTC time as seconds since UNIX_EPOCH too (just a demo)
            let utc = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs();
            let utc_bytes = utc.to_be_bytes();
            let utc_id = StandardId::new(0x101).ok_or(anyhow!("Invalid StandardId"))?;
            let utc_frame = CanFrame::new(utc_id, &utc_bytes[0..8])
                .ok_or(anyhow!("Failed to create UTC frame"))?;
            socket.write_frame(&utc_frame)?;
            println!("Sent UTC: {}", utc);
        }

        sync_counter += 1;
        sleep(Duration::from_secs(1)).await;
    }
}

#[cfg(not(feature = "async"))]
fn main() {
    println!("Async feature disabled. This binary does nothing.");
}
