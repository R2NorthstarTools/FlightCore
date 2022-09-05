#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use app::{check_is_valid_game_path, find_game_install_location, get_northstar_version_number};
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
            get_northstar_version_number_caller,
            check_is_northstar_outdated,
            verify_install_location
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
/// Wrapper for `find_game_install_location` as tauri doesn't allow passing `Result<>` types to front-end
fn find_game_install_location_caller() -> String {
    match find_game_install_location() {
        Ok((path, install_type)) => path,
        Err(err) => {
            println!("{}", err);
            "".to_string()
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
fn get_northstar_version_number_caller() -> String {
    match get_northstar_version_number() {
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
async fn check_is_northstar_outdated() -> bool {
    let index = thermite::api::get_package_index().await.unwrap().to_vec();
    let nmod = index
        .iter()
        .find(|f| f.name.to_lowercase() == "northstar")
        .unwrap();
        // .ok_or_else(|| anyhow!("Couldn't find Northstar on thunderstore???"))?;

    dbg!(nmod);

    let version_number = match get_northstar_version_number() {
        Ok(version_number) => version_number,
        Err(err) => {
            println!("{}",err);
            // If we fail to get new version just assume we are up-to-date
            return false;
        }
    };

    if version_number != nmod.version {
        println!("Installed Northstar version outdated");
        true
    }
    else {
        println!("Installed Northstar version up-to-date");
        false
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
