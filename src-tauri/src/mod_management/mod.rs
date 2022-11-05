// This file contains various mod management functions

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
