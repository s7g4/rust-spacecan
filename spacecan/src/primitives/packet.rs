extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use alloc::string::String;
use super::can_frame::{CanFrame, CanFrameError};

const MAX_DATA_LENGTH: usize = 6;

#[derive(Debug)]
pub struct Packet {
    data: Vec<u8>,
}

impl Packet {
    pub fn new(data: Option<Vec<u8>>) -> Self {
        let data = data.unwrap_or_else(Vec::new);
        Packet { data }
    }

    pub fn split(&self) -> Vec<Vec<u8>> {
        let total_frames = (self.data.len() + MAX_DATA_LENGTH - 1) / MAX_DATA_LENGTH;
        let mut frames = Vec::with_capacity(total_frames);
        
        for (i, chunk) in self.data.chunks(MAX_DATA_LENGTH).enumerate() {
            let mut frame = Vec::with_capacity(2 + chunk.len());
            frame.push((total_frames - 1) as u8); // Total frames
            frame.push(i as u8); // Frame index
            frame.extend_from_slice(chunk);
            frames.push(frame);
        }
        frames
    }
}

pub struct PacketAssembler {
    buffer: BTreeMap<u32, BTreeMap<u8, Vec<u8>>>, // Maps can_id to a map of frame index to data
}

impl PacketAssembler {
    pub fn new() -> Self {
        PacketAssembler {
            buffer: BTreeMap::new(),
        }
    }

    pub fn process_frame(&mut self, can_frame: CanFrame) -> Option<Packet> {
        let can_id = can_frame.can_id();
        let total_frames = can_frame.data().get(0).copied()? + 1;
        let frame_index = can_frame.data().get(1).copied()?;
        
        self.buffer.entry(can_id).or_default().insert(frame_index, can_frame.data()[2..].to_vec());
        
        if self.buffer[&can_id].len() == total_frames as usize {
            let mut data = Vec::new();
            let framebuffer = self.buffer.remove(&can_id).unwrap();
            for i in 0..total_frames {
                if let Some(frame_data) = framebuffer.get(&i) {
                    data.extend(frame_data);
                } else {
                    return None; // Incomplete packet
                }
            }
            Some(Packet::new(Some(data)))
        } else {
            None
        }
    }
}
