use super::super::audio::capture::SharedAudio;
use tauri::{AppHandle, Manager, Runtime, State};

pub async fn session_init<R: Runtime>(
    app: AppHandle<R>,
    target_label: impl Into<Option<String>>, // Optional: "widget" or "meeting-widget"
) {
    // Convert the input into an Option<String>
    let target_label: Option<String> = target_label.into();

    let label = target_label.unwrap_or_else(|| "widget".to_string());

    let main_win = app.get_webview_window("main");
    let target_win = app.get_webview_window(&label);

    if let (Some(main), Some(target)) = (main_win, target_win) {
        //UI Swap Logic
        let _ = main.hide();

        // Prepare the target window
        let _ = target.show();
        let _ = target.set_focus();
        let _ = target.set_always_on_top(true);

        println!(">>> [SUCCESS] Switched to window: {}", label);
    } else {
        eprintln!(">>> [ERROR] Could not find window with label: {}", label);
    }
}
pub async fn session_close(app: AppHandle, audio_state: State<'_, SharedAudio>) {
    let main_win = app.get_webview_window("main");
    let widget_win = app.get_webview_window("widget");

    // Access state within async commands correctly
    {
        let mut manager = audio_state.lock().unwrap();
        manager.stop();
    }

    if let (Some(main), Some(widget)) = (main_win, widget_win) {
        let _ = widget.hide();
        let _ = main.show();
        let _ = main.set_focus();
    }
}
