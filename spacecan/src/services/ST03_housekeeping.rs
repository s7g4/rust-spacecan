use crate::primitives::packet::SpaceCANPacket;
use crate::services::core::Service;
use heapless::FnvIndexMap;
use crate::{PacketData, ParamList};

pub struct HousekeepingService {
    reports: FnvIndexMap<u16, ParamList, 16>,
    parameters: FnvIndexMap<u16, heapless::Vec<u8, 8>, 64>,
}

impl HousekeepingService {
    pub fn new() -> Self {
        HousekeepingService {
            reports: FnvIndexMap::new(),
            parameters: FnvIndexMap::new(),
        }
    }

    pub fn create_report(&mut self, params: ParamList, id: u32) {
        let _ = self.reports.insert(id as u16, params);
    }
}

impl Service for HousekeepingService {
    fn service_type(&self) -> u8 { 3 }
    fn handle_packet(&mut self, _packet: &SpaceCANPacket) -> Result<(), &'static str> {
        Ok(())
    }
}
