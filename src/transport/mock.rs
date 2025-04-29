// Stub mock transport module

pub struct MockTransport;

impl MockTransport {
    pub fn new() -> Self {
        MockTransport
    }

    pub fn send(&self, data: &[u8]) {
        // Mock send implementation
        println!("MockTransport sending data: {:?}", data);
    }
}

// Added receive method separately due to previous write error
impl MockTransport {
    pub fn receive(&self) -> Result<Vec<u8>, String> {
        // Mock receive implementation returning dummy data
        println!("MockTransport receiving data");
        Ok(vec![])
    }
}
