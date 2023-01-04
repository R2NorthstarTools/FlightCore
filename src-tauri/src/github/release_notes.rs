use serde::{Deserialize, Serialize};
use std::vec::Vec;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReleaseInfo {
    pub name: String,
    pub published_at: String,
    pub body: String,
}

#[derive(Debug, Deserialize, Clone)]
struct FlightCoreVersion {
    tag_name: String,
    published_at: String,
}

// Fetches repo release API and returns response as string
async fn fetch_github_releases_api(url: &str) -> Result<String, String> {
    println!("Fetching releases notes from GitHub API");

    let user_agent = "R2NorthstarTools/FlightCore";
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

/// Gets newest FlighCore version from GitHub
async fn get_newest_flightcore_version() -> Result<(String, String), String> {
    // Get newest version number from GitHub API
    println!("Checking GitHub API");
    let url = "https://api.github.com/repos/R2NorthstarTools/FlightCore/releases/latest";
    let res = fetch_github_releases_api(url).await?;

    let flightcore_version: FlightCoreVersion =
        serde_json::from_str(&res).expect("JSON was not well-formatted");
    println!("Done checking GitHub API");

    Ok((flightcore_version.tag_name, flightcore_version.published_at))
}

/// Checks if installed FlightCore version is up-to-date
/// false -> FlightCore install is up-to-date
/// true  -> FlightCore install is outdated
pub async fn check_is_flightcore_outdated() -> Result<bool, String> {
    let (newest_release_version, release_date) = get_newest_flightcore_version().await?;

    // Get version of installed FlightCore...
    let version = env!("CARGO_PKG_VERSION");
    // ...and format it
    let version = format!("v{}", version);

    // TODO: This shouldn't be a string compare but promper semver compare
    let is_outdated = version != newest_release_version;

    // If outdated, check how new the update is
    if is_outdated {
        // Time to wait (2h)    h *  m *  s
        let threshold_seconds = 2 * 60 * 60;

        // Get current time
        let current_time = chrono::Utc::now();

        // Get latest release time from GitHub API response
        let result = chrono::DateTime::parse_from_rfc3339(&release_date)
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

    let json_response: Vec<serde_json::Value> =
        serde_json::from_str(&res).expect("JSON was not well-formatted");
    println!("Done checking GitHub API");

    return Ok(json_response
        .iter()
        .map(|release| ReleaseInfo {
            name: release
                .get("name")
                .and_then(|value| value.as_str())
                .unwrap()
                .to_string(),
            published_at: release
                .get("published_at")
                .and_then(|value| value.as_str())
                .unwrap()
                .to_string(),
            body: release
                .get("body")
                .and_then(|value| value.as_str())
                .unwrap()
                .to_string(),
        })
        .collect());
}
