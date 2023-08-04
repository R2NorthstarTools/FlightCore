use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use std::{cell::RefCell, time::Instant};
use ts_rs::TS;

use crate::constants::{NORTHSTAR_DEFAULT_PROFILE, NORTHSTAR_DLL, TITANFALL2_STEAM_ID};
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
    thermite::core::manage::download_with_progress(
        &mut nfile,
        &nmod.url,
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
                                state: InstallState::Downloading,
                            },
                        )
                        .unwrap();
                    *last_emit.borrow_mut() = Instant::now();
                }
            }
        },
    )?;

    window
        .emit(
            "northstar-install-download-progress",
            InstallProgress {
                current_downloaded: 0,
                total_size: 0,
                state: InstallState::Extracting,
            },
        )
        .unwrap();

    log::info!("Extracting Northstar...");
    extract(nfile, std::path::Path::new(&extract_directory))?;

    // Prepare Northstar for Installation
    log::info!("Preparing Northstar...");
    if game_install.profile != NORTHSTAR_DEFAULT_PROFILE {
        // We are using a non standard Profile, we must:
        // - move the DLL
        // - rename the Profile

        // Move DLL into the default R2Northstar Profile
        let old_dll_path = format!("{}/{}", extract_directory, NORTHSTAR_DLL);
        let new_dll_path = format!(
            "{}/{}/{}",
            extract_directory, NORTHSTAR_DEFAULT_PROFILE, NORTHSTAR_DLL
        );
        std::fs::rename(old_dll_path, new_dll_path)?;

        // rename default R2Northstar Profile to the profile we want to use
        let old_profile_path = format!("{}/{}/", extract_directory, NORTHSTAR_DEFAULT_PROFILE);
        let new_profile_path = format!("{}/{}/", extract_directory, game_install.profile);
        std::fs::rename(old_profile_path, new_profile_path)?;
    }

    log::info!("Installing Northstar...");

    for entry in std::fs::read_dir(extract_directory).unwrap() {
        let entry = entry.unwrap();
        let destination = format!(
            "{}/{}",
            game_install.game_path,
            entry.path().file_name().unwrap().to_str().unwrap()
        );

        log::info!("Installing {}", entry.path().display());
        if !entry.file_type().unwrap().is_dir() {
            std::fs::rename(entry.path(), destination)?;
        } else {
            move_dir_all(entry.path(), destination)?;
        }
    }

    // Delete old copy
    log::info!("Delete temporary directory");
    std::fs::remove_dir_all(temp_dir).unwrap();

    log::info!("Done installing Northstar!");
    window
        .emit(
            "northstar-install-download-progress",
            InstallProgress {
                current_downloaded: 0,
                total_size: 0,
                state: InstallState::Done,
            },
        )
        .unwrap();

    Ok(())
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

/// Attempts to find the game install location
#[tauri::command]
pub fn find_game_install_location() -> Result<GameInstall, String> {
    // Attempt parsing Steam library directly
    match steamlocate::SteamDir::locate() {
        Some(mut steamdir) => {
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

                if steamdir.path.starts_with(snap_dir) {
                    log::warn!("Found Steam installed via Snap, you may encounter issues");
                }
            }

            let titanfall2_steamid = TITANFALL2_STEAM_ID.parse().unwrap();
            match steamdir.app(&titanfall2_steamid) {
                Some(app) => {
                    // println!("{:#?}", app);
                    let game_install = GameInstall {
                        game_path: app.path.to_str().unwrap().to_string(),
                        profile: "R2Northstar".to_string(),
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
