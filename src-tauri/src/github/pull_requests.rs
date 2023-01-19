use crate::github::release_notes::fetch_github_releases_api;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PullsApiResponseElement {
    number: i64,
    title: String,
    url: String,
}

/// Gets launcher PRs
#[tauri::command]
pub async fn get_launcher_prs() -> Result<Vec<PullsApiResponseElement>, String> {
    let json_response = match fetch_github_releases_api(
        "https://api.github.com/repos/R2Northstar/NorthstarMods/pulls",
    )
    .await
    {
        Ok(result) => result,
        Err(err) => return Err(err.to_string()),
    };

    let pulls_response: Vec<PullsApiResponseElement> = match serde_json::from_str(&json_response) {
        Ok(res) => res,
        Err(err) => return Err(err.to_string()),
    };

    Ok(pulls_response)
}
