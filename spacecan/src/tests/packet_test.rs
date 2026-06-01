#[cfg(test)]
mod tests {
    use crate::primitives::packet::SpaceCANPacket;
    use crate::PacketData;

    #[test]
    fn test_packet_creation() {
        let mut data = PacketData::new();
        data.push(0x42).unwrap();
        let packet = SpaceCANPacket::new(0x01, 0, data).unwrap();
        assert_eq!(packet.application_process_id, 0x01);
    }
}
