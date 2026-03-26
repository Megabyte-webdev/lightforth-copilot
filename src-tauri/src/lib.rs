pub mod audio;
pub mod commands;
mod system;

use audio::capture::{AudioManager, SharedAudio};
use std::sync::{atomic::Ordering, Arc, Mutex};
use system::detection::MeetingDetector;
use tauri::{AppHandle, Emitter, Listener, Manager, State, Window};

use crate::commands::session::{session_close, session_init};

#[tauri::command]
fn start_audio(app_handle: AppHandle, audio_state: State<'_, SharedAudio>) {
    // Accessing state via the State guard is the idiomatic way
    let mut manager = audio_state.lock().unwrap();
    if let Err(e) = manager.start(app_handle) {
        eprintln!("Start audio error: {}", e);
    }
}

#[tauri::command]
fn stop_audio(audio_state: State<'_, SharedAudio>) {
    let mut manager = audio_state.lock().unwrap();
    manager.stop();
}

#[tauri::command]
async fn start_session(app: AppHandle) -> Result<(), String> {
    // Await the session initialization
    session_init(app, None).await;
    Ok(())
}

#[tauri::command]
async fn end_session(app: AppHandle, audio_state: State<'_, SharedAudio>) -> Result<(), String> {
    // 2. Run the session close logic
    session_close(app, audio_state).await;

    Ok(())
}

#[tauri::command]
async fn stealth_mode(window: Window, mode: bool) {
    let _ = window.set_skip_taskbar(mode);
    let _ = system::os_helpers::set_stealth_mode(window, mode);
}
#[tauri::command]
fn trigger_test_meeting(app: tauri::AppHandle) -> Result<(), String> {
    // This manually emits the event that the React widget is listening for
    app.emit("meeting-detected", "Test Zoom Session")
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
async fn dismiss_meeting(
    detector: tauri::State<'_, Arc<MeetingDetector<tauri::Wry>>>,
) -> Result<(), String> {
    detector.dismissed.store(true, Ordering::Relaxed);
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(Arc::new(Mutex::new(AudioManager::new())) as SharedAudio)
        .plugin(tauri_plugin_autostart::Builder::new().build())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let handle = app.handle().clone();

            // If the user opened the app manually, you might want to show the main window
            if let Some(main_window) = app.get_webview_window("main") {
                let _ = main_window.show();
                let _ = main_window.set_focus();
            }

            // 2. Retrieve state during setup for the detector
            let audio_manager = app.state::<SharedAudio>().inner().clone();

            let detector = Arc::new(MeetingDetector::new(handle.clone(), audio_manager));
            app.manage(detector.clone());
            detector.run();

            // 3. Cleanup on exit
            let on_exit_handle = app.handle().clone();
            app.listen("tauri://close-requested", move |_| {
                let audio_state = on_exit_handle.state::<SharedAudio>();
                if let Ok(mut manager) = audio_state.lock() {
                    manager.stop();
                };
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            start_audio,
            stop_audio,
            start_session,
            end_session,
            stealth_mode,
            trigger_test_meeting,
            dismiss_meeting
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
