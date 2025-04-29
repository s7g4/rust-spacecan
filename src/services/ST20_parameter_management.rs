use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader};
use std::mem::size_of;

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

#[derive(Debug)]
struct Parameter {
    parameter_id: (u32, u32),
    parameter_name: String,
    encoding: String,
    value: f64, // Assuming the value is a float for this example
}

impl Parameter {
    fn new(parameter_id: (u32, u32), parameter_name: String, encoding: String, value: f64) -> Self {
        Parameter {
            parameter_id,
            parameter_name,
            encoding,
            value,
        }
    }

    fn encode(&self) -> Vec<u8> {
        let encoding = if self.encoding.starts_with("!") {
            &self.encoding
        } else {
            &format!("!{}", self.encoding)
        };
        let size = self.get_encoded_size();
        let mut buffer = vec![0; size];
        // Here we would use the appropriate encoding logic
        // For demonstration, we will just return a placeholder
        buffer // Placeholder for actual encoding
    }

    fn decode(&self, data: &[u8]) -> f64 {
        let encoding = if self.encoding.starts_with("!") {
            &self.encoding
        } else {
            &format!("!{}", self.encoding)
        };
        // Here we would use the appropriate decoding logic
        // For demonstration, we will just return a placeholder
        0.0 // Placeholder for actual decoding
    }

    fn get_encoded_size(&self) -> usize {
        // Calculate the size based on the encoding
        // For demonstration, we will just return a placeholder
        size_of::<f64>() // Placeholder for actual size
    }
}

struct ParameterManagementService {
    parent: Box<dyn Parent>,
    parameter_pool: HashMap<(u32, u32), Parameter>,
}

impl ParameterManagementService {
    fn new(parent: Box<dyn Parent>) -> Self {
        ParameterManagementService {
            parent,
            parameter_pool: HashMap::new(),
        }
    }

    fn add_parameter(&mut self, parameter: Parameter) {
        self.parameter_pool.insert(parameter.parameter_id, parameter);
    }

    fn get_parameter(&self, parameter_id: (u32, u32)) -> Option<&Parameter> {
        self.parameter_pool.get(&parameter_id)
    }

    fn set_parameter_value(&mut self, parameter_id: (u32, u32), value: f64) {
        if let Some(param) = self.parameter_pool.get_mut(&parameter_id) {
            param.value = value;
        }
    }

    fn get_parameter_value(&self, parameter_id: (u32, u32)) -> Option<f64> {
        self.parameter_pool.get(&parameter_id).map(|p| p.value)
    }

    fn get_parameter_encoding(&self, parameter_id: (u32, u32)) -> Option<&String> {
        self.parameter_pool.get(&parameter_id).map(|p| &p.encoding)
    }
}

struct ParameterManagementServiceController {
    service: ParameterManagementService,
}

impl ParameterManagementServiceController {
    fn new(parent: Box<dyn Parent>) -> Self {
        ParameterManagementServiceController {
            service: ParameterManagementService::new(parent),
        }
    }

    fn add_parameters_from_file(&mut self, filepath: &str, node_id: u32) -> io::Result<()> {
        let file = File::open(filepath)?;
        let reader = BufReader::new(file);
        let json: Value = serde_json::from_reader(reader)?;

        if let Some(list_of_dicts) = json["parameters"].as_array() {
            for param in list_of_dicts {
                let parameter_id = (node_id, param["parameter_id"].as_u64().unwrap() as u32);
                let parameter_name = param["parameter_name"].as_str().unwrap().to_string();

                let encoding = param["encoding"].as_str().unwrap().to_string();
                let value = 0.0; // Default value, can be updated later

                let parameter = Parameter::new(parameter_id, parameter_name, encoding, value);
                self.service.add_parameter(parameter);
            }
        }
        Ok(())
    }
}
