//A Service for Handling Packets

use spacecan::SpaceCanPacket;

pub struct PacketService {
    id: u8,
}

impl PacketService {
    pub fn new(id: u8) -> Self {
        PacketService { id }
    }

    pub fn process_packet(&self, packet: SpaceCanPacket) {
        println!("Service {:?} processing packet with ID: {}", self.id, packet.id);
        println!("Packet data: {:?}", packet.data);
    }

    pub fn create_response(&self, packet: &SpaceCanPacket) -> SpaceCanPacket {
        // Simulate creating a response based on the received packet
        let response_data = format!("Received packet with ID: {}", packet.id);
        SpaceCanPacket::new(self.id, response_data.into_bytes())
    }
}

fn main() {
    // Example of service processing a packet
    let packet_service = PacketService::new(1);

    let incoming_packet = SpaceCanPacket::new(1, vec![72, 101, 108, 108, 111]);
    packet_service.process_packet(incoming_packet);

    // Creating a response packet
    let response_packet = packet_service.create_response(&incoming_packet);
    println!("Service response: {:?}", response_packet);
}
