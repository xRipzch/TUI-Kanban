use crate::board::{Board, BoardColumn, Project, Task};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

// This struct represents the old Board structure for migration purposes
#[derive(Deserialize, Serialize, Debug, Clone)]
struct LegacyBoard {
    pub todo: Vec<Task>,
    pub in_progress: Vec<Task>,
    pub testing: Vec<Task>,
    pub done: Vec<Task>,
}

// For projects.json (intermediate format)
#[derive(Deserialize, Serialize)]
struct LegacyProject {
    name: String,
    board: LegacyBoard, // Uses the old board structure
}

// Conversion logic from LegacyBoard to new Board format
impl From<LegacyBoard> for Board {
    fn from(legacy_board: LegacyBoard) -> Self {
        Board {
            columns: vec![
                BoardColumn {
                    id: "todo".to_string(),
                    name: "To Do".to_string(),
                    tasks: legacy_board.todo,
                },
                BoardColumn {
                    id: "in_progress".to_string(),
                    name: "In Progress".to_string(),
                    tasks: legacy_board.in_progress,
                },
                BoardColumn {
                    id: "testing".to_string(),
                    name: "Testing".to_string(),
                    tasks: legacy_board.testing,
                },
                BoardColumn {
                    id: "done".to_string(),
                    name: "Done".to_string(),
                    tasks: legacy_board.done,
                },
            ],
        }
    }
}

// Conversion logic from LegacyProject to new Project format
impl From<LegacyProject> for Project {
    fn from(legacy_project: LegacyProject) -> Self {
        Project {
            name: legacy_project.name,
            board: legacy_project.board.into(), // Use the From<LegacyBoard> impl
        }
    }
}

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

    // 1. Try to load projects in the NEW format (main path)
    if path.exists() {
        if let Ok(content) = fs::read_to_string(&path) {
            if let Ok(projects) = serde_json::from_str::<Vec<Project>>(&content) {
                return projects;
            }
        }
    }

    // 2. Try to migrate from old omarchy-kanban projects.json (intermediate format)
    if old_omarchy_path.exists() {
        if let Ok(content) = fs::read_to_string(&old_omarchy_path) {
            if let Ok(legacy_projects) = serde_json::from_str::<Vec<LegacyProject>>(&content) {
                let projects: Vec<Project> = legacy_projects.into_iter().map(Into::into).collect();
                // Save to new location in new format
                let _ = save_projects(&projects);
                return projects;
            }
        }
    }

    // 3. Try to migrate from old board.json (even older format)
    if old_board_path.exists() {
        if let Ok(content) = fs::read_to_string(&old_board_path) {
            if let Ok(legacy_board) = serde_json::from_str::<LegacyBoard>(&content) {
                let new_board: Board = legacy_board.into();
                let default_project = Project {
                    name: "Default".to_string(),
                    board: new_board,
                };
                // Save as new format
                let _ = save_projects(&vec![default_project.clone()]);
                return vec![default_project];
            }
        }
    }

    // 4. Fallback: incase non exist - return default project in NEW format
    let default_project = Project::new("Default".to_string());
    vec![default_project]
}
