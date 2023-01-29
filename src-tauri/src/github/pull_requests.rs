use crate::github::release_notes::fetch_github_releases_api;

use anyhow::anyhow;
use app::constants::APP_USER_AGENT;
use serde::{Deserialize, Serialize};
use std::fs;
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

/// Gets launcher PRs
#[tauri::command]
pub async fn get_launcher_prs() -> Result<Vec<PullsApiResponseElement>, String> {
    let json_response = match fetch_github_releases_api(
        "https://api.github.com/repos/R2Northstar/NorthstarLauncher/pulls",
    )
    .await
    {
        Ok(result) => result,
        Err(err) => return Err(err.to_string()),
    };

    let pulls_response: Vec<PullsApiResponseElement> = match serde_json::from_str(&json_response) {
        Ok(res) => res,
        Err(err) => return Err(err.to_string()),
    };

    Ok(pulls_response)
}

/// Gets mod PRs
#[tauri::command]
pub async fn get_mods_prs() -> Result<Vec<PullsApiResponseElement>, String> {
    let json_response = match fetch_github_releases_api(
        "https://api.github.com/repos/R2Northstar/NorthstarMods/pulls",
    )
    .await
    {
        Ok(result) => result,
        Err(err) => return Err(err.to_string()),
    };

    let pulls_response: Vec<PullsApiResponseElement> = match serde_json::from_str(&json_response) {
        Ok(res) => res,
        Err(err) => return Err(err.to_string()),
    };

    Ok(pulls_response)
}

pub async fn check_github_api(url: &str) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    println!("Checking GitHub API");

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
    println!("Done checking GitHub API");

    Ok(json)
}

/// Downloads a file from given URL
async fn download_zip(download_url: String, location: String) -> Result<(), anyhow::Error> {
    println!("Downloading file");
    let client = reqwest::Client::new();
    let resp = client
        .get(download_url)
        .header(reqwest::header::USER_AGENT, APP_USER_AGENT)
        .send()
        .await?;

    // Error out earlier if non-successful response
    if !resp.status().is_success() {
        println!("Status: {}", resp.status());
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
    println!("Download done");
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
fn copy_dir_all(
    src: impl AsRef<std::path::Path>,
    dst: impl AsRef<std::path::Path>,
) -> std::io::Result<()> {
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
                println!("Checking: {}", api_url);
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

/// Downloads selected launcher PR and extracts it into game install path
#[tauri::command]
pub async fn apply_launcher_pr(pr_number: i64, game_install_path: &str) -> Result<(), String> {
    println!("{}", pr_number);
    println!("{}", game_install_path);

    // Exit early if wrong game path
    // check_game_path(game_install_path)?;

    let pulls_response = get_launcher_prs().await?;

    // get download link
    let download_url = get_launcher_download_link(pr_number, pulls_response).await?;

    println!("{}", download_url);

    // download
    match download_zip(download_url, ".".to_string()).await {
        Ok(_) => (),
        Err(err) => return Err(err.to_string()),
    };

    // extract
    let zip_extract_folder_name = unzip_launcher_zip("ns-dev-test-helper-temp-pr-files.zip");

    println!("Zip extract done");

    println!("Deleting temp zip download folder");

    fs::remove_file("ns-dev-test-helper-temp-pr-files.zip").unwrap();

    // Copy downloaded folder to game install folder
    match copy_dir_all(zip_extract_folder_name.clone(), game_install_path) {
        Ok(_) => (),
        Err(err) => {
            return Err(format!("Failed copying files: {}", err));
        }
    }

    println!("Deleting old unzipped folder");

    // Delete old copy
    fs::remove_dir_all(zip_extract_folder_name).unwrap();

    println!("All done :D");

    Ok(())
}
