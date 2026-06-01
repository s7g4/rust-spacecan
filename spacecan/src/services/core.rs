use crate::primitives::packet::SpaceCANPacket;
use crate::services::{
    ST01_request_verification::RequestVerificationService,
    ST03_housekeeping::HousekeepingService,
    ST08_function_management::FunctionManagementService,
    ST17_test::TestService,
    ST20_parameter_management::ParameterManagementService,
};

pub trait Service {
    fn service_type(&self) -> u8;
    fn handle_packet(&mut self, packet: &SpaceCANPacket) -> Result<(), &'static str>;
}

pub struct ServiceManager {
    st01: RequestVerificationService,
    st03: HousekeepingService,
    st08: FunctionManagementService,
    st17: TestService,
    st20: ParameterManagementService,
}

impl ServiceManager {
    pub fn new() -> Self {
        ServiceManager {
            st01: RequestVerificationService::new(),
            st03: HousekeepingService::new(),
            st08: FunctionManagementService::new(),
            st17: TestService::new(),
            st20: ParameterManagementService::new(),
        }
    }

    pub fn route_packet(&mut self, packet: &SpaceCANPacket) -> Result<(), &'static str> {
        match packet.packet_type {
            1 => self.st01.handle_packet(packet),
            3 => self.st03.handle_packet(packet),
            8 => self.st08.handle_packet(packet),
            17 => self.st17.handle_packet(packet),
            20 => self.st20.handle_packet(packet),
            _ => Err("Unknown service type"),
        }
    }
}
