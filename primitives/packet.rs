use std::collections::HashMap;

const MAX_DATA_LENGTH: usize = 6;

#[derive(Debug)]
struct Packet {
    data: Vec<u8>,
}

impl Packet {
    fn new(data: Option<Vec<u8>>) -> Self {
        let data = data.unwrap_or_else(|| Vec::new());
        Packet { data }
    }

    fn split(&self) -> Vec<Vec<u8>> {
        let total_frames = (self.data.len() + MAX_DATA_LENGTH - 1) / MAX_DATA_LENGTH; // Ceiling division
        let total_frames = total_frames.max(1); // Ensure at least one frame

        let mut frames = Vec::new();
        for n in 0..total_frames {
            let start = n * MAX_DATA_LENGTH;
            let end = start + MAX_DATA_LENGTH;
            let data_slice = &self.data[start..end.min(self.data.len())];
            let mut frame = Vec::with_capacity(2 + data_slice.len());
            frame.push((total_frames - 1) as u8); // Total frames
            frame.push(n as u8); // Frame index
            frame.extend_from_slice(data_slice);
            frames.push(frame);
        }
        frames
    }
}

struct CanFrame {
    can_id: u32,
    data: Vec<u8>,
}

impl CanFrame {
    fn new(can_id: u32, data: Vec<u8>) -> Self {
        CanFrame { can_id, data }
    }
}

struct PacketAssembler {
    parent: Box<dyn Parent>, // Assuming Parent is a trait that has the necessary methods
    buffer: HashMap<u32, HashMap<u8, Vec<u8>>>, // Maps can_id to a map of frame index to data
}

impl PacketAssembler {
    fn new(parent: Box<dyn Parent>) -> Self {
        PacketAssembler {
            parent,
            buffer: HashMap::new(),
        }
    }

    fn process_frame(&mut self, can_frame: CanFrame) -> Option<Packet> {
        let can_id = can_frame.can_id;
        let total_frames = can_frame.data[0] + 1; // Total frames
        let n = can_frame.data[1]; // Frame index

        self.buffer.entry(can_id).or_default().insert(n, can_frame.data[2..].to_vec());

        if self.buffer[&can_id].len() == total_frames as usize {
            let framebuffer = self.buffer.remove(&can_id).unwrap();
            let mut data = Vec::new();
            for k in 0..total_frames {
                if let Some(frame_data) = framebuffer.get(&(k as u8)) {
                    data.extend(frame_data);
                }
            }
            let packet = Packet::new(Some(data));
            return Some(packet);
        }

        None
    }
}

// Assuming a Parent trait is defined somewhere
trait Parent {
    fn received_packet(&self, packet: Packet);
}

fn main() {
    // Example usage
    let parent: Box<dyn Parent> = Box::new(ExampleParent {});
    let mut assembler = PacketAssembler::new(parent);

    // Create a packet and split it into frames
    let packet = Packet::new(Some(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]));
    let frames = packet.split();

    // Process each frame
    for (i, frame) in frames.iter().enumerate() {
        let can_frame = CanFrame::new(1, frame.clone());
        if let Some(assembled_packet) = assembler.process_frame(can_frame) {
            println!("Assembled packet: {:?}", assembled_packet);
        }
    }
}

// Example implementation of Parent trait
struct ExampleParent;

impl Parent for ExampleParent {
    fn received_packet(&self, packet: Packet) {
        println!("Received packet: {:?}", packet);
    }
}