use crate::PacketData;
use crate::primitives::packet::SpaceCANPacket;
use crate::services::core::Service;

pub struct FunctionManagementService {}

impl Default for FunctionManagementService {
    fn default() -> Self {
        Self::new()
    }
}

impl FunctionManagementService {
    pub fn new() -> Self {
        FunctionManagementService {}
    }

    pub fn perform_function(
        &mut self,
        _func_id: u16,
        _args: PacketData,
    ) -> Result<(), &'static str> {
        Ok(())
    }
}

impl Service for FunctionManagementService {
    fn service_type(&self) -> u8 {
        8
    }
    fn handle_packet(&mut self, _packet: &SpaceCANPacket) -> Result<(), &'static str> {
        Ok(())
    }
}
