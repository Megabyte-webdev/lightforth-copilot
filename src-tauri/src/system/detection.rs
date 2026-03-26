use crate::commands::session::session_init;

use super::super::audio::capture::AudioManager;
use active_win_pos_rs::get_active_window;
use serde::Serialize;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use sysinfo::System;
use tauri::{AppHandle, Emitter, Manager, Runtime};

#[derive(Debug, Serialize, Clone)]
pub enum MeetingPlatform {
    GoogleMeet,
    Zoom,
    Teams,
}

#[derive(Debug)]
enum MeetingState {
    None,
    Detected,
    Active,
}

pub struct MeetingDetector<R: Runtime> {
    pub state: Arc<Mutex<MeetingState>>,
    pub last_trigger: Arc<Mutex<Instant>>,
    pub app_handle: AppHandle<R>,
    pub audio_manager: Arc<Mutex<AudioManager>>,
    pub dismissed: AtomicBool,
}

impl<R: Runtime + 'static> MeetingDetector<R> {
    pub fn new(app_handle: AppHandle<R>, audio_manager: Arc<Mutex<AudioManager>>) -> Self {
        Self {
            state: Arc::new(Mutex::new(MeetingState::None)),
            last_trigger: Arc::new(Mutex::new(Instant::now() - Duration::from_secs(10))),
            app_handle,
            audio_manager,
            dismissed: AtomicBool::new(false),
        }
    }

    /// Detect active meeting by process + window title
    fn detect_meeting(&self) -> Option<String> {
        if let Ok(window) = get_active_window() {
            let title = window.title.to_lowercase();
            if title.contains("meet") {
                return Some(title.to_owned());
            }
            // if title.contains("zoom") {
            //     return Some(MeetingPlatform::Zoom);
            // }
            // if title.contains("teams") {
            //     return Some(MeetingPlatform::Teams);
            // }
        }

        // fallback: process + audio detection
        let mut sys = System::new_all();
        sys.refresh_all();
        for process in sys.processes().values() {
            let name = process.name().to_string_lossy().to_lowercase();
            // if name.contains("zoom") {
            //     return Some(MeetingPlatform::Zoom);
            // }
            // if name.contains("teams") {
            //     return Some(MeetingPlatform::Teams);
            // }
            if name.contains("chrome") && self.audio_manager.lock().unwrap().is_audio_active() {
                return Some(name.to_owned());
            }
        }

        None
    }

    fn should_emit(&self, detected: bool) -> bool {
        let mut state = self.state.lock().unwrap();
        let mut last = self.last_trigger.lock().unwrap();
        let now = Instant::now();

        // Reset dismissal if the meeting is gone
        if !detected {
            self.dismissed.store(false, Ordering::Relaxed);
        }

        match *state {
            MeetingState::None => {
                // Only emit if detected, cooldown passed, AND not dismissed
                if detected
                    && now.duration_since(*last).as_secs() > 2
                    && !self.dismissed.load(Ordering::Relaxed)
                {
                    *state = MeetingState::Detected;
                    *last = now;
                    return true;
                }
            }
            MeetingState::Detected => {
                if !detected {
                    *state = MeetingState::None;
                }
            }
            MeetingState::Active => {}
        }
        false
    }

    /// Run the detector loop in the background
    pub fn run(self: Arc<Self>) {
        let detector = self.clone();
        tauri::async_runtime::spawn(async move {
            loop {
                let platform_opt = detector.detect_meeting();
                let detected = platform_opt.is_some();

                println!(
                    ">>> Detected: {:?}, state: {:?}",
                    platform_opt,
                    detector.state.lock().unwrap()
                );

                if detector.should_emit(detected) {
                    let _ =
                        session_init(detector.app_handle.clone(), "meetingWidget".to_owned()).await;
                    if let Some(platform) = platform_opt {
                        if let Some(widget) =
                            detector.app_handle.get_webview_window("meetingWidget")
                        {
                            let _ = widget.emit("meeting-detected", &platform);
                        }
                    }
                }

                tokio::time::sleep(Duration::from_secs(1)).await; // check every 1s for faster response
            }
        });
    }
}
