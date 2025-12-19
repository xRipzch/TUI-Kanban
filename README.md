# TUI Kanban Board

A simple, lightweight terminal-based kanban board built with Rust. Works on any Linux distribution.

<img width="1896" height="1030" alt="screenshot-2025-12-19_19-10-59" src="https://github.com/user-attachments/assets/359221a8-9e25-46a3-ac01-643b3e35b4d8" />
<img width="1896" height="1030" alt="screenshot-2025-12-19_19-11-17" src="https://github.com/user-attachments/assets/39e8f333-5797-4c4d-93a3-f3a05abd5b8e" />


## Features

- **Four-column kanban board**: To Do, In Progress, Testing, Done
- **Multiple projects**: Organize tasks across different projects with easy switching (Ctrl+P)
- **Tag system**: Categorize tasks with tags (urgent, bug, feature)
- **Color-coded tasks**: Visual distinction based on tags
- **Vim-style navigation**: Use hjkl or arrow keys
- **Task detail view**: Edit titles, add/remove tags, write multi-line descriptions
- **Bi-directional movement**: Move tasks forward and backward through columns
- **Persistent storage**: Tasks are saved automatically to `~/.config/tui-kanban/projects.json`

## Installation

### From AUR (Arch-based distros)

```bash
# Using yay
yay -S tui-kanban-git

# Using paru
paru -S tui-kanban-git

# Manual with makepkg
git clone https://aur.archlinux.org/tui-kanban-git.git
cd tui-kanban-git
makepkg -si
```

### From Source

Requires Rust toolchain (rustc, cargo):

```bash
git clone https://github.com/xRipzch/TUI-Kanban.git
cd TUI-Kanban
cargo build --release
sudo install -Dm755 target/release/tui-kanban /usr/local/bin/tui-kanban
```

## Usage

Run the application:

```bash
tui-kanban
```

### Keyboard Shortcuts

#### Normal Mode
- **h/j/k/l** or **Arrow keys** - Navigate between columns and tasks
- **Enter** - Open task details
- **a** - Add a new task to the selected column
- **t** - Add a tag to the selected task
- **m** - Move task forward (TODO → IN PROGRESS → TESTING → DONE)
- **n** - Move task backward (DONE → TESTING → IN PROGRESS → TODO)
- **d** - Delete the selected task
- **Ctrl+P** - Open project list
- **?** - Show help
- **q** - Quit the application

#### Task Detail View
- **Tab** - Switch between fields (Title, Tags, Description)
- **Enter** - Edit focused field
- **1-9** - Remove tag by number (when Tags field is focused)
- **Esc** - Close task detail view

#### Editing Title/Description
- **Enter** - Save title / Add newline in description
- **Esc** - Save description / Cancel title edit
- **Backspace** - Delete character

#### Project List
- **j/k** or **Arrow keys** - Navigate projects
- **Enter** - Select project
- **a** - Add new project
- **d** - Delete project
- **Esc** - Close project list

### Tags

The following tags have special colors:
- **urgent** - Red
- **bug** - Yellow
- **feature** - Green
- Other tags - White

## Data Storage

Projects and tasks are automatically saved to:
```
~/.config/tui-kanban/projects.json
```

If you're migrating from an older version, your data will be automatically migrated from the old location.

https://github.com/user-attachments/assets/7ce3be1e-343d-4f08-98af-0baf996e2fef



https://github.com/user-attachments/assets/fa467298-e3c5-4770-b4b5-c40280f6f9ab



## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Support

For bugs and feature requests, please create an issue on the [GitHub repository](https://github.com/xRipzch/TUI-Kanban/issues).

---
