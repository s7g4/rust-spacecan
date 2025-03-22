const FULL_MASK: u32 = 0x7FF; // can frame id is 11 bits
const FUNCTION_MASK: u32 = 0x780; // 111 1000 0000
const NODE_MASK: u32 = 0x07F; // 000 0111 1111

// Mapping of CANopen COB-IDs
const ID_SYNC: u32 = 0x080;
const ID_HEARTBEAT: u32 = 0x700;
const ID_SCET: u32 = 0x180;
const ID_UTC: u32 = 0x200;
const ID_TC: u32 = 0x280;
const ID_TM: u32 = 0x300;
const ID_MESSAGE: u32 = 0x380;

#[derive(Debug)]
struct CanFrame {
    can_id: u32,
    data: Vec<u8>,
}

impl CanFrame {
    // Constructor for CanFrame
    fn new(can_id: u32, data: Option<Vec<u8>>) -> Result<Self, String> {
        if data.is_none() {
            return Ok(CanFrame {
                can_id,
                data: Vec::new(),
            });
        }

        let data = data.unwrap();
        if data.len() > 8 {
            return Err("not more than 8 data bytes allowed".to_string());
        }

        Ok(CanFrame { can_id, data })
    }

    // Get the length of the data
    fn len(&self) -> usize {
        self.data.len()
    }

    // Get the node ID
    fn get_node_id(&self) -> u32 {
        self.can_id & NODE_MASK
    }

    // Get the function ID
    fn get_func_id(&self) -> u32 {
        self.can_id & FUNCTION_MASK
    }
}

fn main() {
    // Example usage
    let can_frame = CanFrame::new(ID_SYNC, Some(vec![1, 2, 3, 4, 5])).unwrap();
    println!("{:?}", can_frame);
    println!("Node ID: {:#X}", can_frame.get_node_id());
    println!("Function ID: {:#X}", can_frame.get_func_id());
    println!("Data length: {}", can_frame.len());
}