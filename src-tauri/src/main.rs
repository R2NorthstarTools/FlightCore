#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use tauri::{Manager, State};
use tokio::time::sleep;
use anyhow::anyhow;

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
                    println!("sending backend ping");
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
            get_northstar_version_number_caller
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
/// Wrapper for `find_game_install_location` as tauri doesn't allow passing `Result<>` types to front-end
fn find_game_install_location_caller() -> String {
    match find_game_install_location() {
        Ok(path) => path,
        Err(err) => {
            println!("{}", err);
            "".to_string()
        }
    }
}

/// Attempts to find the game install location
fn find_game_install_location() -> Result<String, anyhow::Error> {
    // Attempt parsing Steam library directly
    match steamlocate::SteamDir::locate() {
        Some(mut steamdir) => {
            let titanfall2_steamid = 1237970;
            match steamdir.app(&titanfall2_steamid) {
                Some(app) => {
                    // println!("{:#?}", app);
                    return Ok(app.path.to_str().unwrap().to_string());
                }
                None => println!("Couldn't locate Titanfall2"),
            }
        }
        None => println!("Couldn't locate Steam on this computer!"),
    }
    Err(anyhow!(
        "Could not auto-detect game install location! Please enter it manually."
    ))
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
fn get_northstar_version_number_caller() -> String {
    match get_northstar_version_number() {
        Ok(version_number) => version_number,
        Err(err) => {
            println!("{}", err);
            "".to_string()
        }
    }
}

/// Check version number of a mod
fn check_mod_version_number(path_to_mod_folder: String) -> Result<String, anyhow::Error> {
    // println!("{}", format!("{}/mod.json", path_to_mod_folder));
    let data = std::fs::read_to_string(format!("{}/mod.json", path_to_mod_folder))?;
    let parsed_json: serde_json::Value = serde_json::from_str(&data)?;
    // println!("{}", parsed_json);
    let mod_version_number = match parsed_json.get("Version").and_then(|value| value.as_str()) {
        Some(version_number) => version_number,
        None => return Err(anyhow!("No version number found")),
    };

    println!("{}", mod_version_number);

    Ok(mod_version_number.to_string())
}

/// Returns the current Northstar version number as a string
fn get_northstar_version_number() -> Result<String, anyhow::Error> {
    let install_location = match find_game_install_location() {
        Ok(path) => path,
        Err(err) => return Err(err),
    };

    println!("{}", install_location);

    // TODO:
    // Check if NorthstarLauncher.exe exists and check its version number
    let profile_folder = "R2Northstar";
    let core_mods = [
        "Northstar.Client",
        "Northstar.Custom",
        "Northstar.CustomServers",
    ];
    let initial_version_number = match check_mod_version_number(format!(
        "{}/{}/mods/{}",
        install_location, profile_folder, core_mods[0]
    )) {
        Ok(version_number) => version_number,
        Err(err) => return Err(err),
    };

    for core_mod in core_mods {
        let current_version_number = match check_mod_version_number(format!(
            "{}/{}/mods/{}",
            install_location, profile_folder, core_mod
        )) {
            Ok(version_number) => version_number,
            Err(err) => return Err(err),
        };
        if current_version_number != initial_version_number {
            // We have a version number mismatch
            return Err(anyhow!("Found version number mismatch"));
        }
    }
    println!("All mods same version");

    Ok(initial_version_number)
}
