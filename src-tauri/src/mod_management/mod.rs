// This file contains various mod management functions

use anyhow::{anyhow, Result};
use app::NorthstarMod;
use std::path::PathBuf;

use app::GameInstall;
use app::get_enabled_mods;

/// Set the status of a passed mod to enabled/disabled
pub fn set_mod_enabled_status(
    game_install: GameInstall,
    mod_name: String,
    is_enabled: bool,
) -> Result<(), String> {
    let enabledmods_json_path = format!("{}/R2Northstar/enabledmods.json", game_install.game_path);

    // Parse JSON
    let mut res: serde_json::Value = get_enabled_mods(game_install)?;

    // Check if key exists
    if res.get(mod_name.clone()).is_none() {
        return Err("Value not found in enabledmod.json".to_string());
    }

    // Update value
    res[mod_name] = serde_json::Value::Bool(is_enabled);

    // Save the JSON structure into the output file
    std::fs::write(
        enabledmods_json_path,
        serde_json::to_string_pretty(&res).unwrap(),
    )
    .unwrap();

    Ok(())
}


/// Parses `mod.json` for mod name
// TODO: Maybe pass PathBuf or serde json object
fn parse_mod_json_for_mod_name(mod_json_path: String) -> Result<String, anyhow::Error> {
    // Read file into string and parse it
    let data = std::fs::read_to_string(mod_json_path)?;
    let parsed_json: serde_json::Value = serde_json::from_str(&data)?;

    // Extract mod name
    let mod_name = match parsed_json.get("Name").and_then(|value| value.as_str()) {
        Some(name) => name,
        None => return Err(anyhow!("No name found")),
    };

    Ok(mod_name.to_string())
}

/// Parse `mods` folder for installed mods.
fn get_installed_mods(game_install: GameInstall) -> Result<Vec<String>, String> {
    let ns_mods_folder = format!("{}/R2Northstar/mods/", game_install.game_path);

    let paths = std::fs::read_dir(ns_mods_folder).unwrap();

    let mut directories: Vec<PathBuf> = Vec::new();
    let mut mod_names: Vec<String> = Vec::new();

    // Get list of folders in `mods` directory
    for path in paths {
        let my_path = path.unwrap().path();

        let md = std::fs::metadata(my_path.clone()).unwrap();
        if md.is_dir() {
            directories.push(my_path);
        }
    }

    // Iterate over folders and check if they are Northstar mods
    for directory in directories {
        // Check if mod.json exists
        let mod_json_path = format!("{}/mod.json", directory.to_str().unwrap());
        if !std::path::Path::new(&mod_json_path).exists() {
            continue;
        }

        // Parse mod.json and get mod name
        let mod_name = match parse_mod_json_for_mod_name(mod_json_path.clone()) {
            Ok(mod_name) => mod_name,
            Err(err) => {
                println!("Failed parsing {} with {}", mod_json_path, err.to_string());
                continue;
            }
        };
        mod_names.push(mod_name);
    }

    // Return found mod names
    Ok(mod_names)
}

/// Gets list of installed mods and their properties
/// - name
/// - is enabled?
pub fn get_installed_mods_and_properties(
    game_install: GameInstall,
) -> Result<Vec<NorthstarMod>, String> {
    // Get actually installed mods
    let found_installed_mods = get_installed_mods(game_install.clone())?;

    // Get enabled mods as JSON
    let enabled_mods: serde_json::Value = match get_enabled_mods(game_install) {
        Ok(enabled_mods) => enabled_mods,
        Err(_) => serde_json::from_str("{}").unwrap(), // `enabledmods.json` not found, create empty object
    };

    let mut installed_mods = Vec::new();
    let mapping = enabled_mods.as_object().unwrap();

    // Use list of installed mods and set enabled based on `enabledmods.json`
    for name in found_installed_mods {
        let current_mod_enabled = match mapping.get(&name) {
            Some(enabled) => enabled.as_bool().unwrap(),
            None => true, // Northstar considers mods not in mapping as enabled.
        };
        let current_mod: NorthstarMod = NorthstarMod {
            name: name,
            enabled: current_mod_enabled,
        };
        installed_mods.push(current_mod);
    }

    Ok(installed_mods)
}
