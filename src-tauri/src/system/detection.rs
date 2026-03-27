use crate::commands::session::session_init;
use crate::system::system_audio::get_system_mic_usage;
use active_win_pos_rs::get_active_window;
use serde::Serialize;
use std::sync::atomic::{ AtomicBool, Ordering };
use std::sync::{ Arc, Mutex };
use std::time::{ Duration, Instant };
use sysinfo::{ System, ProcessRefreshKind, RefreshKind };
use tauri::{ AppHandle, Emitter, Manager, Runtime };

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
    Active, // Simplified state to prevent flickering
}

pub struct MeetingDetector<R: Runtime> {
    pub state: Arc<Mutex<MeetingState>>,
    pub last_active: Arc<Mutex<Instant>>,
    pub current_meeting: Arc<Mutex<Option<MeetingPlatform>>>, // <--- NEW: Lock the meeting here
    pub app_handle: AppHandle<R>,
    pub dismissed: AtomicBool,
    sys: Arc<Mutex<System>>,
}

impl<R: Runtime + 'static> MeetingDetector<R> {
    pub fn new(app_handle: AppHandle<R>) -> Self {
        Self {
            state: Arc::new(Mutex::new(MeetingState::None)),
            last_active: Arc::new(Mutex::new(Instant::now() - Duration::from_secs(10))),
            // Initialize the new latching field as None
            current_meeting: Arc::new(Mutex::new(None)),
            app_handle,
            dismissed: AtomicBool::new(false),
            sys: Arc::new(
                Mutex::new(
                    System::new_with_specifics(
                        RefreshKind::nothing().with_processes(ProcessRefreshKind::everything())
                    )
                )
            ),
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
            #[cfg(not(target_os = "windows"))]
            {
                false
            }
        };

        if !mic_active {
            return None;
        }

        // 2. Parse the Window Title for URL hints
        if let Ok(window) = get_active_window() {
            let title = window.title.clone();
            let title_lower = title.to_lowercase();

            // Google Meet usually puts the meeting code or "Meet" in the title
            if title_lower.contains("meet.google.com") || title_lower.contains("google meet") {
                return Some(MeetingPlatform::GoogleMeet(title));
            }

            // Zoom Web Client vs App
            if title_lower.contains("zoom meeting") || title_lower.contains("zoom.us") {
                return Some(MeetingPlatform::Zoom(title));
            }

            // Microsoft Teams
            if
                title_lower.contains("teams.microsoft.com") ||
                title_lower.contains("microsoft teams")
            {
                return Some(MeetingPlatform::Teams(title));
            }

            // Generic Browser Fallback (captures the Tab Title)
            let app_name = window.app_name.to_lowercase();
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

                    // Check mic and window
                    let active_platform = detector.detect_meeting();

                    if let Some(detected) = active_platform {
                        *last_active = now;

                        // Latch a meeting if not already latched
                        if current_meeting.is_none() {
                            *current_meeting = Some(detected.clone());
                        }

                        // Trigger only if we haven't shown the widget yet
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
                        // No active window, but don't close immediately if mic is on
                        let mic_active = get_system_mic_usage()
                            .iter()
                            .any(|u| u.is_active);

                        if !mic_active && now.duration_since(*last_active) > cooldown {
                            // Only reset if mic has been silent long enough
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

                // Async actions outside of the Mutex locks
                if should_init {
                    let _ = session_init(
                        detector.app_handle.clone(),
                        "meetingWidget".to_string()
                    ).await;
                    if
                        let (Some(window), Some(p)) = (
                            detector.app_handle.get_webview_window("meetingWidget"),
                            platform_to_emit,
                        )
                    {
                        let _ = window.emit("meeting-detected", p);
                    }
                }

                tokio::time::sleep(Duration::from_millis(1000)).await;
            }
        });
    }
}
