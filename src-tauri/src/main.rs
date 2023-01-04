#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{
    env,
    sync::{Arc, Mutex},
    time::Duration,
};

use app::*;

mod github;
use github::release_notes::{
    check_is_flightcore_outdated, get_newest_flightcore_version, get_northstar_release_notes,
};

mod repair_and_verify;
use repair_and_verify::{
    clean_up_download_folder, disable_all_but_core, get_log_list, verify_game_files,
};

mod mod_management;
use mod_management::{
    fc_download_mod_and_install, get_installed_mods_and_properties, set_mod_enabled_status, delete_northstar_mod,
};

mod northstar;
use northstar::get_northstar_version_number;

use tauri::Manager;
use tauri_plugin_store::PluginBuilder;
use tokio::time::sleep;

#[derive(Default)]
struct Counter(Arc<Mutex<i32>>);

fn main() {
    // Only enable Sentry crash logs on release
    #[cfg(not(debug_assertions))]
    let _guard = sentry::init((
        "https://f833732deb2240b0b2dc4abce97d0f1d@o1374052.ingest.sentry.io/6692177",
        sentry::ClientOptions {
            release: sentry::release_name!(),
            ..Default::default()
        },
    ));

    tauri::Builder::default()
        .plugin(PluginBuilder::default().build())
        .setup(|app| {
            let app_handle = app.app_handle();
            tauri::async_runtime::spawn(async move {
                loop {
                    sleep(Duration::from_millis(2000)).await;
                    // println!("sending backend ping");
                    app_handle.emit_all("backend-ping", "ping").unwrap();
                }
            });
            let app_handle = app.app_handle();
            tauri::async_runtime::spawn(async move {
                loop {
                    sleep(Duration::from_millis(2000)).await;
                    app_handle
                        .emit_all("origin-running-ping", check_origin_running())
                        .unwrap();
                }
            });
            let app_handle = app.app_handle();
            tauri::async_runtime::spawn(async move {
                loop {
                    sleep(Duration::from_millis(2000)).await;
                    app_handle
                        .emit_all("northstar-running-ping", check_northstar_running())
                        .unwrap();
                }
            });

            Ok(())
        })
        .manage(Counter(Default::default()))
        .invoke_handler(tauri::generate_handler![
            force_panic,
            find_game_install_location_caller,
            get_flightcore_version_number,
            get_northstar_version_number_caller,
            check_is_northstar_outdated,
            verify_install_location,
            get_host_os_caller,
            install_northstar_caller,
            update_northstar_caller,
            launch_northstar_caller,
            check_is_flightcore_outdated_caller,
            get_log_list_caller,
            verify_game_files_caller,
            get_enabled_mods_caller,
            set_mod_enabled_status_caller,
            disable_all_but_core_caller,
            is_debug_mode,
            get_northstar_release_notes,
            linux_checks,
            get_installed_mods_caller,
            install_mod_caller,
            clean_up_download_folder_caller,
            get_newest_flightcore_version,
            delete_northstar_mod_caller,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
/// Wrapper for `find_game_install_location` as tauri doesn't allow passing `Result<>` types to front-end
async fn find_game_install_location_caller() -> Result<GameInstall, String> {
    find_game_install_location()
}

#[tauri::command]
/// This function's only use is to force a `panic!()`
// This must NOT be async to ensure crashing whole application.
fn force_panic() {
    panic!("Force panicked!");
}

#[tauri::command]
/// Returns true if built in debug mode
async fn is_debug_mode() -> bool {
    return cfg!(debug_assertions);
}

#[tauri::command]
/// Returns true if linux compatible
async fn linux_checks() -> Result<(), String> {
    // Early return if Windows
    if get_host_os() == "windows" {
        return Err("Not available on Windows".to_string());
    }

    linux_checks_librs()
}

#[tauri::command]
/// Returns the current version number as a string
async fn get_flightcore_version_number() -> String {
    let version = env!("CARGO_PKG_VERSION");
    if cfg!(debug_assertions) {
        // Debugging enabled
        format!("v{} (debug mode)", version)
    } else {
        // Debugging disabled
        format!("v{}", version)
    }
}

#[tauri::command]
async fn get_northstar_version_number_caller(game_path: String) -> Result<String, String> {
    match get_northstar_version_number(game_path) {
        Ok(version_number) => Ok(version_number),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
/// Checks if installed Northstar version is up-to-date
/// false -> Northstar install is up-to-date
/// true  -> Northstar install is outdated
async fn check_is_northstar_outdated(
    game_path: String,
    northstar_package_name: Option<String>,
) -> Result<bool, String> {
    let northstar_package_name = match northstar_package_name {
        Some(northstar_package_name) => {
            if northstar_package_name.len() <= 1 {
                "Northstar".to_string()
            } else {
                northstar_package_name
            }
        }
        None => "Northstar".to_string(),
    };

    let index = thermite::api::get_package_index().await.unwrap().to_vec();
    let nmod = index
        .iter()
        .find(|f| f.name.to_lowercase() == northstar_package_name.to_lowercase())
        .expect("Couldn't find Northstar on thunderstore???");
    // .ok_or_else(|| anyhow!("Couldn't find Northstar on thunderstore???"))?;

    let version_number = match get_northstar_version_number(game_path) {
        Ok(version_number) => version_number,
        Err(err) => {
            println!("{}", err);
            // If we fail to get new version just assume we are up-to-date
            return Err(err.to_string());
        }
    };

    // Release candidate version numbers are different between `mods.json` and Thunderstore
    let version_number = convert_release_candidate_number(version_number);

    if version_number != nmod.latest {
        println!("Installed Northstar version outdated");
        Ok(true)
    } else {
        println!("Installed Northstar version up-to-date");
        Ok(false)
    }
}

#[tauri::command]
/// Checks if installed FlightCore version is up-to-date
/// false -> FlightCore install is up-to-date
/// true  -> FlightCore install is outdated
async fn check_is_flightcore_outdated_caller() -> Result<bool, String> {
    check_is_flightcore_outdated().await
}

#[tauri::command]
/// Checks if is valid Titanfall2 install based on certain conditions
async fn verify_install_location(game_path: String) -> bool {
    match check_is_valid_game_path(&game_path) {
        Ok(()) => true,
        Err(err) => {
            println!("{}", err);
            false
        }
    }
}

#[tauri::command]
/// Returns identifier of host OS FlightCore is running on
async fn get_host_os_caller() -> String {
    get_host_os()
}

#[tauri::command]
/// Installs Northstar to the given path
async fn install_northstar_caller(
    game_path: String,
    northstar_package_name: Option<String>,
) -> Result<bool, String> {
    println!("Running");
    match install_northstar(&game_path, northstar_package_name).await {
        Ok(_) => Ok(true),
        Err(err) => {
            println!("{}", err);
            Err(err.to_string())
        }
    }
}

#[tauri::command]
/// Update Northstar install in the given path
async fn update_northstar_caller(
    game_path: String,
    northstar_package_name: Option<String>,
) -> Result<bool, String> {
    println!("Updating");

    // Simply re-run install with up-to-date version for upate
    match install_northstar(&game_path, northstar_package_name).await {
        Ok(_) => Ok(true),
        Err(err) => {
            println!("{}", err);
            Err(err.to_string())
        }
    }
}

#[tauri::command]
/// Launches Northstar
async fn launch_northstar_caller(
    game_install: GameInstall,
    bypass_checks: Option<bool>,
) -> Result<String, String> {
    launch_northstar(game_install, bypass_checks)
}

#[tauri::command]
/// Get list of Northstar logs
async fn get_log_list_caller(game_install: GameInstall) -> Result<Vec<std::path::PathBuf>, String> {
    get_log_list(game_install)
}

#[tauri::command]
async fn verify_game_files_caller(game_install: GameInstall) -> Result<String, String> {
    verify_game_files(game_install)
}

#[tauri::command]
async fn get_enabled_mods_caller(
    game_install: GameInstall,
) -> Result<serde_json::value::Value, String> {
    get_enabled_mods(game_install)
}

#[tauri::command]
async fn set_mod_enabled_status_caller(
    game_install: GameInstall,
    mod_name: String,
    is_enabled: bool,
) -> Result<(), String> {
    set_mod_enabled_status(game_install, mod_name, is_enabled)
}

#[tauri::command]
async fn disable_all_but_core_caller(game_install: GameInstall) -> Result<(), String> {
    disable_all_but_core(game_install)
}

#[tauri::command]
async fn get_installed_mods_caller(game_install: GameInstall) -> Result<Vec<NorthstarMod>, String> {
    get_installed_mods_and_properties(game_install)
}

#[tauri::command]
/// Installs the specified mod
async fn install_mod_caller(
    game_install: GameInstall,
    thunderstore_mod_string: String,
) -> Result<(), String> {
    fc_download_mod_and_install(game_install.clone(), thunderstore_mod_string).await?;
    match clean_up_download_folder(game_install, false) {
        Ok(()) => Ok(()),
        Err(err) => {
            println!("Failed to delete download folder due to {}", err);
            // Failure to delete download folder is not an error in mod install
            // As such ignore. User can still force delete if need be
            Ok(())
        }
    }
}

#[tauri::command]
/// Installs the specified mod
async fn clean_up_download_folder_caller(
    game_install: GameInstall,
    force: bool,
) -> Result<(), String> {
    match clean_up_download_folder(game_install, force) {
        Ok(()) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
/// Deletes the specified mod
async fn delete_northstar_mod_caller(game_install: GameInstall, nsmod_name: String) -> Result<(), String> {
    delete_northstar_mod(game_install, nsmod_name)
}
