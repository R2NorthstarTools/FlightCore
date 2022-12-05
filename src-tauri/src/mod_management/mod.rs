// This file contains various mod management functions
use async_recursion::async_recursion;

use anyhow::{anyhow, Result};
use app::NorthstarMod;
use std::path::PathBuf;

use app::get_enabled_mods;
use app::GameInstall;

use json5;

pub const BLACKLISTED_MODS: [&str; 3] = [
    "northstar-Northstar",
    "northstar-NorthstarReleaseCandidate",
    "ebkr-r2modman",
];

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

/// Parses `mod.json` for mod name
// TODO: Maybe pass PathBuf or serde json object
fn parse_mod_json_for_mod_name(mod_json_path: String) -> Result<String, anyhow::Error> {
    // Read file into string and parse it
    let data = std::fs::read_to_string(mod_json_path)?;
    let parsed_json: serde_json::Value = json5::from_str(&data)?;

    // Extract mod name
    let mod_name = match parsed_json.get("Name").and_then(|value| value.as_str()) {
        Some(name) => name,
        None => return Err(anyhow!("No name found")),
    };

    Ok(mod_name.to_string())
}

/// Parses `mod.json` for Thunderstore mod string
// TODO: Maybe pass PathBuf or serde json object
fn parse_mod_json_for_thunderstore_mod_string(
    mod_json_path: String,
) -> Result<String, anyhow::Error> {
    // Read file into string and parse it
    let data = std::fs::read_to_string(mod_json_path)?;
    let parsed_json: serde_json::Value = json5::from_str(&data)?;

    // Extract TS mod string
    let thunderstore_mod_string = match parsed_json
        .get("ThunderstoreModString")
        .and_then(|value| value.as_str())
    {
        Some(thunderstore_mod_string) => thunderstore_mod_string,
        None => return Err(anyhow!("No ThunderstoreModString found")),
    };

    Ok(thunderstore_mod_string.to_string())
}

/// Parse `mods` folder for installed mods.
fn parse_installed_mods(
    game_install: GameInstall,
) -> Result<Vec<(String, Option<String>)>, String> {
    let ns_mods_folder = format!("{}/R2Northstar/mods/", game_install.game_path);

    let paths = std::fs::read_dir(ns_mods_folder).unwrap();

    let mut directories: Vec<PathBuf> = Vec::new();
    let mut mods: Vec<(String, Option<String>)> = Vec::new();

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
        let thunderstore_mod_string =
            match parse_mod_json_for_thunderstore_mod_string(mod_json_path.clone()) {
                Ok(thunderstore_mod_string) => Some(thunderstore_mod_string),
                Err(_err) => None,
            };

        mods.push((mod_name, thunderstore_mod_string));
    }

    // Return found mod names
    Ok(mods)
}

/// Gets list of installed mods and their properties
/// - name
/// - is enabled?
pub fn get_installed_mods_and_properties(
    game_install: GameInstall,
) -> Result<Vec<NorthstarMod>, String> {
    // Get actually installed mods
    let found_installed_mods = parse_installed_mods(game_install.clone())?;

    // Get enabled mods as JSON
    let enabled_mods: serde_json::Value = match get_enabled_mods(game_install) {
        Ok(enabled_mods) => enabled_mods,
        Err(_) => serde_json::from_str("{}").unwrap(), // `enabledmods.json` not found, create empty object
    };

    let mut installed_mods = Vec::new();
    let mapping = enabled_mods.as_object().unwrap();

    // Use list of installed mods and set enabled based on `enabledmods.json`
    for (name, thunderstore_mod_string) in found_installed_mods {
        let current_mod_enabled = match mapping.get(&name) {
            Some(enabled) => enabled.as_bool().unwrap(),
            None => true, // Northstar considers mods not in mapping as enabled.
        };
        let current_mod: NorthstarMod = NorthstarMod {
            name: name,
            thunderstore_mod_string: thunderstore_mod_string,
            enabled: current_mod_enabled,
        };
        installed_mods.push(current_mod);
    }

    Ok(installed_mods)
}

async fn get_ns_mod_download_url(thunderstore_mod_string: String) -> Result<String, String> {
    // TODO: This will crash the thread if not internet connection exist. `match` should be used instead
    let index = thermite::api::get_package_index().await.unwrap().to_vec();

    // String replace works but more care should be taken in the future
    let ts_mod_string_url = thunderstore_mod_string.replace("-", "/");

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

/// Adds given Thunderstore mod string to the given `mod.json`
/// This way we can later check whether a mod is outdated based on the TS mod string
fn add_thunderstore_mod_string(
    path_to_mod_json: String,
    thunderstore_mod_string: String,
) -> Result<(), anyhow::Error> {
    // Read file into string and parse it
    let data = std::fs::read_to_string(path_to_mod_json.clone())?;
    let parsed_json: serde_json::Value = json5::from_str(&data)?;

    // Insert the Thunderstore mod string
    let mut parsed_json = parsed_json.as_object().unwrap().clone();
    parsed_json.insert(
        "ThunderstoreModString".to_string(),
        serde_json::Value::String(thunderstore_mod_string),
    );

    // And write back to disk
    std::fs::write(
        path_to_mod_json,
        serde_json::to_string_pretty(&parsed_json)?,
    )?;

    Ok(())
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
    let f = match thermite::core::actions::download_file(&download_url, path.clone()).await {
        Ok(f) => f,
        Err(e) => return Err(e.to_string()),
    };

    // Extract the mod to the mods directory
    let pkg = match thermite::core::actions::install_mod(&f, std::path::Path::new(&mods_directory))
    {
        Ok(pkg) => pkg,
        Err(err) => return Err(err.to_string()),
    };
    dbg!(pkg.clone());

    // Add Thunderstore mod string to `mod.json` of installed NorthstarMods
    for nsmod in pkg.mods {
        let path_to_current_mod_json = format!(
            "{}/{}/mod.json",
            mods_directory,
            nsmod.path.to_string_lossy()
        );
        match add_thunderstore_mod_string(path_to_current_mod_json, thunderstore_mod_string.clone())
        {
            Ok(()) => (),
            Err(err) => {
                println!("Failed setting modstring for {}", nsmod.name);
                println!("{}", err);
            }
        }
    }

    // Delete downloaded zip file
    std::fs::remove_file(path).unwrap();

    Ok(())
}
