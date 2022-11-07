use std::{env, path::PathBuf};

use anyhow::{anyhow, Context, Result};

mod platform_specific;
#[cfg(target_os = "windows")]
use platform_specific::windows;

use platform_specific::linux;

use serde::{Deserialize, Serialize};
use sysinfo::SystemExt;
use zip::ZipArchive;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum InstallType {
    STEAM,
    ORIGIN,
    EAPLAY,
    UNKNOWN,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameInstall {
    pub game_path: String,
    pub install_type: InstallType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NorthstarMod {
    pub name: String,
    pub enabled: bool,
}

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

// I intend to add more linux related stuff to check here, so making a func
// for now tho it only checks `ldd --version`
// - salmon

pub fn linux_checks_librs() -> Result<(), String> {
    // Perform various checks in terms of Linux compatibility
    // Return early with error message if a check fails

    // check `ldd --version` to see if glibc is up to date for northstar proton
    let min_required_ldd_version = 2.33;
    let lddv = linux::check_glibc_v();
    if lddv < min_required_ldd_version {
        return Err(format!(
            "GLIBC is not version {} or greater",
            min_required_ldd_version
        )
        .to_string());
    };

    // All checks passed
    Ok(())
}

/// Attempts to find the game install location
pub fn find_game_install_location() -> Result<GameInstall, String> {
    // Attempt parsing Steam library directly
    match steamlocate::SteamDir::locate() {
        Some(mut steamdir) => {
            let titanfall2_steamid = 1237970;
            match steamdir.app(&titanfall2_steamid) {
                Some(app) => {
                    // println!("{:#?}", app);
                    let game_install = GameInstall {
                        game_path: app.path.to_str().unwrap().to_string(),
                        install_type: InstallType::STEAM,
                    };
                    return Ok(game_install);
                }
                None => println!("Couldn't locate Titanfall2"),
            }
        }
        None => println!("Couldn't locate Steam on this computer!"),
    }

    // (On Windows only) try parsing Windows registry for Origin install path
    #[cfg(target_os = "windows")]
    match windows::origin_install_location_detection() {
        Ok(game_path) => {
            let game_install = GameInstall {
                game_path: game_path,
                install_type: InstallType::ORIGIN,
            };
            return Ok(game_install);
        }
        Err(err) => {
            println!("{}", err);
        }
    };

    Err("Could not auto-detect game install location! Please enter it manually.".to_string())
}

/// Returns the current Northstar version number as a string
pub fn get_northstar_version_number(game_path: String) -> Result<String, anyhow::Error> {
    println!("{}", game_path);
    // println!("{:?}", install_type);

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
        game_path, profile_folder, core_mods[0]
    )) {
        Ok(version_number) => version_number,
        Err(err) => return Err(err),
    };

    for core_mod in core_mods {
        let current_version_number = match check_mod_version_number(format!(
            "{}/{}/mods/{}",
            game_path, profile_folder, core_mod
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

/// Checks whether the provided path is a valid Titanfall2 gamepath by checking against a certain set of criteria
pub fn check_is_valid_game_path(game_install_path: &str) -> Result<(), anyhow::Error> {
    let path_to_titanfall2_exe = format!("{}/Titanfall2.exe", game_install_path);
    let is_correct_game_path = std::path::Path::new(&path_to_titanfall2_exe).exists();
    println!("Titanfall2.exe exists in path? {}", is_correct_game_path);

    // Exit early if wrong game path
    if !is_correct_game_path {
        return Err(anyhow!("Incorrect game path \"{}\"", game_install_path)); // Return error cause wrong game path
    }
    Ok(())
}

/// Copied from `papa` source code and modified
///Extract N* zip file to target game path
// fn extract(ctx: &Ctx, zip_file: File, target: &Path) -> Result<()> {
fn extract(zip_file: std::fs::File, target: &std::path::Path) -> Result<()> {
    let mut archive = ZipArchive::new(&zip_file).context("Unable to open zip archive")?;
    for i in 0..archive.len() {
        let mut f = archive.by_index(i).unwrap();

        //This should work fine for N* because the dir structure *should* always be the same
        if f.enclosed_name().unwrap().starts_with("Northstar") {
            let out = target.join(
                f.enclosed_name()
                    .unwrap()
                    .strip_prefix("Northstar")
                    .unwrap(),
            );

            if (*f.name()).ends_with('/') {
                println!("Create directory {}", f.name());
                std::fs::create_dir_all(target.join(f.name()))
                    .context("Unable to create directory")?;
                continue;
            } else if let Some(p) = out.parent() {
                std::fs::create_dir_all(&p).context("Unable to create directory")?;
            }

            let mut outfile = std::fs::OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(&out)?;

            println!("Write file {}", out.display());

            std::io::copy(&mut f, &mut outfile).context("Unable to write to file")?;
        }
    }

    Ok(())
}

/// Copied from `papa` source code and modified
///Install N* from the provided mod
///
///Checks cache, else downloads the latest version
async fn do_install(nmod: &thermite::model::Mod, game_path: &std::path::Path) -> Result<()> {
    let filename = format!("northstar-{}.zip", nmod.version);
    let download_directory = format!("{}/___flightcore-temp-download-dir/", game_path.display());

    std::fs::create_dir_all(download_directory.clone())?;

    let download_path = format!("{}/{}", download_directory.clone(), filename);
    println!("{}", download_path);

    let nfile = thermite::core::actions::download_file(&nmod.url, download_path)
        .await
        .unwrap();

    println!("Extracting Northstar...");
    extract(nfile, game_path)?;

    // Delete old copy
    println!("Delete temp folder again");
    std::fs::remove_dir_all(download_directory).unwrap();

    println!("Done!");

    Ok(())
}

pub async fn install_northstar(
    game_path: &str,
    northstar_package_name: Option<String>,
) -> Result<String, String> {
    let northstar_package_name = match northstar_package_name {
        Some(northstar_package_name) => {
            if northstar_package_name.len() <= 1 {
                "Northstar".to_string()
            } else {
                northstar_package_name
            }
        }
        None => "Northstar".to_string(),
    };

    let index = thermite::api::get_package_index().await.unwrap().to_vec();
    let nmod = index
        .iter()
        .find(|f| f.name.to_lowercase() == northstar_package_name.to_lowercase())
        .ok_or_else(|| panic!("Couldn't find Northstar on thunderstore???"))
        .unwrap();

    do_install(nmod, std::path::Path::new(game_path))
        .await
        .unwrap();

    Ok(nmod.version.clone())
}

/// Returns identifier of host OS FlightCore is running on
pub fn get_host_os() -> String {
    env::consts::OS.to_string()
}

pub fn launch_northstar(game_install: GameInstall) -> Result<String, String> {
    dbg!(game_install.clone());

    // Some safety checks before, should have more in the future
    if get_northstar_version_number(game_install.game_path.clone()).is_err() {
        return Err(anyhow!("Not all checks were met").to_string());
    }

    let host_os = get_host_os();

    // Explicetly fail early certain (currently) unsupported install setups
    if host_os != "windows"
        || !(matches!(game_install.install_type, InstallType::STEAM)
            || matches!(game_install.install_type, InstallType::ORIGIN)
            || matches!(game_install.install_type, InstallType::UNKNOWN))
    {
        return Err(format!(
            "Not yet implemented for \"{}\" with Titanfall2 installed via \"{:?}\"",
            get_host_os(),
            game_install.install_type
        ));
    }

    // Switch to Titanfall2 directory for launching
    // NorthstarLauncher.exe expects to be run from that folder
    if std::env::set_current_dir(game_install.game_path.clone()).is_err() {
        // We failed to get to Titanfall2 directory
        return Err(anyhow!("Couldn't access Titanfall2 directory").to_string());
    }

    // Require Origin to be running to launch Northstar
    let origin_is_running = check_origin_running();
    if !origin_is_running {
        return Err(
            anyhow!("Origin not running, start Origin before launching Northstar").to_string(),
        );
    }

    // Only Windows with Steam or Origin are supported at the moment
    if host_os == "windows"
        && (matches!(game_install.install_type, InstallType::STEAM)
            || matches!(game_install.install_type, InstallType::ORIGIN)
            || matches!(game_install.install_type, InstallType::UNKNOWN))
    {
        let _output =
            std::process::Command::new(format!("{}/NorthstarLauncher.exe", game_install.game_path))
                // .args(&["a", "b"])
                .spawn()
                .expect("failed to execute process");
        return Ok("Launched game".to_string());
    }

    Err(format!(
        "Not yet implemented for {:?} on {}",
        game_install.install_type,
        get_host_os()
    ))
}

pub fn check_origin_running() -> bool {
    let s = sysinfo::System::new_all();
    for _process in s.processes_by_name("Origin.exe") {
        // check here if this is your process
        // dbg!(process);
        return true;
    }
    false
}

/// Checks if Northstar process is running
pub fn check_northstar_running() -> bool {
    let s = sysinfo::System::new_all();
    for _process in s.processes_by_name("NorthstarLauncher.exe") {
        // check here if this is your process
        // dbg!(process);
        return true;
    }
    false
}

/// Helps with converting release candidate numbers which are different on Thunderstore
/// due to restrictions imposed by the platform
pub fn convert_release_candidate_number(version_number: String) -> String {
    // This simply converts `-rc` to `0`
    // Works as intended for RCs < 10, e.g.  `v1.9.2-rc1`  -> `v1.9.201`
    // Doesn't work for larger numbers, e.g. `v1.9.2-rc11` -> `v1.9.2011` (should be `v1.9.211`)
    version_number.replace("-rc", "0").replace("00", "")
}

/// Checks if installed FlightCore version is up-to-date
/// false -> FlightCore install is up-to-date
/// true  -> FlightCore install is outdated
pub async fn check_is_flightcore_outdated() -> Result<bool, String> {
    // Get newest version number from GitHub API
    println!("Checking GitHub API");
    let url = "https://api.github.com/repos/GeckoEidechse/FlightCore/releases/latest";
    let user_agent = "GeckoEidechse/FlightCore";
    let client = reqwest::Client::new();
    let res = client
        .get(url)
        .header(reqwest::header::USER_AGENT, user_agent)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let json_response: serde_json::Value =
        serde_json::from_str(&res).expect("JSON was not well-formatted");
    println!("Done checking GitHub API");

    // Extract version number from JSON
    let newest_release_version = json_response
        .get("tag_name")
        .and_then(|value| value.as_str())
        .unwrap();

    // Get version of installed FlightCore...
    let version = env!("CARGO_PKG_VERSION");
    // ...and format it
    let version = format!("v{}", version);

    // TODO: This shouldn't be a string compare but promper semver compare
    Ok(version != newest_release_version)
}

pub fn get_log_list(game_install: GameInstall) -> Result<Vec<std::path::PathBuf>, String> {
    let ns_log_folder = format!("{}/R2Northstar/logs", game_install.game_path);

    // Check if logs folder exists
    if !std::path::Path::new(&ns_log_folder).exists() {
        return Err("No logs folder found".to_string());
    }

    // List files in logs folder
    let paths = std::fs::read_dir(ns_log_folder).unwrap();

    // Stores paths of log files
    let mut log_files: Vec<std::path::PathBuf> = Vec::new();

    for path in paths {
        let path = path.unwrap().path();
        if path.display().to_string().contains("nslog") {
            log_files.push(path);
        }
    }

    if log_files.len() > 0 {
        Ok(log_files)
    } else {
        Err("No logs found".to_string())
    }
}

/// Returns a serde json object of the parsed `enabledmods.json` file
pub fn get_enabled_mods(game_install: GameInstall) -> Result<serde_json::value::Value, String> {
    let enabledmods_json_path = format!(
        "{}/R2Northstar/enabledmods.json",
        game_install.game_path
    );

    // Check for JSON file
    if !std::path::Path::new(&enabledmods_json_path).exists() {
        return Err("enabledmods.json not found".to_string());
    }

    // Read file
    let data = match std::fs::read_to_string(enabledmods_json_path) {
        Ok(data) => data,
        Err(err) => return Err(err.to_string()),
    };

    // Parse JSON
    let res: serde_json::Value = match serde_json::from_str(&data) {
        Ok(result) => result,
        Err(err) => return Err(format!("Failed to read JSON due to: {}", err.to_string())),
    };

    // Return parsed data
    Ok(res)
}

/// Parses `mod.json` for mod name
// TODO: Maybe pass PathBuf or serde json object
fn parse_mod_json_for_mod_name(mod_json_path: String) -> Result<String, anyhow::Error>{
    // Read file into string and parse it
    let data = std::fs::read_to_string(mod_json_path)?;
    let parsed_json: serde_json::Value = serde_json::from_str(&data)?;

    // Extract mod name
    let mod_name = match parsed_json.get("Name").and_then(|value| value.as_str()) {
        Some(name) => name,
        None => return Err(anyhow!("No name found")),
    };

    Ok(mod_name.to_string())
}

/// Parse `mods` folder for installed mods.
fn get_installed_mods(game_install: GameInstall) -> Result<Vec<String>, String> {
    let ns_mods_folder = format!("{}/R2Northstar/mods/", game_install.game_path);

    let paths = std::fs::read_dir(ns_mods_folder).unwrap();

    let mut directories: Vec<PathBuf> = Vec::new();
    let mut mod_names: Vec<String> = Vec::new();

    // Get list of folders in `mods` directory
    for path in paths {
        let my_path = path.unwrap().path();

        let md = std::fs::metadata(my_path.clone()).unwrap();
        if md.is_dir() {
            directories.push(my_path);
        }
    }

    // Iterate over folders and check if they are Northstar mods
    for directory in directories {
        // Check if mod.json exists
        let mod_json_path = format!("{}/mod.json", directory.to_str().unwrap());
        if  !std::path::Path::new(&mod_json_path).exists() {
            continue;
        }

        // Parse mod.json and get mod name
        let mod_name = match parse_mod_json_for_mod_name(mod_json_path.clone()) {
            Ok(mod_name) => mod_name,
            Err(err) => {
                println!("Failed parsing {} with {}", mod_json_path, err.to_string());
                continue;
            },
        };
        mod_names.push(mod_name);
    }

    // Return found mod names
    Ok(mod_names)
}

/// Gets list of installed mods and their properties
/// - name
/// - is enabled?
pub fn get_installed_mods_and_properties(game_install: GameInstall) -> Result<Vec<NorthstarMod>, String> {

    // Get actually installed mods
    let found_installed_mods = get_installed_mods(game_install.clone())?;

    // Get enabled mods as JSON
    let enabled_mods: serde_json::Value = get_enabled_mods(game_install)?;

    let mut installed_mods = Vec::new();
    let mapping = enabled_mods.as_object().unwrap();

    // Use list of installed mods and set enabled based on `enabledmods.json`
    for name in found_installed_mods {
        let current_mod_enabled = match mapping.get(&name) {
            Some(enabled) => enabled.as_bool().unwrap(),
            None => true, // Northstar considers mods not in mapping as enabled.
        };
        let current_mod: NorthstarMod = NorthstarMod {
            name: name,
            enabled: current_mod_enabled,
        };
        installed_mods.push(current_mod);
    }

    Ok(installed_mods)
}
