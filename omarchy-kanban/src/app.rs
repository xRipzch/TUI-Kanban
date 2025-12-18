use crate::board::{Board, Column, Task};
use crate::storage;

// application state
pub struct App {
    pub board: Board,
    pub selected_column: Column,
    pub selected_index: usize,
    pub should_quit: bool,
    pub input_mode: InputMode,
    pub input_buffer: String,
}

// input mode
#[derive(PartialEq)]
pub enum InputMode {
    Normal,
    AddingTask,
    AddingTag,
}

impl App {
    // create new app state
    pub fn new() -> Self {
        Self {
            board: storage::load_board(),
            selected_column: Column::Todo,
            selected_index: 0,
            should_quit: false,
            input_mode: InputMode::Normal,
            input_buffer: String::new(),
        }
    }

    // move selection up
    pub fn move_up(&mut self) {
        let column_len = self.board.get_column(self.selected_column).len();
        if column_len > 0 && self.selected_index < column_len - 1 {
            self.selected_index += 1;
        }
    }

    // move selection down
    pub fn move_down(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
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
        let column_len = self.board.get_column(self.selected_column).len();
        if column_len == 0 {
            self.selected_index = 0;
        } else if self.selected_index >= column_len {
            self.selected_index = column_len - 1;
        }
    }

    // move selected task to next column
    pub fn move_task_forward(&mut self) {
        if let Some(next_column) = self.selected_column.next() {
            let current_column = self.board.get_column_mut(self.selected_column);
            
            if self.selected_index < current_column.len() {
                let task = current_column.remove(self.selected_index);
                self.board.get_column_mut(next_column).push(task);
                self.clamp_selection();
                let _ = storage::save_board(&self.board);
            }
        }
    }
    // del selected task
    pub fn delete_task(&mut self) {
        let column = self.board.get_column_mut(self.selected_column);
        if self.selected_index < column.len() {
            column.remove(self.selected_index);
            self.clamp_selection();
            let _ = storage::save_board(&self.board);
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
                    self.board.get_column_mut(self.selected_column).push(task);
                    let _ = storage::save_board(&self.board);
                }
            }
            InputMode::AddingTag => {
                if !self.input_buffer.is_empty() {
                    let column = self.board.get_column_mut(self.selected_column);
                    if self.selected_index < column.len() {
                        column[self.selected_index].add_tag(self.input_buffer.clone());
                        let _ = storage::save_board(&self.board);
                    }
                }
            }
            InputMode::Normal => {}
        }
        self.cancel_input();
    }
}