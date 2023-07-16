use crate::GameInstall;
use crate::NorthstarMod;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::{io::Read, path::PathBuf};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ModJson {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "ThunderstoreModString")]
    thunderstore_mod_string: Option<String>,
    #[serde(rename = "Version")]
    version: Option<String>,
}

/// Parses `manifest.json` for Thunderstore mod string
fn parse_for_thunderstore_mod_string(nsmod_path: &str) -> Result<String, anyhow::Error> {
    let manifest_json_path = format!("{}/manifest.json", nsmod_path);
    let ts_author_txt_path = format!("{}/thunderstore_author.txt", nsmod_path);

    // Check if `manifest.json` exists and parse
    let data = std::fs::read_to_string(manifest_json_path)?;
    let thunderstore_manifest: super::ThunderstoreManifest = json5::from_str(&data)?;

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
pub fn parse_installed_mods(
    game_install: &GameInstall,
) -> Result<Vec<NorthstarMod>, anyhow::Error> {
    let ns_mods_folder = format!("{}/R2Northstar/mods/", game_install.game_path);

    let paths = match std::fs::read_dir(ns_mods_folder) {
        Ok(paths) => paths,
        Err(_err) => return Err(anyhow!("No mods folder found")),
    };

    let mut directories: Vec<PathBuf> = Vec::new();
    let mut mods: Vec<NorthstarMod> = Vec::new();

    // Get list of folders in `mods` directory
    for path in paths {
        log::info!("{path:?}");
        let my_path = path.unwrap().path();
        log::info!("{my_path:?}");

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
                log::warn!("Failed parsing {} with {}", mod_json_path, err.to_string());
                continue;
            }
        };
        // Get Thunderstore mod string if it exists
        let thunderstore_mod_string = match parsed_mod_json.thunderstore_mod_string {
            // Attempt legacy method for getting Thunderstore string first
            Some(ts_mod_string) => Some(ts_mod_string),
            // Legacy method failed
            None => match parse_for_thunderstore_mod_string(&directory_str) {
                Ok(thunderstore_mod_string) => Some(thunderstore_mod_string),
                Err(_err) => None,
            },
        };
        // Get directory path
        let mod_directory = directory.to_str().unwrap().to_string();

        let ns_mod = NorthstarMod {
            name: parsed_mod_json.name,
            version: parsed_mod_json.version,
            thunderstore_mod_string,
            enabled: false, // Placeholder
            directory: mod_directory,
        };

        mods.push(ns_mod);
    }

    // Return found mod names
    Ok(mods)
}
