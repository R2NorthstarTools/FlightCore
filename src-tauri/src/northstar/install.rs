use crate::{
    GameInstall, InstallType,
};

#[cfg(target_os = "windows")]
use crate::platform_specific::windows;

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
