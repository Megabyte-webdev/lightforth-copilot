use tauri::Window;

pub fn set_stealth_mode(window: Window, enabled: bool) -> Result<(), String> {
    // Windows: Native content protection (Screenshots show a black window)
    #[cfg(target_os = "windows")]
    {
        window.set_content_protected(enabled).map_err(|e| e.to_string())?;
    }

    // macOS: Adjust sharing policy to prevent screen capture
    #[cfg(target_os = "macos")]
    {
        // Note: WindowSharingPolicy is only available in Tauri v2+.
        // If you are on v1, this block will fail.
        // For v1, you typically omit this or use a plugin.

        // If using Tauri v2, ensure the 'macos-private-api' or similar
        // features aren't required in your Cargo.toml.
        use tauri::image::WindowSharingPolicy; // Check path based on your Tauri version

        let policy = if enabled {
            WindowSharingPolicy::Disallowed
        } else {
            WindowSharingPolicy::OnScreen
        };

        window.set_sharing_policy(policy).map_err(|e: tauri::Error| e.to_string())?; // Added explicit type for E0282
    }

    Ok(())
}
