//! This module deals with handling things around Northstar such as
//! - getting version number

pub const CORE_MODS: [&str; 3] = [
    "Northstar.Client",
    "Northstar.Custom",
    "Northstar.CustomServers",
];

use crate::check_mod_version_number;
use anyhow::anyhow;

/// Returns the current Northstar version number as a string
pub fn get_northstar_version_number(game_path: String) -> Result<String, anyhow::Error> {
    println!("{}", game_path);
    // println!("{:?}", install_type);

    // TODO:
    // Check if NorthstarLauncher.exe exists and check its version number
    let profile_folder = "R2Northstar";
    let initial_version_number = match check_mod_version_number(format!(
        "{}/{}/mods/{}",
        game_path, profile_folder, CORE_MODS[0]
    )) {
        Ok(version_number) => version_number,
        Err(err) => return Err(err),
    };

    for core_mod in CORE_MODS {
        let current_version_number = match check_mod_version_number(format!(
            "{}/{}/mods/{}",
            game_path, profile_folder, core_mod
        )) {
            Ok(version_number) => version_number,
            Err(err) => return Err(err),
        };
        if current_version_number != initial_version_number {
            // We have a version number mismatch
            return Err(anyhow!("Found version number mismatch"));
        }
    }
    println!("All mods same version");

    Ok(initial_version_number)
}
