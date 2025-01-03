use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use std::{cell::RefCell, time::Instant};
use ts_rs::TS;

use crate::constants::{CORE_MODS, NORTHSTAR_DEFAULT_PROFILE, NORTHSTAR_DLL};
use crate::{
    util::{extract, move_dir_all},
    GameInstall, InstallType,
};

#[cfg(target_os = "windows")]
use crate::platform_specific::windows;

#[derive(Serialize, Deserialize, Debug, Clone, TS)]
#[ts(export)]
enum InstallState {
    Downloading,
    Extracting,
    Done,
}

#[derive(Serialize, Deserialize, Debug, Clone, TS)]
#[ts(export)]
struct InstallProgress {
    current_downloaded: u64,
    total_size: u64,
    state: InstallState,
}

/// Installs Northstar to the given path
#[tauri::command]
pub async fn install_northstar_wrapper(
    window: tauri::Window,
    game_install: GameInstall,
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

    match install_northstar(window, game_install, northstar_package_name, version_number).await {
        Ok(_) => Ok(true),
        Err(err) => {
            log::error!("{}", err);
            Err(err)
        }
    }
}

/// Update Northstar install in the given path
#[tauri::command]
pub async fn update_northstar(
    window: tauri::Window,
    game_install: GameInstall,
    northstar_package_name: Option<String>,
) -> Result<bool, String> {
    log::info!("Updating Northstar");

    // Simply re-run install with up-to-date version for upate
    install_northstar_wrapper(window, game_install, northstar_package_name, None).await
}

/// Copied from `papa` source code and modified
///Install N* from the provided mod
///
///Checks cache, else downloads the latest version
async fn do_install(
    window: tauri::Window,
    nmod: &thermite::model::ModVersion,
    game_install: GameInstall,
) -> Result<()> {
    let filename = format!("northstar-{}.zip", nmod.version);
    let temp_dir = format!("{}/___flightcore-temp", game_install.game_path);
    let download_directory = format!("{}/download-dir", temp_dir);
    let extract_directory = format!("{}/extract-dir", temp_dir);

    log::info!("Attempting to create temporary directory {}", temp_dir);
    std::fs::create_dir_all(download_directory.clone())?;
    std::fs::create_dir_all(extract_directory.clone())?;

    let download_path = format!("{}/{}", download_directory, filename);
    log::info!("Download path: {download_path}");

    let last_emit = RefCell::new(Instant::now()); // Keep track of the last time a signal was emitted
    let mut nfile = std::fs::File::options()
        .read(true)
        .write(true)
        .truncate(true)
        .create(true)
        .open(download_path)?;
    todo!()
}

pub async fn install_northstar(
    window: tauri::Window,
    game_install: GameInstall,
    northstar_package_name: String,
    version_number: Option<String>,
) -> Result<String, String> {
    let index = match thermite::api::get_package_index() {
        Ok(res) => res.to_vec(),
        Err(err) => {
            log::warn!("Failed fetching package index due to: {err}");
            return Err("Failed to connect to Thunderstore.".to_string());
        }
    };
    let nmod = index
        .iter()
        .find(|f| f.name.to_lowercase() == northstar_package_name.to_lowercase())
        .ok_or_else(|| panic!("Couldn't find Northstar on thunderstore???"))
        .unwrap();

    // Use passed version or latest if no version was passed
    let version = version_number.as_ref().unwrap_or(&nmod.latest);

    let game_path = game_install.game_path.clone();
    log::info!("Install path \"{}\"", game_path);

    match do_install(window, nmod.versions.get(version).unwrap(), game_install).await {
        Ok(_) => (),
        Err(err) => {
            if game_path
                .to_lowercase()
                .contains(&r"C:\Program Files\".to_lowercase())
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

/// Attempts to find the game install location
#[tauri::command]
pub fn find_game_install_location() -> Result<GameInstall, String> {
    // Attempt parsing Steam library directly
    match steamlocate::SteamDir::locate() {
        Ok(steamdir) => {
            #[cfg(target_os = "linux")]
            {
                let snap_dir = match std::env::var("SNAP_USER_DATA") {
                    Ok(snap_dir) => std::path::PathBuf::from(snap_dir),
                    Err(_) => match dirs::home_dir() {
                        Some(path) => path,
                        None => std::path::PathBuf::new(),
                    }
                    .join("snap"),
                };

                if steamdir.path().starts_with(snap_dir) {
                    log::warn!("Found Steam installed via Snap, you may encounter issues");
                }
            }

            match steamdir.find_app(thermite::TITANFALL2_STEAM_ID) {
                Ok(Some((app, library))) => {
                    let app_path = library
                        .path()
                        .join("steamapps")
                        .join("common")
                        .join(app.install_dir)
                        .into_os_string()
                        .into_string()
                        .unwrap();

                    let game_install = GameInstall {
                        game_path: app_path,
                        profile: "R2Northstar".to_string(),
                        install_type: InstallType::STEAM,
                    };
                    return Ok(game_install);
                }
                Ok(None) => log::info!("Couldn't locate your Titanfall 2 Steam install."),
                Err(err) => log::info!(
                    "Something went wrong while trying to find Titanfall 2 {}",
                    err
                ),
            }
        }
        Err(err) => log::info!("Couldn't locate Steam on this computer! {}", err),
    }

    // (On Windows only) try parsing Windows registry for Origin install path
    #[cfg(target_os = "windows")]
    match windows::origin_install_location_detection() {
        Ok(game_path) => {
            let game_install = GameInstall {
                game_path,
                profile: "R2Northstar".to_string(),
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
