use crate::PacketData;
use crate::primitives::packet::SpaceCANPacket;
use crate::services::core::Service;

pub struct TestService {}

impl Default for TestService {
    fn default() -> Self {
        Self::new()
    }
}

impl TestService {
    pub fn new() -> Self {
        TestService {}
    }

    pub fn create_connection_test(&mut self, id: u32) -> SpaceCANPacket {
        let mut data = PacketData::new();
        let _ = data.extend_from_slice(&id.to_be_bytes());
        SpaceCANPacket::new(17, 0, data).unwrap()
    }
}

impl Service for TestService {
    fn service_type(&self) -> u8 {
        17
    }
    fn handle_packet(&mut self, _packet: &SpaceCANPacket) -> Result<(), &'static str> {
        Ok(())
    }
}
