use crate::constants::{APP_USER_AGENT, NORTHSTAR_LAUNCHER_REPO_NAME, NORTHSTAR_MODS_REPO_NAME};
use crate::repair_and_verify::check_is_valid_game_path;
use crate::GameInstall;
use anyhow::anyhow;
use serde::{Deserialize, Serialize};
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
    number: u64,
    title: String,
    url: String,
    head: CommitHead,
    html_url: String,
    labels: Vec<String>,
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
    name: String,
    workflow_run: WorkflowRun,
}

#[derive(Debug, Deserialize, Clone)]
struct ArtifactsResponse {
    artifacts: Vec<Artifact>,
}

#[derive(Serialize, Deserialize, Debug, Clone, TS)]
#[ts(export)]
pub enum PullRequestType {
    Mods,
    Launcher,
}

/// Parse pull requests from specified URL
pub async fn get_pull_requests(
    repo: PullRequestType,
) -> Result<Vec<PullsApiResponseElement>, anyhow::Error> {
    let repo = match repo {
        PullRequestType::Mods => NORTHSTAR_MODS_REPO_NAME,
        PullRequestType::Launcher => NORTHSTAR_LAUNCHER_REPO_NAME,
    };

    // Grab list of PRs
    let octocrab = octocrab::instance();
    let page = octocrab
        .pulls("R2Northstar", repo)
        .list()
        .state(octocrab::params::State::Open)
        .per_page(50) // Only grab 50 PRs
        .page(1u32)
        .send()
        .await?;

    // Iterate over pull request elements and insert into struct
    let mut all_pull_requests: Vec<PullsApiResponseElement> = vec![];
    for item in page.items {
        let repo = Repo {
            full_name: item
                .head
                .repo
                .ok_or(anyhow!("repo not found"))?
                .full_name
                .ok_or(anyhow!("full_name not found"))?,
        };

        let head = CommitHead {
            sha: item.head.sha,
            gh_ref: item.head.ref_field,
            repo,
        };

        // Get labels and their names and put the into vector
        let label_names: Vec<String> = item
            .labels
            .unwrap_or_else(Vec::new)
            .into_iter()
            .map(|label| label.name)
            .collect();

        // TODO there's probably a way to automatically serialize into the struct but I don't know yet how to
        let elem = PullsApiResponseElement {
            number: item.number,
            title: item.title.ok_or(anyhow!("title not found"))?,
            url: item.url,
            head,
            html_url: item
                .html_url
                .ok_or(anyhow!("html_url not found"))?
                .to_string(),
            labels: label_names,
        };

        all_pull_requests.push(elem);
    }

    Ok(all_pull_requests)
}

/// Gets either launcher or mods PRs
#[tauri::command]
pub async fn get_pull_requests_wrapper(
    install_type: PullRequestType,
) -> Result<Vec<PullsApiResponseElement>, String> {
    match get_pull_requests(install_type).await {
        Ok(res) => Ok(res),
        Err(err) => Err(err.to_string()),
    }
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

/// Downloads a file from given URL into an array in memory
pub async fn download_zip_into_memory(download_url: String) -> Result<Vec<u8>, anyhow::Error> {
    let client = reqwest::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()?;

    let response = client.get(download_url).send().await?;

    if !response.status().is_success() {
        return Err(anyhow!("Request unsuccessful: {}", response.status()));
    }

    let bytes = response.bytes().await?;
    Ok(bytes.to_vec())
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

/// Gets `nightly.link` artifact download link of a launcher commit
#[tauri::command]
pub async fn get_launcher_download_link(commit_sha: String) -> Result<String, String> {
    // Iterate over the first 10 pages of
    for i in 1..=10 {
        // Crossreference with runs API
        let runs_response: ActionsRunsResponse = match check_github_api(&format!(
            "https://api.github.com/repos/R2Northstar/NorthstarLauncher/actions/runs?page={}",
            i
        ))
        .await
        {
            Ok(result) => serde_json::from_value(result).unwrap(),
            Err(err) => return Err(format!("{}", err)),
        };

        // Cross-reference commit sha against workflow runs
        for workflow_run in &runs_response.workflow_runs {
            // If head commit sha of CI run matches the one passed to this function, grab CI output
            if workflow_run.head_sha == commit_sha {
                // Check artifacts
                let api_url = format!("https://api.github.com/repos/R2Northstar/NorthstarLauncher/actions/runs/{}/artifacts", workflow_run.id);
                let artifacts_response: ArtifactsResponse = serde_json::from_value(
                    check_github_api(&api_url).await.expect("Failed request"),
                )
                .unwrap();

                let multiple_artifacts = artifacts_response.artifacts.len() > 1;

                // Iterate over artifacts
                for artifact in artifacts_response.artifacts {
                    if multiple_artifacts && !artifact.name.starts_with("NorthstarLauncher-MSVC") {
                        continue;
                    }

                    // Make sure artifact and CI run commit head sha match
                    if artifact.workflow_run.head_sha == workflow_run.head_sha {
                        // Download artifact
                        return Ok(format!("https://nightly.link/R2Northstar/NorthstarLauncher/actions/artifacts/{}.zip", artifact.id));
                    }
                }
            }
        }
    }

    Err(format!(
        "Couldn't grab download link for \"{}\". Corresponding PR might be too old and therefore no CI build has been detected. Maybe ask author to update?",
        commit_sha
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
        Ok(_) => log::info!("successfully wrote to {}", display),
    }
}

/// Downloads selected launcher PR and extracts it into game install path
#[tauri::command]
pub async fn apply_launcher_pr(
    pull_request: PullsApiResponseElement,
    game_install: GameInstall,
) -> Result<(), String> {
    // Exit early if wrong game path
    check_is_valid_game_path(&game_install.game_path)?;

    // get download link
    let download_url = match get_launcher_download_link(pull_request.head.sha.clone()).await {
        Ok(res) => res,
        Err(err) => {
            return Err(format!(
                "Couldn't grab download link for PR \"{}\". {}",
                pull_request.number, err
            ))
        }
    };

    let archive = match download_zip_into_memory(download_url).await {
        Ok(archive) => archive,
        Err(err) => return Err(err.to_string()),
    };

    let extract_directory = format!(
        "{}/___flightcore-temp/download-dir/launcher-pr-{}",
        game_install.game_path, pull_request.number
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
    match zip_extract::extract(io::Cursor::new(archive), &target_dir, true) {
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
        let destination_file_path = format!("{}/{}", game_install.game_path, file_name);
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

    log::info!("All done with installing launcher PR");
    Ok(())
}

/// Downloads selected mods PR and extracts it into profile in game install path
#[tauri::command]
pub async fn apply_mods_pr(
    pull_request: PullsApiResponseElement,
    game_install: GameInstall,
) -> Result<(), String> {
    // Exit early if wrong game path
    check_is_valid_game_path(&game_install.game_path)?;

    let download_url = match get_mods_download_link(pull_request) {
        Ok(url) => url,
        Err(err) => return Err(err.to_string()),
    };

    let archive = match download_zip_into_memory(download_url).await {
        Ok(archive) => archive,
        Err(err) => return Err(err.to_string()),
    };

    let profile_folder = format!(
        "{}/R2Northstar-PR-test-managed-folder",
        game_install.game_path
    );

    // Delete previously managed folder
    if std::fs::remove_dir_all(profile_folder.clone()).is_err() {
        if std::path::Path::new(&profile_folder).exists() {
            log::error!("Failed removing previous dir");
        } else {
            log::warn!("Failed removing folder that doesn't exist. Probably cause first run");
        }
    };

    // Create profile folder
    match std::fs::create_dir_all(profile_folder.clone()) {
        Ok(()) => (),
        Err(err) => return Err(err.to_string()),
    }

    let target_dir = std::path::PathBuf::from(format!("{}/mods", profile_folder)); // Doesn't need to exist
    match zip_extract::extract(io::Cursor::new(archive), &target_dir, true) {
        Ok(()) => (),
        Err(err) => {
            return Err(format!("Failed unzip: {}", err));
        }
    };
    // Add batch file to launch right profile
    add_batch_file(&game_install.game_path);

    log::info!("All done with installing mods PR");
    Ok(())
}
