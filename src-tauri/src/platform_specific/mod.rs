#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "linux")]
pub mod linux;

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

/// Returns true if linux compatible
#[tauri::command]
pub async fn linux_checks() -> Result<(), String> {
    // Different behaviour depending on OS
    // MacOS is missing as it is not a target
    // in turn this means this application will not build on MacOS.
    #[cfg(target_os = "windows")]
    {
        Err("Not available on Windows".to_string())
    }

    #[cfg(target_os = "linux")]
    {
        linux::linux_checks_librs()
    }
}
