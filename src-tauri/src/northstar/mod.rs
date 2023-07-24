//! This module deals with handling things around Northstar such as
//! - getting version number
pub mod install;

use crate::util::check_ea_app_or_origin_running;
use crate::{
    constants::{CORE_MODS, TITANFALL2_STEAM_ID},
    get_host_os, GameInstall, InstallType,
};
use anyhow::anyhow;

/// Check version number of a mod
pub fn check_mod_version_number(path_to_mod_folder: &str) -> Result<String, anyhow::Error> {
    let data = std::fs::read_to_string(format!("{path_to_mod_folder}/mod.json"))?;
    let parsed_json: serde_json::Value = serde_json::from_str(&data)?;

    let mod_version_number = match parsed_json.get("Version").and_then(|value| value.as_str()) {
        Some(version_number) => version_number,
        None => return Err(anyhow!("No version number found")),
    };

    log::info!("{}", mod_version_number);

    Ok(mod_version_number.to_string())
}

/// Returns the current Northstar version number as a string
#[tauri::command]
pub fn get_northstar_version_number(game_path: &str) -> Result<String, String> {
    log::info!("{}", game_path);

    // TODO:
    // Check if NorthstarLauncher.exe exists and check its version number
    let profile_folder = "R2Northstar";
    let initial_version_number = match check_mod_version_number(&format!(
        "{game_path}/{profile_folder}/mods/{}",
        CORE_MODS[0]
    )) {
        Ok(version_number) => version_number,
        Err(err) => return Err(err.to_string()),
    };

    for core_mod in CORE_MODS {
        let current_version_number = match check_mod_version_number(&format!(
            "{game_path}/{profile_folder}/mods/{core_mod}",
        )) {
            Ok(version_number) => version_number,
            Err(err) => return Err(err.to_string()),
        };
        if current_version_number != initial_version_number {
            // We have a version number mismatch
            return Err("Found version number mismatch".to_string());
        }
    }
    log::info!("All mods same version");

    Ok(initial_version_number)
}

/// Launches Northstar
#[tauri::command]
pub fn launch_northstar(
    game_install: GameInstall,
    bypass_checks: Option<bool>,
    launch_parameters: Option<String>,
) -> Result<String, String> {
    dbg!(game_install.clone());

    let host_os = get_host_os();

    // Explicitly fail early certain (currently) unsupported install setups
    if host_os != "windows" {
        if !matches!(game_install.install_type, InstallType::STEAM) {
            return Err(format!(
                "Not yet implemented for \"{}\" with Titanfall2 installed via \"{:?}\"",
                get_host_os(),
                game_install.install_type
            ));
        }

        return launch_northstar_steam(game_install, bypass_checks, launch_parameters);
    }

    let bypass_checks = bypass_checks.unwrap_or(false);

    // Only check guards if bypassing checks is not enabled
    if !bypass_checks {
        // Some safety checks before, should have more in the future
        if get_northstar_version_number(&game_install.game_path).is_err() {
            return Err(anyhow!("Not all checks were met").to_string());
        }

        // Require EA App or Origin to be running to launch Northstar
        let ea_app_is_running = check_ea_app_or_origin_running();
        if !ea_app_is_running {
            return Err(
                anyhow!("EA App not running, start EA App before launching Northstar").to_string(),
            );
        }
    }

    // Switch to Titanfall2 directory for launching
    // NorthstarLauncher.exe expects to be run from that folder
    if std::env::set_current_dir(game_install.game_path.clone()).is_err() {
        // We failed to get to Titanfall2 directory
        return Err(anyhow!("Couldn't access Titanfall2 directory").to_string());
    }

    // Only Windows with Steam or Origin are supported at the moment
    if host_os == "windows"
        && (matches!(game_install.install_type, InstallType::STEAM)
            || matches!(game_install.install_type, InstallType::ORIGIN)
            || matches!(game_install.install_type, InstallType::UNKNOWN))
    {
        let ns_exe_path = format!("{}/NorthstarLauncher.exe", game_install.game_path);

        let mut args = vec!["/C", "start", "", &ns_exe_path];
        // We cannot add the params directly because of limitations with cmd.exe
        // https://stackoverflow.com/questions/9964865/c-system-not-working-when-there-are-spaces-in-two-different-parameters/9965141#9965141

        let launch_parameters = launch_parameters.unwrap_or_else(|| "".to_string());
        let ns_params: Vec<&str> = launch_parameters.split_whitespace().collect();
        dbg!(ns_params.clone());
        args.extend(ns_params);
        let _output = std::process::Command::new("C:\\Windows\\System32\\cmd.exe")
            .args(args)
            .spawn()
            .expect("failed to execute process");
        return Ok("Launched game".to_string());
    }

    Err(format!(
        "Not yet implemented for {:?} on {}",
        game_install.install_type,
        get_host_os()
    ))
}

/// Prepare Northstar and Launch through Steam using the Browser Protocol
#[tauri::command]
pub fn launch_northstar_steam(
    game_install: GameInstall,
    _bypass_checks: Option<bool>,
    launch_parameters: Option<String>,
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
    if std::env::set_current_dir(game_install.game_path).is_err() {
        // We failed to get to Titanfall2 directory
        return Err("Couldn't access Titanfall2 directory".to_string());
    }

    let launch_parameters = launch_parameters.unwrap_or_else(|| "".to_string());
    match open::that(format!(
        "steam://run/{}//--northstar {}/",
        TITANFALL2_STEAM_ID, launch_parameters
    )) {
        Ok(()) => Ok("Started game".to_string()),
        Err(_err) => Err("Failed to launch Titanfall 2 via Steam".to_string()),
    }
}
