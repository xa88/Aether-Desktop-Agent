//! Dialogue Manager: Voice-to-Action loop.

use ada_adapter_voice::{asr::WhisperEngine, tts::TtsProvider};
use tracing::info;

pub struct DialogueManager {
    asr: WhisperEngine,
    tts: Box<dyn TtsProvider>,
}

impl DialogueManager {
    pub fn new(asr: WhisperEngine, tts: Box<dyn TtsProvider>) -> Self {
        Self { asr, tts }
    }

    pub async fn process_audio_chunk(&self, samples: &[f32]) -> anyhow::Result<()> {
        let text = self.asr.transcribe(samples).await?;
        if text.is_empty() {
            return Ok(());
        }

        info!("Dialogue: Transcribed input: '{}'", text);
        
        // In a real application, we would pass 'text' to the Orchestrator here.
        // For now, we'll just simulate a response.
        
        if text.to_lowercase().contains("status") {
            self.tts.speak("Current task status is in progress. No errors detected.").await?;
        }
        
        Ok(())
    }
}
