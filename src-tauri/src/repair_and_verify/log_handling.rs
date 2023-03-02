use regex::Regex;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, Debug, Clone, TS)]
#[ts(export)]
pub struct ParsedModFromLog {
    mod_name: String,
    enabled: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, TS)]
#[ts(export)]
pub struct ParsedLogResults {
    northstar_launcher_version: String,
    installed_mods: Vec<ParsedModFromLog>,
    has_northstar_crashed: bool,
}

/// Parse logs for installed mods
fn parse_given_log_text_for_installed_mods(
    log_text: String,
) -> Result<Vec<ParsedModFromLog>, String> {
    // Regex to capture mod loading and whether enabled/disabled
    let regex = Regex::new(r"(?m)Loaded mod (.*) successfully\n.*\[NORTHSTAR\] \[info\] Mod (.*) is (enabled|disabled)\n").unwrap();

    // Run regex, result will be an iterator over tuples containing the start and end indices for each match in the string
    let result = regex.captures_iter(&log_text);

    let mut mods = Vec::new();
    for mat in result {
        // Get the captured string, which is the first and only capturing group in the regex
        let mod_name = match mat.get(1) {
            Some(mod_name) => mod_name.as_str().to_string(),
            None => {
                println!("Failed parsing {:?}", mat); // log on failure
                continue;
            }
        };
        let mod_name_copy = match mat.get(2) {
            Some(mod_name) => mod_name.as_str().to_string(),
            None => {
                println!("Failed parsing {:?}", mat); // log on failure
                continue;
            }
        };
        let enabled_disabled = match mat.get(3) {
            Some(mod_name) => mod_name.as_str().to_string(),
            None => {
                println!("Failed parsing {:?}", mat); // log on failure
                continue;
            }
        };
        println!("{}, {}, {}", mod_name, mod_name_copy, enabled_disabled);
        if mod_name != mod_name_copy {
            return Err("Mod names don't match up".to_string());
        }

        // TODO improve checking
        let mod_enabled;
        if enabled_disabled == "enabled" {
            mod_enabled = true;
        } else {
            mod_enabled = false;
        }

        let parsed_mod_from_log = ParsedModFromLog {
            mod_name,
            enabled: mod_enabled,
        };

        // Add mod to list
        mods.push(parsed_mod_from_log);
    }

    // Return the captured mod names
    return Ok(mods);
}

/// Parse logs for Northstar launcher version
fn parse_for_northstar_launcher_version(log_text: String) -> Result<String, String> {
    let regex = Regex::new(r"(?m)NorthstarLauncher version: (.*)\n").unwrap();

    // result will be an iterator over tuples containing the start and end indices for each match in the string
    let mut result = regex.captures_iter(&log_text);

    // Return found Northstar launcher version number
    match result.next() {
        None => Err("Couldn't parse Northstar launcher version".to_string()),
        Some(mat) => match mat.get(1) {
            None => Err("Couldn't parse Northstar launcher version".to_string()),
            Some(mod_name) => Ok(mod_name.as_str().to_string()),
        },
    }
}

fn parse_log_for_crash(log_text: String) -> bool {
    let pattern = Regex::new(r"(?m)Northstar has crashed!").unwrap();
    pattern.is_match(&log_text)
}

/// Parse logs for installed mods
#[tauri::command]
pub async fn parse_given_log_text(log_text: String) -> Result<ParsedLogResults, String> {
    let installed_mods = parse_given_log_text_for_installed_mods(log_text.clone())?;
    let northstar_launcher_version = parse_for_northstar_launcher_version(log_text.clone())?;
    let has_northstar_crashed = parse_log_for_crash(log_text);

    let parsed_log_results = ParsedLogResults {
        northstar_launcher_version,
        installed_mods,
        has_northstar_crashed,
    };

    // Return the parsed results
    return Ok(parsed_log_results);
}
