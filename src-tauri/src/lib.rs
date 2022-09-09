use anyhow::{anyhow, Context, Result};
use zip::ZipArchive;

#[derive(Debug)]
pub enum InstallType {
    STEAM,
    ORIGIN,
    EAPLAY,
    UNKNOWN,
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

/// Attempts to find the game install location
pub fn find_game_install_location() -> Result<(String, InstallType), anyhow::Error> {
    // Attempt parsing Steam library directly
    match steamlocate::SteamDir::locate() {
        Some(mut steamdir) => {
            let titanfall2_steamid = 1237970;
            match steamdir.app(&titanfall2_steamid) {
                Some(app) => {
                    // println!("{:#?}", app);
                    return Ok((app.path.to_str().unwrap().to_string(), InstallType::STEAM));
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
    // TODO: if `find_game_install_location` is unable to find game_path then function will fail to detect
    // Northstar install, even if game_path is known due to user entering it manually
    let (install_location, install_type) = match find_game_install_location() {
        Ok((path, install_type)) => (path, install_type),
        Err(err) => return Err(err),
    };

    println!("{}", install_location);
    println!("{:?}", install_type);

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

/// Checks whether the provided path is a valid Titanfall2 gamepath by checking against a certain set of criteria
pub fn check_is_valid_game_path(game_install_path: &str) -> Result<(), anyhow::Error> {
    let is_correct_game_path =
        std::path::Path::new(&format!("{}/Titanfall2.exe", game_install_path)).exists();
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
    let download_directory = format!("{}/flightcore-temp-download-dir/", game_path.display());

    std::fs::create_dir_all(download_directory.clone())?;


    let download_path = format!("{}/{}", download_directory, filename);
    println!("{}", download_path);

    let nfile = thermite::core::actions::download_file(
        &nmod.url,
        download_path,
    )
    .await
    .unwrap();

    println!("Extracting Northstar...");
    extract(nfile, game_path)?;
    println!("Done!");

    Ok(())
}

pub async fn install_northstar(game_path: &str) -> Result<String> {
    let index = thermite::api::get_package_index().await.unwrap().to_vec();
    let nmod = index
        .iter()
        .find(|f| f.name.to_lowercase() == "northstar")
        .ok_or_else(|| panic!("Couldn't find Northstar on thunderstore???"))
        .unwrap();

    do_install(nmod, std::path::Path::new(game_path))
        .await
        .unwrap();

    Ok(nmod.version.clone())
}
