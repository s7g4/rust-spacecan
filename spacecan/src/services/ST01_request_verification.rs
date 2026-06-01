use crate::PacketData;
use crate::primitives::packet::SpaceCANPacket;
use crate::services::core::Service;
use heapless::FnvIndexMap;

pub struct RequestVerificationService {
    pending_requests: FnvIndexMap<u16, u8, 16>,
}

impl Default for RequestVerificationService {
    fn default() -> Self {
        Self::new()
    }
}

impl RequestVerificationService {
    pub fn new() -> Self {
        RequestVerificationService {
            pending_requests: FnvIndexMap::new(),
        }
    }

    pub fn accept_telecommand(
        &mut self,
        app_id: u16,
        sequence_count: u32,
    ) -> Result<SpaceCANPacket, &'static str> {
        let _ = self.pending_requests.insert(app_id, 1);
        let mut data = PacketData::new();
        let _ = data.extend_from_slice(&sequence_count.to_be_bytes());
        SpaceCANPacket::new(app_id, 0, data)
    }
}

impl Service for RequestVerificationService {
    fn service_type(&self) -> u8 {
        1
    }
    fn handle_packet(&mut self, _packet: &SpaceCANPacket) -> Result<(), &'static str> {
        Ok(())
    }
}
