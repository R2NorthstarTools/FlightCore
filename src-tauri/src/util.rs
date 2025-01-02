//! This module contains various utility/helper functions that do not fit into any other module

/// Returns true if built in debug mode
#[tauri::command]
pub async fn is_debug_mode() -> bool {
    cfg!(debug_assertions)
}

/// Returns the current version number as a string
#[tauri::command]
pub async fn get_flightcore_version_number() -> String {
    let version = env!("CARGO_PKG_VERSION");
    if cfg!(debug_assertions) {
        // Debugging enabled
        format!("v{} (debug mode)", version)
    } else {
        // Debugging disabled
        format!("v{}", version)
    }
}
