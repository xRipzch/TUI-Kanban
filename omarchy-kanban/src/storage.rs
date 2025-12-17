use crate::board::Board;
use directories::ProjectDirs;
use std::fs;
use std::path::PathBuf;


// get path to config file
fn get_config_path() -> PathBuf {
    // ProjectDirs auto find config
    if let Some(proj_dirs) = ProjectDirs::from("", "", "omarchy-kanban") {
        let config_dir = proj_dirs.config_dir();
        // folder exists?

        fs::create_dir_all(config_dir).ok();
        config_dir.join("board.json")
    } else {

        // fallback
        PathBuf::from("board.json")
    }
}

/// svaes board to disc
pub fn save_board(board: &Board) -> Result<(), Box<dyn std::error::Error>> {
    let path = get_config_path();
    let json = serde_json::to_string_pretty(board)?;
    fs::write(path, json)?;
    Ok(())
}

// read board
pub fn load_board() -> Board {
    let path = get_config_path();
    
    // incase non exist - return new board
    if !path.exists() {
        return Board::new();
    }

    // try read file
    match fs::read_to_string(&path) {
        Ok(content) => {
            serde_json::from_str(&content).unwrap_or_else(|_| Board::new())
        }
        Err(_) => Board::new(),
    }
}