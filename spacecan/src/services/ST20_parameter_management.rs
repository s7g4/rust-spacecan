use crate::primitives::packet::SpaceCANPacket;
use crate::services::core::Service;
use crate::{PacketData, ParamList};

pub struct ParameterManagementService {}

impl ParameterManagementService {
    pub fn new() -> Self {
        ParameterManagementService {}
    }

    pub fn report_parameter_values(&self, _params: ParamList) -> Result<SpaceCANPacket, &'static str> {
        SpaceCANPacket::new(20, 0, PacketData::new())
    }
}

impl Service for ParameterManagementService {
    fn service_type(&self) -> u8 { 20 }
    fn handle_packet(&mut self, _packet: &SpaceCANPacket) -> Result<(), &'static str> {
        Ok(())
    }
}
