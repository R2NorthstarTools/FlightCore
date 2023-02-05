use regex::Regex;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, Debug, Clone, TS)]
#[ts(export)]
pub struct ParsedLogResults {
    northstar_launcher_version: String,
    installed_mods: Vec<String>,
}

/// Parse logs for installed mods
fn parse_given_log_text_for_installed_mods(log_text: String) -> Result<Vec<String>, String> {
    // Regex to capture mod loading
    let regex = Regex::new(r"(?m)Loaded mod (.*) successfully\n").unwrap();

    // Run regex, result will be an iterator over tuples containing the start and end indices for each match in the string
    let result = regex.captures_iter(&log_text);

    let mut mods = Vec::new();
    for mat in result {
        // Get the captured string, which is the first and only capturing group in the regex
        match mat.get(1) {
            Some(mod_name) => {
                mods.push(mod_name.as_str().to_string());
            }
            None => println!("Failed parsing {:?}", mat), // log on failure
        };
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

/// Parse logs for installed mods
#[tauri::command]
pub async fn parse_given_log_text(log_text: String) -> Result<ParsedLogResults, String> {
    let installed_mods = parse_given_log_text_for_installed_mods(log_text.clone())?;
    let northstar_launcher_version = parse_for_northstar_launcher_version(log_text)?;

    let parsed_log_results = ParsedLogResults {
        northstar_launcher_version,
        installed_mods,
    };

    // Return the parsed results
    return Ok(parsed_log_results);
}
