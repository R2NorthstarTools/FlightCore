// This file contains various mod management functions

use crate::constants::{BLACKLISTED_MODS, CORE_MODS};
use async_recursion::async_recursion;

use crate::NorthstarMod;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

mod legacy;
use crate::GameInstall;

#[derive(Debug, Clone)]
struct ParsedThunderstoreModString {
    author_name: String,
    mod_name: String,
    version: String,
}

impl std::str::FromStr for ParsedThunderstoreModString {
    type Err = &'static str; // todo use an better error management

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Check whether Thunderstore string passse reges
        let re = regex::Regex::new(r"^[a-zA-Z0-9_]+-[a-zA-Z0-9_]+-\d+\.\d+\.\d++$").unwrap();
        if !re.is_match(s) {
            return Err("Incorrect format");
        }

        let mut parts = s.split('-');

        let author_name = parts.next().ok_or("None value on author_name")?.to_string();
        let mod_name = parts.next().ok_or("None value on mod_name")?.to_string();
        let version = parts.next().ok_or("None value on version")?.to_string();

        Ok(ParsedThunderstoreModString {
            author_name,
            mod_name,
            version,
        })
    }
}

impl ToString for ParsedThunderstoreModString {
    fn to_string(&self) -> String {
        format!("{}-{}-{}", self.author_name, self.mod_name, self.version)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThunderstoreManifest {
    name: String,
    version_number: String,
}

/// A wrapper around a temporary file handle and its path.
///
/// This struct is designed to be used for temporary files that should be automatically deleted
/// when the `TempFile` instance goes out of scope.
#[derive(Debug)]
pub struct TempFile(fs::File, PathBuf);

impl TempFile {
    pub fn new(file: fs::File, path: PathBuf) -> Self {
        Self(file, path)
    }

    pub fn file(&self) -> &fs::File {
        &self.0
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
        _ = fs::remove_file(&self.1)
    }
}

impl std::ops::Deref for TempFile {
    type Target = fs::File;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Returns a serde json object of the parsed `enabledmods.json` file
pub fn get_enabled_mods(game_install: &GameInstall) -> Result<serde_json::value::Value, String> {
    let enabledmods_json_path = format!("{}/R2Northstar/enabledmods.json", game_install.game_path);

    // Check for JSON file
    if !std::path::Path::new(&enabledmods_json_path).exists() {
        return Err("enabledmods.json not found".to_string());
    }

    // Read file
    let data = match std::fs::read_to_string(enabledmods_json_path) {
        Ok(data) => data,
        Err(err) => return Err(err.to_string()),
    };

    // Parse JSON
    let res: serde_json::Value = match serde_json::from_str(&data) {
        Ok(result) => result,
        Err(err) => return Err(format!("Failed to read JSON due to: {}", err)),
    };

    // Return parsed data
    Ok(res)
}

/// Gets all currently installed and enabled/disabled mods to rebuild `enabledmods.json`
pub fn rebuild_enabled_mods_json(game_install: &GameInstall) -> Result<(), String> {
    let enabledmods_json_path = format!("{}/R2Northstar/enabledmods.json", game_install.game_path);
    let mods_and_properties = get_installed_mods_and_properties(game_install.clone())?;

    // Create new mapping
    let mut my_map = serde_json::Map::new();

    // Build mapping
    for ns_mod in mods_and_properties.into_iter() {
        my_map.insert(ns_mod.name, serde_json::Value::Bool(ns_mod.enabled));
    }

    // Turn into serde object
    let obj = serde_json::Value::Object(my_map);

    // Write to file
    std::fs::write(
        enabledmods_json_path,
        serde_json::to_string_pretty(&obj).unwrap(),
    )
    .unwrap();

    Ok(())
}

/// Set the status of a passed mod to enabled/disabled
#[tauri::command]
pub fn set_mod_enabled_status(
    game_install: GameInstall,
    mod_name: String,
    is_enabled: bool,
) -> Result<(), String> {
    let enabledmods_json_path = format!("{}/R2Northstar/enabledmods.json", game_install.game_path);

    // Parse JSON
    let mut res: serde_json::Value = match get_enabled_mods(&game_install) {
        Ok(res) => res,
        Err(err) => {
            log::warn!("Couldn't parse `enabledmod.json`: {}", err);
            log::warn!("Rebuilding file.");

            rebuild_enabled_mods_json(&game_install)?;

            // Then try again
            get_enabled_mods(&game_install)?
        }
    };

    // Check if key exists
    if res.get(mod_name.clone()).is_none() {
        // If it doesn't exist, rebuild `enabledmod.json`
        log::info!("Value not found in `enabledmod.json`. Rebuilding file");
        rebuild_enabled_mods_json(&game_install)?;

        // Then try again
        res = get_enabled_mods(&game_install)?;
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

/// Gets list of installed mods and their properties
/// - name
/// - is enabled?
#[tauri::command]
pub fn get_installed_mods_and_properties(
    game_install: GameInstall,
) -> Result<Vec<NorthstarMod>, String> {
    // Get actually installed mods
    let found_installed_mods = match legacy::parse_installed_mods(&game_install) {
        Ok(res) => res,
        Err(err) => return Err(err.to_string()),
    };

    // Get enabled mods as JSON
    let enabled_mods: serde_json::Value = match get_enabled_mods(&game_install) {
        Ok(enabled_mods) => enabled_mods,
        Err(_) => serde_json::from_str("{}").unwrap(), // `enabledmods.json` not found, create empty object
    };

    let mut installed_mods = Vec::new();
    let binding = serde_json::Map::new(); // Empty map in case treating as object fails
    let mapping = enabled_mods.as_object().unwrap_or(&binding);

    // Use list of installed mods and set enabled based on `enabledmods.json`
    for mut current_mod in found_installed_mods {
        let current_mod_enabled = match mapping.get(&current_mod.name) {
            Some(enabled) => enabled.as_bool().unwrap(),
            None => true, // Northstar considers mods not in mapping as enabled.
        };
        current_mod.enabled = current_mod_enabled;
        installed_mods.push(current_mod);
    }

    Ok(installed_mods)
}

async fn get_ns_mod_download_url(thunderstore_mod_string: &str) -> Result<String, String> {
    // TODO: This will crash the thread if not internet connection exist. `match` should be used instead
    let index = thermite::api::get_package_index().unwrap().to_vec();

    // Parse mod string
    let parsed_ts_mod_string: ParsedThunderstoreModString = match thunderstore_mod_string.parse() {
        Ok(res) => res,
        Err(_) => return Err("Failed to parse mod string".to_string()),
    };

    // Encode as URL
    let ts_mod_string_url = format!(
        "{}/{}/{}",
        parsed_ts_mod_string.author_name,
        parsed_ts_mod_string.mod_name,
        parsed_ts_mod_string.version
    );

    for ns_mod in index {
        // Iterate over all versions of a given mod
        for ns_mod in ns_mod.versions.values() {
            if ns_mod.url.contains(&ts_mod_string_url) {
                dbg!(ns_mod.clone());
                return Ok(ns_mod.url.clone());
            }
        }
    }

    Err("Could not find mod on Thunderstore".to_string())
}

/// Returns a vector of modstrings containing the dependencies of a given mod
async fn get_mod_dependencies(thunderstore_mod_string: &str) -> Result<Vec<String>, anyhow::Error> {
    log::info!("Attempting to get dependencies for: {thunderstore_mod_string}");

    let index = thermite::api::get_package_index()?.to_vec();

    // String replace works but more care should be taken in the future
    let ts_mod_string_url = thunderstore_mod_string.replace('-', "/");

    // Iterate over index
    for ns_mod in index {
        // Iterate over all versions of a given mod
        for ns_mod in ns_mod.versions.values() {
            if ns_mod.url.contains(&ts_mod_string_url) {
                dbg!(ns_mod.clone());
                return Ok(ns_mod.deps.clone());
            }
        }
    }
    Ok(Vec::<String>::new())
}

// Copied from `libtermite` source code and modified
// Should be replaced with a library call to libthermite in the future
/// Download and install mod to the specified target.
#[async_recursion]
pub async fn fc_download_mod_and_install(
    game_install: &GameInstall,
    thunderstore_mod_string: &str,
) -> Result<(), String> {
    log::info!("Attempting to install \"{thunderstore_mod_string}\" to {game_install:?}");
    // Get mods and download directories
    let download_directory = format!(
        "{}/___flightcore-temp-download-dir/",
        game_install.game_path
    );

    // Early return on empty string
    if thunderstore_mod_string.is_empty() {
        return Err("Passed empty string".to_string());
    }

    let deps = match get_mod_dependencies(thunderstore_mod_string).await {
        Ok(deps) => deps,
        Err(err) => return Err(err.to_string()),
    };
    log::info!("Mod dependencies: {deps:?}");

    // Recursively install dependencies
    for dep in deps {
        match fc_download_mod_and_install(game_install, &dep).await {
            Ok(()) => (),
            Err(err) => {
                if err == "Cannot install Northstar as a mod!" {
                    continue; // For Northstar as a dependency, we just skip it
                } else {
                    return Err(err);
                }
            }
        };
    }

    // Prevent installing Northstar as a mod
    // While it would fail during install anyway, having explicit error message is nicer
    for blacklisted_mod in BLACKLISTED_MODS {
        if thunderstore_mod_string.contains(blacklisted_mod) {
            return Err("Cannot install Northstar as a mod!".to_string());
        }
    }

    // Get download URL for the specified mod
    let download_url = get_ns_mod_download_url(thunderstore_mod_string).await?;

    // Create download directory
    match std::fs::create_dir_all(download_directory.clone()) {
        Ok(()) => (),
        Err(err) => return Err(err.to_string()),
    };

    let path = format!(
        "{}/___flightcore-temp-download-dir/{thunderstore_mod_string}.zip",
        game_install.game_path
    );

    // Download the mod
    let temp_file = TempFile::new(
        std::fs::File::options()
            .read(true)
            .write(true)
            .truncate(true)
            .create(true)
            .open(&path)
            .map_err(|e| e.to_string())?,
        (&path).into(),
    );
    match thermite::core::manage::download(temp_file.file(), download_url) {
        Ok(_written_bytes) => (),
        Err(err) => return Err(err.to_string()),
    };

    // Get directory to install to made up of packages directory and Thunderstore mod string
    let install_directory = format!(
        "{}/R2Northstar/packages/{}",
        game_install.game_path, thunderstore_mod_string
    );

    // Extract the mod to the mods directory
    match thermite::core::manage::install_mod(
        temp_file.file(),
        std::path::Path::new(&install_directory),
    ) {
        Ok(_) => (),
        Err(err) => {
            log::warn!("libthermite couldn't install mod {thunderstore_mod_string} due to {err:?}",);
            return Err(err.to_string());
        }
    };

    Ok(())
}

/// Deletes a given Northstar mod folder
fn delete_mod_folder(ns_mod_directory: &str) -> Result<(), String> {
    let ns_mod_dir_path = std::path::Path::new(&ns_mod_directory);

    // Safety check: Check whether `mod.json` exists and exit early if not
    // If it does not exist, we might not be dealing with a Northstar mod
    let mod_json_path = ns_mod_dir_path.join("mod.json");
    if !mod_json_path.exists() {
        // If it doesn't exist, return an error
        return Err(format!("mod.json does not exist in {}", ns_mod_directory));
    }

    match std::fs::remove_dir_all(ns_mod_directory) {
        Ok(()) => Ok(()),
        Err(err) => Err(format!("Failed deleting mod: {err}")),
    }
}

/// Deletes a Northstar mod based on its name
#[tauri::command]
pub fn delete_northstar_mod(game_install: GameInstall, nsmod_name: String) -> Result<(), String> {
    // Prevent deleting core mod
    for core_mod in CORE_MODS {
        if nsmod_name == core_mod {
            return Err(format!("Cannot remove core mod {nsmod_name}"));
        }
    }

    // Get installed mods
    let installed_ns_mods = get_installed_mods_and_properties(game_install)?;

    // Get folder name based on northstarmods
    for installed_ns_mod in installed_ns_mods {
        // Installed mod matches specified mod
        if installed_ns_mod.name == nsmod_name {
            // Delete folder
            return delete_mod_folder(&installed_ns_mod.directory);
        }
    }

    Err(format!("Mod {nsmod_name} not found to be installed"))
}

/// Deletes all NorthstarMods related to a Thunderstore mod
#[tauri::command]
pub fn delete_thunderstore_mod(
    game_install: GameInstall,
    thunderstore_mod_string: String,
) -> Result<(), String> {
    legacy::delete_thunderstore_mod(game_install, thunderstore_mod_string)
}
