use anyhow::Result;

#[cfg(target_os = "windows")]
use windows::{
    Win32::Foundation::RPC_E_CHANGED_MODE,
    Win32::Media::Audio::Endpoints::*,
    Win32::Media::Audio::*,
    Win32::System::Com::*,
};

/// Balances a successful `CoInitializeEx` on drop.
///
/// The scheduler runs on tokio worker threads that are shared with other code
/// (e.g. `notification::show_toast`, which initializes COM as MTA and never
/// uninitializes). If this thread already has COM in a different apartment
/// mode, `CoInitializeEx` returns `RPC_E_CHANGED_MODE`; COM is still usable,
/// so we proceed without calling `CoUninitialize`.
#[cfg(target_os = "windows")]
struct ComGuard {
    should_uninit: bool,
}

#[cfg(target_os = "windows")]
impl ComGuard {
    fn init() -> Result<Self> {
        let hr = unsafe { CoInitializeEx(None, COINIT_MULTITHREADED) };
        if hr == RPC_E_CHANGED_MODE {
            return Ok(Self { should_uninit: false });
        }
        if hr.is_err() {
            return Err(anyhow::anyhow!("COM initialization failed: {:?}", hr));
        }
        Ok(Self { should_uninit: true })
    }
}

#[cfg(target_os = "windows")]
impl Drop for ComGuard {
    fn drop(&mut self) {
        if self.should_uninit {
            unsafe { CoUninitialize() };
        }
    }
}

/// Runs `f` against the default render endpoint's volume control.
#[cfg(target_os = "windows")]
fn with_endpoint_volume<T>(f: impl FnOnce(&IAudioEndpointVolume) -> Result<T>) -> Result<T> {
    // Declared first so it is dropped last, after all COM interfaces are released.
    let _com = ComGuard::init()?;

    unsafe {
        let enumerator: IMMDeviceEnumerator = CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)
            .map_err(|e| anyhow::anyhow!("Failed to create device enumerator: {}", e))?;

        let device = enumerator
            .GetDefaultAudioEndpoint(eRender, eConsole)
            .map_err(|e| anyhow::anyhow!("Failed to get default audio endpoint: {}", e))?;

        let endpoint_volume: IAudioEndpointVolume = device
            .Activate(CLSCTX_ALL, None)
            .map_err(|e| anyhow::anyhow!("Failed to activate audio device: {}", e))?;

        f(&endpoint_volume)
    }
}

#[cfg(target_os = "windows")]
pub fn mute_system_audio() -> Result<()> {
    with_endpoint_volume(|ev| unsafe {
        ev.SetMute(true, std::ptr::null())
            .map_err(|e| anyhow::anyhow!("Failed to mute audio: {}", e))
    })
}

#[cfg(target_os = "windows")]
pub fn unmute_system_audio() -> Result<()> {
    with_endpoint_volume(|ev| unsafe {
        ev.SetMute(false, std::ptr::null())
            .map_err(|e| anyhow::anyhow!("Failed to unmute audio: {}", e))
    })
}

#[cfg(target_os = "windows")]
#[allow(dead_code)]
pub fn set_system_volume(level: f32) -> Result<()> {
    with_endpoint_volume(|ev| unsafe {
        // Set volume level (0.0 to 1.0)
        ev.SetMasterVolumeLevelScalar(level.clamp(0.0, 1.0), std::ptr::null())
            .map_err(|e| anyhow::anyhow!("Failed to set volume: {}", e))
    })
}

#[cfg(target_os = "windows")]
pub fn is_muted() -> Result<bool> {
    with_endpoint_volume(|ev| unsafe {
        ev.GetMute()
            .map(|m| m.as_bool())
            .map_err(|e| anyhow::anyhow!("Failed to get mute status: {}", e))
    })
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
