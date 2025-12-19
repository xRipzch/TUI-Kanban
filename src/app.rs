use crate::board::{Board, Column, Project, Task};
use crate::storage;

// application state
pub struct App {
    pub projects: Vec<Project>,
    pub current_project: usize,
    pub selected_project_index: usize, // for project list view
    pub selected_column: Column,
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
}

impl App {
    // create new app state
    pub fn new() -> Self {
        Self {
            projects: storage::load_projects(),
            current_project: 0,
            selected_project_index: 0,
            selected_column: Column::Todo,
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
        let column_len = self.board().get_column(self.selected_column).len();
        if column_len > 0 && self.selected_index < column_len - 1 {
            self.selected_index += 1;
        }
    }

    // move selection left
    pub fn move_left(&mut self) {
        if let Some(prev) = self.selected_column.prev() {
            self.selected_column = prev;
            self.clamp_selection();
        }
    }

    // move selection right
    pub fn move_right(&mut self) {
        if let Some(next) = self.selected_column.next() {
            self.selected_column = next;
            self.clamp_selection();
        }
    }
    // clamp selection to no go out of bounds
    fn clamp_selection(&mut self) {
        let column_len = self.board().get_column(self.selected_column).len();
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

        // scroll down if selected is below visible area
        if self.selected_index >= self.scroll_offset + self.visible_items {
            self.scroll_offset = self.selected_index - self.visible_items + 1;
        }

        // scroll up if selected is above visible area
        if self.selected_index < self.scroll_offset {
            self.scroll_offset = self.selected_index;
        }
    }

    // update visible items count based on screen height
    pub fn set_visible_items(&mut self, height: u16, card_height: u16, card_spacing: u16) {
        let total_card_height = card_height + card_spacing;
        self.visible_items = (height / total_card_height).max(1) as usize;
    }

    // move selected task to next column
    pub fn move_task_forward(&mut self) {
        if let Some(next_column) = self.selected_column.next() {
            let selected_col = self.selected_column;
            let selected_idx = self.selected_index;
            let current_column = self.board_mut().get_column_mut(selected_col);

            if selected_idx < current_column.len() {
                let task = current_column.remove(selected_idx);
                self.board_mut().get_column_mut(next_column).push(task);
                self.clamp_selection();
                self.save();
            }
        }
    }

    // move selected task to previous column
    pub fn move_task_backward(&mut self) {
        if let Some(prev_column) = self.selected_column.prev() {
            let selected_col = self.selected_column;
            let selected_idx = self.selected_index;
            let current_column = self.board_mut().get_column_mut(selected_col);

            if selected_idx < current_column.len() {
                let task = current_column.remove(selected_idx);
                self.board_mut().get_column_mut(prev_column).push(task);
                self.clamp_selection();
                self.save();
            }
        }
    }

    // del selected task
    pub fn delete_task(&mut self) {
        let selected_col = self.selected_column;
        let selected_idx = self.selected_index;
        let column = self.board_mut().get_column_mut(selected_col);
        if selected_idx < column.len() {
            column.remove(selected_idx);
            self.clamp_selection();
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
        self.input_mode = InputMode::AddingTag;
        self.input_buffer.clear();
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
                    let selected_col = self.selected_column;
                    self.board_mut().get_column_mut(selected_col).push(task);
                    // Select the newly created task (last in the column)
                    let column_len = self.board().get_column(selected_col).len();
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
                    let selected_col = self.selected_column;
                    let selected_idx = self.selected_index;
                    let column = self.board_mut().get_column_mut(selected_col);
                    if selected_idx < column.len() {
                        column[selected_idx].add_tag(tag);
                        self.save();
                    }
                }
            }
            InputMode::EditingTitle => {
                if !self.input_buffer.is_empty() {
                    let title = self.input_buffer.clone();
                    let selected_col = self.selected_column;
                    let selected_idx = self.selected_index;
                    let column = self.board_mut().get_column_mut(selected_col);
                    if selected_idx < column.len() {
                        column[selected_idx].title = title;
                        self.save();
                    }
                }
                self.input_mode = InputMode::ViewingTask;
                self.input_buffer.clear();
                return;
            }
            InputMode::EditingDescription => {
                let description = self.input_buffer.clone();
                let selected_col = self.selected_column;
                let selected_idx = self.selected_index;
                let column = self.board_mut().get_column_mut(selected_col);
                if selected_idx < column.len() {
                    column[selected_idx].description = description;
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
            InputMode::Normal | InputMode::ViewingTask | InputMode::ViewingHelp | InputMode::ProjectList => {}
        }
        self.cancel_input();
    }

    // open task detail view
    pub fn open_task(&mut self) {
        let column = self.board().get_column(self.selected_column);
        if self.selected_index < column.len() {
            self.input_mode = InputMode::ViewingTask;
            self.focused_field = TaskField::Title; // Reset to title when opening
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
        let column = self.board().get_column(self.selected_column);
        if self.selected_index < column.len() {
            self.input_buffer = column[self.selected_index].title.clone();
            self.input_mode = InputMode::EditingTitle;
        }
    }

    // start editing description
    pub fn start_editing_description(&mut self) {
        let column = self.board().get_column(self.selected_column);
        if self.selected_index < column.len() {
            self.input_buffer = column[self.selected_index].description.clone();
            self.input_mode = InputMode::EditingDescription;
        }
    }

    // remove tag by index
    pub fn remove_tag(&mut self, tag_index: usize) {
        let selected_col = self.selected_column;
        let selected_idx = self.selected_index;
        let column = self.board_mut().get_column_mut(selected_col);
        if selected_idx < column.len() {
            let task = &mut column[selected_idx];
            if tag_index < task.tags.len() {
                task.tags.remove(tag_index);
                self.save();
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
        self.selected_column = Column::Todo;
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