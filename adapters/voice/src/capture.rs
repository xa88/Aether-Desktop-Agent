//! Audio capture and ring buffer management using cpal.

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use ringbuf::{HeapRb, Producer};
use std::sync::{Arc, Mutex};
use tracing::{info, error};

pub struct AudioCapturer {
    pub _stream: cpal::Stream,
    /// We use a Mutex to allow the stream callback to push data into the Producer.
    /// In a production environment, we might use a lock-free approach.
    pub producer: Arc<Mutex<Option<Producer<f32, Arc<HeapRb<f32>>>>>>,
}

impl AudioCapturer {
    pub fn new(sample_rate: u32) -> anyhow::Result<Self> {
        let host = cpal::default_host();
        let device = host.default_input_device()
            .ok_or_else(|| anyhow::anyhow!("No input device found"))?;
        
        info!("Audio: Using input device: {}", device.name()?);

        let _config = device.default_input_config()?;
        let stream_config: cpal::StreamConfig = cpal::StreamConfig {
            channels: 1, // Mono
            sample_rate: cpal::SampleRate(sample_rate),
            buffer_size: cpal::BufferSize::Default,
        };

        // 10 seconds of mono audio at 16kHz
        let rb = HeapRb::<f32>::new(sample_rate as usize * 10);
        let (prod, _cons) = rb.split();
        
        // We wrap the producer in a Mutex so it can be accessed from the stream callback
        let producer_for_callback = Arc::new(Mutex::new(prod));
        
        let producer_clone = Arc::clone(&producer_for_callback);
        let stream = device.build_input_stream(
            &stream_config,
            move |data: &[f32], _| {
                if let Ok(mut p) = producer_clone.lock() {
                    for &sample in data {
                        let _ = p.push(sample);
                    }
                }
            },
            |err| error!("Audio stream error: {}", err),
            None
        )?;

        stream.play()?;

        Ok(Self {
            _stream: stream,
            producer: Arc::new(Mutex::new(None)), // Placeholder
        })
    }
}
