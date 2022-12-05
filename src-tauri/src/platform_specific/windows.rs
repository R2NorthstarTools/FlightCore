/// Windows specific code
use anyhow::{anyhow, Result};

use crate::check_is_valid_game_path;

const TITANFALL2_ORIGIN_IDS: [&str; 2] = ["Origin.OFR.50.0001452", "Origin.OFR.50.0001456"];

/// Runs a powershell command and parses output to get Titanfall2 install location on Origin
pub fn origin_install_location_detection() -> Result<String, anyhow::Error> {
    // Iterate over known Titanfall2 Origin IDs
    for origin_id in TITANFALL2_ORIGIN_IDS {
        match game_scanner::origin::find(origin_id) {
            // Origin ID found as installed game
            Ok(game) => {
                if game.path.is_some() {
                    let game_path = game.path.unwrap();
                    let game_path_str = game_path.to_str().unwrap();
                    match check_is_valid_game_path(game_path_str) {
                        Ok(()) => {
                            return Ok(game_path_str.to_string());
                        }
                        Err(err) => {
                            println!("{}", err.to_string());
                            continue; // Not a valid game path
                        }
                    }
                }
            }
            Err(err) => {
                println!("Couldn't find {origin_id}: {err}")
            }
        }
    }

    Err(anyhow!("No Origin install path found"))
}
