#[cfg(target_os = "windows")]
use windows::{
    Win32::{
        Media::Audio::{
            // Check if it's here:
            Endpoints::IAudioMeterInformation,
            eCapture,
            eCommunications,
            IMMDeviceEnumerator,
            MMDeviceEnumerator,
        },
        System::Com::{ CoCreateInstance, CoInitializeEx, CLSCTX_ALL, COINIT_MULTITHREADED },
    },
};

#[cfg(target_os = "windows")]
#[derive(Debug)]
pub struct AppAudioUsage {
    pub process_name: String,
    pub is_active: bool,
    pub _level: f32,
}

#[cfg(target_os = "windows")]
pub fn get_system_mic_usage() -> Vec<AppAudioUsage> {
    unsafe {
        let mut result = Vec::new();

        // 1. Initialize COM
        let _ = CoInitializeEx(None, COINIT_MULTITHREADED);

        // 2. Create device enumerator
        let enumerator: IMMDeviceEnumerator = CoCreateInstance(
            &MMDeviceEnumerator,
            None,
            CLSCTX_ALL
        ).expect("Failed to create enumerator");

        // 3. Get default capture device
        let device = match enumerator.GetDefaultAudioEndpoint(eCapture, eCommunications) {
            Ok(d) => d,
            Err(_) => {
                return result;
            }
        };

        // 4. Activate IAudioMeterInformation
        let meter: IAudioMeterInformation = match device.Activate(CLSCTX_ALL, None) {
            Ok(m) => m,
            Err(e) => {
                eprintln!("Failed to activate meter: {:?}", e);
                return result;
            }
        };

        // 5. Get peak value (Now returns Result<f32>)
        // We catch the returned value directly instead of passing a reference
        if let Ok(peak) = meter.GetPeakValue() {
            result.push(AppAudioUsage {
                process_name: "SystemMic".to_string(),
                is_active: peak > 0.001,
                _level: peak,
            });
        }

        result
    }
}

#[cfg(target_os = "macos")]
pub fn get_system_mic_usage() -> Vec<AppAudioUsage> {
    // macOS doesn't expose mic usage per-app the same way Windows APIs do.
    // Return an empty vec or implement Apple-specific logic here.
    Vec::new()
}
