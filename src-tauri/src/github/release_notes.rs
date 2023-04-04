use crate::constants::APP_USER_AGENT;
use serde::{Deserialize, Serialize};
use std::vec::Vec;
use ts_rs::TS;

#[derive(Serialize, Deserialize, Debug, Clone, TS)]
#[ts(export)]
pub struct ReleaseInfo {
    pub name: String,
    pub published_at: String,
    pub body: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, TS)]
#[ts(export)]
pub struct FlightCoreVersion {
    tag_name: String,
    published_at: String,
}

// Fetches repo release API and returns response as string
pub async fn fetch_github_releases_api(url: &str) -> Result<String, String> {
    log::info!("Fetching releases notes from GitHub API");

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

    Ok(res)
}

/// Gets newest FlighCore version from GitHub
#[tauri::command]
pub async fn get_newest_flightcore_version() -> Result<FlightCoreVersion, String> {
    // Get newest version number from GitHub API
    log::info!("Checking GitHub API");
    let url = "https://api.github.com/repos/R2NorthstarTools/FlightCore/releases/latest";
    let res = fetch_github_releases_api(url).await?;

    let flightcore_version: FlightCoreVersion =
        serde_json::from_str(&res).expect("JSON was not well-formatted");
    log::info!("Done checking GitHub API");

    Ok(flightcore_version)
}

/// Checks if installed FlightCore version is up-to-date
/// false -> FlightCore install is up-to-date
/// true  -> FlightCore install is outdated
pub async fn check_is_flightcore_outdated() -> Result<bool, String> {
    let newest_flightcore_release = get_newest_flightcore_version().await?;

    // Get version of installed FlightCore...
    let version = env!("CARGO_PKG_VERSION");
    // ...and format it
    let version = format!("v{}", version);

    // TODO: This shouldn't be a string compare but promper semver compare
    let is_outdated = version != newest_flightcore_release.tag_name;

    // If outdated, check how new the update is
    if is_outdated {
        // Time to wait (2h)    h *  m *  s
        let threshold_seconds = 2 * 60 * 60;

        // Get current time
        let current_time = chrono::Utc::now();

        // Get latest release time from GitHub API response
        let result = chrono::DateTime::parse_from_rfc3339(&newest_flightcore_release.published_at)
            .unwrap()
            .with_timezone(&chrono::Utc);

        // Check if current time is outside of threshold
        let diff = current_time - result;
        if diff.num_seconds() < threshold_seconds {
            // User would be outdated but the newest release is recent
            // therefore we do not wanna show outdated warning.
            return Ok(false);
        }
        return Ok(true);
    }

    Ok(is_outdated)
}

#[tauri::command]
pub async fn get_northstar_release_notes() -> Result<Vec<ReleaseInfo>, String> {
    let url = "https://api.github.com/repos/R2Northstar/Northstar/releases";
    let res = fetch_github_releases_api(url).await?;

    let release_info_vector: Vec<ReleaseInfo> =
        serde_json::from_str(&res).expect("JSON was not well-formatted");
    log::info!("Done checking GitHub API");

    Ok(release_info_vector)
}
