use crate::primitives::can_frame::CanFrame;
use crate::primitives::packet::SpaceCANPacket;
use crate::transport::base::Bus;
use crate::services::core::{ServiceManager, Service};
use crate::PacketData;
use crate::FrameData;
use heapless::FnvIndexMap;

pub struct SpaceCANProtocol<B: Bus> {
    bus: B,
    node_id: u16,
    assembler: PacketAssembler,
    pub service_manager: ServiceManager,
}

impl<B: Bus> SpaceCANProtocol<B> {
    pub fn new(bus: B, node_id: u16) -> Self {
        SpaceCANProtocol {
            bus,
            node_id,
            assembler: PacketAssembler::new(),
            service_manager: ServiceManager::new(),
        }
    }

    pub fn send_frame(&self, frame: &CanFrame) -> Result<(), &'static str> {
        self.bus.send(frame).map_err(|_| "Failed to send frame")
    }

    pub fn receive_frame(&mut self) -> Result<Option<CanFrame>, &'static str> {
        if let Some(frame) = self.bus.get_frame() {
            if let Some(packet) = self.assembler.process_frame(&frame) {
                let _ = self.service_manager.route_packet(&packet);
            }
            Ok(Some(frame))
        } else {
            Ok(None)
        }
    }

    pub fn send_packet(&self, packet: &SpaceCANPacket) -> Result<(), &'static str> {
        let frames = self.assembler.fragment_packet(packet, self.node_id)?;
        for frame in frames {
            self.send_frame(&frame)?;
        }
        Ok(())
    }
}

pub struct PacketAssembler {
    partial_packets: FnvIndexMap<u16, PacketData, 16>,
}

impl PacketAssembler {
    pub fn new() -> Self {
        PacketAssembler {
            partial_packets: FnvIndexMap::new(),
        }
    }

    pub fn process_frame(&mut self, frame: &CanFrame) -> Option<SpaceCANPacket> {
        let id = frame.can_id();
        let source_id = ((id >> 8) & 0xFF) as u16;
        let data = frame.data();
        if data.is_empty() { return None; }

        let sequence_flags = (data[0] >> 6) & 0x03;
        
        match sequence_flags {
            3 => { // Unsegmented
                let mut p_data = PacketData::new();
                let _ = p_data.extend_from_slice(&data[1..]);
                SpaceCANPacket::new(source_id, 0, p_data).ok()
            }
            1 => { // First fragment
                let mut p_data = PacketData::new();
                let _ = p_data.extend_from_slice(&data[1..]);
                let _ = self.partial_packets.insert(source_id, p_data);
                None
            }
            0 => { // Continuation fragment
                if let Some(p_data) = self.partial_packets.get_mut(&source_id) {
                    let _ = p_data.extend_from_slice(&data[1..]);
                }
                None
            }
            2 => { // Last fragment
                if let Some(mut p_data) = self.partial_packets.remove(&source_id) {
                    let _ = p_data.extend_from_slice(&data[1..]);
                    SpaceCANPacket::new(source_id, 0, p_data).ok()
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    pub fn fragment_packet(&self, packet: &SpaceCANPacket, source_id: u16) -> Result<heapless::Vec<CanFrame, 128>, &'static str> {
        let mut frames = heapless::Vec::new();
        let data = &packet.data;
        if data.is_empty() { return Ok(frames); }

        if data.len() <= 7 {
            let mut payload = FrameData::new();
            let _ = payload.push(3 << 6);
            let _ = payload.extend_from_slice(data);
            let frame = CanFrame::new((source_id as u32) << 8, Some(payload)).unwrap();
            let _ = frames.push(frame);
            return Ok(frames);
        }

        let mut offset = 0;
        let total_len = data.len();

        while offset < total_len {
            let chunk_size = core::cmp::min(7, total_len - offset);
            let sequence_flag = if offset == 0 {
                1 // First
            } else if offset + chunk_size >= total_len {
                2 // Last
            } else {
                0 // Continuation
            };

            let mut payload = FrameData::new();
            let _ = payload.push(sequence_flag << 6);
            let _ = payload.extend_from_slice(&data[offset..offset + chunk_size]);
            
            let frame = CanFrame::new((source_id as u32) << 8, Some(payload)).unwrap();
            let _ = frames.push(frame);
            offset += chunk_size;
        }

        Ok(frames)
    }
}
