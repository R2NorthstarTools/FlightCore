use crate::{mod_management::ThunderstoreManifest, GameInstall, NorthstarMod};
use std::{ffi::OsStr, path::{PathBuf, Path}};
use thermite::prelude::ThermiteError;

pub fn installed_plugins_to_mod(
    manifests: &[(ThunderstoreManifest, PathBuf)],
) -> Vec<NorthstarMod> {
    manifests
        .iter()
        .map(|(m, path)| NorthstarMod {
            name: m.name.clone(),
            version: None, // assume None
            thunderstore_mod_string: Some(m.name.clone()),
            enabled: true, // assume it is enabled
            directory: path.display().to_string(),
        })
        .collect()
}

pub fn find_installed_plugins(
    game_install: &GameInstall,
) -> Result<Vec<(ThunderstoreManifest, PathBuf)>, ThermiteError> {
    let plugins_directory = PathBuf::new()
        .join(&game_install.game_path)
        .join("R2Northstar")
        .join("plugins");

    Ok(plugins_directory
        .read_dir()
        .map_err(|_| ThermiteError::MissingFile(Box::new(plugins_directory)))?
        .filter_map(|f| f.ok())
        .map(|e| e.path())
        .filter_map(|p| find_manifest(p.as_path()).or_else(|| find_plugin_in_root(p.as_path())))
        .collect())
}

fn find_plugin_in_root(file: &Path) -> Option<(ThunderstoreManifest, PathBuf)> {
    if file.extension()? == ".dll" {
        Some((
            ThunderstoreManifest {
                name: file.file_name()?.to_str()?.to_string(),
                version_number: "0.0.0".to_string(), // TODO: peak the dll to find it's version
            },
            file.to_owned(),
        ))
    } else {
        None
    }
}

// this can't be async :(
fn find_manifest(dir: &Path) -> Option<(ThunderstoreManifest, PathBuf)> {
    pasre_manifest_path(
        dir.read_dir()
            .ok()?
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|path| path.file_name() == Some(OsStr::new("manifest.json")))
            .last()?,
    )
}

fn pasre_manifest_path(path: PathBuf) -> Option<(ThunderstoreManifest, PathBuf)> {
    Some((
        json5::from_str(&std::fs::read_to_string(&path).ok()?).ok()?,
        path,
    ))
}
