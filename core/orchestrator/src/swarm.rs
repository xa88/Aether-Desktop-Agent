//! SwarmManager: Orchestrates multiple sub-agents for parallel task execution.

use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use uuid::Uuid;
use anyhow::Result;
use tracing::{info, warn};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AgentStatus {
    Idle,
    Planning,
    Executing,
    Success,
    Failed,
    Recovering,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerTask {
    pub id: String,
    pub goal: String,
    pub dependencies: Vec<String>,
    pub run_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskPacket {
    pub task: WorkerTask,
    pub sender_id: String,
    pub director_ip: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerAgent {
    pub id: String,
    pub name: String,
    pub status: AgentStatus,
    pub current_task: Option<String>,
    pub progress: f32, // 0.0 to 1.0
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SwarmStatus {
    pub active_agents: usize,
    pub total_tasks: usize,
    pub completed_tasks: usize,
    pub workers: Vec<WorkerAgent>,
}

pub struct SwarmManager {
    pub director_id: String,
    workers: Arc<Mutex<Vec<WorkerAgent>>>,
}

impl SwarmManager {
    pub fn new() -> Self {
        Self {
            director_id: Uuid::new_v4().to_string(),
            workers: Arc::new(Mutex::new(vec![])),
        }
    }

    pub fn spawn_worker(&self, name: &str) -> String {
        let id = Uuid::new_v4().to_string();
        let mut workers = self.workers.lock().unwrap();
        workers.push(WorkerAgent {
            id: id.clone(),
            name: name.to_string(),
            status: AgentStatus::Idle,
            current_task: None,
            progress: 0.0,
        });
        info!("Spawned worker agent: {} ({})", name, id);
        id
    }

    pub fn update_worker_status(&self, id: &str, status: AgentStatus, progress: f32) {
        let mut workers = self.workers.lock().unwrap();
        if let Some(w) = workers.iter_mut().find(|w| w.id == id) {
            w.status = status;
            w.progress = progress;
        }
    }

    pub fn get_status(&self) -> SwarmStatus {
        let workers = self.workers.lock().unwrap();
        let active = workers.iter().filter(|w| matches!(w.status, AgentStatus::Executing | AgentStatus::Planning)).count();
        let completed = workers.iter().filter(|w| matches!(w.status, AgentStatus::Success)).count();
        
        SwarmStatus {
            active_agents: active,
            total_tasks: workers.len(),
            completed_tasks: completed,
            workers: workers.clone(),
        }
    }

    /// Attempts to recover a failed agent or reassign its task.
    pub fn recover_agent(&self, id: &str) -> Result<()> {
        let mut workers = self.workers.lock().unwrap();
        if let Some(w) = workers.iter_mut().find(|w| w.id == id) {
            match w.status {
                AgentStatus::Failed => {
                    warn!("Attempting self-healing for agent {}...", id);
                    w.status = AgentStatus::Recovering;
                    // In a real implementation, this would trigger a retry logic
                }
                _ => {}
            }
        }
        Ok(())
    }
}
