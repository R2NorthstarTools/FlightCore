#[tauri::command]
/// This method loads arguments from the ns_startup_args.txt Northstar launch
/// arguments files, filtering out eventual argument duplicates.
/// If this file does not exist, this will return an empty array.
pub fn get_launch_arguments(game_path: &str) -> Result<Vec<String>, ()> {
    let launch_args_path = format!("{}/ns_startup_args.txt", game_path);
    if !std::path::Path::new(&launch_args_path).exists() {
        return Ok(vec![]);
    }

    let data = match std::fs::read_to_string(launch_args_path) {
        Ok(content) => content,
        Err(_) => {
            return Ok(vec![]);
        }
    };

    let mut arguments = data
        .split_whitespace()
        .map(|arg| arg.to_string())
        .collect::<Vec<_>>();

    // associate language argument
    let index = arguments.iter().position(|r| r == "-language").unwrap_or_else(|| { usize::MAX });
    if index != usize::MAX {
        let value_index = index + 1;
        if value_index > arguments.len()-1 {
            println!("-language argument has no associated value.");
        } else {
            println!("{}", value_index);
        }
    }

    arguments.sort_unstable();
    arguments.dedup();
    Ok(arguments)
}

#[tauri::command]
/// This method puts an array of arguments into the ns_startup_args.txt Northstar
/// launch arguments files.
/// If the ns_startup_args.txt file does not exist, this will create it.
pub fn set_launch_arguments(game_path: &str, arguments: Vec<String>) -> Result<(), String> {
    let launch_args_path = format!("{}/ns_startup_args.txt", game_path);
    if std::fs::write(launch_args_path, arguments.join(" ")).is_err() {
        return Err("Failed to save launch arguments.".to_string());
    }
    log::info!("Launch arguments updated.");
    Ok(())
}
