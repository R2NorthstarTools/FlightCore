use crate::util::copy_dir_all;
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

#[tauri::command]
pub fn delete_profile(game_install: GameInstall, profile: String) -> Result<(), String> {
    // Check if the Profile actually exists
    if !validate_profile(game_install.clone(), profile.clone()) {
        return Err(format!("{profile} is not a valid Profile"));
    }

    log::info!("Deleting Profile {}", profile);

    let profile_path = format!("{}/{}", game_install.game_path, profile);

    match std::fs::remove_dir_all(profile_path) {
        Ok(()) => Ok(()),
        Err(err) => Err(format!("Failed to delete Profile: {err}")),
    }
}

/// Clones a profile by simply duplicating the folder under a new name
#[tauri::command]
pub fn clone_profile(
    game_install: GameInstall,
    old_profile: String,
    new_profile: String,
) -> Result<(), String> {
    // Check if the old Profile already exists
    if !validate_profile(game_install.clone(), old_profile.clone()) {
        return Err(format!("{old_profile} is not a valid Profile"));
    }

    // Check that new Profile does not already exist
    if validate_profile(game_install.clone(), new_profile.clone()) {
        return Err(format!("{new_profile} already exists"));
    }

    log::info!("Cloning Profile {} to {}", old_profile, new_profile);

    let old_profile_path = format!("{}/{}", game_install.game_path, old_profile);
    let new_profile_path = format!("{}/{}", game_install.game_path, new_profile);

    copy_dir_all(old_profile_path, new_profile_path).unwrap();

    Ok(())
}
