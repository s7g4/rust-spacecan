#[cfg(test)]
mod tests {
    use crate::protocol::SpaceCANProtocol;
    use crate::transport::base::BusImpl;
    use crate::primitives::packet::SpaceCANPacket;
    use crate::PacketData;

    #[test]
    fn test_protocol_flow() {
        let bus = BusImpl::new();
        let mut protocol = SpaceCANProtocol::new(bus, 1);

        let mut data = PacketData::new();
        data.push(0x42).unwrap();
        let packet = SpaceCANPacket::new(0x01, 0, data).unwrap();
        assert!(protocol.send_packet(&packet).is_ok());
    }
}
