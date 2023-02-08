// This file stores various global constants values

use const_format::concatcp;

pub const APP_USER_AGENT: &str = concatcp!("FlightCore/", env!("CARGO_PKG_VERSION"));

// URL of the Northstar masterserver
pub const MASTER_SERVER_URL: &str = "https://northstar.tf";

// server list endpoint
pub const SERVER_BROWSER_ENDPOINT: &str = "/client/servers";
