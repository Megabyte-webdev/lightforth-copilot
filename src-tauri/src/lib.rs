pub mod audio;
pub mod commands;
mod system;

use audio::capture::AudioManager;
use std::sync::Mutex;
use tauri::{AppHandle, Listener, Manager, Window};
lazy_static::lazy_static! {
    static ref AUDIO_MANAGER: Mutex<AudioManager> = Mutex::new(AudioManager::new());
}

#[tauri::command]
fn start_audio(app_handle: AppHandle) {
    let mut manager = AUDIO_MANAGER.lock().unwrap();
    if let Err(e) = manager.start(app_handle) {
        eprintln!("Start audio error: {}", e);
    }
}

#[tauri::command]
fn stop_audio() {
    let mut manager = AUDIO_MANAGER.lock().unwrap();
    manager.stop();
}

#[tauri::command]
async fn start_session(app: AppHandle) {
    // Safely get windows. If they don't exist, we just return instead of crashing.
    let main_win = app.get_webview_window("main");
    let widget_win = app.get_webview_window("widget");

    if let (Some(main), Some(widget)) = (main_win, widget_win) {
        let _ = main.hide();
        let _ = widget.show();
        let _ = widget.set_focus();
    } else {
        eprintln!(">>> [ERROR] One or more windows (main/widget) not found in config.");
    }
}

#[tauri::command]
async fn end_session(app: AppHandle) {
    let main_win = app.get_webview_window("main");
    let widget_win = app.get_webview_window("widget");

    // Stop audio using global
    let mut manager = AUDIO_MANAGER.lock().unwrap();
    manager.stop();

    if let (Some(main), Some(widget)) = (main_win, widget_win) {
        let _ = widget.hide();
        let _ = main.show();
        let _ = main.set_focus();
    }
}

#[tauri::command]
async fn stealth_mode(window: Window, mode: bool) {
    let _ = window.set_skip_taskbar(mode);
    let _ = system::os_helpers::set_stealth_mode(window, mode);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            //let handle = app.handle();

            // Ensure audio stops on app exit
            app.listen("tauri://close-requested", move |_| {
                let mut manager = AUDIO_MANAGER.lock().unwrap();
                manager.stop();
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            start_audio,
            stop_audio,
            start_session,
            end_session,
            stealth_mode
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
