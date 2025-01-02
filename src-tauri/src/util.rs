//! This module contains various utility/helper functions that do not fit into any other module

/// Returns true if built in debug mode
#[tauri::command]
pub async fn is_debug_mode() -> bool {
    cfg!(debug_assertions)
}
