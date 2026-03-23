use std::sync::Arc;
use tokio::sync::Semaphore;
use tracing::info;

/// Manages parallel execution bounds to prevent resource contention or IO thrashing
pub struct ResourceGate {
    pub default_semaphore: Arc<Semaphore>,
    pub io_semaphore: Arc<Semaphore>,
    pub cpu_semaphore: Arc<Semaphore>,
}

impl ResourceGate {
    pub fn new() -> Self {
        // Defaults: Max 10 general executions, 4 heavy FS tasks, 2 heavyweight CPU compilations
        Self {
            default_semaphore: Arc::new(Semaphore::new(10)),
            io_semaphore: Arc::new(Semaphore::new(4)),
            cpu_semaphore: Arc::new(Semaphore::new(2)),
        }
    }

    /// Acquires a permit matching the Tool operation intent
    pub async fn acquire(&self, tool_name: &str) -> tokio::sync::SemaphorePermit<'_> {
        let semaphore = if tool_name.starts_with("fs_") || tool_name == "git_status" {
            info!("Waiting for IO pool lock for {}", tool_name);
            &self.io_semaphore
        } else if tool_name == "shell_run" {
            info!("Waiting for CPU pool lock for {}", tool_name);
            &self.cpu_semaphore
        } else {
            &self.default_semaphore
        };
        
        semaphore.acquire().await.unwrap()
    }
}

impl Default for ResourceGate {
    fn default() -> Self {
        Self::new()
    }
}
