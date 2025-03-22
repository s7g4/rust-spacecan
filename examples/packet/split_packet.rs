//Splitting large packets into CAN frames

use spacecan::SpaceCanPacket;

#[derive(Debug)]
pub struct CanFrame {
    pub id: u8,
    pub data: Vec<u8>,
}

pub fn split_packet(packet: &SpaceCanPacket, max_frame_size: usize) -> Vec<CanFrame> {
    let mut frames = Vec::new();
    let packet_data = &packet.data;
    let total_frames = (packet_data.len() + max_frame_size - 1) / max_frame_size; // Round up for total frames

    for i in 0..total_frames {
        let start = i * max_frame_size;
        let end = std::cmp::min(start + max_frame_size, packet_data.len());
        let frame_data = packet_data[start..end].to_vec();
        frames.push(CanFrame {
            id: packet.id,
            data: frame_data,
        });
    }
    frames
}

fn main() {
    // Example packet (simulate a large packet)
    let large_packet = SpaceCanPacket::new(1, vec![0; 300]);

    let frames = split_packet(&large_packet, 50); // Split into 50-byte frames

    println!("Splitted Frames: {:?}", frames);
}
