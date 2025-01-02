/// Windows specific code
use anyhow::{anyhow, Result};

#[cfg(target_os = "windows")]
use winreg::{enums::HKEY_LOCAL_MACHINE, RegKey};

use crate::repair_and_verify::check_is_valid_game_path;

/// Gets Titanfall2 install location on Origin
pub fn origin_install_location_detection() -> Result<String, anyhow::Error> {
    #[cfg(target_os = "windows")]
    {
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        match hklm.open_subkey("SOFTWARE\\Respawn\\Titanfall2") {
            Ok(tf) => {
                let game_path_str: String = tf.get_value("Install Dir")?;

                match check_is_valid_game_path(&game_path_str) {
                    Ok(()) => {
                        return Ok(game_path_str.to_string());
                    }
                    Err(err) => {
                        log::warn!("{err}");
                    }
                }
            }
            Err(err) => {
                log::warn!("{err}");
            }
        }
    }

    Err(anyhow!("No Origin / EA App install path found"))
}
