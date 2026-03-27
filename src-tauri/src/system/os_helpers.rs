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
            // Keep visible to user
            window.set_opacity(1.0).ok();

            // Prevent interaction leaks
            window.set_ignore_cursor_events(false).ok();

            // Optional: force always on top to avoid capture layering issues
            window.set_always_on_top(true).ok();
        } else {
            window.set_always_on_top(false).ok();
        }
    }

    Ok(())
}
