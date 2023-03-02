// This file stores various global constants values

use const_format::concatcp;

// FlightCore user agent for web requests
pub const APP_USER_AGENT: &str = concatcp!("FlightCore/", env!("CARGO_PKG_VERSION"));

// URL of the Northstar masterserver
pub const MASTER_SERVER_URL: &str = "https://northstar.tf";

// server list endpoint
pub const SERVER_BROWSER_ENDPOINT: &str = "/client/servers";

// List of core Northstar mods
pub const CORE_MODS: [&str; 3] = [
    "Northstar.Client",
    "Northstar.Custom",
    "Northstar.CustomServers",
];

// List of Thunderstoremods that shouldn't be installable
// as they behave different than common Squirrel mods
pub const BLACKLISTED_MODS: [&str; 3] = [
    "northstar-Northstar",
    "northstar-NorthstarReleaseCandidate",
    "ebkr-r2modman",
];

// Titanfall2 game IDs on Origin/EA-App
pub const TITANFALL2_ORIGIN_IDS: [&str; 2] = ["Origin.OFR.50.0001452", "Origin.OFR.50.0001456"];

// Order in which the sections for release notes should be displayed
pub const SECTION_ORDER: [&str; 9] = [
    "feat", "fix", "docs", "style", "refactor", "build", "test", "chore", "other",
];

// GitHub API endpoints for launcher/mods PRs
pub const PULLS_API_ENDPOINT_LAUNCHER: &str =
    "https://api.github.com/repos/R2Northstar/NorthstarLauncher/pulls";
pub const PULLS_API_ENDPOINT_MODS: &str =
    "https://api.github.com/repos/R2Northstar/NorthstarMods/pulls";
