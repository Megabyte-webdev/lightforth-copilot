#[cfg(target_os = "macos")]
use tauri::Window;

#[cfg(target_os = "macos")]
use objc2_app_kit::{ NSWindow, NSWindowSharingType };

#[cfg(target_os = "macos")]
pub fn set_window_sharing(window: &Window, enabled: bool) -> Result<(), String> {
    unsafe {
        // Tauri gives raw NSWindow pointer
        let ptr = window.ns_window().map_err(|e| e.to_string())?;

        if ptr.is_null() {
            return Err("Failed to get NSWindow".into());
        }

        let ns_window = &*(ptr as *mut NSWindow);

        let policy = if enabled {
            NSWindowSharingType::None
        } else {
            NSWindowSharingType::ReadOnly
        };

        ns_window.setSharingType(policy);

        Ok(())
    }
}
