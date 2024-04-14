// Linux specific code

fn get_proton_dir() -> Option<String> {
    let steam_dir = steamlocate::SteamDir::locate()?;
    let compat_dir = format!("{}/compatibilitytools.d", steam_dir.path.display());

    Some(compat_dir)
}

/// Downloads and installs NS proton
/// Assumes Steam install
pub fn install_ns_proton() -> Result<(), thermite::prelude::ThermiteError> {
    // Get latest NorthstarProton release
    let latest = thermite::core::latest_release()?;

    let temp_dir = std::env::temp_dir();
    let path = format!("{}/nsproton-{}.tar.gz", temp_dir.display(), latest);
    let archive = std::fs::File::create(path.clone())?;

    // Download the latest Proton release
    log::info!("Downloading NorthstarProton to {}", path);
    thermite::core::download_ns_proton(latest, archive)?;
    log::info!("Finished Download");

    let compat_dir = get_proton_dir().unwrap();
    std::fs::create_dir_all(compat_dir.clone())?;

    let finished = std::fs::File::open(path.clone())?;

    // Extract to Proton dir
    log::info!("Installing NorthstarProton to {}", compat_dir);
    thermite::core::install_ns_proton(&finished, compat_dir)?;
    log::info!("Finished Installation");
    drop(finished);

    std::fs::remove_file(path)?;

    Ok(())
}

/// Remove NS Proton
pub fn uninstall_ns_proton() -> Result<(), String> {
    let compat_dir = get_proton_dir().unwrap();
    let pattern = format!("{}/NorthstarProton*", compat_dir);
    for e in glob::glob(&pattern).expect("Failed to read glob pattern") {
        std::fs::remove_dir_all(e.unwrap()).unwrap();
    }

    Ok(())
}

/// Get the latest installed NS Proton version
pub fn get_local_ns_proton_version() -> Result<String, String> {
    let compat_dir = get_proton_dir().unwrap();
    let pattern = format!("{}/NorthstarProton*/version", compat_dir);

    if let Some(e) = glob::glob(&pattern)
        .expect("Failed to read glob pattern")
        .next()
    {
        let version_content = std::fs::read_to_string(e.unwrap()).unwrap();
        let version = version_content.split(' ').nth(1).unwrap().to_string();

        return Ok(version);
    }

    Err("Northstar Proton is not installed".to_string())
}
