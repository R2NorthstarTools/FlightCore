/// Contains various functions to repair common issues and verifying installation

use app::{get_enabled_mods, set_mod_enabled_status, GameInstall};

/// Verifies Titanfall2 game files
pub fn verify_game_files(game_install: GameInstall) -> Result<String, String> {
    dbg!(game_install);
    Err("TODO, not yet implemented".to_string())
}

/// Disables all mods except core ones
/// Enables core mods if disabled
pub fn disable_all_but_core(game_install: GameInstall) -> Result<(), String> {
    let current_mods = get_enabled_mods(game_install.clone())?;

    // These are the mods we do not want to disable
    let core_mods = [
        "Northstar.Client",
        "Northstar.Custom",
        "Northstar.CustomServers",
    ];
    // let sub_values: Vec<HashMap<String, Value>> = serde_json::from_str(&json)?;

    for (key, _value) in current_mods.as_object().unwrap() {
        if core_mods.contains(&key.as_str()) {
            // This is a core mod
            set_mod_enabled_status(game_install.clone(), key.to_string(), true)?;
        } else {
            // Not a core mod
            set_mod_enabled_status(game_install.clone(), key.to_string(), false)?;
        }
    }

    Ok(())
}
