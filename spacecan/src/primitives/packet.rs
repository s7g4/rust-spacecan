extern crate alloc;

use crate::protocol::SpaceCANFrame;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;

const MAX_DATA_LENGTH: usize = 4;

#[derive(Debug)]
pub struct Packet {
    data: Vec<u8>,
}

impl Packet {
    pub fn new(data: Option<Vec<u8>>) -> Self {
        let data = data.unwrap_or_else(Vec::new);
        Packet { data }
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn split(&self) -> Vec<Vec<u8>> {
        let total_frames = (self.data.len() + MAX_DATA_LENGTH - 1) / MAX_DATA_LENGTH;
        let mut frames = Vec::with_capacity(total_frames);

        for (i, chunk) in self.data.chunks(MAX_DATA_LENGTH).enumerate() {
            let mut frame = Vec::with_capacity(2 + chunk.len());
            frame.push((total_frames - 1) as u8); // Remaining frame count
            frame.push(i as u8); // Frame index
            frame.extend_from_slice(chunk);
            frames.push(frame);
        }
        frames
    }
}

pub struct PacketAssembler {
    buffer: BTreeMap<u32, BTreeMap<u8, Vec<u8>>>,
}

impl PacketAssembler {
    pub fn new() -> Self {
        PacketAssembler {
            buffer: BTreeMap::new(),
        }
    }

    /// Processes a fragmented SpaceCAN frame and attempts reassembly.
    /// Returns a complete `Packet` once all fragments for a given CAN ID arrive.
    pub fn process_fragment(&mut self, frame: &SpaceCANFrame) -> Option<Packet> {
        let can_id = frame.can_id;
        let data = &frame.data;
        if data.len() < 2 {
            return None;
        }

        let total_frames = data[0] as usize + 1;
        let frame_index = data[1];
        let payload = data[2..].to_vec();

        self.buffer
            .entry(can_id)
            .or_default()
            .insert(frame_index, payload);

        if self.buffer[&can_id].len() == total_frames {
            let mut assembled = Vec::new();
            let framebuffer = self.buffer.remove(&can_id).unwrap();
            for i in 0..total_frames as u8 {
                if let Some(fragment) = framebuffer.get(&i) {
                    assembled.extend(fragment);
                } else {
                    return None;
                }
            }
            Some(Packet::new(Some(assembled)))
        } else {
            None
        }
    }
}
