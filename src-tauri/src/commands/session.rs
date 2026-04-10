use super::super::audio::capture::SharedAudio;
use tauri::{ AppHandle, Manager, Runtime, State };
use serde_json::json;
pub async fn session_init<R: Runtime>(
    app: AppHandle<R>,
    target_label: impl Into<Option<String>> // Optional: "widget" or "meeting-widget"
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

#[tauri::command]
pub async fn analyze_session(session_id: String, transcript: String) -> Result<String, String> {
    let client = reqwest::Client::new();

    let res = client
        .post(&format!("http://localhost:3000/api/sessions/{}/analyze", session_id))
        .json(&json!({ "transcript": transcript }))
        .send().await
        .map_err(|e| format!("Network error: {}", e))?;

    let status = res.status(); // <-- take status first
    if !status.is_success() {
        let txt = res.text().await.unwrap_or_default(); // moves `res`
        return Err(format!("Backend error {}: {}", status, txt));
    }

    // Now we can still use `res` because we only moved it in the error branch
    let body: serde_json::Value = res.json().await.map_err(|e| format!("Invalid JSON: {}", e))?;

    let insight = body["insight"]
        .as_str()
        .ok_or_else(|| "No insight returned".to_string())?
        .to_string();

    Ok(insight)
}
