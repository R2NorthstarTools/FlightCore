// Linux specific code

use regex::Regex;
use std::process::Command;

// I intend to add more linux related stuff to check here, so making a func
// for now tho it only checks `ldd --version`
// - salmon
pub fn linux_checks_librs() -> Result<(), String> {
    // Perform various checks in terms of Linux compatibility
    // Return early with error message if a check fails

    // check `ldd --version` to see if glibc is up to date for northstar proton
    let min_required_ldd_version = 2.33;
    let lddv = check_glibc_v();
    if lddv < min_required_ldd_version {
        return Err(format!(
            "GLIBC is not version {} or greater",
            min_required_ldd_version
        ));
    };

    // All checks passed
    Ok(())
}

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
    let pattern = format!("{}/NorthstarProton*", compat_dir);
    for e in glob::glob(&pattern).expect("Failed to read glob pattern") {
        match e {
            Ok(path) => match std::fs::remove_dir_all(path.clone()) {
                Ok(_) => {}
                Err(_) => return Err(format!("Failed to remove {}", path.display())),
            },
            Err(e) => return Err(format!("Found unprocessable entry {}", e)),
        }
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

pub fn check_glibc_v() -> f32 {
    let out = Command::new("/bin/ldd")
        .arg("--version")
        .output()
        .expect("failed to run 'ldd --version'");

    // parse the output down to just the first line
    let lddva = String::from_utf8_lossy(&out.stdout);
    let lddvl: Vec<&str> = lddva.split('\n').collect();
    let lddvlo = &lddvl[0];
    let reg = Regex::new(r"(2.\d{2}$)").unwrap();
    if let Some(caps) = reg.captures_iter(lddvlo).next() {
        return caps.get(1).unwrap().as_str().parse::<f32>().unwrap(); // theres prolly a better way ijdk how tho
    }
    0.0 // this shouldnt ever be reached but it has to be here
}

/*
Outputs of ldd --verssion from distros, all we care about is the first line so trimmed, also removed all duplicates
Thanks tony
Distros not included: AmazonLinux, Gentoo, Kali, Debian before 11, Oracle Linux, Scientific Linux, Slackware, Mageia, Neurodebian, RHEL 8 and 9 (Same as AlmaLinux), RockyLinux (Same as AlmaLinux), Ubuntu before 20.04

AlmaLinux 8
ldd (GNU libc) 2.35

Centos Stream 8
ldd (GNU libc) 2.28

Centos Stream 9
ldd (GNU libc) 2.34

Centos 7
ldd (GNU libc) 2.17

Debian 11
ldd (Debian GLIBC 2.31-13+deb11u4) 2.31

Debian Testing
ldd (Debian GLIBC 2.35-1) 2.35

Debian Unstable
ldd (Debian GLIBC 2.35-3) 2.35

Fedora 37
ldd (GNU libc) 2.36

Opensuse Leap
ldd (GNU libc) 2.31

Ubuntu 20.04
ldd (Ubuntu GLIBC 2.31-0ubuntu9.9) 2.31

Ubuntu 22.04
ldd (Ubuntu GLIBC 2.35-0ubuntu3.1) 2.35

Ubuntu 22.10
ldd (Ubuntu GLIBC 2.36-0ubuntu2) 2.36
*/
