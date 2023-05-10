use crate::mod_management::{get_enabled_mods, rebuild_enabled_mods_json, set_mod_enabled_status};
/// Contains various functions to repair common issues and verifying installation
use crate::{constants::CORE_MODS, GameInstall};
use anyhow::anyhow;

/// Verifies Titanfall2 game files
#[tauri::command]
pub fn verify_game_files(game_install: GameInstall) -> Result<String, String> {
    dbg!(game_install);
    Err("TODO, not yet implemented".to_string())
}

/// Disables all mods except core ones
/// Enables core mods if disabled
#[tauri::command]
pub fn disable_all_but_core(game_install: GameInstall) -> Result<(), String> {
    // Rebuild `enabledmods.json` first to ensure all mods are added
    rebuild_enabled_mods_json(&game_install)?;

    let current_mods = get_enabled_mods(&game_install)?;

    // Disable all mods, set core mods to enabled
    for (key, _value) in current_mods.as_object().unwrap() {
        if CORE_MODS.contains(&key.as_str()) {
            // This is a core mod, we do not want to disable it
            set_mod_enabled_status(game_install.clone(), key.to_string(), true)?;
        } else {
            // Not a core mod
            set_mod_enabled_status(game_install.clone(), key.to_string(), false)?;
        }
    }

    Ok(())
}

/// Deletes download folder
/// If `force` is FALSE, bails on non-empty folder
/// If `force` is TRUE, deletes folder even if non-empty
pub fn clean_up_download_folder(
    game_install: &GameInstall,
    force: bool,
) -> Result<(), anyhow::Error> {
    // Get download directory
    let download_directory = format!(
        "{}/___flightcore-temp-download-dir/",
        game_install.game_path
    );

    // Check if files in folder
    let download_dir_contents = std::fs::read_dir(download_directory.clone())?;
    // dbg!(download_dir_contents);

    let mut count = 0;
    download_dir_contents.for_each(|_| count += 1);

    if count > 0 && !force {
        return Err(anyhow!("Folder not empty, not deleting"));
    }

    // Delete folder
    std::fs::remove_dir_all(download_directory)?;

    Ok(())
}

/// Get list of Northstar logs
#[tauri::command]
pub fn get_log_list(game_install: GameInstall) -> Result<Vec<std::path::PathBuf>, String> {
    let ns_log_folder = format!("{}/R2Northstar/logs", game_install.game_path);

    // List files in logs folder
    let paths = match std::fs::read_dir(ns_log_folder) {
        Ok(paths) => paths,
        Err(_err) => return Err("No logs folder found".to_string()),
    };

    // Stores paths of log files
    let mut log_files: Vec<std::path::PathBuf> = Vec::new();

    for path in paths {
        let path = path.unwrap().path();
        if path.display().to_string().contains("nslog") {
            log_files.push(path);
        }
    }

    if !log_files.is_empty() {
        Ok(log_files)
    } else {
        Err("No logs found".to_string())
    }
}
