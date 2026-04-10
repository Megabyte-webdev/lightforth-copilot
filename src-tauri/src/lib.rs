pub mod audio;
pub mod commands;
use commands::ai::ask_ai;
mod system;

use audio::capture::{ AudioManager, SharedAudio };
use std::sync::{ atomic::Ordering, Arc, Mutex };
use system::detection::MeetingDetector;
use tauri::{
    menu::{ Menu, MenuItem },
    tray::{ TrayIconBuilder, TrayIconEvent },
    AppHandle,
    Emitter,
    Listener,
    Manager,
    State,
    Window,
};
use tauri_plugin_autostart::{ ManagerExt, MacosLauncher };
use crate::commands::session::{ session_close, session_init, analyze_session };

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
    app.emit("meeting-detected", "Test Zoom Session").map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
async fn dismiss_meeting(
    detector: tauri::State<'_, Arc<MeetingDetector<tauri::Wry>>>
) -> Result<(), String> {
    detector.dismissed.store(true, Ordering::Relaxed);
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder
        ::default()
        .manage(Arc::new(Mutex::new(AudioManager::new())) as SharedAudio)
        .plugin(tauri_plugin_autostart::init(MacosLauncher::LaunchAgent, Some(vec!["--minimized"])))
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let handle = app.handle().clone();
            let _ = app.autolaunch().enable();

            // 2. Create System Tray (Code from before...)
            let quit_i = MenuItem::with_id(app, "quit", "Quit Lightforth", true, None::<&str>)?;
            let show_i = MenuItem::with_id(app, "show", "Open Dashboard", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_i, &quit_i])?;

            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .on_menu_event(|app, event| {
                    match event.id.as_ref() {
                        "quit" => {
                            app.exit(0);
                        }
                        "show" => {
                            if let Some(window) = app.get_webview_window("main") {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                        _ => {}
                    }
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click { .. } = event {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                })
                .build(app)?;

            let detector = Arc::new(MeetingDetector::new(handle.clone()));
            app.manage(detector.clone());

            let detector_worker = detector.clone();
            std::thread::spawn(move || {
                detector_worker.run();
            });
            // 4. SMART WINDOW VISIBILITY
            // Check if we started with "--minimized" (usually from Autostart)
            let args: Vec<String> = std::env::args().collect();
            let is_minimized = args.contains(&"--minimized".to_string());

            if !is_minimized {
                if let Some(main_window) = app.get_webview_window("main") {
                    let _ = main_window.show();
                    let _ = main_window.set_focus();
                }
            }
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
        .invoke_handler(
            tauri::generate_handler![
                start_audio,
                stop_audio,
                start_session,
                end_session,
                stealth_mode,
                trigger_test_meeting,
                dismiss_meeting,
                ask_ai,
                analyze_session
            ]
        )
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|app_handle, event| {
            match event {
                tauri::RunEvent::ExitRequested { api, .. } => {
                    // Prevent the process from actually killing itself
                    api.prevent_exit();

                    // Optional: Hide all windows so it feels "closed"
                    for window in app_handle.webview_windows().values() {
                        let _ = window.hide();
                    }
                }

                tauri::RunEvent::WindowEvent {
                    label,
                    event: tauri::WindowEvent::CloseRequested { api, .. },
                    ..
                } => {
                    if label == "main" {
                        // Hide the dashboard instead of killing the app
                        let _ = app_handle.get_webview_window("main").unwrap().hide();
                        api.prevent_close();
                    }
                }
                _ => {}
            }
        });
}
