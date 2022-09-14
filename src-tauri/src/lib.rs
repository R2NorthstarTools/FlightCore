use std::env;

use anyhow::{anyhow, Context, Result};
use powershell_script::PsScriptBuilder;
use regex::Regex;
use serde::{Deserialize, Serialize};
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

#[cfg(target_os = "windows")]
/// Runs a powershell command and parses output to get Titanfall2 install location on Origin
fn windows_origin_install_location_detection() -> Result<String, anyhow::Error> {
    dbg!();

    // Run PowerShell command to get Titanfall2 Origin install path
    let ps = PsScriptBuilder::new()
        .no_profile(true)
        .non_interactive(true)
        .hidden(false)
        .print_commands(false)
        .build();
    let output = ps.run(r#"Get-ItemProperty -Path Registry::HKEY_LOCAL_MACHINE\SOFTWARE\Respawn\Titanfall2\ -Name "Install Dir""#).unwrap();

    // Get command output as string
    let string = output.stdout().unwrap();

    // Regex the result out and return value accordingly
    let regex = Regex::new(r"(?m)Install Dir.+: (.+)\r\n").unwrap();
    let mut result = regex.captures_iter(&string);
    match result.next() {
        Some(mat) => {
            let game_path = mat.get(1).map_or("", |m| m.as_str());
            println!("{}", game_path);
            match check_is_valid_game_path(game_path) {
                Ok(()) => return Ok(game_path.to_owned()),
                Err(err) => Err(err),
            }
        }
        None => Err(anyhow!("No Origin install path found")),
    }
}

/// Attempts to find the game install location
pub fn find_game_install_location() -> Result<GameInstall, anyhow::Error> {
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
    match windows_origin_install_location_detection() {
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

    Err(anyhow!(
        "Could not auto-detect game install location! Please enter it manually."
    ))
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

pub async fn install_northstar(game_path: &str) -> Result<String> {
    let northstar_package_name = "Northstar".to_lowercase();

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
            || matches!(game_install.install_type, InstallType::ORIGIN))
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
            || matches!(game_install.install_type, InstallType::ORIGIN))
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

use sysinfo::{System, SystemExt};
pub fn check_origin_running() -> bool {
    let s = System::new_all();
    for _process in s.processes_by_name("Origin.exe") {
        // check here if this is your process
        // dbg!(process);
        return true;
    }
    false
}
