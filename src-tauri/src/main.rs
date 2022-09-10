#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{
    env,
    sync::{Arc, Mutex},
    time::Duration,
};

use anyhow::anyhow;
use app::{
    check_is_valid_game_path, find_game_install_location, get_northstar_version_number,
    install_northstar, GameInstall, InstallType,
};
use tauri::{Manager, State};
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
        .setup(|app| {
            let app_handle = app.app_handle();
            tauri::async_runtime::spawn(async move {
                loop {
                    sleep(Duration::from_millis(2000)).await;
                    // println!("sending backend ping");
                    app_handle.emit_all("backend-ping", "ping").unwrap();
                }
            });

            Ok(())
        })
        .manage(Counter(Default::default()))
        .invoke_handler(tauri::generate_handler![
            hello_world,
            add_count,
            force_panic,
            find_game_install_location_caller,
            get_version_number,
            get_northstar_version_number_caller,
            check_is_northstar_outdated,
            verify_install_location,
            get_host_os,
            install_northstar_caller,
            update_northstar_caller,
            launch_northstar
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
fn hello_world() -> String {
    "Hello World!!!".to_string()
}

#[tauri::command]
fn add_count(num: i32, counter: State<'_, Counter>) -> String {
    let mut val = counter.0.lock().unwrap();
    *val += num;

    format!("{val}")
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
async fn check_is_northstar_outdated(game_path: String) -> Result<bool, String> {
    let index = thermite::api::get_package_index().await.unwrap().to_vec();
    let nmod = index
        .iter()
        .find(|f| f.name.to_lowercase() == "northstar")
        .unwrap();
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

    if version_number != nmod.version {
        println!("Installed Northstar version outdated");
        Ok(true)
    } else {
        println!("Installed Northstar version up-to-date");
        Ok(false)
    }
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
fn get_host_os() -> String {
    env::consts::OS.to_string()
}

#[tauri::command]
/// Installs Northstar to the given path
async fn install_northstar_caller(game_path: String) -> Result<bool, String> {
    println!("Running");
    match install_northstar(&game_path).await {
        Ok(_) => Ok(true),
        Err(err) => {
            println!("{}", err);
            Err(err.to_string())
        }
    }
}

#[tauri::command]
/// Update Northstar install in the given path
async fn update_northstar_caller(game_path: String) -> Result<bool, String> {
    println!("Updating");

    // Simply re-run install with up-to-date version for upate
    match install_northstar(&game_path).await {
        Ok(_) => Ok(true),
        Err(err) => {
            println!("{}", err);
            Err(err.to_string())
        }
    }
}

#[tauri::command]
/// Launches Northstar
fn launch_northstar(game_install: GameInstall) -> Result<String, String> {
    dbg!(game_install.clone());

    // Some safety checks before, should have more in the future
    if get_northstar_version_number(game_install.game_path.clone()).is_err() {
        return Err(anyhow!("Not all checks were met").to_string());
    }

    let host_os = get_host_os();

    // Switch to Titanfall2 directory for launching
    // NorthstarLauncher.exe expects to be run from that folder
    if std::env::set_current_dir(game_install.game_path.clone()).is_err() {
        // We failed to get to Titanfall2 directory
        return Err(anyhow!("Couldn't access Titanfall2 directory").to_string());
    }

    // Only Windows with Steam or Origin are supported at the moment
    if host_os == "windows"
        && (matches!(game_install.install_type, InstallType::STEAM)
            || matches!(game_install.install_type, InstallType::ORIGIN))
    {
        let _output =
            std::process::Command::new(format!("{}/NorthstarLauncher.exe", game_install.game_path))
                // .args(&["a", "b"])
                .spawn()
                .expect("failed to execute process");
        return Ok("Launched game".to_string());
    }

    Err(format!("Not yet implemented for {:?} on {}", game_install.install_type, get_host_os()))
}
