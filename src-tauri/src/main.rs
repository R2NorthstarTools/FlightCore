#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{
    env,
    sync::{Arc, Mutex},
    time::Duration,
};

use crate::constants::REFRESH_DELAY;

mod development;
mod github;
mod mod_management;
mod northstar;
mod repair_and_verify;
mod thunderstore;
mod util;

use semver::Version;
use serde::{Deserialize, Serialize};
#[cfg(target_os = "windows")]
use tauri::api::dialog::blocking::MessageDialogBuilder;
#[cfg(target_os = "windows")]
use tauri::api::dialog::{MessageDialogButtons, MessageDialogKind};
use tauri::{Manager, Runtime};
use tokio::time::sleep;
use ts_rs::TS;

#[derive(Serialize, Deserialize, Debug, Clone, TS)]
#[ts(export)]
struct NorthstarThunderstoreRelease {
    package: String,
    version: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, TS)]
#[ts(export)]
struct NorthstarThunderstoreReleaseWrapper {
    label: String,
    value: NorthstarThunderstoreRelease,
}

#[derive(Default)]
struct Counter(Arc<Mutex<i32>>);

fn main() {
    // Setup logger
    let mut log_builder = pretty_env_logger::formatted_builder();
    log_builder.parse_filters("info");
    let logger = sentry_log::SentryLogger::with_dest(log_builder.build());

    log::set_boxed_logger(Box::new(logger)).unwrap();
    log::set_max_level(log::LevelFilter::Info);

    // Only enable Sentry crash logs on release
    #[cfg(not(debug_assertions))]
    let _guard = sentry::init((
        "https://f833732deb2240b0b2dc4abce97d0f1d@o1374052.ingest.sentry.io/6692177",
        sentry::ClientOptions {
            release: sentry::release_name!(),
            attach_stacktrace: true,
            ..Default::default()
        },
    ));

    match tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
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
                        .emit_all(
                            "ea-app-running-ping",
                            util::check_ea_app_or_origin_running(),
                        )
                        .unwrap();
                }
            });
            let app_handle = app.app_handle();
            tauri::async_runtime::spawn(async move {
                loop {
                    sleep(Duration::from_millis(2000)).await;
                    app_handle
                        .emit_all("northstar-running-ping", util::check_northstar_running())
                        .unwrap();
                }
            });

            // Emit updated player and server count to GUI
            let app_handle = app.app_handle();
            tauri::async_runtime::spawn(async move {
                loop {
                    sleep(REFRESH_DELAY).await;
                    app_handle
                        .emit_all(
                            "northstar-statistics",
                            util::get_server_player_count().await,
                        )
                        .unwrap();
                }
            });

            Ok(())
        })
        .manage(Counter(Default::default()))
        .invoke_handler(tauri::generate_handler![
            util::force_panic,
            northstar::install::find_game_install_location,
            get_flightcore_version_number,
            northstar::get_northstar_version_number,
            check_is_northstar_outdated,
            verify_install_location,
            get_host_os,
            install_northstar_caller,
            update_northstar,
            northstar::launch_northstar,
            northstar::launch_northstar_steam,
            github::release_notes::check_is_flightcore_outdated,
            repair_and_verify::get_log_list,
            repair_and_verify::verify_game_files,
            mod_management::set_mod_enabled_status,
            repair_and_verify::disable_all_but_core,
            util::is_debug_mode,
            github::release_notes::get_northstar_release_notes,
            linux_checks,
            mod_management::get_installed_mods_and_properties,
            install_mod_caller,
            clean_up_download_folder_caller,
            github::release_notes::get_newest_flightcore_version,
            mod_management::delete_northstar_mod,
            util::get_server_player_count,
            mod_management::delete_thunderstore_mod,
            install_northstar_proton_wrapper,
            uninstall_northstar_proton_wrapper,
            get_local_northstar_proton_wrapper_version,
            open_repair_window,
            thunderstore::query_thunderstore_packages_api,
            github::get_list_of_tags,
            github::compare_tags,
            github::pull_requests::get_pull_requests_wrapper,
            github::pull_requests::apply_launcher_pr,
            github::pull_requests::apply_mods_pr,
            github::pull_requests::get_launcher_download_link,
            close_application,
            development::install_git_main,
            get_available_northstar_versions,
        ])
        .run(tauri::generate_context!())
    {
        Ok(()) => (),
        Err(err) => {
            // Failed to launch system native web view

            // Log error on Linux
            #[cfg(not(target_os = "windows"))]
            {
                log::error!("{err}");
            }

            // On Windows we can show an error window using Windows API to show how to install WebView2
            #[cfg(target_os = "windows")]
            {
                log::error!("WebView2 not installed: {err}");
                let dialog = MessageDialogBuilder::new(
                    "WebView2 not found",
                    "FlightCore requires WebView2 to run.\n\nClick OK to open installation instructions."
                )
                .kind(MessageDialogKind::Error)
                .buttons(MessageDialogButtons::Ok);

                if dialog.show() {
                    // Open the installation instructions URL in the user's default web browser
                    open::that("https://github.com/R2NorthstarTools/FlightCore/blob/main/docs/TROUBLESHOOTING.md#flightcore-wont-launch").unwrap();
                }
            }
        }
    };
}

/// Returns true if linux compatible
#[tauri::command]
async fn linux_checks() -> Result<(), String> {
    // Different behaviour depending on OS
    // MacOS is missing as it is not a target
    // in turn this means this application will not build on MacOS.
    #[cfg(target_os = "windows")]
    {
        Err("Not available on Windows".to_string())
    }

    #[cfg(target_os = "linux")]
    {
        linux_checks_librs()
    }
}

/// Returns the current version number as a string
#[tauri::command]
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

/// Helps with converting release candidate numbers which are different on Thunderstore
/// due to restrictions imposed by the platform
pub fn convert_release_candidate_number(version_number: String) -> String {
    // This simply converts `-rc` to `0`
    // Works as intended for RCs < 10, e.g.  `v1.9.2-rc1`  -> `v1.9.201`
    // Doesn't work for larger numbers, e.g. `v1.9.2-rc11` -> `v1.9.2011` (should be `v1.9.211`)
    version_number.replace("-rc", "0").replace("00", "")
}

/// Checks if installed Northstar version is up-to-date
/// false -> Northstar install is up-to-date
/// true  -> Northstar install is outdated
#[tauri::command]
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

    let index = match thermite::api::get_package_index() {
        Ok(res) => res.to_vec(),
        Err(err) => return Err(format!("Couldn't check if Northstar up-to-date: {err}")),
    };
    let nmod = index
        .iter()
        .find(|f| f.name.to_lowercase() == northstar_package_name.to_lowercase())
        .expect("Couldn't find Northstar on thunderstore???");
    // .ok_or_else(|| anyhow!("Couldn't find Northstar on thunderstore???"))?;

    let version_number = match northstar::get_northstar_version_number(&game_path) {
        Ok(version_number) => version_number,
        Err(err) => {
            log::warn!("{}", err);
            // If we fail to get new version just assume we are up-to-date
            return Err(err);
        }
    };

    // Release candidate version numbers are different between `mods.json` and Thunderstore
    let version_number = convert_release_candidate_number(version_number);

    if version_number != nmod.latest {
        log::info!("Installed Northstar version outdated");
        Ok(true)
    } else {
        log::info!("Installed Northstar version up-to-date");
        Ok(false)
    }
}

/// Checks if is valid Titanfall2 install based on certain conditions
#[tauri::command]
async fn verify_install_location(game_path: String) -> bool {
    match check_is_valid_game_path(&game_path) {
        Ok(()) => true,
        Err(err) => {
            log::warn!("{}", err);
            false
        }
    }
}

/// Installs Northstar to the given path
#[tauri::command]
async fn install_northstar_caller(
    window: tauri::Window,
    game_path: String,
    northstar_package_name: Option<String>,
    version_number: Option<String>,
) -> Result<bool, String> {
    log::info!("Running Northstar install");

    // Get Northstar package name (`Northstar` vs `NorthstarReleaseCandidate`)
    let northstar_package_name = northstar_package_name
        .map(|name| {
            if name.len() <= 1 {
                "Northstar".to_string()
            } else {
                name
            }
        })
        .unwrap_or("Northstar".to_string());

    match northstar::install::install_northstar(
        window,
        &game_path,
        northstar_package_name,
        version_number,
    )
    .await
    {
        Ok(_) => Ok(true),
        Err(err) => {
            log::error!("{}", err);
            Err(err)
        }
    }
}

/// Update Northstar install in the given path
#[tauri::command]
async fn update_northstar(
    window: tauri::Window,
    game_path: String,
    northstar_package_name: Option<String>,
) -> Result<bool, String> {
    log::info!("Updating Northstar");

    // Simply re-run install with up-to-date version for upate
    install_northstar_caller(window, game_path, northstar_package_name, None).await
}

/// Installs the specified mod
#[tauri::command]
async fn install_mod_caller(
    game_install: GameInstall,
    thunderstore_mod_string: String,
) -> Result<(), String> {
    match mod_management::fc_download_mod_and_install(&game_install, &thunderstore_mod_string).await
    {
        Ok(()) => (),
        Err(err) => {
            log::warn!("{err}");
            return Err(err);
        }
    };
    match repair_and_verify::clean_up_download_folder(&game_install, false) {
        Ok(()) => Ok(()),
        Err(err) => {
            log::info!("Failed to delete download folder due to {}", err);
            // Failure to delete download folder is not an error in mod install
            // As such ignore. User can still force delete if need be
            Ok(())
        }
    }
}

/// Installs the specified mod
#[tauri::command]
async fn clean_up_download_folder_caller(
    game_install: GameInstall,
    force: bool,
) -> Result<(), String> {
    match repair_and_verify::clean_up_download_folder(&game_install, force) {
        Ok(()) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}

/// Spawns repair window
#[tauri::command]
async fn open_repair_window(handle: tauri::AppHandle) -> Result<(), String> {
    // Spawn new window
    let repair_window = match tauri::WindowBuilder::new(
        &handle,
        "RepairWindow",
        tauri::WindowUrl::App("/#/repair".into()),
    )
    .build()
    {
        Ok(res) => res,
        Err(err) => return Err(err.to_string()),
    };

    // Set window title
    match repair_window.set_title("FlightCore Repair Window") {
        Ok(()) => (),
        Err(err) => return Err(err.to_string()),
    };
    Ok(())
}

/// Closes all windows and exits application
#[tauri::command]
async fn close_application<R: Runtime>(app: tauri::AppHandle<R>) -> Result<(), String> {
    app.exit(0); // Close application
    Ok(())
}

/// Gets list of available Northstar versions from Thunderstore
#[tauri::command]
async fn get_available_northstar_versions() -> Result<Vec<NorthstarThunderstoreReleaseWrapper>, ()>
{
    let northstar_package_name = "Northstar";
    let index = thermite::api::get_package_index().unwrap().to_vec();
    let nsmod = index
        .iter()
        .find(|f| f.name.to_lowercase() == northstar_package_name.to_lowercase())
        .ok_or_else(|| panic!("Couldn't find Northstar on thunderstore???"))
        .unwrap();

    let mut releases: Vec<NorthstarThunderstoreReleaseWrapper> = vec![];
    for (_version_string, nsmod_version_obj) in nsmod.versions.iter() {
        let current_elem = NorthstarThunderstoreRelease {
            package: nsmod_version_obj.name.clone(),
            version: nsmod_version_obj.version.clone(),
        };
        let current_elem_wrapped = NorthstarThunderstoreReleaseWrapper {
            label: format!(
                "{} v{}",
                nsmod_version_obj.name.clone(),
                nsmod_version_obj.version.clone()
            ),
            value: current_elem,
        };

        releases.push(current_elem_wrapped);
    }

    releases.sort_by(|a, b| {
        // Parse version number
        let a_ver = Version::parse(&a.value.version).unwrap();
        let b_ver = Version::parse(&b.value.version).unwrap();
        b_ver.partial_cmp(&a_ver).unwrap() // Sort newest first
    });

    Ok(releases)
}

use anyhow::Result;

pub mod constants;
mod platform_specific;

#[cfg(target_os = "linux")]
use platform_specific::linux;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum InstallType {
    STEAM,
    ORIGIN,
    EAPLAY,
    UNKNOWN,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameInstall {
    pub game_path: String,
    pub launch_parameters: String,
    pub install_type: InstallType,
}

#[derive(Serialize, Deserialize, Debug, Clone, TS)]
#[ts(export)]
pub struct NorthstarMod {
    pub name: String,
    pub version: Option<String>,
    pub thunderstore_mod_string: Option<String>,
    pub enabled: bool,
    pub directory: String,
}

// I intend to add more linux related stuff to check here, so making a func
// for now tho it only checks `ldd --version`
// - salmon
#[cfg(target_os = "linux")]
pub fn linux_checks_librs() -> Result<(), String> {
    // Perform various checks in terms of Linux compatibility
    // Return early with error message if a check fails

    // check `ldd --version` to see if glibc is up to date for northstar proton
    let min_required_ldd_version = 2.33;
    let lddv = linux::check_glibc_v();
    if lddv < min_required_ldd_version {
        return Err(format!(
            "GLIBC is not version {} or greater",
            min_required_ldd_version
        ));
    };

    // All checks passed
    Ok(())
}

/// Checks whether the provided path is a valid Titanfall2 gamepath by checking against a certain set of criteria
pub fn check_is_valid_game_path(game_install_path: &str) -> Result<(), String> {
    let path_to_titanfall2_exe = format!("{game_install_path}/Titanfall2.exe");
    let is_correct_game_path = std::path::Path::new(&path_to_titanfall2_exe).exists();
    log::info!("Titanfall2.exe exists in path? {}", is_correct_game_path);

    // Exit early if wrong game path
    if !is_correct_game_path {
        return Err(format!("Incorrect game path \"{game_install_path}\"")); // Return error cause wrong game path
    }
    Ok(())
}

/// Returns identifier of host OS FlightCore is running on
#[tauri::command]
fn get_host_os() -> String {
    env::consts::OS.to_string()
}

/// On Linux attempts to install NorthstarProton
/// On Windows simply returns an error message
#[tauri::command]
async fn install_northstar_proton_wrapper() -> Result<(), String> {
    #[cfg(target_os = "linux")]
    return linux::install_ns_proton().map_err(|err| err.to_string());

    #[cfg(target_os = "windows")]
    Err("Not supported on Windows".to_string())
}

#[tauri::command]
async fn uninstall_northstar_proton_wrapper() -> Result<(), String> {
    #[cfg(target_os = "linux")]
    return linux::uninstall_ns_proton();

    #[cfg(target_os = "windows")]
    Err("Not supported on Windows".to_string())
}

#[tauri::command]
async fn get_local_northstar_proton_wrapper_version() -> Result<String, String> {
    #[cfg(target_os = "linux")]
    return linux::get_local_ns_proton_version();

    #[cfg(target_os = "windows")]
    Err("Not supported on Windows".to_string())
}
