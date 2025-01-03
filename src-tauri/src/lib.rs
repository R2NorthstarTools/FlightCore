mod constants;
mod github;
mod northstar;
mod platform_specific;
mod repair_and_verify;
mod util;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            github::release_notes::check_is_flightcore_outdated,
            github::release_notes::get_northstar_release_notes,
            northstar::install::find_game_install_location,
            repair_and_verify::verify_install_location,
            util::force_panic,
            util::get_flightcore_version_number,
            util::get_server_player_count,
            util::is_debug_mode,
            util::open_repair_window,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/// Defines how Titanfall2 was installed (Steam, Origin, ...)
#[derive(Serialize, Deserialize, Debug, Clone, TS)]
#[ts(export)]
pub enum InstallType {
    STEAM,
    ORIGIN,
    EAPLAY,
    UNKNOWN,
}

/// Object holding information of the Titanfall2 install, including
/// - Install path
/// - Active profile
/// - Type of installation (Steam, Origin, ...)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameInstall {
    pub game_path: String,
    pub profile: String,
    pub install_type: InstallType,
}
