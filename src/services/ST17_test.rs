use std::collections::HashMap;

#[derive(Debug)]
struct Packet {
    data: Vec<u8>,
}

impl Packet {
    fn new(data: Vec<u8>) -> Self {
        Packet { data }
    }
}

trait Parent {
    fn send(&self, packet: Packet, node_id: u32);
    fn request_verification(&self) -> &dyn RequestVerification;
}

trait RequestVerification {
    fn send_success_acceptance_report(&self, args: &[u8]);
    fn send_success_completion_report(&self, args: &[u8]);
    fn send_fail_acceptance_report(&self, args: &[u8]);
    fn send_fail_completion_report(&self, args: &[u8]);
}

struct TestServiceController {
    parent: Box<dyn Parent>,
}

impl TestServiceController {
    fn new(parent: Box<dyn Parent>) -> Self {
        TestServiceController { parent }
    }

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

    fn send_connection_test(&self, node_id: u32) {
        self.parent.send(Packet::new(vec![17, 1]), node_id);
    }

    fn send_application_connection_test(&self, node_id: u32, apid: u8) {
        let mut packet_data = vec![17, 3];
        packet_data.push(apid);
        self.parent.send(Packet::new(packet_data), node_id);
    }

    fn received_connection_test_report(&self, node_id: u32) {
        // To be overwritten
    }

    fn received_application_connection_test_report(&self, node_id: u32, apid: u8) {
        // To be overwritten
    }
}

struct TestServiceResponder {
    parent: Box<dyn Parent>,
}

impl TestServiceResponder {
    fn new(parent: Box<dyn Parent>) -> Self {
        TestServiceResponder { parent }
    }

    fn process(&self, service: u32, subtype: u32, data: Vec<u8>, node_id: u32) {
        let case = (service, subtype);

        match case {
            (17, 1) => {
                // Send success acceptance report
                self.parent.request_verification().send_success_acceptance_report(&[service as u8, subtype as u8]);

                // Reply with (17, 2)
                self.send_connection_test_report();

                // Send success completion report
                self.parent.request_verification().send_success_completion_report(&[service as u8, subtype as u8]);
            }
            (17, 3) => {
                let apid = data[0]; // Assuming data is at least 1 byte

                // Send success acceptance report
                self.parent.request_verification().send_success_acceptance_report(&[service as u8, subtype as u8]);

                // Run the connection test
                let result = self.received_application_connection_test(apid);

                if result {
                    // Reply with (17, 4)
                    self.send_application_connection_test_report(apid);

                    // Send success completion report
                    self.parent.request_verification().send_success_completion_report(&[service as u8, subtype as u8]);
                } else {
                    // Send fail completion report
                    self.parent.request_verification().send_fail_completion_report(&[service as u8, subtype as u8]);
                }
            }
            _ => {}
        }
    }

    fn send_connection_test_report(&self) {
        // To be implemented
    }

    fn send_application_connection_test_report(&self, apid: u8) {
        // To be implemented
    }

    fn received_application_connection_test(&self, apid: u8) -> bool {
        // To be implemented
        true
    }
}
