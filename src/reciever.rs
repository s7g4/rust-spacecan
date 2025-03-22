use serde::Deserialize;
use std::fs::File;
use std::io::{self, Read};
use std::sync::{Arc, Mutex};

// Placeholder for CanFrame struct
#[derive(Debug)]
pub struct CanFrame {
    pub can_id: u32,
    pub data: Vec<u8>,
}

impl CanFrame {
    pub fn new(can_id: u32, data: Vec<u8>) -> Self {
        Self { can_id, data }
    }

    pub fn get_func_id(&self) -> u32 {
        // Extract function ID from the CAN frame
        self.can_id & 0xFF // Example implementation
    }

    pub fn get_node_id(&self) -> u32 {
        // Extract node ID from the CAN frame
        (self.can_id >> 8) & 0xFF // Example implementation
    }
}

// Placeholder for the Network struct
pub struct Network {
    pub bus_a: Arc<dyn Bus>,
    pub bus_b: Arc<dyn Bus>,
    pub selected_bus: Arc<dyn Bus>,
}

impl Network {
    pub fn new(bus_a: Arc<dyn Bus>, bus_b: Arc<dyn Bus>) -> Self {
        Self {
            bus_a,
            bus_b,
            selected_bus: bus_a.clone(),
        }
    }

    pub fn start(&self) {
        // Start the network
    }

    pub fn stop(&self) {
        // Stop the network
    }

    pub fn send(&self, can_frame: CanFrame) {
        // Send a CAN frame through the selected bus
        self.selected_bus.send(can_frame);
    }
}

// Placeholder for the Bus trait
pub trait Bus {
    fn disconnect(&self);
    fn send(&self, can_frame: CanFrame);
}

// Placeholder for HeartbeatConsumer
pub struct HeartbeatConsumer {
    responder: Arc<Responder>,
}

impl HeartbeatConsumer {
    pub fn new(responder: Arc<Responder>) -> Self {
        Self { responder }
    }

    pub fn start(&self, _period: Option<u32>, _max_miss: u32, _max_switch: Option<u32>) {
        // Start heartbeat consumption
    }

    pub fn stop(&self) {
        // Stop heartbeat consumption
    }

    pub fn received(&self) {
        // Handle received heartbeat
    }
}

// Placeholder for PacketAssembler
pub struct PacketAssembler {
    responder: Arc<Responder>,
}

impl PacketAssembler {
    pub fn new(responder: Arc<Responder>) -> Self {
        Self { responder }
    }

    pub fn process_frame(&self, _can_frame: CanFrame) -> Option<Packet> {
        // Process the CAN frame and return a packet if applicable
        None // Placeholder
    }
}

// Placeholder for Packet struct
pub struct Packet {
    pub data: Vec<u8>,
}

// Struct to hold configuration data
#[derive(Deserialize)]
struct Config {
    interface: String,
    channel_a: u32,
    channel_b: u32,
    node_id: u32,
    heartbeat_period: Option<u32>,
    max_miss_heartbeat: u32,
    max_bus_switch: Option<u32>,
    packet_service: Option<String>,
}

// Responder struct
pub struct Responder {
    node_id: u32,
    interface: String,
    channel_a: u32,
    channel_b: u32,
    heartbeat_period: Option<u32>,
    max_miss_heartbeat: u32,
    max_bus_switch: Option<u32>,
    packet_service: Option<String>,
    network: Option<Network>,
    heartbeat: Option<HeartbeatConsumer>,
    packet_assembler: Option<PacketAssembler>,
}

impl Responder {
    pub fn new(
        interface: String,
        channel_a: u32,
        channel_b: u32,
        node_id: u32,
        heartbeat_period: Option<u32>,
        max_miss_heartbeat: u32,
        max_bus_switch: Option<u32>,
        packet_service: Option<String>,
    ) -> Self {
        if node_id < 1 || node_id > 127 {
            panic!("node id must be in range 1..127");
        }

        let heartbeat = heartbeat_period.map(|_| HeartbeatConsumer::new(Arc::new(Self {
            node_id,
            interface: interface.clone(),
            channel_a,
            channel_b,
            heartbeat_period,
            max_miss_heartbeat,
            max_bus_switch,
            packet_service: packet_service.clone(),
            network: None,
            heartbeat: None,
            packet_assembler: None,
        })));

        let packet_assembler = packet_service.map(|_| PacketAssembler::new(Arc::new(Self {
            node_id,
            interface: interface.clone(),
            channel_a,
            channel_b,
            heartbeat_period,
            max_miss_heartbeat,
            max_bus_switch,
            packet_service,
            network: None,
            heartbeat: None,
            packet_assembler: None,
        })));

        Self {
            node_id,
            interface,
            channel_a,
            channel_b,
            heartbeat_period,
            max_miss_heartbeat,
            max_bus_switch,
            packet_service,
            network: None,
            heartbeat,
            packet_assembler,
        }
    }

    pub fn from_file(filepath: &str) -> io::Result<Self> {
        let file = File::open(filepath)?;
        let config: Config = serde_json::from_reader(file)?;

        Ok(Self::new(
            config.interface,
            config.channel_a,
            config.channel_b,
            config.node_id,
            config.heartbeat_period,
            config.max_miss_heartbeat,
            config.max_bus_switch,
            config.packet_service,
        ))
    }

    pub fn connect(&mut self) {
        if self.interface == "pyboard" {
            // Assuming PyboardCanBus is implemented
            let bus_a = Arc::new(PyboardCanBus::new(self.channel_a));
            let bus_b = Arc::new(PyboardCanBus::new(self.channel_b));
            // receive sync, heartbeat, and telecommands from controller node
            let filters = vec![
                (ID_HEARTBEAT, FULL_MASK),
                (ID_SYNC, FULL_MASK),
                (ID_SCET, FULL_MASK),
                (ID_UTC, FULL_MASK),
                (ID_TC + self.node_id, FULL_MASK),
            ];
            bus_a.set_filters(filters.clone());
            bus_b.set_filters(filters);
            self.network = Some(Network::new(bus_a, bus_b));
        } else {
            panic!("Not implemented");
        }
    }

    pub fn disconnect(&self) {
        if let Some(network) = &self.network {
            network.bus_a.disconnect();
            network.bus_b.disconnect();
        }
    }

    pub fn start(&self) {
        if let Some(network) = &self.network {
            network.start();
        }
        if let Some(heartbeat) = &self.heartbeat {
            heartbeat.start(self.heartbeat_period, self.max_miss_heartbeat, self.max_bus_switch);
        }
    }

    pub fn stop(&self) {
        if let Some(heartbeat) = &self.heartbeat {
            heartbeat.stop();
        }
        if let Some(network) = &self.network {
            network.stop();
        }
    }

    pub fn switch_bus(&mut self) {
        if let Some(network) = &self.network {
            network.stop();
            if Arc::ptr_eq(&network.selected_bus, &network.bus_a) {
                network.selected_bus = network.bus_b.clone();
            } else {
                network.selected_bus = network.bus_a.clone();
            }
            network.start();
            self.on_bus_switch();
        }
    }

    pub fn on_bus_switch(&self) {
        // To be overwritten
    }

    pub fn send_telemetry(&self, data: Vec<u8>) {
        let can_id = ID_TM + self.node_id;
        let can_frame = CanFrame::new(can_id, data);
        if let Some(network) = &self.network {
            network.send(can_frame);
        }
    }

    pub fn send_packet(&self, packet: Vec<Vec<u8>>) {
        let can_id = ID_TM + self.node_id;
        for data in packet {
            let can_frame = CanFrame::new(can_id, data);
            if let Some(network) = &self.network {
                network.send(can_frame);
            }
        }
    }

    pub fn received_frame(&self, can_frame: CanFrame) {
        let func_id = can_frame.get_func_id();
        let node_id = can_frame.get_node_id();

        if func_id == ID_HEARTBEAT {
            if let Some(heartbeat) = &self.heartbeat {
                heartbeat.received();
            }
        } else if func_id == ID_SYNC {
            self.received_sync();
        } else if func_id == ID_SCET {
            let fine_time = u32::from_le_bytes(can_frame.data[0..3].try_into().unwrap());
            let coarse_time = u32::from_le_bytes(can_frame.data[3..7].try_into().unwrap());
            self.received_scet(coarse_time, fine_time);
        } else if func_id == ID_UTC {
            let sub_ms = u16::from_le_bytes(can_frame.data[0..2].try_into().unwrap());
            let ms_of_day = u32::from_le_bytes(can_frame.data[2..6].try_into().unwrap());
            let day = u16::from_le_bytes(can_frame.data[6..8].try_into().unwrap());
            self.received_utc(day, ms_of_day, sub_ms);
        } else if func_id == ID_TC && node_id == self.node_id {
            self.received_telecommand(can_frame.data.clone(), node_id);
            if let Some(packet_assembler) = &self.packet_assembler {
                if let Some(packet) = packet_assembler.process_frame(can_frame) {
                    self.received_packet(packet.data, node_id);
                }
            }
        }
    }

    pub fn received_heartbeat(&self) {
        // To be implemented by application
    }

    pub fn received_sync(&self) {
        // To be implemented by application
    }

    pub fn received_scet(&self, coarse_time: u32, fine_time: u32) {
        // To be implemented by application
    }

    pub fn received_utc(&self, day: u16, ms_of_day: u32, sub_ms: u16) {
        // To be implemented by application
    }

    pub fn received_telecommand(&self, frame_data: Vec<u8>, node_id: u32) {
        // To be implemented by application
    }

    pub fn received_packet(&self, packet_data: Vec<u8>, node_id: u32) {
        // To be implemented by application
    }
}

// Constants for CAN IDs and masks
const FULL_MASK: u32 = 0xFFFFFFFF; // Example mask
const ID_HEARTBEAT: u32 = 0x01; // Heartbeat ID
const ID_SYNC: u32 = 0x02; // Sync ID
const ID_TC: u32 = 0x03; // Telecommand ID
const ID_TM: u32 = 0x04; // Telemetry ID
const ID_SCET: u32 = 0x05; // SCET ID
const ID_UTC: u32 = 0x06; // UTC ID

// Placeholder for PyboardCanBus struct
pub struct PyboardCanBus {
    channel: u32,
}

impl PyboardCanBus {
    pub fn new(channel: u32) -> Self {
        Self { channel }
    }

    pub fn set_filters(&self, _filters: Vec<(u32, u32)>) {
        // Set filters for the CAN bus
    }

    pub fn disconnect(&self) {
        // Disconnect the CAN bus
    }
}