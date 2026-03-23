use std::net::UdpSocket;
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};
use tracing::{info, warn};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaNode {
    pub id: String,
    pub hostname: String,
    pub ip: String,
    pub status: NodeStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NodeStatus {
    Online,
    Busy,
    Offline,
}

pub struct ClusterManager {
    pub local_id: String,
    pub nodes: Arc<Mutex<Vec<AdaNode>>>,
}

impl ClusterManager {
    pub fn new() -> Self {
        let local_id = Uuid::new_v4().to_string();
        Self {
            local_id,
            nodes: Arc::new(Mutex::new(vec![])),
        }
    }

    pub fn start_discovery(&self) {
        let nodes = self.nodes.clone();
        let local_id = self.local_id.clone();

        std::thread::spawn(move || {
            let socket = match UdpSocket::bind("0.0.0.0:42069") {
                Ok(s) => s,
                Err(e) => {
                    warn!("Failed to bind cluster discovery socket: {}", e);
                    return;
                }
            };
            socket.set_broadcast(true).ok();

            info!("ADA Cluster Discovery active on port 42069 (Node: {})", local_id);

            let mut buf = [0u8; 1024];
            loop {
                if let Ok((size, src)) = socket.recv_from(&mut buf) {
                    if let Ok(msg) = serde_json::from_slice::<AdaNode>(&buf[..size]) {
                        if msg.id != local_id {
                            let mut node_list = nodes.lock().unwrap();
                            if !node_list.iter().any(|n| n.id == msg.id) {
                                info!("Discovered new node: {} ({})", msg.hostname, src);
                                node_list.push(msg);
                            }
                        }
                    }
                }
            }
        });
    }

    pub fn broadcast_presence(&self) {
        let local_id = self.local_id.clone();
        std::thread::spawn(move || {
            let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
            socket.set_broadcast(true).ok();
            
            let hostname = hostname::get().unwrap_or_else(|_| "Unknown".into()).to_string_lossy().into_owned();
            let node = AdaNode {
                id: local_id,
                hostname,
                ip: "local".to_string(), // In real impl, would be actual local IP
                status: NodeStatus::Online,
            };

            let msg = serde_json::to_vec(&node).unwrap();
            loop {
                socket.send_to(&msg, "255.255.255.255:42069").ok();
                std::thread::sleep(std::time::Duration::from_secs(10));
            }
        });
    }

    pub fn start_task_listener(&self) -> anyhow::Result<()> {
        let socket = UdpSocket::bind("0.0.0.0:11011")?; // Task execution port
        info!("Cluster Task Listener active on port 11011");
        
        std::thread::spawn(move || {
            let mut buf = [0u8; 65535];
            loop {
                if let Ok((len, addr)) = socket.recv_from(&mut buf) {
                    if let Ok(packet) = serde_json::from_slice::<crate::swarm::TaskPacket>(&buf[..len]) {
                        info!("RECEIVED REMOTE TASK: '{}' from {}", packet.task.goal, addr);
                        // In a real implementation, this would trigger the Orchestrator to run the task
                    }
                }
            }
        });
        Ok(())
    }

    pub fn get_active_nodes(&self) -> Vec<AdaNode> {
        self.nodes.lock().unwrap().clone()
    }
}
