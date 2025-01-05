use std::{env, time::Duration};

mod constants;
mod development;
mod github;
mod mod_management;
mod northstar;
mod platform_specific;
mod repair_and_verify;
mod thunderstore;
mod util;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, Debug, Clone, TS)]
#[ts(export)]
struct NorthstarThunderstoreRelease {
    package: String,
    version: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, TS)]
#[ts(export)]
pub struct NorthstarThunderstoreReleaseWrapper {
    label: String,
    value: NorthstarThunderstoreRelease,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            development::install_git_main,
            github::compare_tags,
            github::get_list_of_tags,
            github::pull_requests::apply_launcher_pr,
            github::pull_requests::apply_mods_pr,
            github::pull_requests::get_launcher_download_link,
            github::pull_requests::get_pull_requests_wrapper,
            github::release_notes::check_is_flightcore_outdated,
            github::release_notes::generate_release_note_announcement,
            github::release_notes::get_newest_flightcore_version,
            github::release_notes::get_northstar_release_notes,
            mod_management::delete_northstar_mod,
            mod_management::delete_thunderstore_mod,
            mod_management::get_installed_mods_and_properties,
            mod_management::install_mod_wrapper,
            mod_management::set_mod_enabled_status,
            northstar::check_is_northstar_outdated,
            northstar::get_available_northstar_versions,
            northstar::get_northstar_version_number,
            northstar::install::find_game_install_location,
            northstar::install::install_northstar_wrapper,
            northstar::install::update_northstar,
            northstar::launch_northstar,
            northstar::profile::clone_profile,
            northstar::profile::delete_profile,
            northstar::profile::fetch_profiles,
            northstar::profile::validate_profile,
            platform_specific::check_cgnat,
            platform_specific::get_host_os,
            platform_specific::get_local_northstar_proton_wrapper_version,
            platform_specific::install_northstar_proton_wrapper,
            platform_specific::uninstall_northstar_proton_wrapper,
            repair_and_verify::clean_up_download_folder_wrapper,
            repair_and_verify::disable_all_but_core,
            repair_and_verify::get_log_list,
            repair_and_verify::verify_game_files,
            repair_and_verify::verify_install_location,
            thunderstore::query_thunderstore_packages_api,
            util::close_application,
            util::force_panic,
            util::get_flightcore_version_number,
            util::get_server_player_count,
            util::is_debug_mode,
            util::kill_northstar,
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

/// Object holding various information about a Northstar mod
#[derive(Serialize, Deserialize, Debug, Clone, TS)]
#[ts(export)]
pub struct NorthstarMod {
    pub name: String,
    pub version: Option<String>,
    pub thunderstore_mod_string: Option<String>,
    pub enabled: bool,
    pub directory: String,
}
