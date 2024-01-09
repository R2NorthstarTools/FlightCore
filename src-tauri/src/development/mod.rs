use crate::constants::NS_LAUNCHER_COMMITS_API_URL;
use crate::github::{
    pull_requests::{check_github_api, download_zip_into_memory, get_launcher_download_link},
    CommitInfo,
};

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
        "{}/___flightcore-temp/download-dir/launcher-pr-{}",
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
