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

            let app_handle = app.app_handle();
            tauri::async_runtime::spawn(async move {
                // Checking install location inside a timed loop is a bad idea
                // If the user has the game on a harddrive for example, it will prevent the harddrive from ever spinning down
                // Instead, install location checks should be event based.
                loop {
                    sleep(Duration::from_millis(5000)).await;
                    println!("sending install location");
                    app_handle
                        .emit_all("install-location-result", find_game_install_location())
                        .unwrap();
                }
            });

            Ok(())
        })
        .manage(Counter(Default::default()))
        .invoke_handler(tauri::generate_handler![
            hello_world,
            add_count,
            force_panic,
            get_version_number
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn find_game_install_location() -> String {
    // Attempt parsing Steam library directly
    match steamlocate::SteamDir::locate() {
        Some(mut steamdir) => {
            let titanfall2_steamid = 1237970;
            match steamdir.app(&titanfall2_steamid) {
                Some(app) => {
                    // println!("{:#?}", app);
                    return app.path.to_str().unwrap().to_string();
                }
                None => println!("Couldn't locate Titanfall2"),
            }
        }
        None => println!("Couldn't locate Steam on this computer!"),
    }
    "NOT FOUND".to_string()
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
