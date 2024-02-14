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

/// Gets newest FlighCore version from GitHub
#[tauri::command]
pub async fn get_newest_flightcore_version() -> Result<FlightCoreVersion, String> {
    // Get newest version number from GitHub API
    log::info!("Checking GitHub API");
    let octocrab = octocrab::instance();
    let page = octocrab
        .repos("R2NorthstarTools", "FlightCore")
        .releases()
        .list()
        // Optional Parameters
        .per_page(1)
        .page(1u32)
        // Send the request
        .send()
        .await
        .map_err(|err| err.to_string())?;

    // Get newest element
    let latest_release_item = &page.items[0];

    let flightcore_version = FlightCoreVersion {
        tag_name: latest_release_item.tag_name.clone(),
        published_at: latest_release_item.published_at.unwrap().to_rfc3339(),
    };
    log::info!("Done checking GitHub API");

    Ok(flightcore_version)
}

/// Checks if installed FlightCore version is up-to-date
/// false -> FlightCore install is up-to-date
/// true  -> FlightCore install is outdated
#[tauri::command]
pub async fn check_is_flightcore_outdated() -> Result<bool, String> {
    let newest_flightcore_release = get_newest_flightcore_version().await?;
    // Parse version number excluding leading `v`
    let newest_version = semver::Version::parse(&newest_flightcore_release.tag_name[1..]).unwrap();

    // Get version of installed FlightCore
    let current_version = env!("CARGO_PKG_VERSION");
    let current_version = semver::Version::parse(current_version).unwrap();

    #[cfg(debug_assertions)]
    let is_outdated = current_version < newest_version;
    #[cfg(not(debug_assertions))]
    let is_outdated = current_version != newest_version;

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
    let octocrab = octocrab::instance();
    let page = octocrab
        .repos("R2Northstar", "Northstar")
        .releases()
        .list()
        // Optional Parameters
        .per_page(25)
        .page(1u32)
        // Send the request
        .send()
        .await
        .unwrap();

    // TODO there's probably a way to automatically serialize into the struct but I don't know yet how to
    let mut release_info_vector: Vec<ReleaseInfo> = vec![];
    for item in page.items {
        let release_info = ReleaseInfo {
            name: item.name.ok_or(String::from("Release name not found"))?,
            published_at: item
                .published_at
                .ok_or(String::from("Release date not found"))?
                .to_rfc3339(),
            body: item.body.ok_or(String::from("Release body not found"))?,
        };
        release_info_vector.push(release_info);
    }

    log::info!("Done checking GitHub API");

    Ok(release_info_vector)
}
