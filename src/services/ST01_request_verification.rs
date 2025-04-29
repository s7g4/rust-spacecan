#[derive(Debug)]
struct Packet {
    data: Vec<u8>,
}

impl Packet {
    fn new(data: Vec<u8>) -> Self {
        Packet { data }
    }
}

use std::sync::Arc;

trait Parent: Send + Sync {
    fn send(&self, packet: Packet);
}

struct RequestVerificationServiceController {
    parent: Arc<dyn Parent>,
}

impl RequestVerificationServiceController {
    fn new(parent: Arc<dyn Parent>) -> Self {
        RequestVerificationServiceController { parent }
    }

    fn process(&self, service: u8, subtype: u8, data: Vec<u8>, node_id: u32) {
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

    fn received_success_acceptance_report(&self, node_id: u32, source_packet: &[u8]) {
        // To be overwritten
    }

    fn received_fail_acceptance_report(&self, node_id: u32, source_packet: &[u8]) {
        // To be overwritten
    }

    fn received_success_completion_report(&self, node_id: u32, source_packet: &[u8]) {
        // To be overwritten
    }

    fn received_fail_completion_report(&self, node_id: u32, source_packet: &[u8]) {
        // To be overwritten
    }
}

struct RequestVerificationServiceResponder {
    parent: Arc<dyn Parent>,
}

impl RequestVerificationServiceResponder {
    fn new(parent: Arc<dyn Parent>) -> Self {
        RequestVerificationServiceResponder { parent }
    }

    fn process(&self, _service: u8, _subtype: u8, _data: Vec<u8>, _node_id: u32) {
        // Implementation for processing the request
    }

    fn send_success_acceptance_report(&self, source_packet: &[u8]) {
        let mut packet_data = vec![1, 1];
        packet_data.extend_from_slice(source_packet);
        self.parent.send(Packet::new(packet_data));
    }

    fn send_fail_acceptance_report(&self, source_packet: &[u8]) {
        let mut packet_data = vec![1, 2];
        packet_data.extend_from_slice(source_packet);
        self.parent.send(Packet::new(packet_data));
    }

    fn send_success_completion_report(&self, source_packet: &[u8]) {
        let mut packet_data = vec![1, 7];
        packet_data.extend_from_slice(source_packet);
        self.parent.send(Packet::new(packet_data));
    }

    fn send_fail_completion_report(&self, source_packet: &[u8]) {
        let mut packet_data = vec![1, 8];
        packet_data.extend_from_slice(source_packet);
        self.parent.send(Packet::new(packet_data));
    }
}

// Example implementation of the Parent trait
struct ExampleParent;

impl Parent for ExampleParent {
    fn send(&self, packet: Packet) {
        println!("Sending packet: {:?}", packet);
    }
}

fn main() {
    use std::sync::Arc;
    let parent: Arc<dyn Parent> = Arc::new(ExampleParent);
    let controller = RequestVerificationServiceController::new(parent.clone());
    let responder = RequestVerificationServiceResponder::new(parent.clone());

    // Example usage
    let data = vec![0, 1]; // Example data
    controller.process(1, 1, data.clone(), 42);
    responder.send_success_acceptance_report(&data);
}
