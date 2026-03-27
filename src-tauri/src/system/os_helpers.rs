use tauri::Window;

pub fn set_stealth_mode(window: Window, enabled: bool) -> Result<(), String> {
    // ---------------------------
    // Windows: TRUE capture protection
    // ---------------------------
    #[cfg(target_os = "windows")]
    {
        window.set_content_protected(enabled).map_err(|e| e.to_string())?;
    }

    // ---------------------------
    // macOS: NO real API → fallback strategy
    // ---------------------------
    #[cfg(target_os = "macos")]
    {
        if enabled {
            window.set_always_on_top(true).ok();
            window.set_ignore_cursor_events(false).ok();
        } else {
            window.set_always_on_top(false).ok();
        }
    }

    Ok(())
}
