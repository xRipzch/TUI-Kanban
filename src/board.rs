use serde::{Deserialize, Serialize};
use ratatui::style::Color;

//simple task with title, tags, and description
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
    //Create task
    pub fn new(title: String) -> Self {
        Self {
            title,
            tags: Vec::new(),
            description: String::new(),
        }
    }

    //add tags to the task
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

    //return color based on tags (for backward compatibility)
    pub fn get_color(&self) -> Color {
        if self.tags.contains(&"urgent".to_string()) {
            Color::Red
        } else if self.tags.contains(&"security".to_string()) {
            Color::LightRed
        } else if self.tags.contains(&"bug".to_string()) {
            Color::Yellow
        } else if self.tags.contains(&"feature".to_string()) {
            Color::Green
        } else if self.tags.contains(&"performance".to_string()) {
            Color::LightGreen
        } else if self.tags.contains(&"enhancement".to_string()) {
            Color::Blue
        } else if self.tags.contains(&"User".to_string()) {
            Color::LightBlue
        } else if self.tags.contains(&"Dev".to_string()) {
            Color::Magenta
        } else if self.tags.contains(&"documentation".to_string()) {
            Color::Cyan
        } else if self.tags.contains(&"design".to_string()) {
            Color::LightCyan
        } else if self.tags.contains(&"refactor".to_string()) {
            Color::LightYellow
        } else {
            Color::White
        }
    }
}

// kanban board with four columns: todo, in_progress, testing, done
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Board {
    pub todo: Vec<Task>,
    pub in_progress: Vec<Task>,
    pub testing: Vec<Task>,
    pub done: Vec<Task>,
}

impl Board {
    //Create new empty board
    pub fn new() -> Self {
        Self {
            todo: Vec::new(),
            in_progress: Vec::new(),
            testing: Vec::new(),
            done: Vec::new(),
        }
    }

    // get column based on index
    pub fn get_column_mut(&mut self, column: Column) -> &mut Vec<Task> {
        match column {
            Column::Todo => &mut self.todo,
            Column::InProgress => &mut self.in_progress,
            Column::Testing => &mut self.testing,
            Column::Done => &mut self.done,
        }
    }

    //get column ((Rread only))
    pub fn get_column(&self, column: Column) -> &Vec<Task> {
        match column {
            Column::Todo => &self.todo,
            Column::InProgress => &self.in_progress,
            Column::Testing => &self.testing,
            Column::Done => &self.done,
        }
    }
}


    // enum to indicate which column we're working with
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Column {
    Todo,
    InProgress,
    Testing,
    Done,
}


impl Column {
    // move to next column (right)
    pub fn next(self) -> Option<Self> {
        match self {
            Column::Todo => Some(Column::InProgress),
            Column::InProgress => Some(Column::Testing),
            Column::Testing => Some(Column::Done),
            Column::Done => None,
        }
    }

    // move to previous column (left)
    pub fn prev(self) -> Option<Self> {
        match self {
            Column::Todo => None,
            Column::InProgress => Some(Column::Todo),
            Column::Testing => Some(Column::InProgress),
            Column::Done => Some(Column::Testing),
        }
    }

    //return column name
    pub fn name(self) -> &'static str {
        match self {
            Column::Todo => "To Do",
            Column::InProgress => "In Progress",
            Column::Testing => "Testing",
            Column::Done => "Done",
        }
    }
}