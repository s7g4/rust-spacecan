//Full Service Demo for Packet Transmission & Reception

use spacecan::SpaceCanPacket;
use crate::split_packet::{split_packet, CanFrame};
use crate::reassemble_packet::reassemble_packet;
use crate::services::packet_service::PacketService;

fn main() {
    // Step 1: Create a packet to simulate packet transmission
    let original_packet = SpaceCanPacket::new(1, vec![72, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100]);

    println!("Original Packet: {:?}", original_packet);

    // Step 2: Split the packet into CAN frames (assuming max frame size of 5 bytes)
    let frames = split_packet(&original_packet, 5);

    println!("Splitted Frames: {:?}", frames);

    // Step 3: Simulate a service receiving the frames and reassembling the packet
    let reassembled_packet = reassemble_packet(&frames);

    println!("Reassembled Packet: {:?}", reassembled_packet);

    // Step 4: Process the reassembled packet using the service
    let packet_service = PacketService::new(1);
    packet_service.process_packet(reassembled_packet);

    // Step 5: Create a response packet from the service
    let response_packet = packet_service.create_response(&reassembled_packet);
    println!("Service Response: {:?}", response_packet);
}
