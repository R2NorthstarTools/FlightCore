//! This module contains various utility/helper functions that do not fit into any other module

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use sysinfo::{ProcessExt, SystemExt};
use zip::ZipArchive;

use crate::constants::{APP_USER_AGENT, MASTER_SERVER_URL, SERVER_BROWSER_ENDPOINT};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NorthstarServer {
    #[serde(rename = "playerCount")]
    pub player_count: i32,
}

/// This function's only use is to force a `panic!()`
// This must NOT be async to ensure crashing whole application.
#[tauri::command]
pub fn force_panic() {
    panic!("Force panicked!");
}

/// Returns true if built in debug mode
#[tauri::command]
pub async fn is_debug_mode() -> bool {
    cfg!(debug_assertions)
}

/// Fetches `/client/servers` endpoint from master server
async fn fetch_server_list() -> Result<String, anyhow::Error> {
    let url = format!("{MASTER_SERVER_URL}{SERVER_BROWSER_ENDPOINT}");
    let client = reqwest::Client::new();
    let res = client
        .get(url)
        .header(reqwest::header::USER_AGENT, APP_USER_AGENT)
        .send()
        .await?
        .text()
        .await?;

    Ok(res)
}

/// Gets server and playercount from master server API
#[tauri::command]
pub async fn get_server_player_count() -> Result<(i32, usize), String> {
    let res = match fetch_server_list().await {
        Ok(res) => res,
        Err(err) => return Err(err.to_string()),
    };

    let ns_servers: Vec<NorthstarServer> =
        serde_json::from_str(&res).expect("JSON was not well-formatted");

    // Get server count
    let server_count = ns_servers.len();

    // Sum up player count
    let total_player_count: i32 = ns_servers.iter().map(|server| server.player_count).sum();

    log::info!("total_player_count: {}", total_player_count);
    log::info!("server_count:       {}", server_count);

    Ok((total_player_count, server_count))
}

#[tauri::command]
pub async fn kill_northstar() -> Result<(), String> {
    if !check_northstar_running() {
        return Err("Northstar is not running".to_string());
    }

    let s = sysinfo::System::new_all();

    for process in s.processes_by_exact_name("Titanfall2.exe") {
        log::info!("Killing Process {}", process.pid());
        process.kill();
    }

    for process in s.processes_by_exact_name("NorthstarLauncher.exe") {
        log::info!("Killing Process {}", process.pid());
        process.kill();
    }

    Ok(())
}

/// Copied from `papa` source code and modified
///Extract N* zip file to target game path
// fn extract(ctx: &Ctx, zip_file: File, target: &Path) -> Result<()> {
pub fn extract(zip_file: std::fs::File, target: &std::path::Path) -> Result<()> {
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
                log::info!("Create directory {}", f.name());
                std::fs::create_dir_all(target.join(f.name()))
                    .context("Unable to create directory")?;
                continue;
            } else if let Some(p) = out.parent() {
                std::fs::create_dir_all(p).context("Unable to create directory")?;
            }

            let mut outfile = std::fs::OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(&out)?;

            log::info!("Write file {}", out.display());

            std::io::copy(&mut f, &mut outfile).context("Unable to write to file")?;
        }
    }

    Ok(())
}

pub fn check_ea_app_or_origin_running() -> bool {
    let s = sysinfo::System::new_all();
    let x = s.processes_by_name("Origin.exe").next().is_some()
        || s.processes_by_name("EADesktop.exe").next().is_some();
    x
}

/// Checks if Northstar process is running
pub fn check_northstar_running() -> bool {
    let s = sysinfo::System::new_all();
    let x = s
        .processes_by_name("NorthstarLauncher.exe")
        .next()
        .is_some()
        || s.processes_by_name("Titanfall2.exe").next().is_some();
    x
}

#[allow(dead_code)]
pub fn copy_dir_all(
    src: impl AsRef<std::path::Path>,
    dst: impl AsRef<std::path::Path>,
) -> std::io::Result<()> {
    std::fs::create_dir_all(&dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            std::fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

pub fn move_dir_all(
    src: impl AsRef<std::path::Path>,
    dst: impl AsRef<std::path::Path>,
) -> std::io::Result<()> {
    std::fs::create_dir_all(&dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            move_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
            std::fs::remove_dir(entry.path())?;
        } else {
            std::fs::rename(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}
