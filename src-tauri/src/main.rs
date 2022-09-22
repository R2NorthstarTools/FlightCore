#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{
    env,
    sync::{Arc, Mutex},
    time::Duration,
};

use app::{
    check_is_flightcore_outdated, check_is_valid_game_path, check_northstar_running,
    check_origin_running, convert_release_candidate_number, find_game_install_location,
    get_host_os, get_log_list, get_northstar_version_number, install_northstar, launch_northstar,
    GameInstall,
};
use tauri::Manager;
use tokio::time::sleep;
use tauri_plugin_store::PluginBuilder;

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
            get_version_number,
            get_northstar_version_number_caller,
            check_is_northstar_outdated,
            verify_install_location,
            get_host_os_caller,
            install_northstar_caller,
            update_northstar_caller,
            launch_northstar_caller,
            check_is_flightcore_outdated_caller,
            get_log_list_caller
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
/// Wrapper for `find_game_install_location` as tauri doesn't allow passing `Result<>` types to front-end
fn find_game_install_location_caller() -> Result<GameInstall, String> {
    match find_game_install_location() {
        Ok(game_install) => Ok(game_install),
        Err(err) => {
            println!("{}", err);
            Err(err.to_string())
        }
    }
}

#[tauri::command]
/// This function's only use is to force a `panic!()`
fn force_panic() {
    panic!("Force panicked!");
}

#[tauri::command]
/// Returns the current version number as a string
fn get_version_number() -> String {
    let version = env!("CARGO_PKG_VERSION");
    format!("v{}", version)
}

#[tauri::command]
fn get_northstar_version_number_caller(game_path: String) -> String {
    match get_northstar_version_number(game_path) {
        Ok(version_number) => version_number,
        Err(err) => {
            println!("{}", err);
            "".to_string()
        }
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

    dbg!(nmod);

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

    if version_number != nmod.version {
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
fn check_is_flightcore_outdated_caller() -> Result<bool, String> {
    check_is_flightcore_outdated()
}

#[tauri::command]
/// Checks if is valid Titanfall2 install based on certain conditions
fn verify_install_location(game_path: String) -> bool {
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
fn get_host_os_caller() -> String {
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
fn launch_northstar_caller(game_install: GameInstall) -> Result<String, String> {
    launch_northstar(game_install)
}

#[tauri::command]
/// Get list of Northstar logs
fn get_log_list_caller(game_install: GameInstall) -> Result<Vec<std::path::PathBuf>, String> {
    get_log_list(game_install)
}
