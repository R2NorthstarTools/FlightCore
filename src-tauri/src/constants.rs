// This file stores various global constants values

pub const APP_USER_AGENT: &str = "R2NorthstarTools/FlightCore";

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
