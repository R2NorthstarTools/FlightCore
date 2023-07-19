use crate::{NorthstarLog, GameInstall};

#[tauri::command]
pub fn fetch_northstar_logs(
    game_install: GameInstall,
) -> Result<Vec<NorthstarLog>, String> {

    let mut northstar_logs: Vec<NorthstarLog> = Vec::new();

    let pattern = format!("{}/R2Northstar/logs/nslog*.txt", game_install.game_path);
    dbg!(&pattern);
    for e in glob::glob(&pattern).expect("Failed to read glob pattern") {
        let file = e.unwrap();

        let log = NorthstarLog {
            filename: file.file_name().unwrap().to_os_string().into_string().unwrap(),
            path: file.to_str().unwrap().to_string(),
        };

    northstar_logs.push(log);
    }

    northstar_logs.reverse();
    Ok(northstar_logs)
}

#[tauri::command]
pub fn load_northstar_log(
    log: NorthstarLog
) -> Result<String, String> {
    let log_content: String = std::fs::read_to_string(log.path).unwrap().parse().unwrap();

    Ok(log_content)
}
