//End-to-End Example of Packet Transmission & Reception

use spacecan::SpaceCanPacket;
use crate::split_packet::{split_packet, CanFrame};
use crate::reassemble_packet::reassemble_packet;

fn main() {
    // Step 1: Create a large packet to simulate data transmission
    let original_packet = SpaceCanPacket::new(1, vec![72, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100, 33]);

    println!("Original Packet: {:?}", original_packet);

    // Step 2: Split the packet into CAN frames (assuming max frame size of 5 bytes)
    let frames = split_packet(&original_packet, 5);

    println!("Splitted Frames: {:?}", frames);

    // Step 3: Reassemble the frames into the original packet
    let reassembled_packet = reassemble_packet(&frames);

    println!("Reassembled Packet: {:?}", reassembled_packet);
}
