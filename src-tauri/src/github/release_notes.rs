use std::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReleaseInfo {
    pub name: String,
    pub published_at: String,
    pub body: String
}

// Fetches repo release API and returns response as string
async fn fetch_github_releases_api(url: &str) -> Result<String, String> {
    println!("Fetching releases notes from GitHub API");

    let user_agent = "GeckoEidechse/FlightCore";
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

    Ok(res)
}

/// Checks if installed FlightCore version is up-to-date
/// false -> FlightCore install is up-to-date
/// true  -> FlightCore install is outdated
pub async fn check_is_flightcore_outdated() -> Result<bool, String> {
    // Get newest version number from GitHub API
    println!("Checking GitHub API");
    let url = "https://api.github.com/repos/GeckoEidechse/FlightCore/releases/latest";
    let user_agent = "GeckoEidechse/FlightCore";
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

    let json_response: serde_json::Value =
        serde_json::from_str(&res).expect("JSON was not well-formatted");
    println!("Done checking GitHub API");

    // Extract version number from JSON
    let newest_release_version = json_response
        .get("tag_name")
        .and_then(|value| value.as_str())
        .unwrap();

    // Get version of installed FlightCore...
    let version = env!("CARGO_PKG_VERSION");
    // ...and format it
    let version = format!("v{}", version);

    // TODO: This shouldn't be a string compare but promper semver compare
    Ok(version != newest_release_version)
}


#[tauri::command]
pub async fn get_northstar_release_notes() -> Result<Vec<ReleaseInfo>, String> {
    let url = "https://api.github.com/repos/R2Northstar/Northstar/releases";
    let res = fetch_github_releases_api(url).await?;

    let json_response: Vec<serde_json::Value> =
        serde_json::from_str(&res).expect("JSON was not well-formatted");
    println!("Done checking GitHub API");

    return Ok(
        json_response.iter().map(|release| ReleaseInfo {
            name: release.get("name")
                .and_then(|value| value.as_str())
                .unwrap()
                .to_string(),
            published_at: release.get("published_at")
                .and_then(|value| value.as_str())
                .unwrap()
                .to_string(),
            body: release.get("body")
                .and_then(|value| value.as_str())
                .unwrap()
                .to_string(),
        }).collect()
    );
}
