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

fn get_proton_dir() -> Option<String> {
    let steam_dir = steamlocate::SteamDir::locate()?;
    let compat_dir = format!("{}/compatibilitytools.d/", steam_dir.path.display());

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
    let pattern = format!("{}/NorthstarProton-*", compat_dir);
    for e in glob::glob(&pattern).expect("Failed to read glob pattern") {
        std::fs::remove_dir_all(e.unwrap()).unwrap();
    }

    Ok(())
}

/// Get the latest installed NS Proton version
pub fn get_local_ns_proton_version() -> Result<String, String> {
    let compat_dir = get_proton_dir().unwrap();
    let ns_prefix = "NorthstarProton-";
    let pattern = format!("{}/{}*/version", compat_dir, ns_prefix);

    let mut version: String = "".to_string();

    for e in glob::glob(&pattern).expect("Failed to read glob pattern") {
        let version_content = std::fs::read_to_string(e.unwrap()).unwrap();
        let version_string = version_content.split(' ').nth(1).unwrap();

        if version_string.starts_with(ns_prefix) {
            version = version_string[ns_prefix.len()..version_string.len() - 1]
                .to_string()
                .clone();
        }
    }

    if version.is_empty() {
        return Err("Northstar Proton is not installed".to_string());
    }

    Ok(version)
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
