mod app;
mod board;
mod storage;
mod ui;

use app::{App, InputMode};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("Error: {:?}", err);
    }

    Ok(())
}

// main loop
fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> io::Result<()> {
    loop {
        // draw UI
        terminal.draw(|f| ui::draw(f, app))?;
        
        // handle input
        if let Event::Key(key) = event::read()? {
            // Handle Ctrl+P globally to open project list
            if key.code == KeyCode::Char('p') && key.modifiers.contains(KeyModifiers::CONTROL) {
                if app.input_mode == InputMode::Normal {
                    app.open_project_list();
                }
                continue;
            }

            match app.input_mode {
                InputMode::Normal => handle_normal_mode(app, key.code),
                InputMode::AddingTask | InputMode::AddingTag => {
                    handle_input_mode(app, key.code)
                }
                InputMode::ViewingTask => handle_viewing_task_mode(app, key.code),
                InputMode::EditingTitle => handle_editing_title_mode(app, key.code),
                InputMode::EditingDescription => handle_editing_description_mode(app, key.code),
                InputMode::ViewingHelp => handle_viewing_help_mode(app, key.code),
                InputMode::ProjectList => handle_project_list_mode(app, key.code),
                InputMode::AddingProject => handle_adding_project_mode(app, key.code),
            }
        }

        // quit on requested
        if app.should_quit {
            return Ok(());
        }
    }
}

// handle keys in normal mode
fn handle_normal_mode(app: &mut App, key: KeyCode) {
    match key {
        // Quit
        KeyCode::Char('q') => app.should_quit = true,

        // Navigation - vim keys
        KeyCode::Char('h') => {
            app.move_left();
            app.update_scroll();
        }
        KeyCode::Char('j') => {
            app.move_down();
            app.update_scroll();
        }
        KeyCode::Char('k') => {
            app.move_up();
            app.update_scroll();
        }
        KeyCode::Char('l') => {
            app.move_right();
            app.update_scroll();
        }

        // Navigation - arrow keys
        KeyCode::Left => {
            app.move_left();
            app.update_scroll();
        }
        KeyCode::Down => {
            app.move_down();
            app.update_scroll();
        }
        KeyCode::Up => {
            app.move_up();
            app.update_scroll();
        }
        KeyCode::Right => {
            app.move_right();
            app.update_scroll();
        }

        // Actions
        KeyCode::Enter => app.open_task(),
        KeyCode::Char('a') => app.start_adding_task(),
        KeyCode::Char('t') => app.start_adding_tag(),
        KeyCode::Char('m') => app.move_task_forward(),
        KeyCode::Char('n') => app.move_task_backward(),
        KeyCode::Char('d') => app.delete_task(),
        KeyCode::Char('?') => app.show_help(),

        _ => {}
    }
}

// handle keys in input mode
fn handle_input_mode(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Enter => app.submit_input(),
        KeyCode::Esc => app.cancel_input(),
        KeyCode::Backspace => app.input_backspace(),
        KeyCode::Char(c) => app.input_char(c),
        _ => {}
    }
}

// handle keys when viewing task details
fn handle_viewing_task_mode(app: &mut App, key: KeyCode) {
    use app::TaskField;

    match key {
        KeyCode::Esc => app.close_view(),
        KeyCode::Tab => app.next_field(),
        KeyCode::Enter => {
            // Start editing based on focused field
            match app.focused_field {
                TaskField::Title => app.start_editing_title(),
                TaskField::Description => app.start_editing_description(),
                TaskField::Tags => {} // No action for tags, use numbers instead
            }
        }
        KeyCode::Char(c @ '1'..='9') => {
            // Remove tag by number (only when focused on tags)
            if app.focused_field == TaskField::Tags {
                let tag_index = (c as u8 - b'1') as usize;
                app.remove_tag(tag_index);
            }
        }
        _ => {}
    }
}

// handle keys when editing title
fn handle_editing_title_mode(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Enter => {
            // Save title and return to viewing
            app.submit_input();
        }
        KeyCode::Esc => {
            // Cancel editing and go back to viewing task
            app.input_mode = InputMode::ViewingTask;
            app.input_buffer.clear();
        }
        KeyCode::Backspace => app.input_backspace(),
        KeyCode::Char(c) => app.input_char(c),
        _ => {}
    }
}

// handle keys when editing description
fn handle_editing_description_mode(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Enter => {
            // Add newline in multi-line mode
            app.input_char('\n');
        }
        KeyCode::Esc => {
            // Save description and return to viewing
            app.submit_input();
        }
        KeyCode::Backspace => app.input_backspace(),
        KeyCode::Char(c) => app.input_char(c),
        _ => {}
    }
}

// handle keys when viewing help
fn handle_viewing_help_mode(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Esc | KeyCode::Char('?') => app.close_view(),
        _ => {}
    }
}

// handle keys in project list mode
fn handle_project_list_mode(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Esc => app.close_view(),
        KeyCode::Char('j') | KeyCode::Down => app.move_project_down(),
        KeyCode::Char('k') | KeyCode::Up => app.move_project_up(),
        KeyCode::Enter => app.select_project(),
        KeyCode::Char('a') => app.start_adding_project(),
        KeyCode::Char('d') => app.delete_project(),
        _ => {}
    }
}

// handle keys when adding project
fn handle_adding_project_mode(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Enter => app.submit_input(),
        KeyCode::Esc => {
            app.input_mode = InputMode::ProjectList;
            app.input_buffer.clear();
        }
        KeyCode::Backspace => app.input_backspace(),
        KeyCode::Char(c) => app.input_char(c),
        _ => {}
    }
}