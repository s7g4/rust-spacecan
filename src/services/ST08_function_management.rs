use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader};
use std::time::{Duration, SystemTime};
use std::sync::{Arc, Mutex};

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
    fn send(&self, packet: Packet);
}

#[derive(Debug)]
struct Argument {
    argument_id: u32,
    argument_name: String,
    encoding: String,
}

impl Argument {
    fn new(argument_id: u32, argument_name: String, encoding: String) -> Self {
        Argument {
            argument_id,
            argument_name,
            encoding,
        }
    }

    fn encode(&self, value: f64) -> Vec<u8> {
        let encoding = if self.encoding.starts_with("!") {
            &self.encoding
        } else {
            &format!("!{}", self.encoding)
        };
        // Use the appropriate encoding logic here
        // For demonstration, we will just return a placeholder
        vec![] // Placeholder for actual encoding
    }

    fn decode(&self, data: &[u8]) -> f64 {
        let encoding = if self.encoding.starts_with("!") {
            &self.encoding
        } else {
            &format!("!{}", self.encoding)
        };
        // Use the appropriate decoding logic here
        // For demonstration, we will just return a placeholder
        0.0 // Placeholder for actual decoding
    }

    fn get_encoded_size(&self) -> usize {
        // Calculate the size based on the encoding
        // For demonstration, we will just return a placeholder
        8 // Placeholder for actual size
    }
}

#[derive(Debug)]
struct Function {
    function_id: u32,
    function_name: String,
    arguments: HashMap<u32, Argument>,
}

impl Function {
    fn new(function_id: u32, function_name: String, arguments: Option<Vec<Argument>>) -> Self {
        let mut function = Function {
            function_id,
            function_name,
            arguments: HashMap::new(),
        };
        if let Some(args) = arguments {
            for arg in args {
                function.add_argument(arg);
            }
        }
        function
    }

    fn add_argument(&mut self, argument: Argument) {
        self.arguments.insert(argument.argument_id, argument);
    }

    fn get_argument(&self, argument_id: u32) -> Option<&Argument> {
        self.arguments.get(&argument_id)
    }
}

struct FunctionManagementService {
    parent: Arc<dyn Parent>,
    function_pool: HashMap<u32, Function>,
}

impl FunctionManagementService {
    fn new(parent: Arc<dyn Parent>) -> Self {
        FunctionManagementService {
            parent,
            function_pool: HashMap::new(),
        }
    }

    fn get_function(&self, function_id: u32) -> Option<&Function> {
        self.function_pool.get(&function_id)
    }

    fn add_function(&mut self, function: Function) {
        self.function_pool.insert(function.function_id, function);
    }
}

struct FunctionManagementServiceController {
    service: FunctionManagementService,
}

impl FunctionManagementServiceController {
    fn new(parent: Arc<dyn Parent>) -> Self {
        FunctionManagementServiceController {
            service: FunctionManagementService::new(parent),
        }
    }

    fn add_functions_from_file(&mut self, filepath: &str, node_id: u32) -> io::Result<()> {
        let file = File::open(filepath)?;
        let reader = BufReader::new(file);
        let json: Value = serde_json::from_reader(reader)?;

        if let Some(list_of_dicts) = json["functions"].as_array() {
            for function in list_of_dicts {
                let function_id = function["function_id"].as_u64().unwrap() as u32;
                let function_name = function["function_name"].as_str().unwrap().to_string();
                let arguments = function["arguments"].as_array().map(|args| {
                    args.iter().map(|arg| {
                        Argument::new(
                            arg["argument_id"].as_u64().unwrap() as u32,
                            arg["argument_name"].as_str().unwrap().to_string(),
                            arg["encoding"].as_str().unwrap().to_string(),
                        )
                    }).collect()
                });

                let function = Function::new(function_id, function_name, arguments);
                self.service.add_function(function);
            }
       