//! Windows toast notifications that work correctly under MSIX packaging.
//!
//! `tauri-plugin-notification` passes the Tauri config identifier (e.g.
//! `AlhomsiDev.Sukun`) as the AUMID to `ToastNotificationManager`, but an MSIX
//! packaged app is registered under `{PackageFamilyName}!{ApplicationId}` (which
//! contains an underscore and an exclamation mark, neither of which Tauri allows
//! in its identifier). The mismatch causes Windows to silently drop the toast.
//!
//! This module resolves the real AUMID for packaged apps and sends toasts
//! directly via the WinRT API.

use std::path::PathBuf;
use windows::{
    core::HSTRING,
    Data::Xml::Dom::XmlDocument,
    UI::Notifications::{ToastNotification, ToastNotificationManager},
};

/// Append a diagnostic line to `notification.log` in the RoamingAppData dir so
/// we can see exactly which branch is taken and whether any WinRT calls fail.
fn log(line: &str) {
    let _ = try_log(line);
}

fn try_log(line: &str) -> std::io::Result<()> {
    let mut path: PathBuf = std::env::var("APPDATA")
        .map(PathBuf::from)
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::NotFound, "APPDATA"))?;
    path.push("Sukun");
    path.push("notification.log");
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    use std::io::Write;
    let mut f = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)?;
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0);
    writeln!(f, "[{ts}] {line}")?;
    Ok(())
}

/// Returns `true` when the process is running inside an MSIX/AppX package.
#[cfg(target_os = "windows")]
pub fn is_packaged() -> bool {
    use windows::Win32::Storage::Packaging::Appx::GetCurrentPackageFullName;

    let mut len: u32 = 0;
    let err = unsafe { GetCurrentPackageFullName(&mut len, windows::core::PWSTR::null()) };
    let packaged = err.0 == 0 || err.0 == 122;
    log(&format!("is_packaged: err={} len={} => {packaged}", err.0, len));
    packaged
}

/// Resolves the AUMID Windows actually registered for this process.
///
/// Strategy for packaged apps:
/// 1. Ask `GetCurrentApplicationUserModelId` (returns PFN!AppId when available).
/// 2. If empty, derive it from `ApplicationModel::Package` family name + "!App".
///
/// For unpackaged apps, returns the fallback identifier unchanged.
#[cfg(target_os = "windows")]
pub fn resolve_aumid(fallback: &str) -> String {
    if !is_packaged() {
        log(&format!("resolve_aumid: not packaged, using fallback={fallback}"));
        return fallback.to_string();
    }

    // Strategy 1: GetCurrentApplicationUserModelId
    if let Some(aumid) = current_aumid() {
        log(&format!("resolve_aumid: GetCurrentApplicationUserModelId => {aumid}"));
        return aumid;
    }

    // Strategy 2: Construct from PackageFamilyName + "!App"
    // "App" matches the <Application Id="App"> in the AppxManifest.xml.
    if let Some(family) = package_family_name() {
        let aumid = format!("{family}!App");
        log(&format!("resolve_aumid: constructed from PFN => {aumid}"));
        return aumid;
    }

    log(&format!("resolve_aumid: all strategies failed, fallback={fallback}"));
    fallback.to_string()
}

#[cfg(target_os = "windows")]
fn current_aumid() -> Option<String> {
    use windows::Win32::Storage::Packaging::Appx::GetCurrentApplicationUserModelId;

    let mut len: u32 = 0;
    let err = unsafe {
        GetCurrentApplicationUserModelId(&mut len, windows::core::PWSTR::null())
    };
    if err.0 != 0 && err.0 != 122 {
        return None;
    }
    if len == 0 {
        return None;
    }
    let mut buf = vec![0u16; len as usize];
    let err = unsafe {
        GetCurrentApplicationUserModelId(&mut len, windows::core::PWSTR(buf.as_mut_ptr()))
    };
    if err.0 != 0 {
        return None;
    }
    let s = String::from_utf16_lossy(&buf);
    let trimmed = s.trim_end_matches('\0').to_string();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed)
    }
}

#[cfg(target_os = "windows")]
fn package_family_name() -> Option<String> {
    use windows::ApplicationModel::Package;
    let package = Package::Current().ok()?;
    let id = package.Id().ok()?;
    let family = id.FamilyName().ok()?;
    let s = family.to_string();
    if s.is_empty() {
        None
    } else {
        Some(s)
    }
}

/// Shows a Windows toast notification with the correct AUMID for the current
/// execution context (packaged MSIX vs unpackaged exe).
pub fn show_toast(
    title: &str,
    body: &str,
    fallback_aumid: &str,
    sound: bool,
) -> Result<(), String> {
    log(&format!(
        "show_toast: title={title:?} fallback={fallback_aumid:?} sound={sound}"
    ));

    #[cfg(target_os = "windows")]
    {
        // WinRT/COM must be initialized on the calling thread, otherwise every
        // WinRT call (XmlDocument, ToastNotificationManager) silently fails with
        // CO_NOTINITIALIZED (0x800401F0). The scheduler runs on a tokio thread
        // that is NOT initialized by default.
        let _ = unsafe {
            windows::Win32::System::Com::CoInitializeEx(
                None,
                windows::Win32::System::Com::COINIT_MULTITHREADED,
            )
        };
        log("show_toast: CoInitializeEx done");

        let audio = if sound {
            r#"<audio src="ms-winsoundevent:Notification.Default" />"#
        } else {
            r#"<audio silent="true" />"#
        };
        let xml = format!(
            r#"<toast><visual><binding template="ToastGeneric"><text id="1">{}</text><text id="2">{}</text></binding></visual>{}</toast>"#,
            quick_xml_escape(title),
            quick_xml_escape(body),
            audio
        );
        log(&format!("show_toast: xml={xml}"));

        let doc = XmlDocument::new().map_err(|e| { let m = format!("XmlDocument::new: {e}"); log(&format!("ERR {m}")); m })?;
        doc.LoadXml(&HSTRING::from(&xml)).map_err(|e| { let m = format!("LoadXml: {e}"); log(&format!("ERR {m}")); m })?;

        let toast = ToastNotification::CreateToastNotification(&doc)
            .map_err(|e| { let m = format!("CreateToastNotification: {e}"); log(&format!("ERR {m}")); m })?;

        let aumid = resolve_aumid(fallback_aumid);
        log(&format!("show_toast: final aumid={aumid}"));

        let notifier = ToastNotificationManager::CreateToastNotifierWithId(&HSTRING::from(&aumid))
            .map_err(|e| { let m = format!("CreateToastNotifierWithId (aumid={aumid}): {e}"); log(&format!("ERR {m}")); m })?;

        notifier.Show(&toast).map_err(|e| { let m = format!("Show: {e}"); log(&format!("ERR {m}")); m })?;
        log("show_toast: OK");
    }
    #[cfg(not(target_os = "windows"))]
    {
        let _ = (title, body, fallback_aumid, sound);
    }
    Ok(())
}

fn quick_xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('\'', "&apos;")
        .replace('"', "&quot;")
}
