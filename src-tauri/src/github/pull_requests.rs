use crate::github::release_notes::fetch_github_releases_api;

use anyhow::anyhow;
use app::check_is_valid_game_path;
use app::constants::APP_USER_AGENT;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use ts_rs::TS;

#[derive(Serialize, Deserialize, Debug, Clone, TS)]
#[ts(export)]
struct Repo {
    full_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, TS)]
#[ts(export)]
struct CommitHead {
    sha: String,
    #[serde(rename = "ref")]
    gh_ref: String,
    repo: Repo,
}

#[derive(Serialize, Deserialize, Debug, Clone, TS)]
#[ts(export)]
pub struct PullsApiResponseElement {
    number: i64,
    title: String,
    url: String,
    head: CommitHead,
    html_url: String,
}

// GitHub API response JSON elements as structs
#[derive(Debug, Deserialize, Clone)]
struct WorkflowRun {
    id: u64,
    head_sha: String,
}
#[derive(Debug, Deserialize, Clone)]
struct ActionsRunsResponse {
    workflow_runs: Vec<WorkflowRun>,
}

#[derive(Debug, Deserialize, Clone)]
struct Artifact {
    id: u64,
    workflow_run: WorkflowRun,
}

#[derive(Debug, Deserialize, Clone)]
struct ArtifactsResponse {
    artifacts: Vec<Artifact>,
}

#[derive(Serialize, Deserialize, Debug, Clone, TS)]
#[ts(export)]
pub enum PullRequestType {
    MODS,
    LAUNCHER,
}

/// Parse pull requests from specified URL
pub async fn get_pull_requests(url: String) -> Result<Vec<PullsApiResponseElement>, String> {
    let json_response = match fetch_github_releases_api(&url).await {
        Ok(result) => result,
        Err(err) => return Err(err.to_string()),
    };

    let pulls_response: Vec<PullsApiResponseElement> = match serde_json::from_str(&json_response) {
        Ok(res) => res,
        Err(err) => return Err(err.to_string()),
    };

    Ok(pulls_response)
}

/// Gets either launcher or mods PRs
#[tauri::command]
pub async fn get_pull_requests_wrapper(
    install_type: PullRequestType,
) -> Result<Vec<PullsApiResponseElement>, String> {
    let api_pr_url = match install_type {
        PullRequestType::MODS => "https://api.github.com/repos/R2Northstar/NorthstarMods/pulls",
        PullRequestType::LAUNCHER => {
            "https://api.github.com/repos/R2Northstar/NorthstarLauncher/pulls"
        }
    };

    get_pull_requests(api_pr_url.to_string()).await
}

fn unzip(zip_file_name: &str) -> String {
    let fname = std::path::Path::new(zip_file_name);
    let file = fs::File::open(fname).unwrap();

    let mut archive = zip::ZipArchive::new(file).unwrap();

    let mut folder_name = "".to_string();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {} comment: {}", i, comment);
            }
        }

        if i == 0 {
            // Sanity check that it's a folder
            assert!((*file.name()).ends_with('/'));

            folder_name = format!("{}", outpath.display());
            println!("{}", folder_name);
        }

        if (*file.name()).ends_with('/') {
            fs::create_dir_all(&outpath).unwrap();
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }

        // Get and Set permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }
    folder_name
}

pub async fn check_github_api(url: &str) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let res = client
        .get(url)
        .header(reqwest::header::USER_AGENT, APP_USER_AGENT)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let json: serde_json::Value = serde_json::from_str(&res).expect("JSON was not well-formatted");

    Ok(json)
}

/// Downloads a file from given URL
async fn download_zip(download_url: String, location: String) -> Result<(), anyhow::Error> {
    let client = reqwest::Client::new();
    let resp = client
        .get(download_url)
        .header(reqwest::header::USER_AGENT, APP_USER_AGENT)
        .send()
        .await?;

    // Error out earlier if non-successful response
    if !resp.status().is_success() {
        // Return error cause wrong game path
        return Err(anyhow!(
            "Couldn't download zip. Received error code \"{}\"",
            resp.status()
        ));
    }

    let mut out = fs::File::create(format!("{}/ns-dev-test-helper-temp-pr-files.zip", location))
        .expect("failed to create file");
    let bytes = resp.bytes().await?;
    let mut cursor = std::io::Cursor::new(bytes);
    std::io::copy(&mut cursor, &mut out)?;
    Ok(())
}

fn unzip_launcher_zip(zip_file_name: &str) -> String {
    let outfolder_name = "ns-dev-test-helper-temp-pr-files";
    let fname = std::path::Path::new(zip_file_name);
    let file = fs::File::open(fname).unwrap();

    let mut archive = zip::ZipArchive::new(file).unwrap();

    fs::create_dir_all(outfolder_name).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {} comment: {}", i, comment);
            }
        }

        // Only extract two hardcoded files
        if *file.name() == *"NorthstarLauncher.exe" || *file.name() == *"Northstar.dll" {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.display(),
                file.size()
            );
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p).unwrap();
                }
            }
            let mut outfile =
                fs::File::create(format!("{}/{}", outfolder_name, outpath.display())).unwrap();
            std::io::copy(&mut file, &mut outfile).unwrap();
        }

        // Get and Set permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }
    outfolder_name.to_string()
}

/// Recursively copies files from one directory to another
fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

/// Gets GitHub download link of a mods PR
fn get_mods_download_link(pull_request: PullsApiResponseElement) -> Result<String, anyhow::Error> {
    // {pr object} -> number == pr_number
    //             -> head -> ref
    //                     -> repo -> full_name

    // Use repo and branch name to get download link
    let download_url = format!(
        "https://github.com/{}/archive/refs/heads/{}.zip",
        pull_request.head.repo.full_name, // repo name
        pull_request.head.gh_ref,         // branch name
    );

    Ok(download_url)
}

/// Gets `nightly.link` artifact download link of a launcher PR
async fn get_launcher_download_link(
    pr_number: i64,
    pulls_response: Vec<PullsApiResponseElement>,
) -> Result<String, String> {
    // Crossreference with runs API
    let runs_response: ActionsRunsResponse = match check_github_api(
        "https://api.github.com/repos/R2Northstar/NorthstarLauncher/actions/runs",
    )
    .await
    {
        Ok(result) => serde_json::from_value(result).unwrap(),
        Err(err) => return Err(format!("{}", err)),
    };

    // Get top commit SHA
    for pull_request in pulls_response {
        // Early return if PR number is not the right one
        if pull_request.number != pr_number {
            continue;
        }

        // Cross-reference PR head commit sha against workflow runs
        for workflow_run in &runs_response.workflow_runs {
            // If head commit sha of run and PR match, grab CI output
            if workflow_run.head_sha == pull_request.head.sha {
                // Check artifacts
                let api_url = format!("https://api.github.com/repos/R2Northstar/NorthstarLauncher/actions/runs/{}/artifacts", workflow_run.id);
                let artifacts_response: ArtifactsResponse = serde_json::from_value(
                    check_github_api(&api_url).await.expect("Failed request"),
                )
                .unwrap();

                // Iterate over artifacts
                for artifact in artifacts_response.artifacts {
                    // Make sure run is from PR head commit
                    if artifact.workflow_run.head_sha == workflow_run.head_sha {
                        dbg!(artifact.id);

                        // Download artifact
                        return Ok(format!("https://nightly.link/R2Northstar/NorthstarLauncher/actions/artifacts/{}.zip", artifact.id));
                    }
                }
            }
        }
    }
    Err(format!(
        "Couldn't grab download link for PR \"{}\"",
        pr_number
    ))
}

/// Adds a batch file that allows for launching Northstar with mods PR profile
fn add_batch_file(game_install_path: &str) {
    let batch_path = format!("{}/r2ns-launch-mod-pr-version.bat", game_install_path);
    let path = Path::new(&batch_path);
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    // Write the string to `file`, returns `io::Result<()>`
    let batch_file_content =
        "NorthstarLauncher.exe -profile=R2Northstar-PR-test-managed-folder\r\n";

    match file.write_all(batch_file_content.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}

/// Downloads selected launcher PR and extracts it into game install path
#[tauri::command]
pub async fn apply_launcher_pr(pr_number: i64, game_install_path: &str) -> Result<(), String> {
    // Exit early if wrong game path
    check_is_valid_game_path(game_install_path)?;

    let pulls_response = get_pull_requests_wrapper(PullRequestType::LAUNCHER).await?;

    // get download link
    let download_url = get_launcher_download_link(pr_number, pulls_response).await?;

    // download
    match download_zip(download_url, ".".to_string()).await {
        Ok(_) => (),
        Err(err) => return Err(err.to_string()),
    };

    // extract
    let zip_extract_folder_name = unzip_launcher_zip("ns-dev-test-helper-temp-pr-files.zip");
    fs::remove_file("ns-dev-test-helper-temp-pr-files.zip").unwrap();

    // Copy downloaded folder to game install folder
    match copy_dir_all(zip_extract_folder_name.clone(), game_install_path) {
        Ok(_) => (),
        Err(err) => {
            return Err(format!("Failed copying files: {}", err));
        }
    }

    // Delete old unzipped
    fs::remove_dir_all(zip_extract_folder_name).unwrap();
    Ok(())
}

/// Downloads selected mods PR and extracts it into profile in game install path
#[tauri::command]
pub async fn apply_mods_pr(
    pr_number: PullsApiResponseElement,
    game_install_path: &str,
) -> Result<(), String> {
    // Exit early if wrong game path
    check_is_valid_game_path(game_install_path)?;

    let download_url = match get_mods_download_link(pr_number) {
        Ok(url) => url,
        Err(err) => return Err(err.to_string()),
    };

    match download_zip(download_url, ".".to_string()).await {
        Ok(()) => (),
        Err(err) => return Err(err.to_string()),
    };

    // Extract folder and delete zip
    let zip_extract_folder_name = unzip("ns-dev-test-helper-temp-pr-files.zip");
    fs::remove_file("ns-dev-test-helper-temp-pr-files.zip").unwrap();

    // Delete previously managed folder
    if std::fs::remove_dir_all(format!(
        "{}/R2Northstar-PR-test-managed-folder",
        game_install_path
    ))
    .is_err()
    {
        if std::path::Path::new(&format!(
            "{}/R2Northstar-PR-test-managed-folder",
            game_install_path
        ))
        .exists()
        {
            println!("Failed removing previous dir");
        } else {
            println!("Failed removing folder that doesn't exist. Probably cause first run");
        }
    };

    // Copy downloaded folder to game install folder
    copy_dir_all(
        zip_extract_folder_name.clone(),
        format!(
            "{}/R2Northstar-PR-test-managed-folder/mods",
            game_install_path
        ),
    )
    .unwrap();

    // Delete old copy
    std::fs::remove_dir_all(zip_extract_folder_name).unwrap();

    // Add batch file to launch right profile
    add_batch_file(game_install_path);

    Ok(())
}
