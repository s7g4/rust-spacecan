extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;
use alloc::boxed::Box;
use alloc::sync::Arc;

// Cannot import private traits and structs from ST01_request_verification, so redefine minimal needed here:

/// Represents a packet with data payload.
#[derive(Debug)]
pub struct Packet {
    pub data: Vec<u8>,
}

impl Packet {
    /// Creates a new packet with the given data.
    pub fn new(data: Vec<u8>) -> Self {
        Packet { data }
    }
}

/// Trait defining the parent interface for sending packets.
pub trait Parent: Send + Sync {
    fn send(&self, packet: Packet);
}

/// Mock or placeholder for RequestVerificationServiceResponder interface
pub trait RequestVerification {
    fn send_success_acceptance_report(&self, args: &[u8]);
    fn send_success_completion_report(&self, args: &[u8]);
    fn send_fail_acceptance_report(&self, args: &[u8]);
    fn send_fail_completion_report(&self, args: &[u8]);
}

/// Dummy struct to implement RequestVerification trait for testing
pub struct RequestVerificationServiceResponder;

impl RequestVerificationServiceResponder {
    pub fn new(_parent: Arc<dyn Parent>) -> Self {
        RequestVerificationServiceResponder
    }
}

impl RequestVerification for RequestVerificationServiceResponder {
    fn send_success_acceptance_report(&self, _args: &[u8]) {}
    fn send_success_completion_report(&self, _args: &[u8]) {}
    fn send_fail_acceptance_report(&self, _args: &[u8]) {}
    fn send_fail_completion_report(&self, _args: &[u8]) {}
}

/// Controller for the Test Service.
pub struct TestServiceController {
    parent: Arc<dyn Parent>,
}

impl TestServiceController {
    /// Creates a new controller with the given parent.
    pub fn new(parent: Arc<dyn Parent>) -> Self {
        TestServiceController { parent }
    }

    /// Processes incoming packets based on service and subtype.
    pub fn process(&self, service: u8, subtype: u8, data: Vec<u8>, node_id: u32) {
        let case = (service, subtype);

        match case {
            (17, 2) => self.received_connection_test_report(node_id),
            (17, 4) => {
                let apid = data[0]; // Assuming data is at least 1 byte
                self.received_application_connection_test_report(node_id, apid);
            }
            _ => {}
        }
    }

    /// Sends a connection test packet.
    pub fn send_connection_test(&self, node_id: u32) {
        self.parent.send(Packet::new(vec![17, 1]));
    }

    /// Sends an application connection test packet with APID.
    pub fn send_application_connection_test(&self, node_id: u32, apid: u8) {
        let mut packet_data = vec![17, 3];
        packet_data.push(apid);
        self.parent.send(Packet::new(packet_data));
    }

    /// Handler for received connection test report.
    pub fn received_connection_test_report(&self, _node_id: u32) {
        // To be overwritten.
    }

    /// Handler for received application connection test report.
    pub fn received_application_connection_test_report(&self, _node_id: u32, _apid: u8) {
        // To be overwritten.
    }
}

/// Responder for the Test Service.
pub struct TestServiceResponder {
    parent: Arc<dyn Parent>,
    request_verification: RequestVerificationServiceResponder,
}

impl TestServiceResponder {
    /// Creates a new responder with the given parent.
    pub fn new(parent: Arc<dyn Parent>) -> Self {
        let request_verification = RequestVerificationServiceResponder::new(parent.clone());
        TestServiceResponder { parent, request_verification }
    }

    /// Processes incoming packets based on service and subtype.
    pub fn process(&self, service: u8, subtype: u8, data: Vec<u8>, node_id: u32) {
        let case = (service, subtype);

        match case {
            (17, 1) => {
                // Send success acceptance report.
                self.request_verification.send_success_acceptance_report(&[service, subtype]);

                // Reply with connection test report (17, 2).
                self.send_connection_test_report(node_id);

                // Send success completion report.
                self.request_verification.send_success_completion_report(&[service, subtype]);
            }
            (17, 3) => {
                let apid = data[0]; // Assuming data is at least 1 byte.

                // Send success acceptance report.
                self.request_verification.send_success_acceptance_report(&[service, subtype]);

                // Run the connection test.
                let result = self.received_application_connection_test(apid);

                if result {
                    // Reply with application connection test report (17, 4).
                    self.send_application_connection_test_report(node_id, apid);

                    // Send success completion report.
                    self.request_verification.send_success_completion_report(&[service, subtype]);
                } else {
                    // Send fail completion report.
                    self.request_verification.send_fail_completion_report(&[service, subtype]);
                }
            }
            _ => {}
        }
    }

    /// Sends a connection test report.
    pub fn send_connection_test_report(&self, node_id: u32) {
        self.parent.send(Packet::new(vec![17, 2]));
    }

    /// Sends an application connection test report.
    pub fn send_application_connection_test_report(&self, node_id: u32, apid: u8) {
        self.parent.send(Packet::new(vec![17, 4, apid]));
    }

    /// Handles an application connection test.
    pub fn received_application_connection_test(&self, _apid: u8) -> bool {
        // Implement actual test logic here.
        true
    }
}

/// Mock implementation of Parent for testing.
struct MockParent;

impl Parent for MockParent {
    fn send(&self, packet: Packet) {
        // Mock send: print or log the packet
        // e.g. println!("Sending packet: {:?}", packet);
    }
}

/// Test function to demonstrate the Test Service functionality.
pub fn st17_test() {
    let parent = Arc::new(MockParent);
    let controller = TestServiceController::new(parent.clone());
    let responder = TestServiceResponder::new(parent.clone());

    let node_id = 1;

    // Controller sends connection test
    controller.send_connection_test(node_id);

    // Controller sends application connection test with apid 42
    controller.send_application_connection_test(node_id, 42);

    // Responder processes connection test packet (service 17, subtype 1)
    responder.process(17, 1, vec![], node_id);

    // Responder processes application connection test packet (service 17, subtype 3, apid 42)
    responder.process(17, 3, vec![42], node_id);
}
