#[tauri::command]
pub fn get_launch_arguments(game_path: &str) -> Result<Vec<String>, ()> {
    // If file does not exist, we return an empty list
    let launch_args_path = format!("{}/ns_startup_args.txt", game_path);
    if !std::path::Path::new(&launch_args_path).exists() {
        return Ok(vec![]);
    }

    let data =  match std::fs::read_to_string(launch_args_path.clone()) {
        Ok(content) => content,
        Err(_) => { return Ok(vec![]); }
    };

    Ok(data.split_whitespace().map(|arg| arg.to_string()).collect())
}

#[tauri::command]
pub fn set_launch_arguments(game_path: &str, arguments: Vec<String>) -> Result<(), String> {
    // If file does not exist, we return an empty list
    let launch_args_path = format!("{}/ns_startup_args.txt", game_path);
    if !std::path::Path::new(&launch_args_path).exists() {
        return Err("Launch arguments file does not exist.".to_string());
    }

    match std::fs::write(launch_args_path.clone(), arguments.join(" ")) {
        Ok(..) => (),
        Err(_) => return Err("Failed to save launch arguments.".to_string()),
    }

    log::info!("Launch arguments updated.");
    Ok(())
}
