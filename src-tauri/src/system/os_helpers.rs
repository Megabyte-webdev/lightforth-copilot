use tauri::Window;

#[cfg(target_os = "macos")]
use crate::system::macos;

pub fn set_stealth_mode(window: Window, enabled: bool) -> Result<(), String> {
    // Windows
    #[cfg(target_os = "windows")]
    {
        window.set_content_protected(enabled).map_err(|e| e.to_string())?;
    }

    // macOS
    #[cfg(target_os = "macos")]
    {
        macos::set_window_sharing(&window, enabled)?;
    }

    Ok(())
}
