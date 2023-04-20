use app::GameInstall;
use once_cell::sync::OnceCell;
use std::{
    fs::{self, File, OpenOptions},
    io,
    path::PathBuf,
};
use tauri::{
    async_runtime::{block_on, channel, Mutex, Receiver, Sender},
    Manager, State,
};
use thermite::{core::utils::TempDir, prelude::ThermiteError};
use zip::ZipArchive;

use crate::APP_HANDLE;

static INSTALL_STATUS_RECV: OnceCell<Mutex<Receiver<bool>>> = OnceCell::new();

pub struct InstallStatusSender(Mutex<Sender<bool>>);

impl InstallStatusSender {
    pub fn new() -> Self {
        let (send, recv) = channel(1);

        INSTALL_STATUS_RECV
            .set(Mutex::new(recv))
            .expect("failed to set INSTALL_STATUS_RECV");

        Self(Mutex::new(send))
    }
}

/// Tries to install plugins from a thunderstore zip
pub async fn install_plugin(
    game_install: &GameInstall,
    zip_file: &File,
    can_install_plugins: bool,
) -> Result<(), ThermiteError> {
    let plugins_directory = PathBuf::new()
        .join(&game_install.game_path)
        .join("R2Northstar")
        .join("plugins");
    let temp_dir = TempDir::create(plugins_directory.join("___flightcore-temp-plugin-dir"))?;
    let mut archive = ZipArchive::new(zip_file)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;

        if file.enclosed_name().is_none() || file.enclosed_name().unwrap().starts_with(".") {
            continue;
        }

        let out = temp_dir.join(file.enclosed_name().unwrap());

        if (*file.name()).ends_with('/') {
            fs::create_dir_all(&out)?;
            continue;
        } else if let Some(p) = out.parent() {
            fs::create_dir_all(p)?;
        }
        let mut outfile = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&out)?;
        io::copy(&mut file, &mut outfile)?;
    }

    let plugins: Vec<fs::DirEntry> = temp_dir
        .join("plugins")
        .read_dir()
        .map_err(|_| ThermiteError::MissingFile(Box::new(temp_dir.join("plugins"))))?
        .filter_map(|f| f.ok()) // ignore any errors
        .filter(|f| f.path().extension().map(|e| e == "dll").unwrap_or(false)) // check for dll extension
        .collect();

    // warn user
    if !plugins.is_empty() {
        
        // check here instead if we can install plugins so people don't get broken mods without plugins
        if !can_install_plugins {
            Err(ThermiteError::MiscError(
                "plugin installing is disabled; this mod contains a plugin; plugin can be enabled in the dev menu".to_string(),
            ))?
        }

        APP_HANDLE
            .wait()
            .emit_all("display-plugin-warning", ())
            .map_err(|err| ThermiteError::MiscError(err.to_string()))?;

        if !INSTALL_STATUS_RECV
            .wait()
            .lock()
            .await
            .recv()
            .await
            .unwrap_or(false)
        {
            Err(ThermiteError::MiscError(
                "user denided plugin installing".to_string(),
            ))?
        }
    } else {
        Err(ThermiteError::MissingFile(Box::new(
            temp_dir.join("plugins/anyplugins.dll"),
        )))?;
    }

    for file in plugins.iter().inspect(|f| {
        _ = fs::remove_file(plugins_directory.join(f.file_name().to_string_lossy().to_string()))
        // try remove plugins to update
    }) {
        fs::copy(file.path(), plugins_directory.join(file.file_name()))?;
    }

    Ok(())
}

#[tauri::command]
pub fn receive_install_status(
    sender: State<'_, InstallStatusSender>,
    comfirmed_install: bool,
) -> Result<(), String> {
    block_on(async { sender.0.lock().await.send(comfirmed_install).await })
        .map_err(|err| err.to_string())
}
