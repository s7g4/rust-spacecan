//Reassembling packets from received frames

use spacecan::SpaceCanPacket;
use crate::split_packet::CanFrame;

pub fn reassemble_packet(frames: &[CanFrame]) -> SpaceCanPacket {
    let mut packet_data = Vec::new();
    let id = frames[0].id;

    for frame in frames {
        packet_data.extend_from_slice(&frame.data);
    }

    SpaceCanPacket::new(id, packet_data)
}

fn main() {
    // Simulate receiving frames (splitted data)
    let frames = vec![
        CanFrame { id: 1, data: vec![0; 50] },
        CanFrame { id: 1, data: vec![0; 50] },
        CanFrame { id: 1, data: vec![0; 50] },
    ];

    let reassembled_packet = reassemble_packet(&frames);
    println!("Reassembled Packet: {:?}", reassembled_packet);
}
