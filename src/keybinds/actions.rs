use crossterm::event::KeyCode;
use std::collections::HashMap;
use crate::d_io::Output;

/// Represents an action that can be performed by a keybind
#[derive(Debug, Clone)]
pub enum Action {
    /// No action (used for unbound keys)
    NoOp,
    /// Quit the application
    Quit,
    /// Save the current file
    Save,
    /// Find/search in the current file
    Find,
    /// Copy selected text
    Copy,
    /// Cut selected text
    Cut,
    /// Paste from clipboard
    Paste,
    /// Undo last action
    Undo,
    /// Redo last action
    Redo,
    /// Move cursor in a direction
    MoveCursor(KeyCode),
    /// Start text selection
    StartSelection,
    /// Clear text selection
    ClearSelection,
    /// Insert a character
    InsertChar(char),
    /// Insert a newline
    InsertNewline,
    /// Delete a character
    DeleteChar,
    /// Page up/down
    PageUp,
    PageDown,
    /// Custom action with parameters
    Custom(String, HashMap<String, String>),
}

impl Action {
    /// Returns a human-readable name for the action
    pub fn name(&self) -> &str {
        match self {
            Action::NoOp => "NoOp",
            Action::Quit => "Quit",
            Action::Save => "Save",
            Action::Find => "Find",
            Action::Copy => "Copy",
            Action::Cut => "Cut",
            Action::Paste => "Paste",
            Action::Undo => "Undo",
            Action::Redo => "Redo",
            Action::MoveCursor(_) => "MoveCursor",
            Action::StartSelection => "StartSelection",
            Action::ClearSelection => "ClearSelection",
            Action::InsertChar(_) => "InsertChar",
            Action::InsertNewline => "InsertNewline",
            Action::DeleteChar => "DeleteChar",
            Action::PageUp => "PageUp",
            Action::PageDown => "PageDown",
            Action::Custom(name, _) => name,
        }
    }

    /// Returns a description of the action
    pub fn description(&self) -> String {
        match self {
            Action::NoOp => "No operation".to_string(),
            Action::Quit => "Quit the application".to_string(),
            Action::Save => "Save the current file".to_string(),
            Action::Find => "Find/search in the current file".to_string(),
            Action::Copy => "Copy selected text".to_string(),
            Action::Cut => "Cut selected text".to_string(),
            Action::Paste => "Paste from clipboard".to_string(),
            Action::Undo => "Undo last action".to_string(),
            Action::Redo => "Redo last action".to_string(),
            Action::MoveCursor(direction) => format!("Move cursor {:?}", direction),
            Action::StartSelection => "Start text selection".to_string(),
            Action::ClearSelection => "Clear text selection".to_string(),
            Action::InsertChar(ch) => format!("Insert character '{}'", ch),
            Action::InsertNewline => "Insert newline".to_string(),
            Action::DeleteChar => "Delete character".to_string(),
            Action::PageUp => "Page up".to_string(),
            Action::PageDown => "Page down".to_string(),
            Action::Custom(name, params) => {
                if params.is_empty() {
                    format!("Custom action: {}", name)
                } else {
                    format!("Custom action: {} ({:?})", name, params)
                }
            }
        }
    }
}

impl Default for Action {
    fn default() -> Self {
        Action::NoOp
    }
}

/// Registry of available actions
pub struct ActionRegistry {
    actions: HashMap<String, Action>,
}

impl ActionRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            actions: HashMap::new(),
        };
        
        // Register default actions
        registry.register_default_actions();
        
        registry
    }

    fn register_default_actions(&mut self) {
        self.register("quit", Action::Quit);
        self.register("save", Action::Save);
        self.register("find", Action::Find);
        self.register("copy", Action::Copy);
        self.register("cut", Action::Cut);
        self.register("paste", Action::Paste);
        self.register("undo", Action::Undo);
        self.register("redo", Action::Redo);
        self.register("start_selection", Action::StartSelection);
        self.register("clear_selection", Action::ClearSelection);
        self.register("insert_newline", Action::InsertNewline);
        self.register("delete_char", Action::DeleteChar);
        self.register("page_up", Action::PageUp);
        self.register("page_down", Action::PageDown);
        
        // Register movement actions
        self.register("move_up", Action::MoveCursor(KeyCode::Up));
        self.register("move_down", Action::MoveCursor(KeyCode::Down));
        self.register("move_left", Action::MoveCursor(KeyCode::Left));
        self.register("move_right", Action::MoveCursor(KeyCode::Right));
        self.register("move_home", Action::MoveCursor(KeyCode::Home));
        self.register("move_end", Action::MoveCursor(KeyCode::End));
    }

    pub fn register(&mut self, name: &str, action: Action) {
        self.actions.insert(name.to_string(), action);
    }

    pub fn get(&self, name: &str) -> Option<&Action> {
        self.actions.get(name)
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut Action> {
        self.actions.get_mut(name)
    }

    pub fn list(&self) -> Vec<(&String, &Action)> {
        self.actions.iter().collect()
    }
}

impl Default for ActionRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Executes actions on the editor output
pub struct ActionExecutor;

impl ActionExecutor {
    /// Executes an action on the given output
    pub fn execute(action: &Action, output: &mut Output) -> Result<bool, String> {
        match action {
            Action::NoOp => Ok(true),
            Action::Quit => Ok(false), // Return false to quit
            Action::Save => {
                // This will be handled by the editor's save logic
                Ok(true)
            }
            Action::Find => {
                output.find().map_err(|e| e.to_string())?;
                Ok(true)
            }
            Action::Copy => {
                if output.has_selection() {
                    output.copy_selection();
                }
                Ok(true)
            }
            Action::Cut => {
                if output.has_selection() {
                    output.cut_selection();
                }
                Ok(true)
            }
            Action::Paste => {
                output.paste_clipboard();
                Ok(true)
            }
            Action::Undo => {
                output.pop_undo();
                Ok(true)
            }
            Action::Redo => {
                // TODO: Implement redo functionality
                Ok(true)
            }
            Action::MoveCursor(direction) => {
                output.move_cursor(*direction);
                Ok(true)
            }
            Action::StartSelection => {
                output.start_selection();
                Ok(true)
            }
            Action::ClearSelection => {
                output.clear_selection();
                Ok(true)
            }
            Action::InsertChar(ch) => {
                output.insert_char(*ch);
                Ok(true)
            }
            Action::InsertNewline => {
                output.insert_newline();
                Ok(true)
            }
            Action::DeleteChar => {
                output.delete_char();
                Ok(true)
            }
            Action::PageUp => {
                // Handle page up logic
                output.cursor_controller.cursor_y = output.cursor_controller.row_offset;
                for _ in 0..output.win_size.1 {
                    output.move_cursor(KeyCode::Up);
                }
                Ok(true)
            }
            Action::PageDown => {
                // Handle page down logic
                output.cursor_controller.cursor_y = std::cmp::min(
                    output.win_size.1 + output.cursor_controller.row_offset - 1,
                    output.editor_rows.number_of_rows(),
                );
                for _ in 0..output.win_size.1 {
                    output.move_cursor(KeyCode::Down);
                }
                Ok(true)
            }
            Action::Custom(name, _params) => {
                // Handle custom actions
                Err(format!("Custom action '{}' not implemented", name))
            }
        }
    }
} 