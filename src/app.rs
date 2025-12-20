use crate::board::{Board, BoardColumn, Project, Task};
use crate::storage;

// application state
pub struct App {
    pub projects: Vec<Project>,
    pub current_project: usize,
    pub selected_project_index: usize, // for project list view
    pub selected_column: usize,        // Changed from Column enum to usize
    pub selected_index: usize,
    pub scroll_offset: usize,
    pub visible_items: usize,
    pub should_quit: bool,
    pub input_mode: InputMode,
    pub input_buffer: String,
    pub focused_field: TaskField,
}

// which field is focused in task detail view
#[derive(PartialEq, Clone, Copy)]
pub enum TaskField {
    Title,
    Tags,
    Description,
}

// input mode
#[derive(PartialEq)]
pub enum InputMode {
    Normal,
    AddingTask,
    AddingTag,
    ViewingTask,
    EditingTitle,
    EditingDescription,
    ViewingHelp,
    ProjectList,
    AddingProject,
    AddingColumn,   // New
    RenamingColumn, // New
}

impl App {
    // create new app state
    pub fn new() -> Self {
        Self {
            projects: storage::load_projects(),
            current_project: 0,
            selected_project_index: 0,
            selected_column: 0, // Default to the first column
            selected_index: 0,
            scroll_offset: 0,
            visible_items: 5, // default, updated during draw
            should_quit: false,
            input_mode: InputMode::Normal,
            input_buffer: String::new(),
            focused_field: TaskField::Title,
        }
    }

    // get current board
    pub fn board(&self) -> &Board {
        &self.projects[self.current_project].board
    }

    // get current board mutably
    pub fn board_mut(&mut self) -> &mut Board {
        &mut self.projects[self.current_project].board
    }

    // get current project name
    pub fn project_name(&self) -> &str {
        &self.projects[self.current_project].name
    }

    // save current state
    fn save(&self) {
        let _ = storage::save_projects(&self.projects);
    }

    // move selection up
    pub fn move_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    // move selection down
    pub fn move_down(&mut self) {
        let column_len = self
            .board()
            .get_column(self.selected_column)
            .map_or(0, |col| col.tasks.len());
        if column_len > 0 && self.selected_index < column_len - 1 {
            self.selected_index += 1;
        }
    }

    // move selection left
    pub fn move_left(&mut self) {
        if self.selected_column > 0 {
            self.selected_column -= 1;
            self.clamp_selection();
        }
    }

    // move selection right
    pub fn move_right(&mut self) {
        if self.selected_column < self.board().columns.len() - 1 {
            self.selected_column += 1;
            self.clamp_selection();
        }
    }

    // clamp selection to no go out of bounds
    fn clamp_selection(&mut self) {
        let column_len = self
            .board()
            .get_column(self.selected_column)
            .map_or(0, |col| col.tasks.len()); // Safely get task count
        if column_len == 0 {
            self.selected_index = 0;
            self.scroll_offset = 0;
        } else if self.selected_index >= column_len {
            self.selected_index = column_len - 1;
        }
    }

    // update scroll offset to keep selected item visible
    pub fn update_scroll(&mut self) {
        if self.visible_items == 0 {
            return;
        }

        let column_len = self
            .board()
            .get_column(self.selected_column)
            .map_or(0, |col| col.tasks.len());
        let max_scroll = if column_len > self.visible_items {
            column_len - self.visible_items
        } else {
            0
        };

        // scroll down if selected is below visible area
        if self.selected_index >= self.scroll_offset + self.visible_items {
            self.scroll_offset = self.selected_index - self.visible_items + 1;
        }

        // scroll up if selected is above visible area
        if self.selected_index < self.scroll_offset {
            self.scroll_offset = self.selected_index;
        }

        // ensure we don't scroll past the end (fixes bug when switching to columns with fewer items)
        if self.scroll_offset > max_scroll {
            self.scroll_offset = max_scroll;
        }
    }

    // move selected task to next column
    pub fn move_task_forward(&mut self) {
        let current_column_idx = self.selected_column;
        let next_column_idx = current_column_idx + 1;

        if next_column_idx < self.board().columns.len() {
            let selected_idx = self.selected_index; // Capture before mutable borrow

            // Remove task from current column
            let task = {
                let current_column = self.board_mut().get_column_mut(current_column_idx).unwrap();
                if selected_idx < current_column.tasks.len() {
                    current_column.tasks.remove(selected_idx)
                } else {
                    return; // No task to move
                }
            };

            // Add task to next column
            let next_column = self.board_mut().get_column_mut(next_column_idx).unwrap();
            next_column.tasks.push(task);

            self.clamp_selection();
            self.save();
        }
    }

    // move selected task to previous column
    pub fn move_task_backward(&mut self) {
        let current_column_idx = self.selected_column;
        if current_column_idx > 0 {
            let prev_column_idx = current_column_idx - 1;
            let selected_idx = self.selected_index; // Capture before mutable borrow

            // Remove task from current column
            let task = {
                let current_column = self.board_mut().get_column_mut(current_column_idx).unwrap();
                if selected_idx < current_column.tasks.len() {
                    current_column.tasks.remove(selected_idx)
                } else {
                    return; // No task to move
                }
            };

            // Add task to previous column
            let prev_column = self.board_mut().get_column_mut(prev_column_idx).unwrap();
            prev_column.tasks.push(task);

            self.clamp_selection();
            self.save();
        }
    }

    // del selected task
    pub fn delete_task(&mut self) {
        let current_column_idx = self.selected_column;
        let selected_idx = self.selected_index; // Capture before mutable borrow
        let column = self.board_mut().get_column_mut(current_column_idx).unwrap(); // Directly get mutable column
        if selected_idx < column.tasks.len() {
            column.tasks.remove(selected_idx);
            self.clamp_selection();
            self.save();
        }
    }

    // Column Management Methods

    pub fn start_adding_column(&mut self) {
        self.input_mode = InputMode::AddingColumn;
        self.input_buffer.clear();
    }

    pub fn start_renaming_column(&mut self) {
        if let Some(column) = self.board().get_column(self.selected_column) {
            self.input_buffer = column.name.clone();
            self.input_mode = InputMode::RenamingColumn;
        }
    }

    pub fn delete_column(&mut self) {
        let board_len = self.board().columns.len();
        if board_len <= 1 {
            return; // Don't delete the last column
        }

        // Only delete if empty for safety, or prompt (simplified here: must be empty)
        let is_empty = if let Some(col) = self.board().get_column(self.selected_column) {
            col.tasks.is_empty()
        } else {
            false
        };

        if is_empty {
            let col_idx = self.selected_column; // Capture before mutable borrow
            self.board_mut().columns.remove(col_idx);
            if self.selected_column >= self.board().columns.len() {
                self.selected_column = self.board().columns.len().saturating_sub(1);
            }
            self.clamp_selection();
            self.save();
        }
    }

    pub fn move_column_left(&mut self) {
        if self.selected_column > 0 {
            let idx = self.selected_column;
            self.board_mut().columns.swap(idx, idx - 1);
            self.selected_column -= 1;
            self.save();
        }
    }

    pub fn move_column_right(&mut self) {
        if self.selected_column < self.board().columns.len() - 1 {
            let idx = self.selected_column;
            self.board_mut().columns.swap(idx, idx + 1);
            self.selected_column += 1;
            self.save();
        }
    }

    // start input mode for adding task
    pub fn start_adding_task(&mut self) {
        self.input_mode = InputMode::AddingTask;
        self.input_buffer.clear();
    }

    // start input mode for adding tag
    pub fn start_adding_tag(&mut self) {
        // Only allow adding tags if there's a selected task in the selected column
        if let Some(column) = self.board().get_column(self.selected_column) {
            if self.selected_index < column.tasks.len() {
                self.input_mode = InputMode::AddingTag;
                self.input_buffer.clear();
            }
        }
    }

    // cancel input
    pub fn cancel_input(&mut self) {
        self.input_mode = InputMode::Normal;
        self.input_buffer.clear();
    }
    // add character to input buffer
    pub fn input_char(&mut self, c: char) {
        self.input_buffer.push(c);
    }

    // del last character from input buffer
    pub fn input_backspace(&mut self) {
        self.input_buffer.pop();
    }

    // submit input
    pub fn submit_input(&mut self) {
        match self.input_mode {
            InputMode::AddingTask => {
                if !self.input_buffer.is_empty() {
                    let task = Task::new(self.input_buffer.clone());
                    let selected_col_idx = self.selected_column; // Capture before mutable borrow
                    let current_column = self.board_mut().get_column_mut(selected_col_idx).unwrap();
                    current_column.tasks.push(task);
                    // Select the newly created task (last in the column)
                    let column_len = current_column.tasks.len();
                    if column_len > 0 {
                        self.selected_index = column_len - 1;
                        self.update_scroll();
                    }
                    self.save();
                }
            }
            InputMode::AddingTag => {
                if !self.input_buffer.is_empty() {
                    let tag = self.input_buffer.clone();
                    let current_column_idx = self.selected_column; // Capture before mutable borrow
                    let selected_idx = self.selected_index; // Capture before mutable borrow
                    let column = self.board_mut().get_column_mut(current_column_idx).unwrap();
                    if selected_idx < column.tasks.len() {
                        column.tasks[selected_idx].add_tag(tag);
                        self.save();
                    }
                }
            }
            InputMode::EditingTitle => {
                if !self.input_buffer.is_empty() {
                    let title = self.input_buffer.clone();
                    let current_column_idx = self.selected_column; // Capture before mutable borrow
                    let selected_idx = self.selected_index; // Capture before mutable borrow
                    let column = self.board_mut().get_column_mut(current_column_idx).unwrap();
                    if selected_idx < column.tasks.len() {
                        column.tasks[selected_idx].title = title;
                        self.save();
                    }
                }
                self.input_mode = InputMode::ViewingTask;
                self.input_buffer.clear();
                return;
            }
            InputMode::EditingDescription => {
                let description = self.input_buffer.clone();
                let current_column_idx = self.selected_column; // Capture before mutable borrow
                let selected_idx = self.selected_index; // Capture before mutable borrow
                let column = self.board_mut().get_column_mut(current_column_idx).unwrap();
                if selected_idx < column.tasks.len() {
                    column.tasks[selected_idx].description = description;
                    self.save();
                }
                self.input_mode = InputMode::ViewingTask;
                self.input_buffer.clear();
                return;
            }
            InputMode::AddingProject => {
                if !self.input_buffer.is_empty() {
                    let new_project = Project::new(self.input_buffer.clone());
                    self.projects.push(new_project);
                    self.current_project = self.projects.len() - 1;
                    self.selected_project_index = self.current_project;
                    self.save();
                }
                self.input_mode = InputMode::ProjectList;
                self.input_buffer.clear();
                return;
            }
            InputMode::AddingColumn => {
                if !self.input_buffer.is_empty() {
                    let name = self.input_buffer.clone();
                    // Simple ID generation: slugify name or random? For now, just use name as ID for simplicity or generate a simple one.
                    let id = name.to_lowercase().replace(" ", "_");
                    let new_column = BoardColumn::new(id, name);
                    self.board_mut().columns.push(new_column);
                    self.save();
                }
            }
            InputMode::RenamingColumn => {
                if !self.input_buffer.is_empty() {
                    let name = self.input_buffer.clone();
                    let col_idx = self.selected_column; // Capture before mutable borrow
                    if let Some(column) = self.board_mut().get_column_mut(col_idx) {
                        column.name = name;
                        self.save();
                    }
                }
            }
            InputMode::Normal
            | InputMode::ViewingTask
            | InputMode::ViewingHelp
            | InputMode::ProjectList => {}
        }
        self.cancel_input();
    }

    // open task detail view
    pub fn open_task(&mut self) {
        if let Some(column) = self.board().get_column(self.selected_column) {
            if self.selected_index < column.tasks.len() {
                self.input_mode = InputMode::ViewingTask;
                self.focused_field = TaskField::Title; // Reset to title when opening
            }
        }
    }

    // cycle to next field in task detail view
    pub fn next_field(&mut self) {
        self.focused_field = match self.focused_field {
            TaskField::Title => TaskField::Tags,
            TaskField::Tags => TaskField::Description,
            TaskField::Description => TaskField::Title,
        };
    }

    // start editing title
    pub fn start_editing_title(&mut self) {
        if let Some(column) = self.board().get_column(self.selected_column) {
            if self.selected_index < column.tasks.len() {
                self.input_buffer = column.tasks[self.selected_index].title.clone();
                self.input_mode = InputMode::EditingTitle;
            }
        }
    }

    // start editing description
    pub fn start_editing_description(&mut self) {
        if let Some(column) = self.board().get_column(self.selected_column) {
            if self.selected_index < column.tasks.len() {
                self.input_buffer = column.tasks[self.selected_index].description.clone();
                self.input_mode = InputMode::EditingDescription;
            }
        }
    }

    // remove tag by index
    pub fn remove_tag(&mut self, tag_index: usize) {
        let current_column_idx = self.selected_column; // Capture before mutable borrow
        let selected_idx = self.selected_index; // Capture before mutable borrow
        if let Some(column) = self.board_mut().get_column_mut(current_column_idx) {
            if selected_idx < column.tasks.len() {
                let task = &mut column.tasks[selected_idx];
                if tag_index < task.tags.len() {
                    task.tags.remove(tag_index);
                    self.save();
                }
            }
        }
    }

    // project management
    pub fn open_project_list(&mut self) {
        self.input_mode = InputMode::ProjectList;
        self.selected_project_index = self.current_project;
    }

    pub fn select_project(&mut self) {
        self.current_project = self.selected_project_index;
        self.input_mode = InputMode::Normal;
        self.selected_column = 0; // Reset to first column when changing projects
        self.selected_index = 0;
        self.scroll_offset = 0;
    }

    pub fn move_project_up(&mut self) {
        if self.selected_project_index > 0 {
            self.selected_project_index -= 1;
        }
    }

    pub fn move_project_down(&mut self) {
        if self.selected_project_index < self.projects.len() - 1 {
            self.selected_project_index += 1;
        }
    }

    pub fn start_adding_project(&mut self) {
        self.input_mode = InputMode::AddingProject;
        self.input_buffer.clear();
    }

    pub fn delete_project(&mut self) {
        if self.projects.len() > 1 {
            self.projects.remove(self.selected_project_index);
            if self.selected_project_index >= self.projects.len() {
                self.selected_project_index = self.projects.len() - 1;
            }
            if self.current_project >= self.projects.len() {
                self.current_project = self.projects.len() - 1;
            }
            self.save();
        }
    }

    // show help view
    pub fn show_help(&mut self) {
        self.input_mode = InputMode::ViewingHelp;
    }

    // close detail/help view
    pub fn close_view(&mut self) {
        self.input_mode = InputMode::Normal;
        self.input_buffer.clear();
    }
}
