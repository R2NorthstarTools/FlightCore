use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use std::{cell::RefCell, time::Instant};
use ts_rs::TS;

use crate::constants::TITANFALL2_STEAM_ID;
use crate::{util::extract, GameInstall, InstallType};

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
    game_path: &std::path::Path,
) -> Result<()> {
    let filename = format!("northstar-{}.zip", nmod.version);
    let download_directory = format!("{}/___flightcore-temp-download-dir/", game_path.display());

    log::info!(
        "Attempting to create temporary directory {}",
        download_directory
    );
    std::fs::create_dir_all(download_directory.clone())?;

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
                state: InstallState::Done,
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
                        launch_parameters: "".to_string(),
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
