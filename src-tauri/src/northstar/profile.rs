use crate::GameInstall;

// These folders are part of Titanfall 2 and
// should NEVER be used as a Profile
const SKIP_PATHS: [&str; 8] = [
    "___flightcore-temp",
    "__overlay",
    "bin",
    "Core",
    "r2",
    "vpk",
    "platform",
    "Support",
];

// A profile may have one of these to be detected
const MAY_CONTAIN: [&str; 10] = [
    "mods/",
    "plugins/",
    "packages/",
    "logs/",
    "runtime/",
    "save_data/",
    "Northstar.dll",
    "enabledmods.json",
    "placeholder.playerdata.pdata",
    "LEGAL.txt",
];

/// Returns a list of Profile names
/// All the returned Profiles can be found relative to the game path
#[tauri::command]
pub fn fetch_profiles(game_install: GameInstall) -> Result<Vec<String>, String> {
    let mut profiles: Vec<String> = Vec::new();

    for content in MAY_CONTAIN {
        let pattern = format!("{}/*/{}", game_install.game_path, content);
        for e in glob::glob(&pattern).expect("Failed to read glob pattern") {
            let path = e.unwrap();
            let mut ancestors = path.ancestors();

            ancestors.next();

            let profile_path = std::path::Path::new(ancestors.next().unwrap());
            let profile_name = profile_path
                .file_name()
                .unwrap()
                .to_os_string()
                .into_string()
                .unwrap();

            if !profiles.contains(&profile_name) {
                profiles.push(profile_name);
            }
        }
    }

    Ok(profiles)
}

/// Validates if a given profile is actually a valid profile
#[tauri::command]
pub fn validate_profile(game_install: GameInstall, profile: String) -> bool {
    // Game files are never a valid profile
    // Prevent users with messed up installs from making it even worse
    if SKIP_PATHS.contains(&profile.as_str()) {
        return false;
    }

    log::info!("Validating Profile {}", profile);

    let profile_path = format!("{}/{}", game_install.game_path, profile);
    let profile_dir = std::path::Path::new(profile_path.as_str());

    profile_dir.is_dir()
}
