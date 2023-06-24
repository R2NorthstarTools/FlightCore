/// Windows specific code
use anyhow::{anyhow, Result};

#[cfg(target_os = "windows")]
use winreg::{enums::HKEY_LOCAL_MACHINE, RegKey};

use crate::check_is_valid_game_path;

/// Gets Titanfall2 install location on Origin
pub fn origin_install_location_detection() -> Result<String, anyhow::Error> {
    #[cfg(target_os = "windows")]
    {
        let error;
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        match hklm.open_subkey("SOFTWARE\\Respawn\\Titanfall2") {
            Ok(tf) => match cur_ver.get_value("Install Dir") {
                Ok(install_dir) => match check_is_valid_game_path(install_dir) {
                    Ok(()) => {
                        return Ok(install_dir.to_string());
                    }
                    Err(err) => {
                        error = err;
                    }
                },
                Err(err) => {
                    error = err;
                }
            },
            Err(err) => {
                error = err;
            }
        }
        log::warn!("{}", error);
    }

    Err(anyhow!("No Origin / EA App install path found"))
}
