use crate::board::{Board, Project};
use directories::ProjectDirs;
use std::fs;
use std::path::PathBuf;


// get path to config file
fn get_config_path() -> PathBuf {
    // ProjectDirs auto find config
    if let Some(proj_dirs) = ProjectDirs::from("", "", "tui-kanban") {
        let config_dir = proj_dirs.config_dir();
        // folder exists?

        fs::create_dir_all(config_dir).ok();
        config_dir.join("projects.json")
    } else {

        // fallback
        PathBuf::from("projects.json")
    }
}

// get old omarchy-kanban config path for migration
fn get_old_omarchy_config_path() -> PathBuf {
    if let Some(proj_dirs) = ProjectDirs::from("", "", "omarchy-kanban") {
        let config_dir = proj_dirs.config_dir();
        config_dir.join("projects.json")
    } else {
        PathBuf::from("old_projects.json")
    }
}

// get old board.json path for migration
fn get_old_board_path() -> PathBuf {
    if let Some(proj_dirs) = ProjectDirs::from("", "", "omarchy-kanban") {
        let config_dir = proj_dirs.config_dir();
        config_dir.join("board.json")
    } else {
        PathBuf::from("board.json")
    }
}

/// saves projects to disc
pub fn save_projects(projects: &[Project]) -> Result<(), Box<dyn std::error::Error>> {
    let path = get_config_path();
    let json = serde_json::to_string_pretty(projects)?;
    fs::write(path, json)?;
    Ok(())
}

// read projects
pub fn load_projects() -> Vec<Project> {
    let path = get_config_path();
    let old_omarchy_path = get_old_omarchy_config_path();
    let old_board_path = get_old_board_path();

    // try migrate from old omarchy-kanban projects.json
    if !path.exists() && old_omarchy_path.exists() {
        if let Ok(content) = fs::read_to_string(&old_omarchy_path) {
            if let Ok(projects) = serde_json::from_str::<Vec<Project>>(&content) {
                // save to new location
                let _ = save_projects(&projects);
                return projects;
            }
        }
    }

    // try migrate from old board.json (even older format)
    if !path.exists() && old_board_path.exists() {
        if let Ok(content) = fs::read_to_string(&old_board_path) {
            if let Ok(board) = serde_json::from_str::<Board>(&content) {
                let default_project = Project {
                    name: "Default".to_string(),
                    board,
                };
                // save as new format
                let _ = save_projects(&vec![default_project.clone()]);
                return vec![default_project];
            }
        }
    }

    // incase non exist - return default project
    if !path.exists() {
        let default_project = Project::new("Default".to_string());
        return vec![default_project];
    }

    // try read file
    match fs::read_to_string(&path) {
        Ok(content) => {
            serde_json::from_str(&content).unwrap_or_else(|_| vec![Project::new("Default".to_string())])
        }
        Err(_) => vec![Project::new("Default".to_string())],
    }
}