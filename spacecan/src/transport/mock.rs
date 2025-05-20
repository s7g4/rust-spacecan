// Stub mock transport module

pub struct MockTransport {
    last_sent: std::sync::Mutex<Option<Vec<u8>>>,
}

impl MockTransport {
    pub fn new() -> Self {
        MockTransport {
            last_sent: std::sync::Mutex::new(None),
        }
    }

    pub fn send(&self, data: &[u8]) {
        // Mock send implementation
        println!("MockTransport sending data: {:?}", data);
        let mut last_sent = self.last_sent.lock().unwrap();
        *last_sent = Some(data.to_vec());
    }
}

impl MockTransport {
    pub fn receive(&self) -> Result<Vec<u8>, String> {
        // Mock receive implementation returning last sent data
        println!("MockTransport receiving data");
        let last_sent = self.last_sent.lock().unwrap();
        match &*last_sent {
            Some(data) => Ok(data.clone()),
            None => Err("No data available".to_string()),
        }
    }
}
