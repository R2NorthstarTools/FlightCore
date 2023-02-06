//! For interacting with Thunderstore API

use app::constants::APP_USER_AGENT;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ThunderstorePackageElement {
    pub name: String,
    pub full_name: String,
    pub owner: String,
    pub package_url: String,
    pub date_created: String,
    pub date_updated: String,
    pub uuid4: String,
    pub rating_score: i64,
    pub is_pinned: bool,
    pub is_deprecated: bool,
    pub has_nsfw_content: bool,
    pub categories: Vec<String>,
    pub versions: Vec<TSModVersion>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct TSModVersion {
    pub name: String,
    pub full_name: String,
    pub description: String,
    pub icon: String,
    pub version_number: String,
    pub dependencies: Vec<String>,
    pub download_url: String,
    pub downloads: i64,
    pub date_created: String,
    pub website_url: String,
    pub is_active: bool,
    pub uuid4: String,
    pub file_size: i64,
}

/// Queries Thunderstore packages API
#[tauri::command]
pub async fn query_thunderstore_api() -> Result<Vec<ThunderstorePackageElement>, String> {
    println!("Fetching Thunderstore API");

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

    match serde_json::from_str(&res) {
        Ok(res) => Ok(res),
        Err(err) => Err(err.to_string()),
    }
}
