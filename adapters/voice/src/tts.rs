//! TTS Engine: Piper and System fallbacks.

use tracing::info;

#[async_trait::async_trait]
pub trait TtsProvider: Send + Sync {
    async fn speak(&self, text: &str) -> anyhow::Result<()>;
}

pub struct SystemTtsProvider;

#[async_trait::async_trait]
impl TtsProvider for SystemTtsProvider {
    async fn speak(&self, text: &str) -> anyhow::Result<()> {
        info!("TTS [System]: Speaking: {}", text);
        
        #[cfg(target_os = "windows")]
        {
            // Simple MVP: use PowerShell to speak if no native binding is available yet
            let script = format!("Add-Type -AssemblyName System.Speech; $speak = New-Object System.Speech.Synthesis.SpeechSynthesizer; $speak.Speak('{}')", text);
            std::process::Command::new("powershell")
                .arg("-Command")
                .arg(script)
                .spawn()?;
        }
        
        Ok(())
    }
}
