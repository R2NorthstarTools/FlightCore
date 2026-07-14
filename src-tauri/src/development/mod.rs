use std::io::{self, Write};

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

    // Use a temp file to store archive
    let mut tmpfile = tempfile::tempfile().unwrap();
    match tmpfile.write_all(&archive) {
        Ok(_) => (),
        Err(err) => return Err(err.to_string()),
    }
    let mut zip = zip::ZipArchive::new(tmpfile).unwrap();

    // Copy only necessary files from temp dir
    // Copy:
    // - NorthstarLauncher.exe
    // - Northstar.dll
    let files_to_copy = vec!["NorthstarLauncher.exe", "Northstar.dll"];
    for file_name in files_to_copy {
        let mut zip_file = match zip.by_name(file_name) {
            Ok(file) => file,
            Err(err) => return Err(err.to_string()),
        };
        let destination_file_path = format!("{}/{}", game_install_path, file_name);
        let mut file = match std::fs::File::create(std::path::Path::new(&destination_file_path)) {
            Ok(f) => f,
            Err(err) => return Err(err.to_string()),
        };
        match io::copy(&mut zip_file, &mut file) {
            Ok(_) => (),
            Err(err) => return Err(err.to_string()),
        };
    }

    log::info!(
        "All done with installing launcher from {}",
        latest_commit_sha
    );
    Ok(latest_commit_sha)
}
