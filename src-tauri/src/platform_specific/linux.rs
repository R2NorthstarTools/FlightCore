// Linux specific code

fn get_proton_dir() -> Result<String, String> {
    let steam_dir = match steamlocate::SteamDir::locate() {
        Ok(result) => result,
        Err(_) => return Err("Unable to find Steam directory".to_string()),
    };
    let compat_dir = format!("{}/compatibilitytools.d", steam_dir.path().display());

    Ok(compat_dir)
}

/// Downloads and installs NS proton
/// Assumes Steam install
pub fn install_ns_proton() -> Result<(), String> {
    // Get latest NorthstarProton release
    let latest = match thermite::core::latest_release() {
        Ok(result) => result,
        Err(_) => return Err("Failed to fetch latest NorthstarProton release".to_string()),
    };

    let temp_dir = std::env::temp_dir();
    let path = format!("{}/nsproton-{}.tar.gz", temp_dir.display(), latest);
    let archive = match std::fs::File::create(path.clone()) {
        Ok(result) => result,
        Err(_) => return Err("Failed to allocate NorthstarProton archive on disk".to_string()),
    };

    // Download the latest Proton release
    log::info!("Downloading NorthstarProton to {}", path);
    match thermite::core::download_ns_proton(latest, archive) {
        Ok(_) => {}
        Err(_) => return Err("Failed to download NorthstarProton".to_string()),
    }

    log::info!("Finished Download");

    let compat_dir = get_proton_dir()?;

    match std::fs::create_dir_all(compat_dir.clone()) {
        Ok(_) => {}
        Err(_) => return Err("Failed to create compatibilitytools directory".to_string()),
    }

    let finished = match std::fs::File::open(path.clone()) {
        Ok(result) => result,
        Err(_) => return Err("Failed to open NorthstarProton archive".to_string()),
    };

    // Extract to Proton dir
    log::info!("Installing NorthstarProton to {}", compat_dir);
    match thermite::core::install_ns_proton(&finished, compat_dir) {
        Ok(_) => {}
        Err(_) => return Err("Failed to create install NorthstarProton".to_string()),
    }
    log::info!("Finished Installation");
    drop(finished);

    // We installed NSProton, lets ignore this if it fails
    let _ = std::fs::remove_file(path);

    Ok(())
}

/// Remove NS Proton
pub fn uninstall_ns_proton() -> Result<(), String> {
    let compat_dir = get_proton_dir()?;
    let pattern = format!("{compat_dir}/NorthstarProton*");
    for e in glob::glob(&pattern).expect("Failed to read glob pattern") {
        match e {
            Ok(path) => match std::fs::remove_dir_all(path.clone()) {
                Ok(_) => {}
                Err(_) => return Err(format!("Failed to remove {}", path.display())),
            },
            Err(e) => return Err(format!("Found unprocessable entry {e}")),
        }
    }

    Ok(())
}

/// Get the latest installed NS Proton version
pub fn get_local_ns_proton_version() -> Result<String, String> {
    let compat_dir = get_proton_dir().unwrap();
    let pattern = format!("{compat_dir}/NorthstarProton*/version");

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
