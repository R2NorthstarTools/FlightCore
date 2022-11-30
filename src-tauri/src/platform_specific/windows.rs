/// Windows specific code
use powershell_script::PsScriptBuilder;
use regex::Regex;

use anyhow::{anyhow, Result};

use crate::check_is_valid_game_path;

/// Runs a powershell command and parses output to get Titanfall2 install location on Origin
pub fn origin_install_location_detection() -> Result<String, anyhow::Error> {
    dbg!();

    // Run PowerShell command to get Titanfall2 Origin install path
    let ps = PsScriptBuilder::new()
        .no_profile(true)
        .non_interactive(true)
        .hidden(false)
        .print_commands(false)
        .build();
    let output = ps.run(r#"Get-ItemProperty -Path Registry::HKEY_LOCAL_MACHINE\SOFTWARE\Respawn\Titanfall2\ -Name "Install Dir""#).unwrap();

    // Get command output as string
    let string = output
        .stdout()
        .ok_or(anyhow!("Couldn't get PowerShell output"))?;

    // Regex the result out and return value accordingly
    let regex = Regex::new(r"(?m)Install Dir.+: (.+)\r\n")?;
    let mut result = regex.captures_iter(&string);
    match result.next() {
        Some(mat) => {
            let game_path = mat.get(1).map_or("", |m| m.as_str());
            println!("{}", game_path);
            match check_is_valid_game_path(game_path) {
                Ok(()) => return Ok(game_path.to_owned()),
                Err(err) => Err(err),
            }
        }
        None => Err(anyhow!("No Origin install path found")),
    }
}
