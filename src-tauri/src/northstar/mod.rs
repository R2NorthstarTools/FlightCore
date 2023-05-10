//! This module deals with handling things around Northstar such as
//! - getting version number

use crate::constants::CORE_MODS;
use crate::{check_origin_running, get_host_os, GameInstall, InstallType};
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
pub fn get_northstar_version_number(game_path: &str) -> Result<String, anyhow::Error> {
    log::info!("{}", game_path);

    // TODO:
    // Check if NorthstarLauncher.exe exists and check its version number
    let profile_folder = "R2Northstar";
    let initial_version_number = match check_mod_version_number(&format!(
        "{game_path}/{profile_folder}/mods/{}",
        CORE_MODS[0]
    )) {
        Ok(version_number) => version_number,
        Err(err) => return Err(err),
    };

    for core_mod in CORE_MODS {
        let current_version_number = match check_mod_version_number(&format!(
            "{game_path}/{profile_folder}/mods/{core_mod}",
        )) {
            Ok(version_number) => version_number,
            Err(err) => return Err(err),
        };
        if current_version_number != initial_version_number {
            // We have a version number mismatch
            return Err(anyhow!("Found version number mismatch"));
        }
    }
    log::info!("All mods same version");

    Ok(initial_version_number)
}

pub fn launch_northstar(
    game_install: &GameInstall,
    bypass_checks: Option<bool>,
) -> Result<String, String> {
    dbg!(game_install.clone());

    let host_os = get_host_os();

    // Explicitly fail early certain (currently) unsupported install setups
    if host_os != "windows"
        || !(matches!(game_install.install_type, InstallType::STEAM)
            || matches!(game_install.install_type, InstallType::ORIGIN)
            || matches!(game_install.install_type, InstallType::UNKNOWN))
    {
        return Err(format!(
            "Not yet implemented for \"{}\" with Titanfall2 installed via \"{:?}\"",
            get_host_os(),
            game_install.install_type
        ));
    }

    let bypass_checks = bypass_checks.unwrap_or(false);

    // Only check guards if bypassing checks is not enabled
    if !bypass_checks {
        // Some safety checks before, should have more in the future
        if get_northstar_version_number(&game_install.game_path).is_err() {
            return Err(anyhow!("Not all checks were met").to_string());
        }

        // Require Origin to be running to launch Northstar
        let origin_is_running = check_origin_running();
        if !origin_is_running {
            return Err(
                anyhow!("Origin not running, start Origin before launching Northstar").to_string(),
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
        let _output = std::process::Command::new("C:\\Windows\\System32\\cmd.exe")
            .args(["/C", "start", "", &ns_exe_path])
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
