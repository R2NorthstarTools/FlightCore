/// Windows specific code
use anyhow::{anyhow, Result};
use std::net::Ipv4Addr;

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

/// Check whether the current device might be behind a CGNAT
pub async fn check_cgnat() -> Result<String, String> {
    // Use external service to grap IP
    let url = "https://api.ipify.org";
    let response = reqwest::get(url).await.unwrap().text().await.unwrap();

    // Check if valid IPv4 address and return early if not
    if response.parse::<Ipv4Addr>().is_err() {
        return Err(format!("Not valid IPv4 address: {}", response));
    }

    let hops_count = run_tracert(&response)?;
    Ok(format!("Counted {} hops to {}", hops_count, response))
}

/// Count number of hops in tracert output
fn count_hops(output: &str) -> usize {
    // Split the output into lines
    let lines: Vec<&str> = output.lines().collect();

    // Filter lines that appear to represent hops
    let hop_lines: Vec<&str> = lines
        .iter()
        .filter(|&line| line.contains("ms") || line.contains("*")) // TODO check if it contains just the `ms` surrounded by whitespace, otherwise it might falsely pick up some domain names as well
        .cloned()
        .collect();

    // Return the number of hops
    hop_lines.len()
}

/// Run `tracert`
fn run_tracert(target_ip: &str) -> Result<usize, String> {
    // Ensure valid IPv4 address to avoid prevent command injection
    assert!(target_ip.parse::<Ipv4Addr>().is_ok());

    // Execute the `tracert` command
    let output = match std::process::Command::new("tracert")
        .arg("-4") // Force IPv4
        .arg("-d") // Prevent resolving intermediate IP addresses
        .arg("-w") // Set timeout to 1 second
        .arg("1000")
        .arg("-h") // Set max hop count
        .arg("5")
        .arg(target_ip)
        .output()
    {
        Ok(res) => res,
        Err(err) => return Err(format!("Failed running tracert: {}", err)),
    };

    // Check if the command was successful
    if output.status.success() {
        // Convert the output to a string
        let stdout =
            std::str::from_utf8(&output.stdout).expect("Invalid UTF-8 sequence in command output");
        println!("{}", stdout);

        // Count the number of hops
        let hop_count = count_hops(stdout);
        Ok(hop_count)
    } else {
        let stderr =
            std::str::from_utf8(&output.stderr).expect("Invalid UTF-8 sequence in command error output");
        println!("{}", stderr);
        Err(format!("Failed collecting tracert output: {}", stderr))
    }
}
