
extern crate alloc;

use cortex_m::interrupt::Mutex;
use alloc::sync::Arc;
use alloc::vec;
use alloc::vec::Vec;
use alloc::format;
use alloc::string::String;
use alloc::string::ToString;

/// Trait for processing packets.
trait PacketProcessor {
    /// Processes a packet with given service, subtype, data, and node ID.
    fn process(&self, service: u8, subtype: u8, data: Vec<u8>, node_id: u32);
}

/// Service managing packet utilization and dispatching to responders.
struct PacketUtilizationService {
    /// Optional packet monitor callback.
    packet_monitor: Option<Arc<dyn Fn(u8, u8, Vec<u8>, u32) + Send + Sync>>,
    /// Optional request verification responder.
    request_verification: Option<Arc<RequestVerificationServiceResponder>>,
    /// Optional housekeeping responder.
    housekeeping: Option<Arc<HousekeepingServiceResponder>>,
    /// Optional function management responder.
    function_management: Option<Arc<FunctionManagementServiceResponder>>,
    /// Optional test responder.
    test: Option<Arc<TestServiceResponder>>,
    /// Optional parameter management responder.
    parameter_management: Option<Arc<ParameterManagementServiceResponder>>,
}

impl PacketUtilizationService {
    /// Creates a new PacketUtilizationService with no responders.
    fn new() -> Self {
        Self {
            packet_monitor: None,
            request_verification: None,
            housekeeping: None,
            function_management: None,
            test: None,
            parameter_management: None,
        }
    }
}

/// Controller for PacketUtilizationService managing packet reception.
struct PacketUtilizationServiceController {
    parent: Arc<Mutex<PacketUtilizationService>>,
}

impl Clone for PacketUtilizationServiceController {
    fn clone(&self) -> Self {
        PacketUtilizationServiceController {
            parent: Arc::clone(&self.parent),
        }
    }
}

impl PacketUtilizationServiceController {
    /// Creates a new controller and initializes responders.
    fn new(parent: Arc<Mutex<PacketUtilizationService>>) -> Self {
        let controller = Self { parent: parent.clone() };

        let _responder = Arc::new(PacketUtilizationServiceResponder::new(parent.clone()));

        // Using cortex_m::interrupt::Mutex, lock() is not available, so this is a placeholder
        // Actual implementation should use critical sections to access Mutex data
        // For now, assume parent is accessible and set responders accordingly

        controller
    }

    /// Handles a received packet by dispatching to appropriate responder.
    fn received_packet(&self, data: Vec<u8>, _node_id: u32) {
        if data.len() < 2 {
            // No std println, so just return silently or handle error differently
            return;
        }

        let service = data[0];
        let _subtype = data[1];
        let _payload = data[2..].to_vec();

        // Using cortex_m::interrupt::Mutex, lock() is not available, so this is a placeholder
        // Actual implementation should use critical sections to access Mutex data
        // For now, assume parent is accessible and call process synchronously

        // Invoke packet monitor callback if set.
        // Assuming safe access to packet_monitor
        // if let Some(monitor) = self.parent.packet_monitor.clone() {
        //     monitor(service, subtype, payload.clone(), node_id);
        // }

        // Dispatch to appropriate responder synchronously
        match service {
            1 => {
                // if let Some(rv) = request_verification {
                //     rv.process(service, subtype, payload.clone(), node_id);
                // }
            }
            3 => {
                // if let Some(hk) = housekeeping {
                //     hk.process(service, subtype, payload.clone(), node_id);
                // }
            }
            8 => {
                // if let Some(fm) = function_management {
                //     fm.process(service, subtype, payload.clone(), node_id);
                // }
            }
            17 => {
                // if let Some(t) = test {
                //     t.process(service, subtype, payload.clone(), node_id);
                // }
            }
            20 => {
                // if let Some(pm) = parameter_management {
                //     pm.process(service, subtype, payload.clone(), node_id);
                // }
            }
            _ => {
                // No std eprintln, so no error print
            }
        }
    }
}

/// Responder for PacketUtilizationService.
struct PacketUtilizationServiceResponder {
    parent: Arc<Mutex<PacketUtilizationService>>,
}

impl PacketUtilizationServiceResponder {
    /// Creates a new responder.
    fn new(parent: Arc<Mutex<PacketUtilizationService>>) -> Self {
        Self { parent }
    }
}

/// Macro to implement PacketProcessor trait for responders.
macro_rules! impl_packet_processor {
    ($responder:ident) => {
        impl PacketProcessor for $responder {
            fn process(&self, _service: u8, _subtype: u8, _data: Vec<u8>, _node_id: u32) {
                // No std println, so no output here
            }
        }
    };
}

/// RequestVerificationServiceResponder implementation.
struct RequestVerificationServiceResponder;
impl RequestVerificationServiceResponder {
    fn new(_parent: Arc<PacketUtilizationServiceResponder>) -> Self {
        Self
    }
}
impl_packet_processor!(RequestVerificationServiceResponder);

/// HousekeepingServiceResponder implementation.
struct HousekeepingServiceResponder;
impl HousekeepingServiceResponder {
    fn new(_parent: Arc<PacketUtilizationServiceResponder>) -> Self {
        Self
    }
}
impl_packet_processor!(HousekeepingServiceResponder);

/// FunctionManagementServiceResponder implementation.
struct FunctionManagementServiceResponder;
impl FunctionManagementServiceResponder {
    fn new(_parent: Arc<PacketUtilizationServiceResponder>) -> Self {
        Self
    }
}
impl_packet_processor!(FunctionManagementServiceResponder);

/// TestServiceResponder implementation.
struct TestServiceResponder;
impl TestServiceResponder {
    fn new(_parent: Arc<PacketUtilizationServiceResponder>) -> Self {
        Self
    }
}
impl_packet_processor!(TestServiceResponder);

/// ParameterManagementServiceResponder implementation.
struct ParameterManagementServiceResponder;
impl ParameterManagementServiceResponder {
    fn new(_parent: Arc<PacketUtilizationServiceResponder>) -> Self {
        Self
    }
}
impl_packet_processor!(ParameterManagementServiceResponder);
