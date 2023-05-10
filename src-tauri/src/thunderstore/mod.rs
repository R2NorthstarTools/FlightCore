//! For interacting with Thunderstore API
use crate::constants::{APP_USER_AGENT, BLACKLISTED_MODS};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use ts_rs::TS;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ThunderstoreMod {
    pub name: String,
    pub full_name: String,
    pub owner: String,
    pub package_url: String,
    pub date_created: String,
    pub date_updated: String,
    pub uuid4: String,
    pub rating_score: i32,
    pub is_pinned: bool,
    pub is_deprecated: bool,
    pub has_nsfw_content: bool,
    pub categories: Vec<String>,
    pub versions: Vec<ThunderstoreModVersion>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ThunderstoreModVersion {
    pub name: String,
    pub full_name: String,
    pub description: String,
    pub icon: String,
    pub version_number: String,
    pub dependencies: Vec<String>,
    pub download_url: String,
    pub downloads: i32,
    pub date_created: String,
    pub website_url: String,
    pub is_active: bool,
    pub uuid4: String,
    pub file_size: i64,
}

/// Queries Thunderstore packages API
#[tauri::command]
pub async fn query_thunderstore_packages_api() -> Result<Vec<ThunderstoreMod>, String> {
    log::info!("Fetching Thunderstore API");

    // Fetches
    let url = "https://northstar.thunderstore.io/api/v1/package/";

    let client = reqwest::Client::new();
    let res = client
        .get(url)
        .header(reqwest::header::USER_AGENT, APP_USER_AGENT)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    // Parse response
    let parsed_json: Vec<ThunderstoreMod> = match serde_json::from_str(&res) {
        Ok(res) => res,
        Err(err) => return Err(err.to_string()),
    };

    // Remove some mods from listing
    let to_remove_set: HashSet<&str> = BLACKLISTED_MODS.iter().copied().collect();
    let filtered_packages = parsed_json
        .into_iter()
        .filter(|package| !to_remove_set.contains(&package.full_name.as_ref()))
        .collect::<Vec<ThunderstoreMod>>();

    Ok(filtered_packages)
}
