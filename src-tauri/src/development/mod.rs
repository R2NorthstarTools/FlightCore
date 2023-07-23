use crate::constants::NS_LAUNCHER_COMMITS_API_URL;
use crate::github::{
    pull_requests::{check_github_api, download_zip_into_memory, get_launcher_download_link},
    CommitInfo,
};
use crate::GameInstall;
use serde::{Deserialize, Serialize};
use std::io::Read;

#[tauri::command]
pub async fn install_git_main(game_install_path: &str) -> Result<String, String> {
    // Get list of commits
    let commits: Vec<CommitInfo> = serde_json::from_value(
        check_github_api(NS_LAUNCHER_COMMITS_API_URL)
            .await
            .expect("Failed request"),
    )
    .unwrap();

    // Get latest commit...
    let latest_commit_sha = commits[0].sha.clone();
    // ...and according artifact download URL
    let download_url = get_launcher_download_link(latest_commit_sha.clone()).await?;

    let archive = match download_zip_into_memory(download_url).await {
        Ok(archive) => archive,
        Err(err) => return Err(err.to_string()),
    };

    let extract_directory = format!(
        "{}/___flightcore-temp-download-dir/launcher-pr-{}",
        game_install_path, latest_commit_sha
    );
    match std::fs::create_dir_all(extract_directory.clone()) {
        Ok(_) => (),
        Err(err) => {
            return Err(format!(
                "Failed creating temporary download directory: {}",
                err
            ))
        }
    };

    let target_dir = std::path::PathBuf::from(extract_directory.clone()); // Doesn't need to exist
    match zip_extract::extract(std::io::Cursor::new(archive), &target_dir, true) {
        Ok(()) => (),
        Err(err) => {
            return Err(format!("Failed unzip: {}", err));
        }
    };

    // Copy only necessary files from temp dir
    // Copy:
    // - NorthstarLauncher.exe
    // - Northstar.dll
    let files_to_copy = vec!["NorthstarLauncher.exe", "Northstar.dll"];
    for file_name in files_to_copy {
        let source_file_path = format!("{}/{}", extract_directory, file_name);
        let destination_file_path = format!("{}/{}", game_install_path, file_name);
        match std::fs::copy(source_file_path, destination_file_path) {
            Ok(_result) => (),
            Err(err) => {
                return Err(format!(
                    "Failed to copy necessary file {} from temp dir: {}",
                    file_name, err
                ))
            }
        };
    }

    // delete extract directory
    match std::fs::remove_dir_all(&extract_directory) {
        Ok(()) => (),
        Err(err) => {
            return Err(format!(
                "Failed to delete temporary download directory: {}",
                err
            ))
        }
    }

    log::info!(
        "All done with installing launcher from {}",
        latest_commit_sha
    );
    Ok(latest_commit_sha)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Checksum {
    path: String,
    checksum: String,
}

impl ToString for Checksum {
    fn to_string(&self) -> String {
        format!("{} {}", self.checksum, self.path)
    }
}

/// Computes the checksum of a given file
fn compute_checksum<P: AsRef<std::path::Path>>(
    path: P,
) -> Result<Checksum, Box<dyn std::error::Error>> {
    dbg!(path.as_ref().to_string_lossy().into_owned());
    let mut file = std::fs::File::open(&path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let checksum = crypto_hash::hex_digest(crypto_hash::Algorithm::SHA256, &buffer);
    Ok(Checksum {
        path: path.as_ref().to_string_lossy().into_owned(),
        checksum,
    })
}

fn convert_to_string(checksums: Vec<Checksum>) -> String {
    let mut result = String::new();

    for entry in checksums {
        result.push_str(&entry.to_string());
        result.push('\n');
    }
    result
}

#[tauri::command]
/// Calculates checksums over the passed game_install folder and returns results
pub async fn calculate_checksums_gameinstall(game_install: GameInstall) -> Result<String, String> {
    log::info!("Computing checksums");

    let path = game_install.game_path;
    let mut checksums = Vec::new();

    // Iterate over folder
    for entry in walkdir::WalkDir::new(path.clone()) {
        let entry = entry.unwrap();
        if !entry.file_type().is_file() {
            continue;
        }

        match compute_checksum(entry.path()) {
            Ok(mut checksum) => {
                checksum.path = checksum
                    .path
                    .strip_prefix(&path.clone())
                    .unwrap()
                    .to_string();
                checksums.push(checksum)
            }
            Err(err) => log::warn!("Failed to compute checksum for {:?}: {:?}", entry, err),
        }
    }

    for checksum in &checksums {
        println!("{:?}", checksum);
    }

    log::info!("Done calculating");
    let s = convert_to_string(checksums);
    Ok(s)
}
