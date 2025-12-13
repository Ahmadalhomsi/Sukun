use anyhow::Result;

#[cfg(target_os = "windows")]
use windows::{
    core::*,
    Win32::Media::Audio::Endpoints::*,
    Win32::Media::Audio::*,
    Win32::System::Com::*,
};

#[cfg(target_os = "windows")]
pub fn mute_system_audio() -> Result<()> {
    unsafe {
        // Initialize COM
        let hr = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
        if hr.is_err() {
            return Err(anyhow::anyhow!("COM initialization failed: {:?}", hr));
        }

        // Create device enumerator
        let enumerator: IMMDeviceEnumerator =
            match CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL) {
                Ok(e) => e,
                Err(e) => {
                    CoUninitialize();
                    return Err(anyhow::anyhow!("Failed to create device enumerator: {}", e));
                }
            };

        // Get default audio endpoint
        let device = match enumerator.GetDefaultAudioEndpoint(eRender, eConsole) {
            Ok(d) => d,
            Err(e) => {
                CoUninitialize();
                return Err(anyhow::anyhow!("Failed to get default audio endpoint: {}", e));
            }
        };

        // Activate the device
        let endpoint_volume: IAudioEndpointVolume = match device.Activate(CLSCTX_ALL, None) {
            Ok(ev) => ev,
            Err(e) => {
                CoUninitialize();
                return Err(anyhow::anyhow!("Failed to activate audio device: {}", e));
            }
        };

        // Mute the audio
        let result = endpoint_volume.SetMute(true, std::ptr::null());
        CoUninitialize();
        
        result.map_err(|e| anyhow::anyhow!("Failed to mute audio: {}", e))?;
        Ok(())
    }
}

#[cfg(target_os = "windows")]
pub fn unmute_system_audio() -> Result<()> {
    unsafe {
        let hr = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
        if hr.is_err() {
            return Err(anyhow::anyhow!("COM initialization failed: {:?}", hr));
        }

        let enumerator: IMMDeviceEnumerator =
            match CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL) {
                Ok(e) => e,
                Err(e) => {
                    CoUninitialize();
                    return Err(anyhow::anyhow!("Failed to create device enumerator: {}", e));
                }
            };

        let device = match enumerator.GetDefaultAudioEndpoint(eRender, eConsole) {
            Ok(d) => d,
            Err(e) => {
                CoUninitialize();
                return Err(anyhow::anyhow!("Failed to get default audio endpoint: {}", e));
            }
        };

        let endpoint_volume: IAudioEndpointVolume = match device.Activate(CLSCTX_ALL, None) {
            Ok(ev) => ev,
            Err(e) => {
                CoUninitialize();
                return Err(anyhow::anyhow!("Failed to activate audio device: {}", e));
            }
        };

        // Unmute the audio
        let result = endpoint_volume.SetMute(false, std::ptr::null());
        CoUninitialize();
        
        result.map_err(|e| anyhow::anyhow!("Failed to unmute audio: {}", e))?;
        Ok(())
    }
}

#[cfg(target_os = "windows")]
pub fn set_system_volume(level: f32) -> Result<()> {
    unsafe {
        let hr = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
        if hr.is_err() {
            return Err(anyhow::anyhow!("COM initialization failed: {:?}", hr));
        }

        let enumerator: IMMDeviceEnumerator =
            match CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL) {
                Ok(e) => e,
                Err(e) => {
                    CoUninitialize();
                    return Err(anyhow::anyhow!("Failed to create device enumerator: {}", e));
                }
            };

        let device = match enumerator.GetDefaultAudioEndpoint(eRender, eConsole) {
            Ok(d) => d,
            Err(e) => {
                CoUninitialize();
                return Err(anyhow::anyhow!("Failed to get default audio endpoint: {}", e));
            }
        };

        let endpoint_volume: IAudioEndpointVolume = match device.Activate(CLSCTX_ALL, None) {
            Ok(ev) => ev,
            Err(e) => {
                CoUninitialize();
                return Err(anyhow::anyhow!("Failed to activate audio device: {}", e));
            }
        };

        // Set volume level (0.0 to 1.0)
        let result = endpoint_volume.SetMasterVolumeLevelScalar(level.clamp(0.0, 1.0), std::ptr::null());
        CoUninitialize();
        
        result.map_err(|e| anyhow::anyhow!("Failed to set volume: {}", e))?;
        Ok(())
    }
}

#[cfg(target_os = "windows")]
pub fn is_muted() -> Result<bool> {
    unsafe {
        let hr = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
        if hr.is_err() {
            return Err(anyhow::anyhow!("COM initialization failed: {:?}", hr));
        }

        let enumerator: IMMDeviceEnumerator =
            match CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL) {
                Ok(e) => e,
                Err(e) => {
                    CoUninitialize();
                    return Err(anyhow::anyhow!("Failed to create device enumerator: {}", e));
                }
            };

        let device = match enumerator.GetDefaultAudioEndpoint(eRender, eConsole) {
            Ok(d) => d,
            Err(e) => {
                CoUninitialize();
                return Err(anyhow::anyhow!("Failed to get default audio endpoint: {}", e));
            }
        };

        let endpoint_volume: IAudioEndpointVolume = match device.Activate(CLSCTX_ALL, None) {
            Ok(ev) => ev,
            Err(e) => {
                CoUninitialize();
                return Err(anyhow::anyhow!("Failed to activate audio device: {}", e));
            }
        };

        let muted = match endpoint_volume.GetMute() {
            Ok(m) => m,
            Err(e) => {
                CoUninitialize();
                return Err(anyhow::anyhow!("Failed to get mute status: {}", e));
            }
        };

        CoUninitialize();
        Ok(muted.as_bool())
    }
}

// Placeholder implementations for non-Windows platforms
#[cfg(not(target_os = "windows"))]
pub fn mute_system_audio() -> Result<()> {
    println!("Audio muting not implemented for this platform");
    Ok(())
}

#[cfg(not(target_os = "windows"))]
pub fn unmute_system_audio() -> Result<()> {
    println!("Audio unmuting not implemented for this platform");
    Ok(())
}

#[cfg(not(target_os = "windows"))]
pub fn set_system_volume(_level: f32) -> Result<()> {
    println!("Volume control not implemented for this platform");
    Ok(())
}

#[cfg(not(target_os = "windows"))]
pub fn is_muted() -> Result<bool> {
    Ok(false)
}
