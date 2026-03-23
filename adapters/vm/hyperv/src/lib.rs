//! ada-adapter-vm-hyperv: Sandboxing execution via Hyper-V checkpointing and PSRemoting.

use ada_sandbox::SandboxProvider;
use ada_tool_api::{ToolError, ToolErrorCode, ToolRequest, ToolResponse};
use async_trait::async_trait;
use tokio::process::Command;
use tracing::info;

pub struct HypervProvider {
    pub vm_name: String,
}

impl HypervProvider {
    pub fn new(vm_name: String) -> Self {
        Self { vm_name }
    }

    /// Helper to run powershell commands.
    async fn run_ps(&self, script: &str) -> anyhow::Result<String> {
        let output = Command::new("powershell")
            .args(["-NoProfile", "-NonInteractive", "-Command", script])
            .output()
            .await?;

        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();

        if output.status.success() {
            Ok(stdout)
        } else {
            Err(anyhow::anyhow!("PowerShell error [{}]: {}", output.status, stderr))
        }
    }
}

#[async_trait]
impl SandboxProvider for HypervProvider {
    async fn create_env(&self) -> anyhow::Result<()> {
        info!("Ensuring Hyper-V VM '{}' is running...", self.vm_name);
        
        let state = self.run_ps(&format!("(Get-VM -Name '{}').State", self.vm_name)).await?;
        if state != "Running" {
            info!("Starting VM: {}", self.vm_name);
            self.run_ps(&format!("Start-VM -Name '{}'", self.vm_name)).await?;
            
            // Wait for boot conceptually
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        }
        Ok(())
    }

    async fn run_step(&self, req: &ToolRequest) -> anyhow::Result<ToolResponse> {
        info!("Hyper-V Sandbox routing {}/{} to VM {}", req.tool, req.action, self.vm_name);
        
        // MVP: serialize the request, invoke inside the VM via Invoke-Command, and decode.
        // For actual Windows host bridging, `Invoke-Command -VMName` or `-ComputerName` works 
        // if PSRemoting / Hyper-V integration services are active.
        
        let json_arg = serde_json::to_string(req).unwrap_or_default();
        let escaped_json = json_arg.replace("\"", "\"\"");

        // We wrap the JSON execution into a bootstrapper script block.
        // For demonstration, we simply echo back or execute shell directly inside VM.
        let script = format!(
            "Invoke-Command -VMName '{}' -ScriptBlock {{ param($json) echo 'Sandbox VM Received payload'; return $json }} -ArgumentList '{}'",
            self.vm_name, escaped_json
        );

        let out = match self.run_ps(&script).await {
            Ok(s) => s,
            Err(e) => return Ok(ToolResponse::err(&req.id, ToolError {
                code: ToolErrorCode::ExecFailed,
                message: format!("VM Invoke Failed: {}", e),
                detail: None,
            }, 0))
        };

        Ok(ToolResponse::ok(&req.id, serde_json::json!({ "vm_out": out }), 0))
    }

    async fn snapshot_create(&self, name: &str) -> anyhow::Result<()> {
        info!("Hyper-V: Creating checkpoint '{}' for VM '{}'", name, self.vm_name);
        let script = format!("Checkpoint-VM -Name '{}' -SnapshotName '{}'", self.vm_name, name);
        self.run_ps(&script).await?;
        Ok(())
    }

    async fn snapshot_restore(&self, name: &str) -> anyhow::Result<()> {
        info!("Hyper-V: Restoring checkpoint '{}' for VM '{}'", name, self.vm_name);
        // We use -Confirm:$false to auto-apply
        let script = format!("Restore-VMCheckpoint -Name '{}' -VMName '{}' -Confirm:$false", name, self.vm_name);
        self.run_ps(&script).await?;
        Ok(())
    }

    async fn teardown(&self) -> anyhow::Result<()> {
        // Optional: stop the VM or revert to base snapshot
        info!("Hyper-V: Teardown VM '{}'", self.vm_name);
        Ok(())
    }
}
