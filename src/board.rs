use ratatui::style::Color;
use serde::{Deserialize, Serialize};

// simple task with title, tags, and description
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub title: String,
    pub tags: Vec<String>,
    pub description: String,
}

// project contains a name and a board
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Project {
    pub name: String,
    pub board: Board,
}

impl Project {
    pub fn new(name: String) -> Self {
        Self {
            name,
            board: Board::new(),
        }
    }
}

impl Task {
    // Create task
    pub fn new(title: String) -> Self {
        Self {
            title,
            tags: Vec::new(),
            description: String::new(),
        }
    }

    // add tags to the task
    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
        }
    }

    // return color for a specific tag
    pub fn get_tag_color(tag: &str) -> Color {
        match tag {
            "urgent" => Color::Red,
            "security" => Color::LightRed,
            "bug" => Color::Yellow,
            "feature" => Color::Green,
            "performance" => Color::LightGreen,
            "enhancement" => Color::Blue,
            "User" => Color::LightBlue,
            "Dev" => Color::Magenta,
            "documentation" => Color::Cyan,
            "design" => Color::LightCyan,
            "refactor" => Color::LightYellow,
            _ => Color::White,
        }
    }
}

// A single column in the board
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BoardColumn {
    pub id: String,
    pub name: String,
    pub tasks: Vec<Task>,
}

impl BoardColumn {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            tasks: Vec::new(),
        }
    }
}

// Kanban board with dynamic columns
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Board {
    pub columns: Vec<BoardColumn>,
}

impl Board {
    // Create new board with default columns
    pub fn new() -> Self {
        Self {
            columns: vec![
                BoardColumn::new("todo".to_string(), "To Do".to_string()),
                BoardColumn::new("in_progress".to_string(), "In Progress".to_string()),
                BoardColumn::new("testing".to_string(), "Testing".to_string()),
                BoardColumn::new("done".to_string(), "Done".to_string()),
            ],
        }
    }

    // get column by index (Read only)
    pub fn get_column(&self, index: usize) -> Option<&BoardColumn> {
        self.columns.get(index)
    }

    // get column by index (Mutable)
    pub fn get_column_mut(&mut self, index: usize) -> Option<&mut BoardColumn> {
        self.columns.get_mut(index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::style::Color;

    #[test]
    fn test_task_creation() {
        let task = Task::new("Test Task".to_string());
        assert_eq!(task.title, "Test Task");
        assert!(task.tags.is_empty());
        assert!(task.description.is_empty());
    }

    #[test]
    fn test_task_add_tag() {
        let mut task = Task::new("Task".to_string());
        task.add_tag("bug".to_string());
        task.add_tag("urgent".to_string());
        task.add_tag("bug".to_string()); // Duplicate

        assert_eq!(task.tags.len(), 2);
        assert!(task.tags.contains(&"bug".to_string()));
        assert!(task.tags.contains(&"urgent".to_string()));
    }

    #[test]
    fn test_tag_colors() {
        assert_eq!(Task::get_tag_color("urgent"), Color::Red);
        assert_eq!(Task::get_tag_color("feature"), Color::Green);
        assert_eq!(Task::get_tag_color("unknown_tag"), Color::White);
    }

    #[test]
    fn test_board_creation() {
        let board = Board::new();
        assert_eq!(board.columns.len(), 4);
        assert_eq!(board.columns[0].name, "To Do");
        assert_eq!(board.columns[3].name, "Done");
    }

    #[test]
    fn test_board_column_creation() {
        let col = BoardColumn::new("col_id".to_string(), "Column Name".to_string());
        assert_eq!(col.id, "col_id");
        assert_eq!(col.name, "Column Name");
        assert!(col.tasks.is_empty());
    }
}
