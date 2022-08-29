use anyhow::anyhow;

/// Check version number of a mod
pub fn check_mod_version_number(path_to_mod_folder: String) -> Result<String, anyhow::Error> {
    // println!("{}", format!("{}/mod.json", path_to_mod_folder));
    let data = std::fs::read_to_string(format!("{}/mod.json", path_to_mod_folder))?;
    let parsed_json: serde_json::Value = serde_json::from_str(&data)?;
    // println!("{}", parsed_json);
    let mod_version_number = match parsed_json.get("Version").and_then(|value| value.as_str()) {
        Some(version_number) => version_number,
        None => return Err(anyhow!("No version number found")),
    };

    println!("{}", mod_version_number);

    Ok(mod_version_number.to_string())
}

/// Attempts to find the game install location
pub fn find_game_install_location() -> Result<String, anyhow::Error> {
    // Attempt parsing Steam library directly
    match steamlocate::SteamDir::locate() {
        Some(mut steamdir) => {
            let titanfall2_steamid = 1237970;
            match steamdir.app(&titanfall2_steamid) {
                Some(app) => {
                    // println!("{:#?}", app);
                    return Ok(app.path.to_str().unwrap().to_string());
                }
                None => println!("Couldn't locate Titanfall2"),
            }
        }
        None => println!("Couldn't locate Steam on this computer!"),
    }
    Err(anyhow!(
        "Could not auto-detect game install location! Please enter it manually."
    ))
}

/// Returns the current Northstar version number as a string
pub fn get_northstar_version_number() -> Result<String, anyhow::Error> {
    let install_location = match find_game_install_location() {
        Ok(path) => path,
        Err(err) => return Err(err),
    };

    println!("{}", install_location);

    // TODO:
    // Check if NorthstarLauncher.exe exists and check its version number
    let profile_folder = "R2Northstar";
    let core_mods = [
        "Northstar.Client",
        "Northstar.Custom",
        "Northstar.CustomServers",
    ];
    let initial_version_number = match check_mod_version_number(format!(
        "{}/{}/mods/{}",
        install_location, profile_folder, core_mods[0]
    )) {
        Ok(version_number) => version_number,
        Err(err) => return Err(err),
    };

    for core_mod in core_mods {
        let current_version_number = match check_mod_version_number(format!(
            "{}/{}/mods/{}",
            install_location, profile_folder, core_mod
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
