#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{
    env,
    sync::{Arc, Mutex},
    time::Duration,
};

#[cfg(target_os = "windows")]
use std::ptr::null_mut;
#[cfg(target_os = "windows")]
use winapi::um::winuser::{MessageBoxW, MB_ICONERROR, MB_OK, MB_USERICON};

use crate::constants::{APP_USER_AGENT, MASTER_SERVER_URL, REFRESH_DELAY, SERVER_BROWSER_ENDPOINT};

mod development;

mod github;
use github::release_notes::check_is_flightcore_outdated;

mod repair_and_verify;
use repair_and_verify::clean_up_download_folder;

mod mod_management;
use mod_management::fc_download_mod_and_install;

mod northstar;
use northstar::get_northstar_version_number;

mod thunderstore;
use thunderstore::query_thunderstore_packages_api;

use semver::Version;
use serde::{Deserialize, Serialize};
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

            // Emit updated player and server count to GUI
            let app_handle = app.app_handle();
            tauri::async_runtime::spawn(async move {
                loop {
                    sleep(REFRESH_DELAY).await;
                    app_handle
                        .emit_all("northstar-statistics", get_server_player_count().await)
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
            launch_northstar_steam_caller,
            check_is_flightcore_outdated_caller,
            repair_and_verify::get_log_list,
            repair_and_verify::verify_game_files,
            mod_management::set_mod_enabled_status,
            repair_and_verify::disable_all_but_core,
            is_debug_mode,
            github::release_notes::get_northstar_release_notes,
            linux_checks,
            mod_management::get_installed_mods_and_properties,
            install_mod_caller,
            clean_up_download_folder_caller,
            github::release_notes::get_newest_flightcore_version,
            mod_management::delete_northstar_mod,
            get_server_player_count,
            mod_management::delete_thunderstore_mod,
            open_repair_window,
            query_thunderstore_packages_api,
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
                // Display a message box to the user with a button to open the installation instructions
                let title = "WebView2 not found"
                    .encode_utf16()
                    .chain(Some(0))
                    .collect::<Vec<_>>();
                let message = "FlightCore requires WebView2 to run.\n\nClick OK to open installation instructions.".encode_utf16().chain(Some(0)).collect::<Vec<_>>();
                unsafe {
                    let result = MessageBoxW(
                        null_mut(),
                        message.as_ptr(),
                        title.as_ptr(),
                        MB_OK | MB_ICONERROR | MB_USERICON,
                    );
                    if result == 1 {
                        // Open the installation instructions URL in the user's default web browser
                        open::that("https://github.com/R2NorthstarTools/FlightCore/blob/main/docs/TROUBLESHOOTING.md#flightcore-wont-launch").unwrap();
                    }
                }
            }
        }
    };
}

/// Wrapper for `find_game_install_location` as tauri doesn't allow passing `Result<>` types to front-end
#[tauri::command]
async fn find_game_install_location_caller() -> Result<GameInstall, String> {
    find_game_install_location()
}

/// This function's only use is to force a `panic!()`
// This must NOT be async to ensure crashing whole application.
#[tauri::command]
fn force_panic() {
    panic!("Force panicked!");
}

/// Returns true if built in debug mode
#[tauri::command]
async fn is_debug_mode() -> bool {
    cfg!(debug_assertions)
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

#[tauri::command]
async fn get_northstar_version_number_caller(game_path: String) -> Result<String, String> {
    match get_northstar_version_number(&game_path) {
        Ok(version_number) => Ok(version_number),
        Err(err) => Err(err.to_string()),
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

    let index = thermite::api::get_package_index().unwrap().to_vec();
    let nmod = index
        .iter()
        .find(|f| f.name.to_lowercase() == northstar_package_name.to_lowercase())
        .expect("Couldn't find Northstar on thunderstore???");
    // .ok_or_else(|| anyhow!("Couldn't find Northstar on thunderstore???"))?;

    let version_number = match get_northstar_version_number(&game_path) {
        Ok(version_number) => version_number,
        Err(err) => {
            log::warn!("{}", err);
            // If we fail to get new version just assume we are up-to-date
            return Err(err.to_string());
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

/// Checks if installed FlightCore version is up-to-date
/// false -> FlightCore install is up-to-date
/// true  -> FlightCore install is outdated
#[tauri::command]
async fn check_is_flightcore_outdated_caller() -> Result<bool, String> {
    check_is_flightcore_outdated().await
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

/// Returns identifier of host OS FlightCore is running on
#[tauri::command]
async fn get_host_os_caller() -> String {
    get_host_os()
}

/// Installs Northstar to the given path
#[tauri::command]
async fn install_northstar_caller(
    window: tauri::Window,
    game_path: String,
    northstar_package_name: Option<String>,
    version_number: Option<String>,
) -> Result<bool, String> {
    log::info!("Running");

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

    match install_northstar(window, &game_path, northstar_package_name, version_number).await {
        Ok(_) => Ok(true),
        Err(err) => {
            log::error!("{}", err);
            Err(err)
        }
    }
}

/// Update Northstar install in the given path
#[tauri::command]
async fn update_northstar_caller(
    window: tauri::Window,
    game_path: String,
    northstar_package_name: Option<String>,
) -> Result<bool, String> {
    log::info!("Updating Northstar");

    // Simply re-run install with up-to-date version for upate
    install_northstar_caller(window, game_path, northstar_package_name, None).await
}

/// Launches Northstar
#[tauri::command]
async fn launch_northstar_caller(
    game_install: GameInstall,
    bypass_checks: Option<bool>,
) -> Result<String, String> {
    northstar::launch_northstar(&game_install, bypass_checks)
}

/// Launches Northstar
#[tauri::command]
async fn launch_northstar_steam_caller(
    game_install: GameInstall,
    bypass_checks: Option<bool>,
) -> Result<String, String> {
    launch_northstar_steam(&game_install, bypass_checks)
}

/// Installs the specified mod
#[tauri::command]
async fn install_mod_caller(
    game_install: GameInstall,
    thunderstore_mod_string: String,
) -> Result<(), String> {
    fc_download_mod_and_install(&game_install, &thunderstore_mod_string).await?;
    match clean_up_download_folder(&game_install, false) {
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
    match clean_up_download_folder(&game_install, force) {
        Ok(()) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}

/// Fetches `/client/servers` endpoint from master server
async fn fetch_server_list() -> Result<String, anyhow::Error> {
    let url = format!("{MASTER_SERVER_URL}{SERVER_BROWSER_ENDPOINT}");
    let client = reqwest::Client::new();
    let res = client
        .get(url)
        .header(reqwest::header::USER_AGENT, APP_USER_AGENT)
        .send()
        .await?
        .text()
        .await?;

    Ok(res)
}

/// Gets server and playercount from master server API
#[tauri::command]
async fn get_server_player_count() -> Result<(i32, usize), String> {
    let res = match fetch_server_list().await {
        Ok(res) => res,
        Err(err) => return Err(err.to_string()),
    };

    let ns_servers: Vec<NorthstarServer> =
        serde_json::from_str(&res).expect("JSON was not well-formatted");

    // Get server count
    let server_count = ns_servers.len();

    // Sum up player count
    let total_player_count: i32 = ns_servers.iter().map(|server| server.player_count).sum();

    log::info!("total_player_count: {}", total_player_count);
    log::info!("server_count:       {}", server_count);

    Ok((total_player_count, server_count))
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

// The remaining below was originally in `lib.rs`.
// As this was causing issues it was moved into `main.rs` until being later moved into dedicated modules
use std::{cell::RefCell, fs, path::Path, time::Instant};

use anyhow::{Context, Result};

pub mod constants;
mod platform_specific;
#[cfg(target_os = "windows")]
use platform_specific::windows;

#[cfg(target_os = "linux")]
use platform_specific::linux;

use sysinfo::SystemExt;
use zip::ZipArchive;

use crate::constants::TITANFALL2_STEAM_ID;

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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NorthstarServer {
    #[serde(rename = "playerCount")]
    pub player_count: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone, TS)]
#[ts(export)]
pub enum InstallState {
    DOWNLOADING,
    EXTRACTING,
    DONE,
}

#[derive(Serialize, Deserialize, Debug, Clone, TS)]
#[ts(export)]
struct InstallProgress {
    current_downloaded: u64,
    total_size: u64,
    state: InstallState,
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

/// Attempts to find the game install location
pub fn find_game_install_location() -> Result<GameInstall, String> {
    // Attempt parsing Steam library directly
    match steamlocate::SteamDir::locate() {
        Some(mut steamdir) => {
            let titanfall2_steamid = TITANFALL2_STEAM_ID.parse().unwrap();
            match steamdir.app(&titanfall2_steamid) {
                Some(app) => {
                    // println!("{:#?}", app);
                    let game_install = GameInstall {
                        game_path: app.path.to_str().unwrap().to_string(),
                        install_type: InstallType::STEAM,
                    };
                    return Ok(game_install);
                }
                None => log::info!("Couldn't locate Titanfall2 Steam install"),
            }
        }
        None => log::info!("Couldn't locate Steam on this computer!"),
    }

    // (On Windows only) try parsing Windows registry for Origin install path
    #[cfg(target_os = "windows")]
    match windows::origin_install_location_detection() {
        Ok(game_path) => {
            let game_install = GameInstall {
                game_path,
                install_type: InstallType::ORIGIN,
            };
            return Ok(game_install);
        }
        Err(err) => {
            log::info!("{}", err);
        }
    };

    Err("Could not auto-detect game install location! Please enter it manually.".to_string())
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

/// Copied from `papa` source code and modified
///Extract N* zip file to target game path
// fn extract(ctx: &Ctx, zip_file: File, target: &Path) -> Result<()> {
fn extract(zip_file: std::fs::File, target: &std::path::Path) -> Result<()> {
    let mut archive = ZipArchive::new(&zip_file).context("Unable to open zip archive")?;
    for i in 0..archive.len() {
        let mut f = archive.by_index(i).unwrap();

        //This should work fine for N* because the dir structure *should* always be the same
        if f.enclosed_name().unwrap().starts_with("Northstar") {
            let out = target.join(
                f.enclosed_name()
                    .unwrap()
                    .strip_prefix("Northstar")
                    .unwrap(),
            );

            if (*f.name()).ends_with('/') {
                log::info!("Create directory {}", f.name());
                std::fs::create_dir_all(target.join(f.name()))
                    .context("Unable to create directory")?;
                continue;
            } else if let Some(p) = out.parent() {
                std::fs::create_dir_all(p).context("Unable to create directory")?;
            }

            let mut outfile = std::fs::OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(&out)?;

            log::info!("Write file {}", out.display());

            std::io::copy(&mut f, &mut outfile).context("Unable to write to file")?;
        }
    }

    Ok(())
}

/// Copied from `papa` source code and modified
///Install N* from the provided mod
///
///Checks cache, else downloads the latest version
async fn do_install(
    window: tauri::Window,
    nmod: &thermite::model::ModVersion,
    game_path: &std::path::Path,
) -> Result<()> {
    let filename = format!("northstar-{}.zip", nmod.version);
    let download_directory = format!("{}/___flightcore-temp-download-dir/", game_path.display());

    std::fs::create_dir_all(download_directory.clone())?;

    let download_path = format!("{}/{}", download_directory, filename);
    log::info!("Download path: {download_path}");

    let last_emit = RefCell::new(Instant::now()); // Keep track of the last time a signal was emitted
    let nfile = thermite::core::manage::download_file_with_progress(
        &nmod.url,
        download_path,
        |delta, current, total| {
            if delta != 0 {
                // Only emit a signal once every 100ms
                // This way we don't bombard the frontend with events on fast download speeds
                let time_since_last_emit = Instant::now().duration_since(*last_emit.borrow());
                if time_since_last_emit >= Duration::from_millis(100) {
                    window
                        .emit(
                            "northstar-install-download-progress",
                            InstallProgress {
                                current_downloaded: current,
                                total_size: total,
                                state: InstallState::DOWNLOADING,
                            },
                        )
                        .unwrap();
                    *last_emit.borrow_mut() = Instant::now();
                }
            }
        },
    )
    .unwrap();

    window
        .emit(
            "northstar-install-download-progress",
            InstallProgress {
                current_downloaded: 0,
                total_size: 0,
                state: InstallState::EXTRACTING,
            },
        )
        .unwrap();

    log::info!("Extracting Northstar...");
    extract(nfile, game_path)?;

    // Delete old copy
    log::info!("Delete temp folder again");
    std::fs::remove_dir_all(download_directory).unwrap();

    log::info!("Done installing Northstar!");
    window
        .emit(
            "northstar-install-download-progress",
            InstallProgress {
                current_downloaded: 0,
                total_size: 0,
                state: InstallState::DONE,
            },
        )
        .unwrap();

    Ok(())
}

pub async fn install_northstar(
    window: tauri::Window,
    game_path: &str,
    northstar_package_name: String,
    version_number: Option<String>,
) -> Result<String, String> {
    let index = thermite::api::get_package_index().unwrap().to_vec();
    let nmod = index
        .iter()
        .find(|f| f.name.to_lowercase() == northstar_package_name.to_lowercase())
        .ok_or_else(|| panic!("Couldn't find Northstar on thunderstore???"))
        .unwrap();

    // Use passed version or latest if no version was passed
    let version = version_number.as_ref().unwrap_or(&nmod.latest);

    log::info!("Install path \"{}\"", game_path);

    match do_install(
        window,
        nmod.versions.get(version).unwrap(),
        std::path::Path::new(game_path),
    )
    .await
    {
        Ok(_) => (),
        Err(err) => {
            if game_path
                .to_lowercase()
                .contains(&r#"C:\Program Files\"#.to_lowercase())
            // default is `C:\Program Files\EA Games\Titanfall2`
            {
                return Err(
                    "Cannot install to default EA App install path, please move Titanfall2 to a different install location.".to_string(),
                );
            } else {
                return Err(err.to_string());
            }
        }
    }

    Ok(nmod.latest.clone())
}

/// Returns identifier of host OS FlightCore is running on
pub fn get_host_os() -> String {
    env::consts::OS.to_string()
}

/// Prepare Northstar and Launch through Steam using the Browser Protocol
pub fn launch_northstar_steam(
    game_install: &GameInstall,
    _bypass_checks: Option<bool>,
) -> Result<String, String> {
    if !matches!(game_install.install_type, InstallType::STEAM) {
        return Err("Titanfall2 was not installed via Steam".to_string());
    }

    match steamlocate::SteamDir::locate() {
        Some(mut steamdir) => {
            if get_host_os() != "windows" {
                let titanfall2_steamid: u32 = TITANFALL2_STEAM_ID.parse().unwrap();
                match steamdir.compat_tool(&titanfall2_steamid) {
                    Some(compat) => {
                        if !compat
                            .name
                            .clone()
                            .unwrap()
                            .to_ascii_lowercase()
                            .contains("northstarproton")
                        {
                            return Err(
                                "Titanfall2 was not configured to use NorthstarProton".to_string()
                            );
                        }
                    }
                    None => {
                        return Err(
                            "Titanfall2 was not configured to use a compatibility tool".to_string()
                        );
                    }
                }
            }
        }
        None => {
            return Err("Couldn't access Titanfall2 directory".to_string());
        }
    }

    // Switch to Titanfall2 directory to set everything up
    if std::env::set_current_dir(game_install.game_path.clone()).is_err() {
        // We failed to get to Titanfall2 directory
        return Err("Couldn't access Titanfall2 directory".to_string());
    }

    let run_northstar = "run_northstar.txt";
    let run_northstar_bak = "run_northstar.txt.bak";

    if Path::new(run_northstar).exists() {
        // rename should ovewrite existing files
        fs::rename(run_northstar, run_northstar_bak).unwrap();
    }

    // Passing arguments gives users a prompt, so we use run_northstar.txt
    fs::write(run_northstar, b"1").unwrap();

    let retval = match open::that(format!("steam://run/{}/", TITANFALL2_STEAM_ID)) {
        Ok(()) => Ok("Started game".to_string()),
        Err(_err) => Err("Failed to launch Titanfall 2 via Steam".to_string()),
    };

    let is_err = retval.is_err();

    // Handle the rest in the backround
    tauri::async_runtime::spawn(async move {
        // Starting the EA app and Titanfall might take a good minute or three
        let mut wait_countdown = 60 * 3;
        while wait_countdown > 0 && !check_northstar_running() && !is_err {
            sleep(Duration::from_millis(1000)).await;
            wait_countdown -= 1;
        }

        // Northstar may be running, but it may not have loaded the file yet
        sleep(Duration::from_millis(2000)).await;

        // intentionally ignore Result
        let _ = fs::remove_file(run_northstar);

        if Path::new(run_northstar_bak).exists() {
            fs::rename(run_northstar_bak, run_northstar).unwrap();
        }
    });

    retval
}

pub fn check_origin_running() -> bool {
    let s = sysinfo::System::new_all();
    let x = s.processes_by_name("Origin.exe").next().is_some()
        || s.processes_by_name("EADesktop.exe").next().is_some();
    x
}

/// Checks if Northstar process is running
pub fn check_northstar_running() -> bool {
    let s = sysinfo::System::new_all();
    let x = s
        .processes_by_name("NorthstarLauncher.exe")
        .next()
        .is_some()
        || s.processes_by_name("Titanfall2.exe").next().is_some();
    x
}
