// This file contains various mod management functions
use async_recursion::async_recursion;

use anyhow::{anyhow, Result};
use app::NorthstarMod;
use serde::{Deserialize, Serialize};
use std::io::Read;
use std::path::PathBuf;

use app::get_enabled_mods;
use app::GameInstall;

use json5;

use crate::northstar::CORE_MODS;

pub const BLACKLISTED_MODS: [&str; 3] = [
    "northstar-Northstar",
    "northstar-NorthstarReleaseCandidate",
    "ebkr-r2modman",
];

#[derive(Debug, Clone)]
struct ParsedThunderstoreModString {
    author_name: String,
    mod_name: String,
    version: Option<String>,
}

impl std::str::FromStr for ParsedThunderstoreModString {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split("-");

        let author_name = parts.next().unwrap().to_string();
        let mod_name = parts.next().unwrap().to_string();
        let version = parts.next().map(|s| s.to_string());

        Ok(ParsedThunderstoreModString {
            author_name,
            mod_name,
            version,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThunderstoreManifest {
    name: String,
    version_number: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ModJson {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "ThunderstoreModString")]
    thunderstore_mod_string: Option<String>,
    #[serde(rename = "Version")]
    version: Option<String>,
}

/// Gets all currently installed and enabled/disabled mods to rebuild `enabledmods.json`
pub fn rebuild_enabled_mods_json(game_install: GameInstall) -> Result<(), String> {
    let enabledmods_json_path = format!("{}/R2Northstar/enabledmods.json", game_install.game_path);
    let mods_and_properties = get_installed_mods_and_properties(game_install)?;

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
    let mut res: serde_json::Value = match get_enabled_mods(game_install.clone()) {
        Ok(res) => res,
        Err(err) => {
            println!("Couldn't parse `enabledmod.json`: {}", err);
            println!("Rebuilding file.");

            rebuild_enabled_mods_json(game_install.clone())?;

            // Then try again
            let res = get_enabled_mods(game_install.clone())?;
            res
        }
    };

    // Check if key exists
    if res.get(mod_name.clone()).is_none() {
        // If it doesn't exist, rebuild `enabledmod.json`
        println!("Value not found in `enabledmod.json`. Rebuilding file");
        rebuild_enabled_mods_json(game_install.clone())?;

        // Then try again
        res = get_enabled_mods(game_install)?;
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

/// Parses `manifest.json` for Thunderstore mod string
fn parse_for_thunderstore_mod_string(nsmod_path: String) -> Result<String, anyhow::Error> {
    let manifest_json_path = format!("{}/manifest.json", nsmod_path);
    let ts_author_txt_path = format!("{}/thunderstore_author.txt", nsmod_path);

    // Check if `manifest.json` exists and parse
    let data = std::fs::read_to_string(manifest_json_path)?;
    let thunderstore_manifest: ThunderstoreManifest = json5::from_str(&data)?;

    // Check if `thunderstore_author.txt` exists and parse
    let mut file = std::fs::File::open(ts_author_txt_path)?;
    let mut thunderstore_author = String::new();
    file.read_to_string(&mut thunderstore_author)?;

    // Build mod string
    let thunderstore_mod_string = format!(
        "{}-{}-{}",
        thunderstore_author, thunderstore_manifest.name, thunderstore_manifest.version_number
    );

    Ok(thunderstore_mod_string)
}

/// Parse `mods` folder for installed mods.
fn parse_installed_mods(game_install: GameInstall) -> Result<Vec<NorthstarMod>, anyhow::Error> {
    let ns_mods_folder = format!("{}/R2Northstar/mods/", game_install.game_path);

    let paths = match std::fs::read_dir(ns_mods_folder) {
        Ok(paths) => paths,
        Err(_err) => return Err(anyhow!("No mods folder found")),
    };

    let mut directories: Vec<PathBuf> = Vec::new();
    let mut mods: Vec<NorthstarMod> = Vec::new();

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
        let directory_str = directory.to_str().unwrap().to_string();
        // Check if mod.json exists
        let mod_json_path = format!("{}/mod.json", directory_str);
        if !std::path::Path::new(&mod_json_path).exists() {
            continue;
        }

        // Parse mod.json and get mod name

        // Read file into string and parse it
        let data = std::fs::read_to_string(mod_json_path.clone())?;
        let parsed_mod_json: ModJson = match json5::from_str(&data) {
            Ok(parsed_json) => parsed_json,
            Err(err) => {
                println!("Failed parsing {} with {}", mod_json_path, err.to_string());
                continue;
            }
        };
        // Get Thunderstore mod string if it exists
        let thunderstore_mod_string = match parsed_mod_json.thunderstore_mod_string {
            // Attempt legacy method for getting Thunderstore string first
            Some(ts_mod_string) => Some(ts_mod_string),
            // Legacy method failed
            None => match parse_for_thunderstore_mod_string(directory_str) {
                Ok(thunderstore_mod_string) => Some(thunderstore_mod_string),
                Err(_err) => None,
            },
        };
        // Get directory path
        let mod_directory = directory.to_str().unwrap().to_string();

        let ns_mod = NorthstarMod {
            name: parsed_mod_json.name,
            version: parsed_mod_json.version,
            thunderstore_mod_string: thunderstore_mod_string,
            enabled: false, // Placeholder
            directory: mod_directory,
        };

        mods.push(ns_mod);
    }

    // Return found mod names
    Ok(mods)
}

/// Gets list of installed mods and their properties
/// - name
/// - is enabled?
#[tauri::command]
pub fn get_installed_mods_and_properties(
    game_install: GameInstall,
) -> Result<Vec<NorthstarMod>, String> {
    // Get actually installed mods
    let found_installed_mods = match parse_installed_mods(game_install.clone()) {
        Ok(res) => res,
        Err(err) => return Err(err.to_string()),
    };

    // Get enabled mods as JSON
    let enabled_mods: serde_json::Value = match get_enabled_mods(game_install) {
        Ok(enabled_mods) => enabled_mods,
        Err(_) => serde_json::from_str("{}").unwrap(), // `enabledmods.json` not found, create empty object
    };

    let mut installed_mods = Vec::new();
    let mapping = enabled_mods.as_object().unwrap();

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

async fn get_ns_mod_download_url(thunderstore_mod_string: String) -> Result<String, String> {
    // TODO: This will crash the thread if not internet connection exist. `match` should be used instead
    let index = thermite::api::get_package_index().await.unwrap().to_vec();

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
        parsed_ts_mod_string.version.unwrap()
    );

    for ns_mod in index {
        // Iterate over all versions of a given mod
        for (_key, ns_mod) in &ns_mod.versions {
            if ns_mod.url.contains(&ts_mod_string_url) {
                dbg!(ns_mod.clone());
                return Ok(ns_mod.url.clone());
            }
        }
    }

    Err("Could not find mod on Thunderstore".to_string())
}

/// Returns a vector of modstrings containing the dependencies of a given mod
async fn get_mod_dependencies(
    thunderstore_mod_string: String,
) -> Result<Vec<String>, anyhow::Error> {
    dbg!(thunderstore_mod_string.clone());

    // TODO: This will crash the thread if not internet connection exist. `match` should be used instead
    let index = thermite::api::get_package_index().await.unwrap().to_vec();

    // String replace works but more care should be taken in the future
    let ts_mod_string_url = thunderstore_mod_string.replace("-", "/");

    // Iterate over index
    for ns_mod in index {
        // Iterate over all versions of a given mod
        for (_key, ns_mod) in &ns_mod.versions {
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
    game_install: GameInstall,
    thunderstore_mod_string: String,
) -> Result<(), String> {
    // Get mods and download directories
    let download_directory = format!(
        "{}/___flightcore-temp-download-dir/",
        game_install.game_path
    );
    let mods_directory = format!("{}/R2Northstar/mods/", game_install.game_path);

    // Early return on empty string
    if thunderstore_mod_string.len() == 0 {
        return Err("Passed empty string".to_string());
    }

    let deps = match get_mod_dependencies(thunderstore_mod_string.clone()).await {
        Ok(deps) => deps,
        Err(err) => return Err(err.to_string()),
    };
    dbg!(deps.clone());

    // Recursively install dependencies
    for dep in deps {
        match fc_download_mod_and_install(game_install.clone(), dep).await {
            Ok(()) => (),
            Err(err) => {
                if err.to_string() == "Cannot install Northstar as a mod!" {
                    continue; // For Northstar as a dependency, we just skip it
                } else {
                    return Err(err.to_string());
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
    let download_url = get_ns_mod_download_url(thunderstore_mod_string.clone()).await?;

    // Create download directory
    match std::fs::create_dir_all(download_directory.clone()) {
        Ok(()) => (),
        Err(err) => return Err(err.to_string()),
    };

    let name = thunderstore_mod_string.clone();
    let path = format!(
        "{}/___flightcore-temp-download-dir/{}.zip",
        game_install.game_path, name
    );

    // Download the mod
    let f = match thermite::core::manage::download_file(&download_url, path.clone()).await {
        Ok(f) => f,
        Err(e) => return Err(e.to_string()),
    };

    // Get Thunderstore mod author
    let author = thunderstore_mod_string.split("-").next().unwrap();

    // Extract the mod to the mods directory
    match thermite::core::manage::install_mod(author, &f, std::path::Path::new(&mods_directory)) {
        Ok(()) => (),
        Err(err) => return Err(err.to_string()),
    };

    // Delete downloaded zip file
    std::fs::remove_file(path).unwrap();

    Ok(())
}

/// Deletes a given Northstar mod folder
fn delete_mod_folder(ns_mod_directory: String) -> Result<(), String> {
    let ns_mod_dir_path = std::path::Path::new(&ns_mod_directory);

    // Safety check: Check whether `mod.json` exists and exit early if not
    // If it does not exist, we might not be dealing with a Northstar mod
    let mod_json_path = ns_mod_dir_path.join("mod.json");
    if !mod_json_path.exists() {
        // If it doesn't exist, return an error
        return Err(format!("mod.json does not exist in {}", ns_mod_directory));
    }

    match std::fs::remove_dir_all(&ns_mod_directory) {
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
            return delete_mod_folder(installed_ns_mod.directory);
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
    // Prevent deleting core mod
    for core_ts_mod in BLACKLISTED_MODS {
        if thunderstore_mod_string == core_ts_mod {
            return Err(format!("Cannot remove core mod {thunderstore_mod_string}"));
        }
    }

    let parsed_ts_mod_string: ParsedThunderstoreModString =
        thunderstore_mod_string.parse().unwrap();

    // Get installed mods
    let installed_ns_mods = get_installed_mods_and_properties(game_install)?;

    // List of mod folders to remove
    let mut mod_folders_to_remove: Vec<String> = Vec::new();

    // Get folder name based on Thundestore mod string
    for installed_ns_mod in installed_ns_mods {
        if installed_ns_mod.thunderstore_mod_string.is_none() {
            // Not a Thunderstore mod
            continue;
        }

        let installed_ns_mod_ts_string: ParsedThunderstoreModString = installed_ns_mod
            .thunderstore_mod_string
            .unwrap()
            .parse()
            .unwrap();

        // Installed mod matches specified Thunderstore mod string
        if parsed_ts_mod_string.author_name == installed_ns_mod_ts_string.author_name
            && parsed_ts_mod_string.mod_name == installed_ns_mod_ts_string.mod_name
        {
            // Add folder to list of folder to remove
            mod_folders_to_remove.push(installed_ns_mod.directory);
        }
    }

    if !(mod_folders_to_remove.len() > 0) {
        return Err(format!(
            "No mods removed as no Northstar mods matching {thunderstore_mod_string} were found to be installed."
        ));
    }

    // Delete given folders
    for mod_folder in mod_folders_to_remove {
        delete_mod_folder(mod_folder)?;
    }

    Ok(())
}
