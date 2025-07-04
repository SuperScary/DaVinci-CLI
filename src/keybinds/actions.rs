//! # Action System Module
//! 
//! This module provides the action system for the Ninja editor's keyboard binding
//! functionality. It defines all available actions, provides action execution,
//! and manages action registration and lookup.
//! 
//! ## Features
//! 
//! - **Action Definitions**: Comprehensive set of editor actions
//! - **Action Registry**: Central registry for action lookup and management
//! - **Action Execution**: Safe execution of actions on editor state
//! - **Custom Actions**: Support for user-defined custom actions
//! - **Action Metadata**: Human-readable names and descriptions
//! 
//! ## Action Types
//! 
//! The action system supports several categories of actions:
//! - **Navigation**: Cursor movement and page navigation
//! - **Editing**: Text insertion, deletion, and modification
//! - **File Operations**: Save, quit, and file management
//! - **Selection**: Text selection and manipulation
//! - **Clipboard**: Copy, cut, and paste operations
//! - **History**: Undo and redo functionality
//! - **Search**: Find and search operations
//! - **Custom**: User-defined actions with parameters
//! 
//! ## Usage
//! 
//! ```rust
//! use ninja::config::NinjaConfig;
//! use ninja::keybinds::actions::{Action, ActionRegistry, ActionExecutor};
//! use ninja::transput::transput::Output;
//!
//! let mut registry = ActionRegistry::new();
//! let action = Action::Save;
//! let mut output = Output::new(NinjaConfig::default());
//!
//! // Execute an action
//! let result = ActionExecutor::execute(&action, &mut output)?;
//! ```

use crossterm::event::KeyCode;
use std::collections::HashMap;
use crate::transput::transput::Output;

/// Represents an action that can be performed by a keybind.
/// 
/// This enum defines all the different types of actions that can be
/// bound to keyboard shortcuts in the editor. Actions range from
/// simple cursor movements to complex file operations.
/// 
/// # Action Categories
/// 
/// - **No Operation**: `NoOp` - Does nothing (for unbound keys)
/// - **Application Control**: `Quit` - Exit the application
/// - **File Operations**: `Save` - Save the current file
/// - **Search**: `Find` - Initiate search functionality
/// - **Clipboard**: `Copy`, `Cut`, `Paste` - Clipboard operations
/// - **History**: `Undo`, `Redo` - Undo/redo operations
/// - **Navigation**: `MoveCursor`, `PageUp`, `PageDown` - Movement
/// - **Selection**: `StartSelection`, `ClearSelection` - Text selection
/// - **Editing**: `InsertChar`, `InsertNewline`, `DeleteChar` - Text editing
/// - **Custom**: `Custom` - User-defined actions with parameters
/// 
/// # Example
/// 
/// ```rust
/// use std::collections::HashMap;
/// use ninja::keybinds::actions::Action;
/// use crossterm::event::KeyCode;
///
/// let quit_action = Action::Quit;
/// let move_action = Action::MoveCursor(KeyCode::Up);
/// let custom_action = Action::Custom("my_action".to_string(), HashMap::new());
/// ```
#[derive(Debug, Clone)]
pub enum Action {
    /// No operation (used for unbound keys)
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
    /// Returns a human-readable name for the action.
    /// 
    /// This method provides a short, descriptive name for each action
    /// that can be used in user interfaces, configuration files, or
    /// debugging output.
    /// 
    /// # Returns
    /// 
    /// Returns a string slice containing the action name.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ninja::keybinds::actions::Action;
    /// 
    /// assert_eq!(Action::Quit.name(), "Quit");
    /// assert_eq!(Action::Save.name(), "Save");
    /// assert_eq!(Action::Copy.name(), "Copy");
    /// ```
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

    /// Returns a detailed description of the action.
    /// 
    /// This method provides a longer, more descriptive explanation of
    /// what each action does, including any parameters or context.
    /// 
    /// # Returns
    /// 
    /// Returns a `String` containing the action description.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ninja::keybinds::actions::Action;
    /// 
    /// assert_eq!(Action::Quit.description(), "Quit the application");
    /// assert_eq!(Action::Save.description(), "Save the current file");
    /// assert_eq!(Action::Copy.description(), "Copy selected text");
    /// ```
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
    /// Returns the default action (NoOp).
    /// 
    /// # Returns
    /// 
    /// Returns `Action::NoOp` as the default action.
    fn default() -> Self {
        Action::NoOp
    }
}

/// Registry of available actions for lookup and management.
/// 
/// This struct provides a centralized registry for all available actions
/// in the editor. It allows for dynamic registration, lookup, and
/// management of actions by name.
/// 
/// # Features
/// 
/// - **Action Registration**: Register new actions by name
/// - **Action Lookup**: Find actions by name
/// - **Default Actions**: Pre-registered set of common actions
/// - **Action Listing**: List all registered actions
/// 
/// # Default Actions
/// 
/// The registry is initialized with a comprehensive set of default actions:
/// - **File Operations**: `quit`, `save`
/// - **Search**: `find`
/// - **Clipboard**: `copy`, `cut`, `paste`
/// - **History**: `undo`, `redo`
/// - **Selection**: `start_selection`, `clear_selection`
/// - **Editing**: `insert_newline`, `delete_char`
/// - **Navigation**: `page_up`, `page_down`
/// - **Movement**: `move_up`, `move_down`, `move_left`, `move_right`, `move_home`, `move_end`
/// 
/// # Example
/// 
/// ```rust
/// use std::collections::HashMap;
/// use ninja::keybinds::actions::{ActionRegistry, Action};
///
/// let mut registry = ActionRegistry::new();
///
/// // Register a custom action
/// registry.register("my_action", Action::Custom("my_action".to_string(), HashMap::new()));
///
/// // Look up an action
/// if let Some(action) = registry.get("save") {
///     println!("Found action: {}", action.name());
/// }
///
/// // List all actions
/// for (name, action) in registry.list() {
///     println!("{}: {}", name, action.description());
/// }
/// ```
pub struct ActionRegistry {
    /// Internal storage of actions mapped by name
    actions: HashMap<String, Action>,
}

impl ActionRegistry {
    /// Creates a new action registry with default actions.
    /// 
    /// This method initializes a new registry and automatically
    /// registers all the default actions that come with the editor.
    /// 
    /// # Returns
    /// 
    /// Returns a new `ActionRegistry` instance with default actions.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ninja::keybinds::actions::ActionRegistry;
    /// 
    /// let registry = ActionRegistry::new();
    /// assert!(registry.get("save").is_some());
    /// assert!(registry.get("quit").is_some());
    /// ```
    pub fn new() -> Self {
        let mut registry = Self {
            actions: HashMap::new(),
        };
        
        // Register default actions
        registry.register_default_actions();
        
        registry
    }

    /// Registers the default set of actions.
    /// 
    /// This method registers all the standard actions that are
    /// available by default in the editor.
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

    /// Registers a new action with the given name.
    /// 
    /// This method adds a new action to the registry, making it
    /// available for lookup and execution. If an action with the
    /// same name already exists, it will be overwritten.
    /// 
    /// # Arguments
    /// 
    /// * `name` - The name to register the action under
    /// * `action` - The action to register
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ninja::keybinds::actions::{ActionRegistry, Action};
    /// use std::collections::HashMap;
    /// 
    /// let mut registry = ActionRegistry::new();
    /// let custom_action = Action::Custom("my_action".to_string(), HashMap::new());
    /// registry.register("my_action", custom_action);
    /// ```
    pub fn register(&mut self, name: &str, action: Action) {
        self.actions.insert(name.to_string(), action);
    }

    /// Retrieves an action by name.
    /// 
    /// This method looks up an action in the registry by its name.
    /// 
    /// # Arguments
    /// 
    /// * `name` - The name of the action to look up
    /// 
    /// # Returns
    /// 
    /// Returns `Some(&Action)` if the action is found, or `None` if not found.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ninja::keybinds::actions::ActionRegistry;
    /// 
    /// let registry = ActionRegistry::new();
    /// 
    /// if let Some(action) = registry.get("save") {
    ///     println!("Found save action: {}", action.description());
    /// }
    /// ```
    pub fn get(&self, name: &str) -> Option<&Action> {
        self.actions.get(name)
    }

    /// Retrieves a mutable reference to an action by name.
    /// 
    /// This method looks up an action in the registry and returns
    /// a mutable reference, allowing the action to be modified.
    /// 
    /// # Arguments
    /// 
    /// * `name` - The name of the action to look up
    /// 
    /// # Returns
    /// 
    /// Returns `Some(&mut Action)` if the action is found, or `None` if not found.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ninja::keybinds::actions::ActionRegistry;
    /// 
    /// let mut registry = ActionRegistry::new();
    /// 
    /// if let Some(action) = registry.get_mut("save") {
    ///     // Modify the action if needed
    ///     println!("Modified save action");
    /// }
    /// ```
    pub fn get_mut(&mut self, name: &str) -> Option<&mut Action> {
        self.actions.get_mut(name)
    }

    /// Returns a list of all registered actions.
    /// 
    /// This method returns a vector of tuples containing the name
    /// and reference to each registered action.
    /// 
    /// # Returns
    /// 
    /// Returns a `Vec<(&String, &Action)>` containing all registered actions.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ninja::keybinds::actions::ActionRegistry;
    /// 
    /// let registry = ActionRegistry::new();
    /// 
    /// for (name, action) in registry.list() {
    ///     println!("{}: {}", name, action.description());
    /// }
    /// ```
    pub fn list(&self) -> Vec<(&String, &Action)> {
        self.actions.iter().collect()
    }
}

impl Default for ActionRegistry {
    /// Returns a default action registry.
    /// 
    /// # Returns
    /// 
    /// Returns a new `ActionRegistry` instance with default actions.
    fn default() -> Self {
        Self::new()
    }
}

/// Executes actions on the editor output.
/// 
/// This struct provides the functionality to execute actions on the
/// editor's output state. It handles the translation from abstract
/// actions to concrete editor operations.
/// 
/// # Execution Behavior
/// 
/// - **Success**: Returns `Ok(true)` when action completes successfully
/// - **Quit**: Returns `Ok(false)` when the action should quit the editor
/// - **Error**: Returns `Err(String)` when the action fails
/// 
/// # Example
/// 
/// ```rust
/// use ninja::keybinds::actions::{Action, ActionExecutor};
/// use ninja::transput::transput::Output;
/// use ninja::config::NinjaConfig;
/// 
/// let mut output = Output::new(NinjaConfig::default());
/// let action = Action::Save;
/// 
/// match ActionExecutor::execute(&action, &mut output) {
///     Ok(true) => println!("Action completed successfully"),
///     Ok(false) => println!("Editor should quit"),
///     Err(e) => println!("Action failed: {}", e),
/// }
/// ```
pub struct ActionExecutor;

impl ActionExecutor {
    /// Executes an action on the given output.
    /// 
    /// This method takes an action and applies it to the editor's
    /// output state. It handles all the different types of actions
    /// and translates them into appropriate editor operations.
    /// 
    /// # Arguments
    /// 
    /// * `action` - The action to execute
    /// * `output` - The editor output to operate on
    /// 
    /// # Returns
    /// 
    /// Returns `Ok(bool)` where:
    /// - `true` means the action completed successfully and the editor should continue
    /// - `false` means the editor should quit (e.g., for `Action::Quit`)
    /// 
    /// Returns `Err(String)` if the action fails with an error message.
    /// 
    /// # Action Execution
    /// 
    /// Each action type is handled appropriately:
    /// - **NoOp**: Does nothing, returns success
    /// - **Quit**: Returns false to signal editor shutdown
    /// - **Save**: Triggers file save operation
    /// - **Find**: Initiates search functionality
    /// - **Clipboard**: Performs copy/cut/paste operations
    /// - **Navigation**: Moves cursor or pages
    /// - **Editing**: Inserts or deletes text
    /// - **Selection**: Manages text selection
    /// - **History**: Performs undo/redo operations
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ninja::keybinds::actions::{Action, ActionExecutor};
    /// use ninja::transput::transput::Output;
    /// use ninja::config::NinjaConfig;
    /// 
    /// let mut output = Output::new(NinjaConfig::default());
    /// 
    /// // Execute a save action
    /// let result = ActionExecutor::execute(&Action::Save, &mut output);
    /// assert!(result.is_ok());
    /// 
    /// // Execute a quit action
    /// let result = ActionExecutor::execute(&Action::Quit, &mut output);
    /// assert_eq!(result, Ok(false));
    /// ```
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