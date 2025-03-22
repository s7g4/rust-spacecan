//Service for Splitting Large Packets

use spacecan::SpaceCanPacket;
use crate::split_packet::{split_packet, CanFrame};

pub struct PacketSplitterService;

impl PacketSplitterService {
    pub fn new() -> Self {
        PacketSplitterService
    }

    pub fn split_large_packet(&self, packet: &SpaceCanPacket, max_frame_size: usize) -> Vec<CanFrame> {
        split_packet(packet, max_frame_size)
    }
}

fn main() {
    // Example of splitting a large packet into frames
    let splitter_service = PacketSplitterService::new();

    let large_packet = SpaceCanPacket::new(1, vec![0; 300]);
    let frames = splitter_service.split_large_packet(&large_packet, 50);

    println!("Splitted Frames: {:?}", frames);
}
