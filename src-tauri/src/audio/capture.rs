use super::super::commands::ai;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Sample, SampleFormat, SizedSample, StreamConfig};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use tauri::{AppHandle, Emitter};

pub struct AudioManager {
    stream: Option<cpal::Stream>,
    is_running: Arc<AtomicBool>,
}

impl AudioManager {
    pub fn new() -> Self {
        Self {
            stream: None,
            is_running: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn start(&mut self, app_handle: AppHandle) -> Result<(), String> {
        if self.is_running.load(Ordering::Relaxed) {
            return Ok(());
        }

        let host = cpal::default_host();
        let device = host
            .default_input_device()
            .ok_or("No input device available. Check Windows Privacy Settings.")?;
        let config_raw = device.default_input_config().map_err(|e| e.to_string())?;

        // 2. Extract the format and convert to the final StreamConfig
        let sample_format = config_raw.sample_format();
        let config: cpal::StreamConfig = config_raw.into();

        println!(">>> [AUDIO] Using: {:?}", device.description());
        println!(
            ">>> [AUDIO] Format: {:?} | Rate: {}",
            sample_format, config.sample_rate
        );

        let buffer = Arc::new(Mutex::new(Vec::<f32>::new()));
        let is_running = self.is_running.clone();
        is_running.store(true, Ordering::Relaxed);

        let stream = match sample_format {
            SampleFormat::F32 => {
                build_stream::<f32>(&device, &config, buffer, app_handle, is_running)?
            }
            SampleFormat::I16 => {
                build_stream::<i16>(&device, &config, buffer, app_handle, is_running)?
            }
            SampleFormat::U16 => {
                build_stream::<u16>(&device, &config, buffer, app_handle, is_running)?
            }
            _ => return Err(format!("Unsupported format: {:?}", sample_format)),
        };

        stream.play().map_err(|e| e.to_string())?;
        self.stream = Some(stream);
        Ok(())
    }

    pub fn stop(&mut self) {
        self.is_running.store(false, Ordering::Relaxed);
        self.stream = None; // Dropping the stream stops it
        println!(">>> [AUDIO] Stream Terminated");
    }
}

fn build_stream<T>(
    device: &cpal::Device,
    config: &StreamConfig,
    buffer: Arc<Mutex<Vec<f32>>>,
    app_handle: AppHandle,
    is_running: Arc<AtomicBool>,
) -> Result<cpal::Stream, String>
where
    T: Sample + SizedSample + cpal::FromSample<T>,
    f32: cpal::FromSample<T>,
{
    let channels = config.channels as usize;
    let sample_rate = config.sample_rate;

    device
        .build_input_stream(
            config,
            move |data: &[T], _| {
                if !is_running.load(Ordering::Relaxed) {
                    return;
                }
                // DIAGNOSTIC: Print the actual value of the first few samples
                if let Some(first) = data.get(0) {
                    let val = first.to_sample::<f32>();
                    // This will flood your console, but it tells us if the driver is 100% zero
                    if val != 0.0 {
                        println!(">>> [DRIVER ALIVE] Sample: {}", val);
                    }
                }

                println!(">>> [STREAM CALLBACK] {} samples", data.len());

                let mut buf = match buffer.lock() {
                    Ok(b) => b,
                    Err(_) => return,
                };

                // 2. Average channels to Mono & convert to f32
                for frame in data.chunks(channels) {
                    let sum: f32 = frame.iter().map(|s| s.to_sample::<f32>()).sum();
                    buf.push(sum / channels as f32);
                }

                // 3. Process chunk (Target 1s of audio based on sample rate)
                if !buf.is_empty() {
                    let voice_data = std::mem::take(&mut *buf);
                    println!(
                        ">>> [AI] Sending {} samples to transcription",
                        voice_data.len()
                    );
                    if !voice_data.is_empty() {
                        let handle = app_handle.clone();
                        tauri::async_runtime::spawn(async move {
                            let _ = handle.emit("ai_status", "processing");
                            if let Ok(text) = ai::transcribe_audio(voice_data, sample_rate).await {
                                if !text.trim().is_empty() {
                                    println!(">>> [GROQ]: {}", text);
                                    let _ = handle.emit("ai_response_chunk", text);
                                }
                            }
                            let _ = handle.emit("ai_status", "idle");
                        });
                    }
                }
            },
            |err| eprintln!(">>> [STREAM ERROR] {:?}", err),
            None,
        )
        .map_err(|e| e.to_string())
}
