#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "linux")]
pub mod linux;

/// Returns identifier of host OS FlightCore is running on
#[tauri::command]
pub fn get_host_os() -> String {
    std::env::consts::OS.to_string()
}

/// On Linux attempts to install NorthstarProton
/// On Windows simply returns an error message
#[tauri::command]
pub async fn install_northstar_proton_wrapper() -> Result<(), String> {
    #[cfg(target_os = "linux")]
    return linux::install_ns_proton().map_err(|err| err.to_string());

    #[cfg(target_os = "windows")]
    Err("Not supported on Windows".to_string())
}

#[tauri::command]
pub async fn uninstall_northstar_proton_wrapper() -> Result<(), String> {
    #[cfg(target_os = "linux")]
    return linux::uninstall_ns_proton();

    #[cfg(target_os = "windows")]
    Err("Not supported on Windows".to_string())
}

#[tauri::command]
pub async fn get_local_northstar_proton_wrapper_version() -> Result<String, String> {
    #[cfg(target_os = "linux")]
    return linux::get_local_ns_proton_version();

    #[cfg(target_os = "windows")]
    Err("Not supported on Windows".to_string())
}

/// Check whether the current device might be behind a CGNAT
#[tauri::command]
pub async fn check_cgnat() -> Result<String, String> {
    #[cfg(target_os = "linux")]
    return Err("Not supported on Linux".to_string());

    #[cfg(target_os = "windows")]
    windows::check_cgnat().await
}
