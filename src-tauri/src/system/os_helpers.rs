use tauri::Window;

pub fn set_stealth_mode(window: Window, enabled: bool) -> Result<(), String> {
    // For Windows: This prevents the window from appearing in screenshots/screencasts
    #[cfg(target_os = "windows")]
    window
        .set_content_protected(enabled)
        .map_err(|e| e.to_string())?;

    // For macOS: This sets the window sharing policy to Disallowed
    #[cfg(target_os = "macos")]
    {
        use tauri::WindowSharingPolicy;
        let policy = if enabled {
            WindowSharingPolicy::Disallowed
        } else {
            WindowSharingPolicy::OnScreen
        };
        window
            .set_sharing_policy(policy)
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}
