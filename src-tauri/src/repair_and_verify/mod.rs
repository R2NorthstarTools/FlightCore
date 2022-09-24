/// Contains various functions to repair common issues and verifying installation

use app::GameInstall;

/// Verifies Titanfall2 game files
pub fn verify_game_files(game_install: GameInstall) -> Result<String, String> {
    dbg!(game_install);
    Err("TODO, not yet implemented".to_string())
}
