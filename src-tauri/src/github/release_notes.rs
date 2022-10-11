#[tauri::command]
pub fn get_northstar_release_notes() -> Result<String, String> {
    println!("Fetching releases notes from GitHub API");

    let url = "https://api.github.com/repos/R2Northstar/Northstar/releases";
    let user_agent = "R2Northstar/Northstar";
    let client = reqwest::blocking::Client::new();
    let res = client
        .get(url)
        .header(reqwest::header::USER_AGENT, user_agent)
        .send()
        .unwrap()
        .text()
        .unwrap();

    let json_response: serde_json::Value =
        serde_json::from_str(&res).expect("JSON was not well-formatted");
    println!("Done checking GitHub API");
    
    return Ok(json_response.to_string());
}