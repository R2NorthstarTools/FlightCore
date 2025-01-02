

/// Checks whether the provided path is a valid Titanfall2 gamepath by checking against a certain set of criteria
pub fn check_is_valid_game_path(game_install_path: &str) -> Result<(), String> {
    let path_to_titanfall2_exe = format!("{game_install_path}/Titanfall2.exe");
    let is_correct_game_path = std::path::Path::new(&path_to_titanfall2_exe).exists();
    log::info!("Titanfall2.exe exists in path? {}", is_correct_game_path);

    // Exit early if wrong game path
    if !is_correct_game_path {
        return Err(format!("Incorrect game path \"{game_install_path}\"")); // Return error cause wrong game path
    }
    Ok(())
}
