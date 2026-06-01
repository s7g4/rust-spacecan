use crate::PacketData;
use crate::primitives::packet::SpaceCANPacket;
use crate::services::core::ServiceManager;

#[test]
fn test_service_routing() {
    let mut manager = ServiceManager::new();

    // Test routing to ST17
    let mut data = PacketData::new();
    let _ = data.push(0x01); // SUBTYPE = 1 (Connection test)
    let mut packet = SpaceCANPacket::new(100, 0, data).unwrap();
    packet.packet_type = 17;

    let result = manager.route_packet(&packet);
    assert!(result.is_ok());

    // Test invalid route
    let mut packet_invalid = SpaceCANPacket::new(100, 0, PacketData::new()).unwrap();
    packet_invalid.packet_type = 99;
    let result_invalid = manager.route_packet(&packet_invalid);
    assert!(result_invalid.is_err());
}
