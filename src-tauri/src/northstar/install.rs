use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::time::Duration;
use std::{cell::RefCell, time::Instant};
use tauri::{AppHandle, Emitter};
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
    app: AppHandle,
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

    match install_northstar(app, game_install, northstar_package_name, version_number).await {
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
    app: AppHandle,
    game_install: GameInstall,
    northstar_package_name: Option<String>,
) -> Result<bool, String> {
    log::info!("Updating Northstar");

    // Simply re-run install with up-to-date version for upate
    install_northstar_wrapper(app, game_install, northstar_package_name, None).await
}

/// Copied from `papa` source code and modified
///Install N* from the provided mod
///
///Checks cache, else downloads the latest version
async fn do_install(
    app: AppHandle,
    nmod: &thermite::model::ModVersion,
    game_install: GameInstall,
) -> Result<()> {
    let filename = format!("northstar-{}.zip", nmod.version);
    let temp_dir = format!("{}/___flightcore-temp", game_install.game_path);
    let download_directory = format!("{temp_dir}/download-dir");
    let extract_directory = format!("{temp_dir}/extract-dir");

    if std::path::Path::new(&temp_dir).exists() {
        log::info!("Removing stale temporary directory {}", temp_dir);
        std::fs::remove_dir_all(&temp_dir).context("Unable to remove temporary directory")?;
    }

    log::info!("Attempting to create temporary directory {}", temp_dir);
    std::fs::create_dir_all(&download_directory).context("Unable to create download directory")?;
    std::fs::create_dir_all(&extract_directory).context("Unable to create extraction directory")?;

    let download_path = format!("{download_directory}/{filename}");
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
                    app.emit(
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

    app.emit(
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
    prepare_profile(
        std::path::Path::new(&extract_directory),
        &game_install.profile,
    )?;

    log::info!("Installing Northstar...");

    // Delete previous version here
    for core_mod in CORE_MODS {
        let path_to_delete_string = format!(
            "{}/{}/mods/{}/",
            game_install.game_path, game_install.profile, core_mod
        );
        log::info!("Preparing to remove {}", path_to_delete_string);

        // Check if folder exists
        let path_to_delete = std::path::Path::new(&path_to_delete_string);

        // Check if path even exists before we attempt to remove
        if !path_to_delete.exists() {
            log::info!("{} does not exist. Skipping", path_to_delete_string);
            continue;
        }

        if !path_to_delete.is_dir() {
            log::error!(
                "{} exists but is a file? This should never happen",
                path_to_delete_string
            );
            continue;
        }

        // Safety check for mod.json
        // Just so that we won't ever have a https://github.com/ValveSoftware/steam-for-linux/issues/3671 moment
        let mod_json_path = format!("{path_to_delete_string}/mod.json");
        let mod_json_path = std::path::Path::new(&mod_json_path);

        if !mod_json_path.exists() {
            log::error!("Missing mod.json for {path_to_delete_string} this shouldn't happen");
            continue;
        }

        // Finally delete file
        match std::fs::remove_dir_all(path_to_delete) {
            Ok(()) => {
                log::info!("Succesfully removed")
            }
            Err(err) => {
                log::error!("Failed removing {} due to {}", path_to_delete_string, err)
            }
        };
    }

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
    std::fs::remove_dir_all(temp_dir).context("Unable to delete temporary directory")?;

    log::info!("Done installing Northstar!");
    app.emit(
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

fn prepare_profile(extract_directory: &Path, profile: &str) -> Result<()> {
    if profile == NORTHSTAR_DEFAULT_PROFILE {
        return Ok(());
    }

    let old_profile_path = extract_directory.join(NORTHSTAR_DEFAULT_PROFILE);
    let new_profile_path = extract_directory.join(profile);
    let new_dll_path = old_profile_path.join(NORTHSTAR_DLL);

    // Newer packages keep the DLL inside R2Northstar, while older packages
    // placed it at the root of the extracted archive.
    if !new_dll_path.exists() {
        let old_dll_path = extract_directory.join(NORTHSTAR_DLL);
        if old_dll_path.exists() {
            std::fs::rename(&old_dll_path, &new_dll_path)
                .context("Unable to move Northstar.dll into the default profile")?;
        } else {
            return Err(anyhow::anyhow!(
                "Extracted Northstar package is missing {}",
                NORTHSTAR_DLL
            ));
        }
    }

    std::fs::rename(old_profile_path, new_profile_path)
        .context("Unable to rename the extracted Northstar profile")?;

    Ok(())
}

pub async fn install_northstar(
    app: AppHandle,
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

    match do_install(app, nmod.versions.get(version).unwrap(), game_install).await {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    fn test_directory(name: &str) -> PathBuf {
        let path = std::env::temp_dir().join(format!(
            "flightcore-install-test-{}-{}",
            std::process::id(),
            name
        ));
        if path.exists() {
            fs::remove_dir_all(&path).unwrap();
        }
        fs::create_dir_all(&path).unwrap();
        path
    }

    #[test]
    fn prepares_custom_profile_with_dll_in_default_profile() {
        let extract_directory = test_directory("profile-package-layout");
        let default_profile = extract_directory.join(NORTHSTAR_DEFAULT_PROFILE);
        fs::create_dir_all(&default_profile).unwrap();
        fs::write(default_profile.join(NORTHSTAR_DLL), b"northstar").unwrap();

        prepare_profile(&extract_directory, "CustomProfile").unwrap();

        assert!(extract_directory
            .join("CustomProfile")
            .join(NORTHSTAR_DLL)
            .is_file());
        fs::remove_dir_all(extract_directory).unwrap();
    }

    #[test]
    fn prepares_custom_profile_with_legacy_root_dll() {
        let extract_directory = test_directory("legacy-profile-package-layout");
        let default_profile = extract_directory.join(NORTHSTAR_DEFAULT_PROFILE);
        fs::create_dir_all(&default_profile).unwrap();
        fs::write(extract_directory.join(NORTHSTAR_DLL), b"northstar").unwrap();

        prepare_profile(&extract_directory, "CustomProfile").unwrap();

        assert!(extract_directory
            .join("CustomProfile")
            .join(NORTHSTAR_DLL)
            .is_file());
        fs::remove_dir_all(extract_directory).unwrap();
    }
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
