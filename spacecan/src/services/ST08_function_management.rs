use crate::primitives::packet::SpaceCANPacket;
use crate::services::core::Service;
use crate::PacketData;

pub struct FunctionManagementService {}

impl FunctionManagementService {
    pub fn new() -> Self {
        FunctionManagementService {}
    }

    pub fn perform_function(&mut self, _func_id: u16, _args: PacketData) -> Result<(), &'static str> {
        Ok(())
    }
}

impl Service for FunctionManagementService {
    fn service_type(&self) -> u8 { 8 }
    fn handle_packet(&mut self, _packet: &SpaceCANPacket) -> Result<(), &'static str> {
        Ok(())
    }
}
