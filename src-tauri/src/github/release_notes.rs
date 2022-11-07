use std::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReleaseInfo {
    pub name: String,
    pub published_at: String,
    pub body: String
}

#[tauri::command]
pub async fn get_northstar_release_notes() -> Result<Vec<ReleaseInfo>, String> {
    println!("Fetching releases notes from GitHub API");

    let url = "https://api.github.com/repos/R2Northstar/Northstar/releases";
    let user_agent = "R2Northstar/Northstar";
    let client = reqwest::Client::new();
    let res = client
        .get(url)
        .header(reqwest::header::USER_AGENT, user_agent)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let json_response: Vec<serde_json::Value> =
        serde_json::from_str(&res).expect("JSON was not well-formatted");
    println!("Done checking GitHub API");
    
    return Ok(json_response.iter().map(|release| ReleaseInfo {
        name: release.get("name").and_then(|value| value.as_str()).unwrap().to_string(),
        published_at: release.get("published_at").and_then(|value| value.as_str()).unwrap().to_string(),
        body: release.get("body").and_then(|value| value.as_str()).unwrap().to_string(),
    }).collect());
}