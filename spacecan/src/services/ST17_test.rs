/// Test Service module.
///
/// This module defines the controller and responder for the Test Service,
/// handling connection test packets and interaction with request verification.

extern crate alloc;

use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use core::option::Option;
use core::option::Option::{Some, None};
use core::result::Result;
use core::result::Result::{Ok, Err};

/// Represents a packet with data payload.
#[derive(Debug)]
struct Packet {
    data: Vec<u8>,
}

impl Packet {
    /// Creates a new packet with the given data.
    fn new(data: Vec<u8>) -> Self {
        Packet { data }
    }
}

/// Trait defining the parent interface for sending packets and accessing request verification.
trait Parent {
    /// Sends a packet to a node.
    fn send(&self, packet: Packet, node_id: u32);
    /// Provides access to the request verification interface.
    fn request_verification(&self) -> &dyn RequestVerification;
}

/// Trait defining request verification methods.
trait RequestVerification {
    fn send_success_acceptance_report(&self, args: &[u8]);
    fn send_success_completion_report(&self, args: &[u8]);
    fn send_fail_acceptance_report(&self, args: &[u8]);
    fn send_fail_completion_report(&self, args: &[u8]);
}

/// Controller for the Test Service.
struct TestServiceController {
    parent: Box<dyn Parent>,
}

impl TestServiceController {
    /// Creates a new controller with the given parent.
    fn new(parent: Box<dyn Parent>) -> Self {
        TestServiceController { parent }
    }

    /// Processes incoming packets based on service and subtype.
    fn process(&self, service: u32, subtype: u32, data: Vec<u8>, node_id: u32) {
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
    fn send_connection_test(&self, node_id: u32) {
        self.parent.send(Packet::new(vec![17, 1]), node_id);
    }

    /// Sends an application connection test packet with APID.
    fn send_application_connection_test(&self, node_id: u32, apid: u8) {
        let mut packet_data = vec![17, 3];
        packet_data.push(apid);
        self.parent.send(Packet::new(packet_data), node_id);
    }

    /// Handler for received connection test report.
    fn received_connection_test_report(&self, _node_id: u32) {
        // To be overwritten.
    }

    /// Handler for received application connection test report.
    fn received_application_connection_test_report(&self, _node_id: u32, _apid: u8) {
        // To be overwritten.
    }
}

/// Responder for the Test Service.
struct TestServiceResponder {
    parent: Box<dyn Parent>,
}

impl TestServiceResponder {
    /// Creates a new responder with the given parent.
    fn new(parent: Box<dyn Parent>) -> Self {
        TestServiceResponder { parent }
    }

    /// Processes incoming packets based on service and subtype.
    fn process(&self, service: u32, subtype: u32, data: Vec<u8>, _node_id: u32) {
        let case = (service, subtype);

        match case {
            (17, 1) => {
                // Send success acceptance report.
                self.parent.request_verification().send_success_acceptance_report(&[service as u8, subtype as u8]);

                // Reply with connection test report (17, 2).
                self.send_connection_test_report();

                // Send success completion report.
                self.parent.request_verification().send_success_completion_report(&[service as u8, subtype as u8]);
            }
            (17, 3) => {
                let apid = data[0]; // Assuming data is at least 1 byte.

                // Send success acceptance report.
                self.parent.request_verification().send_success_acceptance_report(&[service as u8, subtype as u8]);

                // Run the connection test.
                let result = self.received_application_connection_test(apid);

                if result {
                    // Reply with application connection test report (17, 4).
                    self.send_application_connection_test_report(apid);

                    // Send success completion report.
                    self.parent.request_verification().send_success_completion_report(&[service as u8, subtype as u8]);
                } else {
                    // Send fail completion report.
                    self.parent.request_verification().send_fail_completion_report(&[service as u8, subtype as u8]);
                }
            }
            _ => {}
        }
    }

    /// Sends a connection test report.
    fn send_connection_test_report(&self) {
        // To be implemented.
    }

    /// Sends an application connection test report.
    fn send_application_connection_test_report(&self, _apid: u8) {
        // To be implemented.
    }

    /// Handles an application connection test.
    fn received_application_connection_test(&self, _apid: u8) -> bool {
        // To be implemented.
        true
    }
}
