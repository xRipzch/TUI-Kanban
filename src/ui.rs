use crate::app::{App, InputMode};
use crate::board::Column;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

// render UI
pub fn draw(f: &mut Frame, app: &mut App) {
    // check if we're in a special view mode
    match app.input_mode {
        InputMode::ViewingTask | InputMode::EditingTitle | InputMode::EditingDescription => {
            draw_task_detail(f, app);
            return;
        }
        InputMode::ViewingHelp => {
            draw_help(f, app);
            return;
        }
        InputMode::ProjectList | InputMode::AddingProject => {
            draw_project_list(f, app);
            return;
        }
        _ => {}
    }

    // make three workspaces: header, main area, and footer
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),   // Header with project name
            Constraint::Min(0),      // Main area
            Constraint::Length(3),   // Footer
        ])
        .split(f.area());

    // draw header with project name
    draw_header(f, app, chunks[0]);

    // draw the three columns
    draw_columns(f, app, chunks[1]);

    // footer with help text or input field
    draw_footer(f, app, chunks[2]);
}

// draw header with project name
fn draw_header(f: &mut Frame, app: &App, area: Rect) {
    let project_name = app.project_name();
    let header_text = vec![Line::from(vec![
        Span::styled("Project: ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::styled(project_name, Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        Span::styled("  (Ctrl+P to switch)", Style::default().fg(Color::DarkGray)),
    ])];

    let header = Paragraph::new(header_text)
        .block(Block::default().borders(Borders::ALL));

    f.render_widget(header, area);
}

// draw the four columns
fn draw_columns(f: &mut Frame, app: &mut App, area: Rect) {
    // split main area into four equal columns
    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ])
        .split(area);

    // draw each column
    draw_column(f, app, Column::Todo, columns[0]);
    draw_column(f, app, Column::InProgress, columns[1]);
    draw_column(f, app, Column::Testing, columns[2]);
    draw_column(f, app, Column::Done, columns[3]);
}

/// draw single column with task cards
fn draw_column(f: &mut Frame, app: &mut App, column: Column, area: Rect) {
    let is_selected_column = app.selected_column == column;

    // highlight border if selected column
    let border_style = if is_selected_column {
        Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };

    let outer_block = Block::default()
        .borders(Borders::ALL)
        .border_style(border_style)
        .title(column.name());

    let inner_area = outer_block.inner(area);
    f.render_widget(outer_block, area);

    // calculate card height (5 lines: top border, title, tags, padding, bottom border)
    let card_height = 5;
    let card_spacing = 1; // space between cards

    // update visible items for selected column based on actual screen height
    if is_selected_column {
        app.set_visible_items(inner_area.height, card_height, card_spacing);
    }

    // determine scroll offset for this column (must get before borrowing tasks)
    let scroll_offset = if is_selected_column {
        app.scroll_offset
    } else {
        0
    };

    // now get the tasks (immutable borrow)
    let tasks = app.board().get_column(column);

    // render each task as a card, starting from scroll_offset
    let mut rendered = 0;
    for (i, task) in tasks.iter().enumerate().skip(scroll_offset) {
        let y_offset = rendered as u16 * (card_height + card_spacing);

        // stop if we run out of space
        if y_offset + card_height > inner_area.height {
            break;
        }

        let card_area = Rect {
            x: inner_area.x,
            y: inner_area.y + y_offset,
            width: inner_area.width,
            height: card_height,
        };

        draw_task_card(f, task, card_area, is_selected_column && i == app.selected_index);
        rendered += 1;
    }
}

/// draw a single task card
fn draw_task_card(f: &mut Frame, task: &crate::board::Task, area: Rect, is_selected: bool) {
    // card border style
    let border_style = if is_selected {
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Gray)
    };

    // background color for selected task
    let bg_color = if is_selected {
        Color::DarkGray
    } else {
        Color::Reset
    };

    let card_block = Block::default()
        .borders(Borders::ALL)
        .border_style(border_style)
        .style(Style::default().bg(bg_color));

    let inner = card_block.inner(area);
    f.render_widget(card_block, area);

    // render task title and tags on separate lines
    if inner.height >= 2 {
        // truncate title to fit width
        let max_title_len = inner.width as usize;
        let truncated_title: String = task.title.chars().take(max_title_len).collect();

        let mut lines = vec![
            // Line 1: Title
            Line::from(Span::styled(
                truncated_title,
                Style::default()
                    .fg(Color::White)
                    .add_modifier(if is_selected { Modifier::BOLD } else { Modifier::empty() })
            ))
        ];

        // Line 2: Tags (if any) - each tag with its own color
        if !task.tags.is_empty() {
            let mut tag_spans = vec![];
            for tag in &task.tags {
                tag_spans.push(Span::styled(
                    format!("#{} ", tag),
                    Style::default()
                        .fg(crate::board::Task::get_tag_color(tag))
                        .add_modifier(Modifier::DIM)
                ));
            }
            lines.push(Line::from(tag_spans));
        }

        let content = Paragraph::new(lines);
        f.render_widget(content, inner);
    }
}

// draw footer with help text or input field
fn draw_footer(f: &mut Frame, app: &mut App, area: Rect) {
    let text = match app.input_mode {
        InputMode::Normal => {
            vec![
                Line::from(vec![
                    Span::raw("hjkl/arrows: navigate | "),
                    Span::raw("Enter: open task | "),
                    Span::raw("a: add task | "),
                    Span::raw("t: add tag | "),
                    Span::raw("m: move forward | "),
                    Span::raw("n: move back | "),
                    Span::raw("d: delete | "),
                    Span::raw("?: help | "),
                    Span::raw("q: quit"),
                ])
            ]
        }
        InputMode::AddingTask => {
            vec![
                Line::from(vec![
                    Span::styled("Add Task: ", Style::default().fg(Color::Yellow)),
                    Span::raw(&app.input_buffer),
                ]),
                Line::from("Press Enter to submit, Esc to cancel"),
            ]
        }
        InputMode::AddingTag => {
            vec![
                Line::from(vec![
                    Span::styled("Add Tag: ", Style::default().fg(Color::Yellow)),
                    Span::raw(&app.input_buffer),
                ]),
                Line::from("Press Enter to submit, Esc to cancel"),
            ]
        }
        _ => vec![Line::from("")],
    };

    let paragraph = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL));

    f.render_widget(paragraph, area);
}

// draw task detail view
fn draw_task_detail(f: &mut Frame, app: &mut App) {
    let area = f.area();

    // get the selected task
    let column = app.board().get_column(app.selected_column);
    if app.selected_index >= column.len() {
        return;
    }
    let task = &column[app.selected_index];

    // check what editing mode we're in
    let is_editing_title = app.input_mode == InputMode::EditingTitle;
    let is_editing_description = app.input_mode == InputMode::EditingDescription;

    // create main container with context-aware title
    let title = if is_editing_title {
        " Task Details - EDITING TITLE (Enter to save, Esc to cancel) "
    } else if is_editing_description {
        " Task Details - EDITING DESCRIPTION (Enter for newline, Esc to save) "
    } else {
        " Task Details (Tab: switch field | Enter: edit | 1-9: remove tag | Esc: close) "
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
        .title(title);

    let inner = block.inner(area);
    f.render_widget(block, area);

    // split into sections
    let sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Title
            Constraint::Length(12), // Tags (enough for header + up to 9 tags)
            Constraint::Min(5),     // Description
        ])
        .split(inner);

    // title section - show editable input if editing, otherwise show read-only
    use crate::app::TaskField;
    let is_title_focused = app.focused_field == TaskField::Title && !is_editing_title && !is_editing_description;

    if is_editing_title {
        let title_para = Paragraph::new(app.input_buffer.as_str())
            .block(Block::default()
                .borders(Borders::ALL)
                .title("Title [EDITING]")
                .border_style(Style::default().fg(Color::Yellow)))
            .style(Style::default().bg(Color::DarkGray));
        f.render_widget(title_para, sections[0]);
    } else {
        let title_text = vec![
            Line::from(vec![
                Span::styled("Title: ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                Span::raw(&task.title),
            ]),
        ];
        let border_style = if is_title_focused {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        };
        let title_para = Paragraph::new(title_text)
            .block(Block::default().borders(Borders::ALL).border_style(border_style));
        f.render_widget(title_para, sections[0]);
    }

    // tags section - show numbered tags for easy removal
    let is_tags_focused = app.focused_field == TaskField::Tags && !is_editing_title && !is_editing_description;

    let tags_lines = if !task.tags.is_empty() {
        let mut lines = vec![
            Line::from(vec![
                Span::styled("Tags ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                Span::styled("(press 1-9 to remove):", Style::default().fg(Color::DarkGray)),
            ])
        ];
        for (i, tag) in task.tags.iter().enumerate() {
            if i < 9 {
                lines.push(Line::from(vec![
                    Span::styled(format!(" {} ", i + 1), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                    Span::styled(format!("#{}", tag), Style::default().fg(crate::board::Task::get_tag_color(tag))),
                ]));
            }
        }
        lines
    } else {
        vec![Line::from(Span::styled("No tags", Style::default().fg(Color::DarkGray)))]
    };
    let border_style = if is_tags_focused {
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };
    let tags_para = Paragraph::new(tags_lines)
        .block(Block::default().borders(Borders::ALL).border_style(border_style));
    f.render_widget(tags_para, sections[1]);

    // description section - show input field if editing, otherwise show text
    let is_desc_focused = app.focused_field == TaskField::Description && !is_editing_title && !is_editing_description;

    if is_editing_description {
        // Show editable input field
        let desc_para = Paragraph::new(app.input_buffer.as_str())
            .block(Block::default()
                .borders(Borders::ALL)
                .title("Description [EDITING]")
                .border_style(Style::default().fg(Color::Yellow)))
            .wrap(ratatui::widgets::Wrap { trim: false })
            .style(Style::default().bg(Color::DarkGray));
        f.render_widget(desc_para, sections[2]);
    } else {
        // Show read-only description
        let desc_text = if task.description.is_empty() {
            "No description (press Enter to add)"
        } else {
            &task.description
        };
        let border_style = if is_desc_focused {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        };
        let desc_para = Paragraph::new(desc_text)
            .block(Block::default().borders(Borders::ALL).title("Description").border_style(border_style))
            .wrap(ratatui::widgets::Wrap { trim: false });
        f.render_widget(desc_para, sections[2]);
    }
}

// draw help view
fn draw_help(f: &mut Frame, _app: &mut App) {
    let area = f.area();

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
        .title(" Help (Press Esc or ? to close) ");

    let inner = block.inner(area);
    f.render_widget(block, area);

    let help_text = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("Navigation:", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        ]),
        Line::from("  h/← : Move left (previous column)"),
        Line::from("  j/↓ : Move down (next task)"),
        Line::from("  k/↑ : Move up (previous task)"),
        Line::from("  l/→ : Move right (next column)"),
        Line::from(""),
        Line::from(vec![
            Span::styled("Task Management:", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        ]),
        Line::from("  Enter : Open task details"),
        Line::from("  a     : Add new task to current column"),
        Line::from("  t     : Add tag to selected task"),
        Line::from("  m     : Move task forward (TODO → IN PROGRESS → TESTING → DONE)"),
        Line::from("  n     : Move task backward (DONE → TESTING → IN PROGRESS → TODO)"),
        Line::from("  d     : Delete selected task"),
        Line::from("  e     : Edit description (when viewing task)"),
        Line::from(""),
        Line::from(vec![
            Span::styled("Predefined Tags:", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::raw("  "),
            Span::styled("urgent", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
            Span::raw("        : Red - High priority"),
        ]),
        Line::from(vec![
            Span::raw("  "),
            Span::styled("security", Style::default().fg(Color::LightRed).add_modifier(Modifier::BOLD)),
            Span::raw("      : Light Red - Security work"),
        ]),
        Line::from(vec![
            Span::raw("  "),
            Span::styled("bug", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw("           : Yellow - Needs fixing"),
        ]),
        Line::from(vec![
            Span::raw("  "),
            Span::styled("feature", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::raw("       : Green - New feature"),
        ]),
        Line::from(vec![
            Span::raw("  "),
            Span::styled("performance", Style::default().fg(Color::LightGreen).add_modifier(Modifier::BOLD)),
            Span::raw("   : Light Green - Optimization"),
        ]),
        Line::from(vec![
            Span::raw("  "),
            Span::styled("enhancement", Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD)),
            Span::raw("   : Blue - Improvement"),
        ]),
        Line::from(vec![
            Span::raw("  "),
            Span::styled("User", Style::default().fg(Color::LightBlue).add_modifier(Modifier::BOLD)),
            Span::raw("          : Light Blue - User-facing"),
        ]),
        Line::from(vec![
            Span::raw("  "),
            Span::styled("Dev", Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD)),
            Span::raw("           : Magenta - Developer work"),
        ]),
        Line::from(vec![
            Span::raw("  "),
            Span::styled("documentation", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(" : Cyan - Documentation"),
        ]),
        Line::from(vec![
            Span::raw("  "),
            Span::styled("design", Style::default().fg(Color::LightCyan).add_modifier(Modifier::BOLD)),
            Span::raw("        : Light Cyan - UI/UX work"),
        ]),
        Line::from(vec![
            Span::raw("  "),
            Span::styled("refactor", Style::default().fg(Color::LightYellow).add_modifier(Modifier::BOLD)),
            Span::raw("      : Light Yellow - Code quality"),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Other:", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        ]),
        Line::from("  ?     : Show this help"),
        Line::from("  q     : Quit application"),
    ];

    let help_para = Paragraph::new(help_text);
    f.render_widget(help_para, inner);
}

// draw project list view
fn draw_project_list(f: &mut Frame, app: &mut App) {
    let area = f.area();

    let is_adding = app.input_mode == InputMode::AddingProject;

    let title = if is_adding {
        " Projects - ADD NEW (Enter to save, Esc to cancel) "
    } else {
        " Projects (j/k: navigate | Enter: select | a: add | d: delete | Esc: cancel) "
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
        .title(title);

    let inner = block.inner(area);
    f.render_widget(block, area);

    if is_adding {
        // Show input for new project name
        let input_area = Rect {
            x: inner.x,
            y: inner.y,
            width: inner.width,
            height: 3,
        };

        let input_text = vec![
            Line::from(vec![
                Span::styled("New Project Name: ", Style::default().fg(Color::Yellow)),
                Span::raw(&app.input_buffer),
            ]),
        ];

        let input_para = Paragraph::new(input_text)
            .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::Yellow)))
            .style(Style::default().bg(Color::DarkGray));

        f.render_widget(input_para, input_area);
    } else {
        // Show list of projects
        let mut lines = vec![
            Line::from(Span::styled("Select a project:", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))),
            Line::from(""),
        ];

        for (i, project) in app.projects.iter().enumerate() {
            let is_selected = i == app.selected_project_index;
            let is_current = i == app.current_project;

            let mut spans = vec![];

            // Selection indicator
            if is_selected {
                spans.push(Span::styled("> ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)));
            } else {
                spans.push(Span::raw("  "));
            }

            // Project name
            let style = if is_current {
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
            } else if is_selected {
                Style::default().fg(Color::White).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };

            spans.push(Span::styled(&project.name, style));

            // Current indicator
            if is_current {
                spans.push(Span::styled(" (current)", Style::default().fg(Color::DarkGray)));
            }

            lines.push(Line::from(spans));
        }

        let list_para = Paragraph::new(lines);
        f.render_widget(list_para, inner);
    }
}