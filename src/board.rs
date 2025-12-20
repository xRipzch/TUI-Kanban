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

    // return color based on tags (for backward compatibility)
    // pub fn get_color(&self) -> Color { // Removed as unused
    //     if self.tags.contains(&"urgent".to_string()) {
    //         Color::Red
    //     } else if self.tags.contains(&"security".to_string()) {
    //         Color::LightRed
    //     } else if self.tags.contains(&"bug".to_string()) {
    //         Color::Yellow
    //     } else if self.tags.contains(&"feature".to_string()) {
    //         Color::Green
    //     } else if self.tags.contains(&"performance".to_string()) {
    //         Color::LightGreen
    //     } else if self.tags.contains(&"enhancement".to_string()) {
    //         Color::Blue
    //     } else if self.tags.contains(&"User".to_string()) {
    //         Color::LightBlue
    //     } else if self.tags.contains(&"Dev".to_string()) {
    //         Color::Magenta
    //     } else if self.tags.contains(&"documentation".to_string()) {
    //         Color::Cyan
    //     } else if self.tags.contains(&"design".to_string()) {
    //         Color::LightCyan
    //     } else if self.tags.contains(&"refactor".to_string()) {
    //         Color::LightYellow
    //     } else {
    //         Color::White
    //     }
    // }
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
