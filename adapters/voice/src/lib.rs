//! Voice Adapter: Audio, ASR, and TTS.

pub mod capture;
pub mod asr;
pub mod tts;

pub use capture::AudioCapturer;
pub use asr::WhisperEngine;
pub use tts::{TtsProvider, SystemTtsProvider};
