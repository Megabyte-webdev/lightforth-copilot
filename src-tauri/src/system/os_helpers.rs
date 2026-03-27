use tauri::Window;

pub fn set_stealth_mode(window: Window, enabled: bool) -> Result<(), String> {
    // ---------------------------
    // Windows: Native content protection
    // ---------------------------
    #[cfg(target_os = "windows")]
    {
        window.set_content_protected(enabled).map_err(|e| e.to_string())?;
    }

    // ---------------------------
    // macOS: call native capture protection
    // ---------------------------
    #[cfg(target_os = "macos")]
    {
        set_macos_capture_protection(&window, enabled);
    }

    Ok(())
}

// ---------------------------
// macOS native capture protection
// ---------------------------
#[cfg(target_os = "macos")]
fn set_macos_capture_protection(window: &Window, enabled: bool) {
    use cocoa::appkit::{ NSWindow, NSWindowSharingType };
    use cocoa::base::id;
    use objc::{ msg_send, sel, sel_impl };

    unsafe {
        let ns_window = window.ns_window().unwrap() as id;

        let policy = if enabled {
            NSWindowSharingType::NSWindowSharingNone
        } else {
            NSWindowSharingType::NSWindowSharingReadOnly
        };

        let _: () = msg_send![ns_window, setSharingType: policy];
    }
}
