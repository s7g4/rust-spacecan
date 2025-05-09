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

/// Produces heartbeat frames for the controller.
pub struct HeartbeatProducer {
    controller: Arc<Controller>,
}

impl HeartbeatProducer {
    /// Creates a new heartbeat producer.
    pub fn new(controller: Arc<Controller>) -> Self {
        Self { controller }
    }

    /// Starts heartbeat production with an optional period.
    pub fn start(&self, _period: Option<u32>) {
        // Start heartbeat production.
    }

    /// Stops heartbeat production.
    pub fn stop(&self) {
        // Stop heartbeat production.
    }
}

/// Produces sync frames for the controller.
pub struct SyncProducer {
    controller: Arc<Controller>,
}

impl SyncProducer {
    /// Creates a new sync producer.
    pub fn new(controller: Arc<Controller>) -> Self {
        Self { controller }
    }

    /// Starts sync production with an optional period.
    pub fn start(&self, _period: Option<u32>) {
        // Start sync production.
    }

    /// Stops sync production.
    pub fn stop(&self) {
        // Stop sync production.
    }
}

/// Configuration data for the controller.
#[derive(Deserialize)]
struct Config {
    interface: String,
    channel_a: u32,
    channel_b: u32,
    heartbeat_period: Option<u32>,
    sync_period: Option<u32>,
    packet_service: Option<String>,
}

/// Assembles packets from CAN frames.
pub struct PacketAssembler {
    controller: Arc<Controller>,
}

impl PacketAssembler {
    /// Creates a new packet assembler.
    pub fn new(controller: Arc<Controller>) -> Self {
        Self { controller }
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

/// Main controller struct managing CAN communication and services.
pub struct Controller {
    node_id: u32,
    interface: String,
    channel_a: u32,
    channel_b: u32,
    heartbeat_period: Option<u32>,
    sync_period: Option<u32>,
    packet_service: Option<String>,
    network: Option<Network>,
    heartbeat: Option<HeartbeatProducer>,
    sync: Option<SyncProducer>,
    packet_assembler: Option<PacketAssembler>,
}

impl Controller {
    /// Creates a new controller with the specified configuration.
    pub fn new(
        interface: String,
        channel_a: u32,
        channel_b: u32,
        heartbeat_period: Option<u32>,
        sync_period: Option<u32>,
        packet_service: Option<String>,
    ) -> Self {
        let node_id = 0; // Controller node ID is always 0.

        // Initialize heartbeat producer if heartbeat_period is specified.
        let heartbeat = heartbeat_period.map(|_| HeartbeatProducer::new(Arc::new(Self {
            node_id,
            interface: interface.clone(),
            channel_a,
            channel_b,
            heartbeat_period,
            sync_period,
            packet_service: packet_service.clone(),
            network: None,
            heartbeat: None,
            sync: None,
            packet_assembler: None,
        })));

        // Initialize sync producer if sync_period is specified.
        let sync = sync_period.map(|_| SyncProducer::new(Arc::new(Self {
            node_id,
            interface: interface.clone(),
            channel_a,
            channel_b,
            heartbeat_period,
            sync_period,
            packet_service: packet_service.clone(),
            network: None,
            heartbeat: None,
            sync: None,
            packet_assembler: None,
        })));

        // Initialize packet assembler if packet_service is specified.
        let packet_assembler = packet_service.map(|_| PacketAssembler::new(Arc::new(Self {
            node_id,
            interface: interface.clone(),
            channel_a,
            channel_b,
            heartbeat_period,
            sync_period,
            packet_service,
            network: None,
            heartbeat: None,
            sync: None,
            packet_assembler: None,
        })));

        Self {
            node_id,
            interface,
            channel_a,
            channel_b,
            heartbeat_period,
            sync_period,
            packet_service,
            network: None,
            heartbeat,
            sync,
            packet_assembler,
        }
    }

    /// Creates a controller from a configuration file.
    pub fn from_file(filepath: &str) -> io::Result<Self> {
        let file = File::open(filepath)?;
        let config: Config = serde_json::from_reader(file)?;

        Ok(Self::new(
            config.interface,
            config.channel_a,
            config.channel_b,
            config.heartbeat_period,
            config.sync_period,
            config.packet_service,
        ))
    }

    /// Connects the controller to the CAN network.
    pub fn connect(&mut self) {
        if self.interface == "pyboard" {
            // Assuming PyboardCanBus is implemented.
            let bus_a = Arc::new(PyboardCanBus::new(self.channel_a));
            let bus_b = Arc::new(PyboardCanBus::new(self.channel_b));
            // Receive telemetry from all responder nodes.
            let filters = vec![(ID_TM, FUNCTION_MASK)];
            bus_a.set_filters(filters.clone());
            bus_b.set_filters(filters);
            self.network = Some(Network::new(bus_a, bus_b));
        } else {
            panic!("Not implemented");
        }
    }

    /// Disconnects the controller from the CAN network.
    pub fn disconnect(&self) {
        if let Some(network) = &self.network {
            network.bus_a.disconnect();
            network.bus_b.disconnect();
        }
    }

    /// Starts the controller services.
    pub fn start(&self) {
        if let Some(network) = &self.network {
            network.start();
        }
        if let Some(heartbeat) = &self.heartbeat {
            heartbeat.start(self.heartbeat_period);
        }
        if let Some(sync) = &self.sync {
            sync.start(self.sync_period);
        }
    }

    /// Stops the controller services.
    pub fn stop(&self) {
        if let Some(sync) = &self.sync {
            sync.stop();
        }
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
        }
    }

    /// Sends SCET time data as a CAN frame.
    pub fn send_scet(&self, coarse_time: u32, fine_time: u32) {
        let can_id = ID_SCET;
        let data = vec![
            (fine_time >> 16) as u8,
            (fine_time >> 8) as u8,
            fine_time as u8,
            (coarse_time >> 24) as u8,
            (coarse_time >> 16) as u8,
            (coarse_time >> 8) as u8,
            coarse_time as u8,
        ];
        let can_frame = CanFrame::new(can_id, data);
        if let Some(network) = &self.network {
            network.send(can_frame);
        }
    }

    /// Sends UTC time data as a CAN frame.
    pub fn send_utc(&self, day: u32, ms_of_day: u32, sub_ms: u32) {
        let can_id = ID_UTC;
        let data = vec![
            (sub_ms >> 8) as u8,
            (sub_ms & 0xFF) as u8,
            (ms_of_day >> 24) as u8,
            (ms_of_day >> 16) as u8,
            (ms_of_day >> 8) as u8,
            (ms_of_day & 0xFF) as u8,
            (day >> 8) as u8,
            (day & 0xFF) as u8,
        ];
        let can_frame = CanFrame::new(can_id, data);
        if let Some(network) = &self.network {
            network.send(can_frame);
        }
    }

    /// Sends a sync frame.
    pub fn send_sync(&self) {
        let can_id = ID_SYNC;
        let can_frame = CanFrame::new(can_id, Vec::new());
        if let Some(network) = &self.network {
            network.send(can_frame);
        }
    }

    /// Sends a telecommand frame to a specific node.
    pub fn send_telecommand(&self, data: Vec<u8>, node_id: u32) {
        let can_id = ID_TC + node_id;
        let can_frame = CanFrame::new(can_id, data);
        if let Some(network) = &self.network {
            network.send(can_frame);
        }
    }

    /// Sends a packet consisting of multiple CAN frames to a specific node.
    pub fn send_packet(&self, packet: Vec<Vec<u8>>, node_id: u32) {
        let can_id = ID_TC + node_id;
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

        // Controller should only receive telemetry from other nodes.
        if func_id == ID_TM {
            self.received_telemetry(can_frame.data.clone(), node_id);
            if let Some(packet_assembler) = &self.packet_assembler {
                if let Some(packet) = packet_assembler.process_frame(can_frame) {
                    self.received_packet(packet.data, node_id);
                }
            }
        }
    }

    /// Handles received telemetry data.
    pub fn received_telemetry(&self, frame_data: Vec<u8>, node_id: u32) {
        // To be implemented by application.
    }

    /// Handles received packet data.
    pub fn received_packet(&self, packet_data: Vec<u8>, node_id: u32) {
        // To be implemented by application.
    }

    /// Called when a heartbeat is sent.
    pub fn sent_heartbeat(&self) {
        // To be implemented by application.
    }
}

/// Constants for CAN IDs and masks.
const FUNCTION_MASK: u32 = 0xFF; // Example mask.
const ID_TM: u32 = 0x01; // Telemetry ID.
const ID_TC: u32 = 0x02; // Telecommand ID.
const ID_SCET: u32 = 0x03; // SCET ID.
const ID_UTC: u32 = 0x04; // UTC ID.
const ID_SYNC: u32 = 0x05; // Sync ID.

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
