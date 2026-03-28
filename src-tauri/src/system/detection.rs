use crate::commands::session::session_init;
use crate::system::system_audio::get_system_mic_usage;
use active_win_pos_rs::get_active_window;
use serde::Serialize;
use std::sync::atomic::{ AtomicBool, Ordering };
use std::sync::{ Arc, Mutex };
use std::time::{ Duration, Instant };
use tauri::{ AppHandle, Emitter, Manager, Runtime };

#[cfg(target_os = "macos")]
use cpal::traits::{ DeviceTrait, HostTrait, StreamTrait };

#[cfg(target_os = "macos")]
static MIC_ACTIVE: AtomicBool = AtomicBool::new(false);

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub enum MeetingPlatform {
    GoogleMeet(String),
    Zoom(String),
    Teams(String),
    Discord(String),
    Browser(String),
}

#[derive(Debug, PartialEq)]
pub(crate) enum MeetingState {
    None,
    Active,
}

pub struct MeetingDetector<R: Runtime> {
    pub state: Arc<Mutex<MeetingState>>,
    pub last_active: Arc<Mutex<Instant>>,
    pub current_meeting: Arc<Mutex<Option<MeetingPlatform>>>,
    pub app_handle: AppHandle<R>,
    pub dismissed: AtomicBool,
    //sys: Arc<Mutex<System>>,
}

impl<R: Runtime + 'static> MeetingDetector<R> {
    pub fn new(app_handle: AppHandle<R>) -> Self {
        #[cfg(target_os = "macos")]
        {
            let handle_clone = app_handle.clone();
            std::thread::spawn(move || {
                // Fixed: Call as an associated function or ensure it's in scope
                Self::start_virtual_device_monitoring("BlackHole 2ch", &handle_clone);
            });
        }

        Self {
            state: Arc::new(Mutex::new(MeetingState::None)),
            last_active: Arc::new(Mutex::new(Instant::now() - Duration::from_secs(10))),
            current_meeting: Arc::new(Mutex::new(None)),
            app_handle,
            dismissed: AtomicBool::new(false),
        }
    }

    fn detect_meeting(&self) -> Option<MeetingPlatform> {
        // 1. Mic must be active first
        let mic_active = {
            #[cfg(target_os = "windows")]
            {
                get_system_mic_usage()
                    .iter()
                    .any(|u| u.is_active)
            }
            #[cfg(target_os = "macos")]
            {
                MIC_ACTIVE.load(Ordering::Relaxed)
            }
            #[cfg(not(any(target_os = "windows", target_os = "macos")))]
            {
                false
            }
        };

        if !mic_active {
            return None;
        }

        // 2. Parse the active window title
        if let Ok(window) = get_active_window() {
            let title = window.title.clone();
            let title_lower = title.to_lowercase();
            let app_name = window.app_name.to_lowercase();

            if title_lower.contains("meet.google.com") || title_lower.contains("google meet") {
                return Some(MeetingPlatform::GoogleMeet(title));
            }
            if title_lower.contains("zoom meeting") || title_lower.contains("zoom.us") {
                return Some(MeetingPlatform::Zoom(title));
            }
            if
                title_lower.contains("teams.microsoft.com") ||
                title_lower.contains("microsoft teams")
            {
                return Some(MeetingPlatform::Teams(title));
            }
            if
                app_name.contains("chrome") ||
                app_name.contains("edge") ||
                app_name.contains("brave")
            {
                return Some(MeetingPlatform::Browser(title));
            }
        }

        None
    }

    pub fn run(self: Arc<Self>) {
        let detector = self.clone();
        tauri::async_runtime::spawn(async move {
            let cooldown = Duration::from_secs(5);

            loop {
                let now = Instant::now();

                let (should_init, platform_to_emit) = {
                    let mut state = detector.state.lock().unwrap();
                    let mut last_active = detector.last_active.lock().unwrap();
                    let mut current_meeting = detector.current_meeting.lock().unwrap();

                    let mut init_needed = false;
                    let mut p_emit = None;

                    let active_platform = detector.detect_meeting();

                    if let Some(detected) = active_platform {
                        *last_active = now;
                        if current_meeting.is_none() {
                            *current_meeting = Some(detected.clone());
                        }
                        if
                            current_meeting.is_some() &&
                            *state == MeetingState::None &&
                            !detector.dismissed.load(Ordering::Relaxed)
                        {
                            *state = MeetingState::Active;
                            init_needed = true;
                            p_emit = current_meeting.clone();
                        }
                    } else {
                        let mic_active = {
                            #[cfg(target_os = "windows")]
                            {
                                get_system_mic_usage()
                                    .iter()
                                    .any(|u| u.is_active)
                            }
                            #[cfg(target_os = "macos")]
                            {
                                MIC_ACTIVE.load(Ordering::Relaxed)
                            }
                        };

                        if !mic_active && now.duration_since(*last_active) > cooldown {
                            *state = MeetingState::None;
                            *current_meeting = None;
                            detector.dismissed.store(false, Ordering::Relaxed);
                            if
                                let Some(window) =
                                    detector.app_handle.get_webview_window("meetingWidget")
                            {
                                let _ = window.emit("meeting-ended", ());
                            }
                        }
                    }

                    (init_needed, p_emit)
                };

                if should_init {
                    let _ = session_init(
                        detector.app_handle.clone(),
                        "meetingWidget".to_string()
                    ).await;
                    if let Some(window) = detector.app_handle.get_webview_window("meetingWidget") {
                        let _ = window.show();
                        let _ = window.unminimize();
                        let _ = window.set_focus();
                        let _ = window.set_always_on_top(true);
                        if let Some(p) = platform_to_emit {
                            let _ = window.emit("meeting-detected", p);
                        }
                    }
                }

                tokio::time::sleep(Duration::from_millis(1000)).await;
            }
        });
    }

    #[cfg(target_os = "macos")]
    fn start_virtual_device_monitoring(device_name: &str, app_handle: &AppHandle<R>) {
        use cpal::{ SampleFormat, StreamConfig };
        // NOTE: tauri::api::dialog is removed in V2.
        // For now, we use eprintln. To show a UI dialog, install `tauri-plugin-dialog`.

        let host = cpal::default_host();
        let device = match
            host
                .input_devices()
                .unwrap()
                .find(|d| {
                    d.name()
                        .map(|n| n == device_name)
                        .unwrap_or(false)
                })
        {
            Some(d) => d,
            None => {
                eprintln!("Virtual audio device '{}' not found.", device_name);
                return;
            }
        };

        // Correctly get the format and config
        let default_config = device
            .default_input_config()
            .expect("Failed to get default input config");
        let sample_format = default_config.sample_format();
        let config: StreamConfig = default_config.into();

        let err_fn = |err| eprintln!("Stream error: {:?}", err);

        let stream = (
            match sample_format {
                SampleFormat::F32 =>
                    device.build_input_stream(
                        &config,
                        move |data: &[f32], _: &cpal::InputCallbackInfo| {
                            let peak = data
                                .iter()
                                .copied()
                                .fold(0.0_f32, |a, b| a.max(b.abs()));
                            MIC_ACTIVE.store(peak > 0.001, Ordering::Relaxed);
                        },
                        err_fn,
                        None // Added 4th argument (timeout)
                    ),
                SampleFormat::I16 =>
                    device.build_input_stream(
                        &config,
                        move |data: &[i16], _: &cpal::InputCallbackInfo| {
                            let peak = data
                                .iter()
                                .copied()
                                .map(|v| (v as f32) / (i16::MAX as f32))
                                .fold(0.0_f32, |a, b| a.max(b.abs()));
                            MIC_ACTIVE.store(peak > 0.001, Ordering::Relaxed);
                        },
                        err_fn,
                        None // Added 4th argument
                    ),
                _ => {
                    eprintln!("Unsupported sample format");
                    return;
                }
            }
        ).expect("Failed to build input stream");

        // Important: Keep the stream alive
        Box::leak(Box::new(stream)).play().expect("Failed to play stream");
    }
}
