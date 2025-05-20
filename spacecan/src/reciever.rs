use serde::Deserialize;
use std::fs::File;
use std::io::{self, Read};
use std::sync::{Arc, Mutex};

/// Represents a CAN frame with an identifier and data payload.
#[derive(Debug)]
pub struct CanFrame {
    /// The CAN identifier.
    pub can_id: u32,
    /// The data payload of the CAN frame.
    pub data: Vec<u8>,
}

impl CanFrame {
    /// Creates a new CAN frame with the given identifier and data.
    pub fn new(can_id: u32, data: Vec<u8>) -> Self {
        Self { can_id, data }
    }

    /// Extracts the function ID from the CAN frame identifier.
    pub fn get_func_id(&self) -> u32 {
        // Extract function ID from the CAN frame (lowest 8 bits).
        self.can_id & 0xFF
    }

    /// Extracts the node ID from the CAN frame identifier.
    pub fn get_node_id(&self) -> u32 {
        // Extract node ID from the CAN frame (bits 8-15).
        (self.can_id >> 8) & 0xFF
    }
}

/// Represents the network with two CAN buses and a selected bus for communication.
pub struct Network {
    /// CAN bus A.
    pub bus_a: Arc<dyn Bus>,
    /// CAN bus B.
    pub bus_b: Arc<dyn Bus>,
    /// Currently selected CAN bus.
    pub selected_bus: Arc<dyn Bus>,
}

impl Network {
    /// Creates a new network with two CAN buses.
    pub fn new(bus_a: Arc<dyn Bus>, bus_b: Arc<dyn Bus>) -> Self {
        Self {
            bus_a,
            bus_b,
            selected_bus: bus_a.clone(),
        }
    }

    /// Starts the network communication.
    pub fn start(&self) {
        // Start the network communication.
    }

    /// Stops the network communication.
    pub fn stop(&self) {
        // Stop the network communication.
    }

    /// Sends a CAN frame through the selected bus.
    pub fn send(&self, can_frame: CanFrame) {
        self.selected_bus.send(can_frame);
    }
}

/// Trait representing a CAN bus interface.
pub trait Bus {
    /// Disconnects the bus.
    fn disconnect(&self);
    /// Sends a CAN frame on the bus.
    fn send(&self, can_frame: CanFrame);
}

/// Consumes heartbeat frames for the responder.
pub struct HeartbeatConsumer {
    responder: Arc<Responder>,
}

impl HeartbeatConsumer {
    /// Creates a new heartbeat consumer.
    pub fn new(responder: Arc<Responder>) -> Self {
        Self { responder }
    }

    /// Starts heartbeat consumption with optional parameters.
    pub fn start(&self, _period: Option<u32>, _max_miss: u32, _max_switch: Option<u32>) {
        // Start heartbeat consumption.
    }

    /// Stops heartbeat consumption.
    pub fn stop(&self) {
        // Stop heartbeat consumption.
    }

    /// Handles a received heartbeat.
    pub fn received(&self) {
        // Handle received heartbeat.
    }
}

/// Assembles packets from CAN frames.
pub struct PacketAssembler {
    responder: Arc<Responder>,
}

impl PacketAssembler {
    /// Creates a new packet assembler.
    pub fn new(responder: Arc<Responder>) -> Self {
        Self { responder }
    }

    /// Processes a CAN frame and returns a packet if applicable.
    pub fn process_frame(&self, _can_frame: CanFrame) -> Option<Packet> {
        // Process the CAN frame and return a packet if applicable.
        None
    }
}

/// Represents a data packet.
pub struct Packet {
    /// The data contained in the packet.
    pub data: Vec<u8>,
}

/// Configuration data for the responder.
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

/// Main responder struct managing CAN communication and services.
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
    /// Creates a new responder with the specified configuration.
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

        // Initialize heartbeat consumer if heartbeat_period is specified.
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

        // Initialize packet assembler if packet_service is specified.
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

    /// Creates a responder from a configuration file.
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

    /// Connects the responder to the CAN network.
    pub fn connect(&mut self) {
        if self.interface == "pyboard" {
            // Assuming PyboardCanBus is implemented.
            let bus_a = Arc::new(PyboardCanBus::new(self.channel_a));
            let bus_b = Arc::new(PyboardCanBus::new(self.channel_b));
            // Receive sync, heartbeat, and telecommands from controller node.
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

    /// Disconnects the responder from the CAN network.
    pub fn disconnect(&self) {
        if let Some(network) = &self.network {
            network.bus_a.disconnect();
            network.bus_b.disconnect();
        }
    }

    /// Starts the responder services.
    pub fn start(&self) {
        if let Some(network) = &self.network {
            network.start();
        }
        if let Some(heartbeat) = &self.heartbeat {
            heartbeat.start(self.heartbeat_period, self.max_miss_heartbeat, self.max_bus_switch);
        }
    }

    /// Stops the responder services.
    pub fn stop(&self) {
        if let Some(heartbeat) = &self.heartbeat {
            heartbeat.stop();
        }
        if let Some(network) = &self.network {
            network.stop();
        }
    }

    /// Switches the active CAN bus.
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

    /// Called when the bus is switched. Can be overridden.
    pub fn on_bus_switch(&self) {
        // To be overwritten.
    }

    /// Sends telemetry data as a CAN frame.
    pub fn send_telemetry(&self, data: Vec<u8>) {
        let can_id = ID_TM + self.node_id;
        let can_frame = CanFrame::new(can_id, data);
        if let Some(network) = &self.network {
            network.send(can_frame);
        }
    }

    /// Sends a packet consisting of multiple CAN frames.
    pub fn send_packet(&self, packet: Vec<Vec<u8>>) {
        let can_id = ID_TM + self.node_id;
        for data in packet {
            let can_frame = CanFrame::new(can_id, data);
            if let Some(network) = &self.network {
                network.send(can_frame);
            }
        }
    }

    /// Handles a received CAN frame.
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

    /// Handles received heartbeat.
    pub fn received_heartbeat(&self) {
        // To be implemented by application.
    }

    /// Handles received sync.
    pub fn received_sync(&self) {
        // To be implemented by application.
    }

    /// Handles received SCET time.
    pub fn received_scet(&self, coarse_time: u32, fine_time: u32) {
        // To be implemented by application.
    }

    /// Handles received UTC time.
    pub fn received_utc(&self, day: u16, ms_of_day: u32, sub_ms: u16) {
        // To be implemented by application.
    }

    /// Handles received telecommand data.
    pub fn received_telecommand(&self, frame_data: Vec<u8>, node_id: u32) {
        // To be implemented by application.
    }

    /// Handles received packet data.
    pub fn received_packet(&self, packet_data: Vec<u8>, node_id: u32) {
        // To be implemented by application.
    }
}

/// Constants for CAN IDs and masks.
const FULL_MASK: u32 = 0xFFFFFFFF; // Example mask.
const ID_HEARTBEAT: u32 = 0x01; // Heartbeat ID.
const ID_SYNC: u32 = 0x02; // Sync ID.
const ID_TC: u32 = 0x03; // Telecommand ID.
const ID_TM: u32 = 0x04; // Telemetry ID.
const ID_SCET: u32 = 0x05; // SCET ID.
const ID_UTC: u32 = 0x06; // UTC ID.

/// Represents a Pyboard CAN bus interface.
pub struct PyboardCanBus {
    channel: u32,
}

impl PyboardCanBus {
    /// Creates a new Pyboard CAN bus on the specified channel.
    pub fn new(channel: u32) -> Self {
        Self { channel }
    }

    /// Sets filters for the CAN bus.
    pub fn set_filters(&self, _filters: Vec<(u32, u32)>) {
        // Set filters for the CAN bus.
    }

    /// Disconnects the CAN bus.
    pub fn disconnect(&self) {
        // Disconnect the CAN bus.
    }
}
