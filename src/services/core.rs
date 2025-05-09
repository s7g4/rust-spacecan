use std::sync::{Arc, Mutex};
use std::thread;

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

        let responder = Arc::new(PacketUtilizationServiceResponder::new(parent.clone()));

        let mut parent_service = parent.lock().unwrap();
        parent_service.request_verification = Some(Arc::new(RequestVerificationServiceResponder::new(responder.clone())));
        parent_service.housekeeping = Some(Arc::new(HousekeepingServiceResponder::new(responder.clone())));
        parent_service.function_management = Some(Arc::new(FunctionManagementServiceResponder::new(responder.clone())));
        parent_service.test = Some(Arc::new(TestServiceResponder::new(responder.clone())));
        parent_service.parameter_management = Some(Arc::new(ParameterManagementServiceResponder::new(responder.clone())));

        controller
    }

    /// Handles a received packet by dispatching to appropriate responder.
    fn received_packet(&self, data: Vec<u8>, node_id: u32) {
        if data.len() < 2 {
            eprintln!("Invalid packet: insufficient data");
            return;
        }

        let service = data[0];
        let subtype = data[1];
        let payload = data[2..].to_vec();

        // Invoke packet monitor callback if set.
        if let Some(monitor) = self.parent.lock().unwrap().packet_monitor.clone() {
            monitor(service, subtype, payload.clone(), node_id);
        }

        // Extract service responders.
        let parent = self.parent.lock().unwrap();
        let request_verification = parent.request_verification.clone();
        let housekeeping = parent.housekeeping.clone();
        let function_management = parent.function_management.clone();
        let test = parent.test.clone();
        let parameter_management = parent.parameter_management.clone();
        drop(parent); // Release lock early.

        // Dispatch to appropriate responder in a new thread.
        match service {
            1 => {
                if let Some(rv) = request_verification {
                    let data_clone = payload.clone();
                    thread::spawn(move || rv.process(service, subtype, data_clone, node_id));
                }
            }
            3 => {
                if let Some(hk) = housekeeping {
                    let data_clone = payload.clone();
                    thread::spawn(move || hk.process(service, subtype, data_clone, node_id));
                }
            }
            8 => {
                if let Some(fm) = function_management {
                    let data_clone = payload.clone();
                    thread::spawn(move || fm.process(service, subtype, data_clone, node_id));
                }
            }
            17 => {
                if let Some(t) = test {
                    let data_clone = payload.clone();
                    thread::spawn(move || t.process(service, subtype, data_clone, node_id));
                }
            }
            20 => {
                if let Some(pm) = parameter_management {
                    let data_clone = payload.clone();
                    thread::spawn(move || pm.process(service, subtype, data_clone, node_id));
                }
            }
            _ => {
                eprintln!("Unknown service: {}", service);
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
            fn process(&self, service: u8, subtype: u8, data: Vec<u8>, node_id: u32) {
                println!(
                    "{} processing: service={}, subtype={}, data={:?}, node_id={}",
                    stringify!($responder),
                    service,
                    subtype,
                    data,
                    node_id
                );
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
