extern crate alloc;

use super::can_frame::{CanFrame, CanFrameError};
use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkError {
    NodeNotFound,
    InvalidNodeId,
    SendFailed,
}

impl fmt::Display for NetworkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NetworkError::NodeNotFound => write!(f, "Node not found"),
            NetworkError::InvalidNodeId => write!(f, "Invalid node ID"),
            NetworkError::SendFailed => write!(f, "Failed to send to node"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct NodeInfo {
    pub node_id: u32,
    pub status: u8,
    pub last_seen: u32, // timestamp
}

impl NodeInfo {
    pub fn new(node_id: u32) -> Self {
        NodeInfo {
            node_id,
            status: 0,
            last_seen: 0,
        }
    }

    pub fn update_status(&mut self, status: u8, timestamp: u32) {
        self.status = status;
        self.last_seen = timestamp;
    }
}

pub struct NetworkManager {
    nodes: BTreeMap<u32, NodeInfo>,
    local_node_id: u32,
}

impl NetworkManager {
    pub fn new(local_node_id: u32) -> Self {
        NetworkManager {
            nodes: BTreeMap::new(),
            local_node_id,
        }
    }

    pub fn add_node(&mut self, node_id: u32) -> Result<(), NetworkError> {
        if node_id > crate::constants::NODE_ID_MASK {
            return Err(NetworkError::InvalidNodeId);
        }

        self.nodes.insert(node_id, NodeInfo::new(node_id));
        Ok(())
    }

    pub fn remove_node(&mut self, node_id: u32) -> Result<(), NetworkError> {
        self.nodes
            .remove(&node_id)
            .map(|_| ())
            .ok_or(NetworkError::NodeNotFound)
    }

    pub fn update_node_status(
        &mut self,
        node_id: u32,
        status: u8,
        timestamp: u32,
    ) -> Result<(), NetworkError> {
        self.nodes
            .get_mut(&node_id)
            .map(|node| node.update_status(status, timestamp))
            .ok_or(NetworkError::NodeNotFound)
    }

    pub fn get_node(&self, node_id: u32) -> Option<&NodeInfo> {
        self.nodes.get(&node_id)
    }

    pub fn get_all_nodes(&self) -> Vec<&NodeInfo> {
        self.nodes.values().collect()
    }

    pub fn is_node_active(&self, node_id: u32, current_time: u32, timeout: u32) -> bool {
        if let Some(node) = self.nodes.get(&node_id) {
            current_time.saturating_sub(node.last_seen) <= timeout
        } else {
            false
        }
    }

    pub fn get_local_node_id(&self) -> u32 {
        self.local_node_id
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }
}
