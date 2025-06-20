extern crate alloc;

use alloc::sync::Arc;
use alloc::vec;
#[cfg(feature = "std")]
use std::vec::Vec;
#[cfg(feature = "std")]
use std::println;


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

/// Trait defining the parent interface for sending packets.
trait Parent: Send + Sync {
    /// Sends a packet.
    fn send(&self, packet: Packet);
}

/// Controller for the Request Verification Service.
pub struct RequestVerificationServiceController {
    parent: Arc<dyn Parent>,
}

impl RequestVerificationServiceController {
    /// Creates a new controller with the given parent.
    pub fn new(parent: Arc<dyn Parent>) -> Self {
        RequestVerificationServiceController { parent }
    }
 
    /// Processes incoming packets based on service and subtype.
    pub fn process(&self, service: u8, subtype: u8, data: Vec<u8>, node_id: u32) {
        let case = (service, subtype);
        let source_packet = &data[0..2]; // Assuming data has at least 2 elements

        match case {
            (1, 1) => self.received_success_acceptance_report(node_id, source_packet),
            (1, 2) => self.received_fail_acceptance_report(node_id, source_packet),
            (1, 7) => self.received_success_completion_report(node_id, source_packet),
            (1, 8) => self.received_fail_completion_report(node_id, source_packet),
            _ => {}
        }
    }

    /// Handler for success acceptance report.
    pub fn received_success_acceptance_report(&self, node_id: u32, source_packet: &[u8]) {
        println!("Received success acceptance report from node {}: {:?}", node_id, source_packet);
    }

    /// Handler for fail acceptance report.
    pub fn received_fail_acceptance_report(&self, node_id: u32, source_packet: &[u8]) {
        println!("Received fail acceptance report from node {}: {:?}", node_id, source_packet);
    }

    /// Handler for success completion report.
    pub fn received_success_completion_report(&self, node_id: u32, source_packet: &[u8]) {
        println!("Received success completion report from node {}: {:?}", node_id, source_packet);
    }

    /// Handler for fail completion report.
    pub fn received_fail_completion_report(&self, node_id: u32, source_packet: &[u8]) {
        println!("Received fail completion report from node {}: {:?}", node_id, source_packet);
    }
}

/// Responder for the Request Verification Service.
pub struct RequestVerificationServiceResponder {
    parent: Arc<dyn Parent>,
}

impl RequestVerificationServiceResponder {
    /// Creates a new responder with the given parent.
    pub fn new(parent: Arc<dyn Parent>) -> Self {
        RequestVerificationServiceResponder { parent }
    }

    /// Processes incoming packets (implementation placeholder).
    pub fn process(&self, _service: u8, _subtype: u8, _data: Vec<u8>, _node_id: u32) {
        // Implementation for processing the request.
    }

    /// Sends a success acceptance report.
    pub fn send_success_acceptance_report(&self, source_packet: &[u8]) {
        let mut packet_data = vec![1, 1];
        packet_data.extend_from_slice(source_packet);
        self.parent.send(Packet::new(packet_data));
    }

    /// Sends a fail acceptance report.
    pub fn send_fail_acceptance_report(&self, source_packet: &[u8]) {
        let mut packet_data = vec![1, 2];
        packet_data.extend_from_slice(source_packet);
        self.parent.send(Packet::new(packet_data));
    }

    /// Sends a success completion report.
    pub fn send_success_completion_report(&self, source_packet: &[u8]) {
        let mut packet_data = vec![1, 7];
        packet_data.extend_from_slice(source_packet);
        self.parent.send(Packet::new(packet_data));
    }

    /// Sends a fail completion report.
    pub fn send_fail_completion_report(&self, source_packet: &[u8]) {
        let mut packet_data = vec![1, 8];
        packet_data.extend_from_slice(source_packet);
        self.parent.send(Packet::new(packet_data));
    }
}
