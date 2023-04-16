use app::GameInstall;
use once_cell::sync::Lazy;
use std::{
    fs::{self, File, OpenOptions},
    io,
    path::PathBuf,
};
use tauri::async_runtime::Mutex;
use thermite::{core::utils::TempDir, prelude::ThermiteError};
use zip::ZipArchive;

static CAN_INSTALL_PLUGINS: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));

/// Tries to install plugins from a thunderstore zip
pub async fn install_plugin(
    game_install: &GameInstall,
    zip_file: &File,
) -> Result<(), ThermiteError> {
    if !*CAN_INSTALL_PLUGINS.lock().await {
        return Err(ThermiteError::MiscError("Plugin installing disabled! If you know what you are doing plugins can be enabled in dev settings.".to_string()));
    }

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

    for file in temp_dir
        .join("plugins")
        .read_dir()
        .map_err(|_| ThermiteError::MissingFile(Box::new(temp_dir.join("plugins"))))?
        .filter_map(|f| f.ok()) // ignore any errors
        .filter(|f| f.path().extension().map(|e| e == "dll").unwrap_or(false)) // check for dll extension
        .inspect(|f| {
            _ = fs::remove_file(plugins_directory.join(f.file_name().to_string_lossy().to_string()))
            // try remove plugins to update
        })
    {
        fs::copy(file.path(), plugins_directory.join(file.file_name()))?;
    }

    Ok(())
}

#[tauri::command]
pub async fn toggle_plugin_install() -> bool {
    let mut lock = CAN_INSTALL_PLUGINS.lock().await;
    *lock = !*lock;
    *lock
}
