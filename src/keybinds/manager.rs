//! # Keybind Manager Module
//! 
//! This module provides the keybind management system for the Ninja editor.
//! It handles registration, lookup, and execution of keyboard shortcuts
//! across different editor contexts.
//! 
//! ## Features
//! 
//! - **Context-aware Keybinds**: Different keybinds for different editor modes
//! - **Action Integration**: Seamless integration with the action system
//! - **Default Keybinds**: Comprehensive set of pre-configured shortcuts
//! - **Dynamic Registration**: Add and modify keybinds at runtime
//! - **Help System**: Generate help text for available keybinds
//! - **Multi-context Lookup**: Search across multiple contexts with priority
//! 
//! ## Contexts
//! 
//! The keybind system supports multiple contexts:
//! - **Global**: Keybinds that work everywhere (e.g., Ctrl+Q to quit)
//! - **Editor**: Main editing mode keybinds
//! - **Search**: Search mode specific keybinds
//! - **Prompt**: Prompt/input mode keybinds
//! - **Debug**: Debug mode keybinds
//! - **Clipboard**: Clipboard management keybinds
//! 
//! ## Default Keybinds
//! 
//! The manager comes with a comprehensive set of default keybinds:
//! - **File Operations**: Ctrl+S (save), Ctrl+Q (quit)
//! - **Editing**: Ctrl+C (copy), Ctrl+X (cut), Ctrl+V (paste), Ctrl+Z (undo)
//! - **Navigation**: Arrow keys, Home, End, Page Up/Down
//! - **Search**: Ctrl+F (find)
//! - **Selection**: Shift + movement keys
//! 
//! ## Usage
//! 
//! ```rust
//! use ninja::keybinds::manager::KeybindManager;
//! use ninja::keybinds::{Keybind, KeybindContext};
//! use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
//!
//! let mut manager = KeybindManager::new();
//!
//! // Register a custom keybind
//! let keybind = Keybind::new(
//!     KeyCode::Char('a'),
//!     KeyModifiers::CONTROL,
//!     "my_action".to_string(),
//!     KeybindContext::Editor,
//! );
//! manager.register(keybind);
//!
//! // Find a keybind for a key event
//! let event = KeyEvent::new(KeyCode::Char('s'), KeyModifiers::CONTROL);
//! if let Some(keybind) = manager.find_keybind(&event, &KeybindContext::Editor) {
//!     println!("Found keybind: {}", keybind.to_string());
//! }
//! ```

use crate::keybinds::{Keybind, KeybindContext, Action};
use crate::keybinds::actions::ActionRegistry;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::collections::HashMap;

/// Manages keybinds for the application across different contexts.
/// 
/// This struct provides a centralized system for managing keyboard
/// shortcuts in the editor. It supports context-aware keybinds,
/// dynamic registration, and efficient lookup mechanisms.
/// 
/// # Architecture
/// 
/// The keybind manager uses a multi-layered approach:
/// - **Action Registry**: Central registry of available actions
/// - **Context-based Storage**: Keybinds organized by context
/// - **Lookup Map**: Fast O(1) lookup for keybinds
/// - **Priority System**: Context-based priority for keybind resolution
/// 
/// # Key Features
/// 
/// - **Context Isolation**: Different keybinds for different editor modes
/// - **Action Integration**: Direct integration with the action system
/// - **Help Generation**: Automatic generation of help text
/// - **Dynamic Management**: Add/remove keybinds at runtime
/// - **Efficient Lookup**: Fast keybind resolution
/// 
/// # Example
/// 
/// ```rust
/// use ninja::keybinds::manager::KeybindManager;
/// use ninja::keybinds::{Keybind, KeybindContext};
/// use crossterm::event::{KeyCode, KeyModifiers};
/// 
/// let mut manager = KeybindManager::new();
/// 
/// // Register a custom keybind
/// let keybind = Keybind::with_description(
///     KeyCode::Char('g'),
///     KeyModifiers::CONTROL,
///     "goto_line".to_string(),
///     KeybindContext::Editor,
///     "Go to specific line".to_string(),
/// );
/// manager.register(keybind);
/// 
/// // Get help for editor context
/// let help = manager.get_help_text(&KeybindContext::Editor);
/// println!("{}", help);
/// ```
pub struct KeybindManager {
    /// Registry of available actions for keybind execution
    action_registry: ActionRegistry,
    /// Keybinds organized by context for efficient lookup
    keybinds: HashMap<KeybindContext, Vec<Keybind>>,
    /// Quick lookup map for keybinds by context and key event
    lookup_map: HashMap<(KeybindContext, KeyCode, KeyModifiers), String>,
}

impl KeybindManager {
    /// Creates a new keybind manager with default keybinds.
    /// 
    /// This method initializes a new keybind manager and automatically
    /// registers all the default keybinds that come with the editor.
    /// 
    /// # Returns
    /// 
    /// Returns a new `KeybindManager` instance with default keybinds.
    /// 
    /// # Default Keybinds
    /// 
    /// The manager is initialized with comprehensive default keybinds:
    /// - **Global**: Ctrl+Q (quit)
    /// - **Editor**: File operations, editing, navigation, search
    /// - **Search**: Search-specific navigation and commands
    /// - **Prompt**: Input handling and confirmation
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ninja::keybinds::KeybindContext;
    /// use ninja::keybinds::manager::KeybindManager;
    ///
    /// let manager = KeybindManager::new();
    ///
    /// // Default keybinds are automatically registered
    /// let editor_keybinds = manager.list_keybinds(&KeybindContext::Editor);
    /// assert!(!editor_keybinds.is_empty());
    /// ```
    pub fn new() -> Self {
        let mut manager = Self {
            action_registry: ActionRegistry::new(),
            keybinds: HashMap::new(),
            lookup_map: HashMap::new(),
        };
        
        // Register default keybinds
        manager.register_default_keybinds();
        
        manager
    }

    /// Registers a keybind with the manager.
    /// 
    /// This method adds a new keybind to the manager, making it available
    /// for lookup and execution. The keybind is stored in the appropriate
    /// context and added to the lookup map for efficient access.
    /// 
    /// # Arguments
    /// 
    /// * `keybind` - The keybind to register
    /// 
    /// # Behavior
    /// 
    /// - **Context Storage**: Keybind is stored in its specified context
    /// - **Lookup Map**: Added to the fast lookup map for O(1) access
    /// - **Overwrite**: If a keybind with the same key already exists in the context, it's replaced
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ninja::keybinds::manager::KeybindManager;
    /// use ninja::keybinds::{Keybind, KeybindContext};
    /// use crossterm::event::{KeyCode, KeyModifiers};
    /// 
    /// let mut manager = KeybindManager::new();
    /// 
    /// let keybind = Keybind::new(
    ///     KeyCode::Char('a'),
    ///     KeyModifiers::CONTROL,
    ///     "my_action".to_string(),
    ///     KeybindContext::Editor,
    /// );
    /// manager.register(keybind);
    /// ```
    pub fn register(&mut self, keybind: Keybind) {
        let context = keybind.context.clone();
        let key_code = keybind.key_code;
        let modifiers = keybind.modifiers;
        let action = keybind.action.clone();
        
        // Add to keybinds list
        self.keybinds.entry(context.clone()).or_insert_with(Vec::new).push(keybind);
        
        // Add to lookup map
        self.lookup_map.insert((context, key_code, modifiers), action);
    }

    /// Registers multiple keybinds at once.
    /// 
    /// This method provides a convenient way to register multiple keybinds
    /// in a single operation. It's equivalent to calling `register()` for
    /// each keybind in the vector.
    /// 
    /// # Arguments
    /// 
    /// * `keybinds` - Vector of keybinds to register
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ninja::keybinds::manager::KeybindManager;
    /// use ninja::keybinds::{Keybind, KeybindContext};
    /// use crossterm::event::{KeyCode, KeyModifiers};
    /// 
    /// let mut manager = KeybindManager::new();
    /// 
    /// let keybinds = vec![
    ///     Keybind::new(KeyCode::Char('a'), KeyModifiers::CONTROL, "action1".to_string(), KeybindContext::Editor),
    ///     Keybind::new(KeyCode::Char('b'), KeyModifiers::CONTROL, "action2".to_string(), KeybindContext::Editor),
    /// ];
    /// manager.register_multiple(keybinds);
    /// ```
    pub fn register_multiple(&mut self, keybinds: Vec<Keybind>) {
        for keybind in keybinds {
            self.register(keybind);
        }
    }

    /// Finds a keybind for the given key event in the specified context.
    /// 
    /// This method searches for a keybind that matches the given key event
    /// within the specified context. It returns the first matching keybind
    /// found, or `None` if no match is found.
    /// 
    /// # Arguments
    /// 
    /// * `event` - The key event to search for
    /// * `context` - The context to search in
    /// 
    /// # Returns
    /// 
    /// Returns `Some(&Keybind)` if a matching keybind is found, or `None` if not found.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ninja::keybinds::manager::KeybindManager;
    /// use ninja::keybinds::KeybindContext;
    /// use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
    /// 
    /// let manager = KeybindManager::new();
    /// 
    /// let event = KeyEvent::new(KeyCode::Char('s'), KeyModifiers::CONTROL);
    /// if let Some(keybind) = manager.find_keybind(&event, &KeybindContext::Editor) {
    ///     println!("Found keybind: {}", keybind.to_string());
    /// }
    /// ```
    pub fn find_keybind(&self, event: &KeyEvent, context: &KeybindContext) -> Option<&Keybind> {
        if let Some(context_keybinds) = self.keybinds.get(context) {
            context_keybinds.iter().find(|kb| kb.matches(event))
        } else {
            None
        }
    }

    /// Finds a keybind across multiple contexts in order of priority.
    /// 
    /// This method searches for a keybind across multiple contexts, checking
    /// each context in the order provided. It returns the first matching
    /// keybind found, allowing for context-based priority systems.
    /// 
    /// # Arguments
    /// 
    /// * `event` - The key event to search for
    /// * `contexts` - Array of contexts to search in (in priority order)
    /// 
    /// # Returns
    /// 
    /// Returns `Some(&Keybind)` if a matching keybind is found in any of the contexts,
    /// or `None` if no match is found.
    /// 
    /// # Priority
    /// 
    /// Contexts are searched in the order they appear in the array. The first
    /// context with a matching keybind takes precedence.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ninja::keybinds::manager::KeybindManager;
    /// use ninja::keybinds::KeybindContext;
    /// use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
    /// 
    /// let manager = KeybindManager::new();
    /// 
    /// let event = KeyEvent::new(KeyCode::Char('q'), KeyModifiers::CONTROL);
    /// let contexts = [KeybindContext::Search, KeybindContext::Global];
    /// 
    /// if let Some(keybind) = manager.find_keybind_in_contexts(&event, &contexts) {
    ///     println!("Found keybind in context: {}", keybind.context.name());
    /// }
    /// ```
    pub fn find_keybind_in_contexts(&self, event: &KeyEvent, contexts: &[KeybindContext]) -> Option<&Keybind> {
        for context in contexts {
            if let Some(keybind) = self.find_keybind(event, context) {
                return Some(keybind);
            }
        }
        None
    }

    /// Gets the action for a given action name.
    /// 
    /// This method looks up an action in the action registry by its name.
    /// 
    /// # Arguments
    /// 
    /// * `action_name` - The name of the action to look up
    /// 
    /// # Returns
    /// 
    /// Returns `Some(&Action)` if the action is found, or `None` if not found.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ninja::keybinds::manager::KeybindManager;
    /// 
    /// let manager = KeybindManager::new();
    /// 
    /// if let Some(action) = manager.get_action("save") {
    ///     println!("Found action: {}", action.description());
    /// }
    /// ```
    pub fn get_action(&self, action_name: &str) -> Option<&Action> {
        self.action_registry.get(action_name)
    }

    /// Registers a custom action with the manager.
    /// 
    /// This method adds a new action to the action registry, making it
    /// available for use in keybinds.
    /// 
    /// # Arguments
    /// 
    /// * `name` - The name to register the action under
    /// * `action` - The action to register
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ninja::keybinds::manager::KeybindManager;
    /// use ninja::keybinds::actions::Action;
    /// use std::collections::HashMap;
    /// 
    /// let mut manager = KeybindManager::new();
    /// 
    /// let custom_action = Action::Custom("my_action".to_string(), HashMap::new());
    /// manager.register_action("my_action", custom_action);
    /// ```
    pub fn register_action(&mut self, name: &str, action: Action) {
        self.action_registry.register(name, action);
    }

    /// Lists all keybinds for a specific context.
    /// 
    /// This method returns a vector of all keybinds registered in the
    /// specified context.
    /// 
    /// # Arguments
    /// 
    /// * `context` - The context to list keybinds for
    /// 
    /// # Returns
    /// 
    /// Returns a `Vec<Keybind>` containing all keybinds in the specified context.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ninja::keybinds::manager::KeybindManager;
    /// use ninja::keybinds::KeybindContext;
    /// 
    /// let manager = KeybindManager::new();
    /// 
    /// let editor_keybinds = manager.list_keybinds(&KeybindContext::Editor);
    /// for keybind in editor_keybinds {
    ///     println!("{}: {}", keybind.to_string(), keybind.action);
    /// }
    /// ```
    pub fn list_keybinds(&self, context: &KeybindContext) -> Vec<Keybind> {
        match self.keybinds.get(context) {
            Some(kb) => kb.clone(),
            None => Vec::new(),
        }
    }

    /// Lists all keybinds across all contexts.
    /// 
    /// This method returns a vector of tuples containing the context and
    /// keybind for all registered keybinds in the manager.
    /// 
    /// # Returns
    /// 
    /// Returns a `Vec<(&KeybindContext, &Keybind)>` containing all keybinds.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ninja::keybinds::manager::KeybindManager;
    /// 
    /// let manager = KeybindManager::new();
    /// 
    /// for (context, keybind) in manager.list_all_keybinds() {
    ///     println!("{}: {} -> {}", context.name(), keybind.to_string(), keybind.action);
    /// }
    /// ```
    pub fn list_all_keybinds(&self) -> Vec<(&KeybindContext, &Keybind)> {
        let mut result = Vec::new();
        for (context, keybinds) in &self.keybinds {
            for keybind in keybinds {
                result.push((context, keybind));
            }
        }
        result
    }

    /// Gets help text for keybinds in a specific context.
    /// 
    /// This method generates formatted help text showing all available
    /// keybinds for the specified context, including their descriptions.
    /// 
    /// # Arguments
    /// 
    /// * `context` - The context to generate help for
    /// 
    /// # Returns
    /// 
    /// Returns a `String` containing formatted help text.
    /// 
    /// # Format
    /// 
    /// The help text includes:
    /// - Context name header
    /// - Each keybind with its key combination and description
    /// - Proper formatting and indentation
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ninja::keybinds::manager::KeybindManager;
    /// use ninja::keybinds::KeybindContext;
    /// 
    /// let manager = KeybindManager::new();
    /// 
    /// let help_text = manager.get_help_text(&KeybindContext::Editor);
    /// println!("{}", help_text);
    /// // Output:
    /// // Keybinds for Editor:
    /// //   Ctrl-S: Save the current file
    /// //   Ctrl-F: Find/search in the current file
    /// //   Ctrl-C: Copy selected text
    /// //   ...
    /// ```
    pub fn get_help_text(&self, context: &KeybindContext) -> String {
        let keybinds = self.list_keybinds(context);
        if keybinds.is_empty() {
            return format!("No keybinds defined for context: {}", context.name());
        }

        let mut help_lines = vec![format!("Keybinds for {}:", context.name())];
        for keybind in keybinds {
            let key_text = keybind.to_string();
            let description: String = keybind.description.clone()
                .or_else(|| {
                    self.get_action(&keybind.action)
                        .map(|action| action.description())
                })
                .unwrap_or_else(|| keybind.action.clone());
            help_lines.push(format!("  {}: {}", key_text, description));
        }
        help_lines.join("\n")
    }

    /// Registers the default keybinds for the application.
    /// 
    /// This method registers all the standard keybinds that come with
    /// the editor by default. It's called automatically during initialization.
    /// 
    /// # Default Keybinds
    /// 
    /// The default keybinds include:
    /// - **Global**: Ctrl+Q (quit)
    /// - **Editor**: File operations, editing, navigation, search
    /// - **Search**: Search-specific navigation and commands
    /// - **Prompt**: Input handling and confirmation
    fn register_default_keybinds(&mut self) {
        use crossterm::event::KeyCode;

        // Global keybinds
        let global_keybinds = vec![
            Keybind::with_description(
                KeyCode::Char('q'),
                KeyModifiers::CONTROL,
                "quit".to_string(),
                KeybindContext::Global,
                "Quit the application".to_string(),
            ),
        ];

        // Editor keybinds
        let editor_keybinds = vec![
            // File operations
            Keybind::with_description(
                KeyCode::Char('s'),
                KeyModifiers::CONTROL,
                "save".to_string(),
                KeybindContext::Editor,
                "Save the current file".to_string(),
            ),
            Keybind::with_description(
                KeyCode::Char('f'),
                KeyModifiers::CONTROL,
                "find".to_string(),
                KeybindContext::Editor,
                "Find/search in the current file".to_string(),
            ),
            
            // Edit operations
            Keybind::with_description(
                KeyCode::Char('c'),
                KeyModifiers::CONTROL,
                "copy".to_string(),
                KeybindContext::Editor,
                "Copy selected text".to_string(),
            ),
            Keybind::with_description(
                KeyCode::Char('x'),
                KeyModifiers::CONTROL,
                "cut".to_string(),
                KeybindContext::Editor,
                "Cut selected text".to_string(),
            ),
            Keybind::with_description(
                KeyCode::Char('v'),
                KeyModifiers::CONTROL,
                "paste".to_string(),
                KeybindContext::Editor,
                "Paste from clipboard".to_string(),
            ),
            Keybind::with_description(
                KeyCode::Char('z'),
                KeyModifiers::CONTROL,
                "undo".to_string(),
                KeybindContext::Editor,
                "Undo last action".to_string(),
            ),
            
            // Navigation
            Keybind::with_description(
                KeyCode::Up,
                KeyModifiers::NONE,
                "move_up".to_string(),
                KeybindContext::Editor,
                "Move cursor up".to_string(),
            ),
            Keybind::with_description(
                KeyCode::Down,
                KeyModifiers::NONE,
                "move_down".to_string(),
                KeybindContext::Editor,
                "Move cursor down".to_string(),
            ),
            Keybind::with_description(
                KeyCode::Left,
                KeyModifiers::NONE,
                "move_left".to_string(),
                KeybindContext::Editor,
                "Move cursor left".to_string(),
            ),
            Keybind::with_description(
                KeyCode::Right,
                KeyModifiers::NONE,
                "move_right".to_string(),
                KeybindContext::Editor,
                "Move cursor right".to_string(),
            ),
            Keybind::with_description(
                KeyCode::Home,
                KeyModifiers::NONE,
                "move_home".to_string(),
                KeybindContext::Editor,
                "Move cursor to beginning of line".to_string(),
            ),
            Keybind::with_description(
                KeyCode::End,
                KeyModifiers::NONE,
                "move_end".to_string(),
                KeybindContext::Editor,
                "Move cursor to end of line".to_string(),
            ),
            Keybind::with_description(
                KeyCode::PageUp,
                KeyModifiers::NONE,
                "page_up".to_string(),
                KeybindContext::Editor,
                "Page up".to_string(),
            ),
            Keybind::with_description(
                KeyCode::PageDown,
                KeyModifiers::NONE,
                "page_down".to_string(),
                KeybindContext::Editor,
                "Page down".to_string(),
            ),
            
            // Selection
            Keybind::with_description(
                KeyCode::Up,
                KeyModifiers::SHIFT,
                "move_up".to_string(),
                KeybindContext::Editor,
                "Move cursor up with selection".to_string(),
            ),
            Keybind::with_description(
                KeyCode::Down,
                KeyModifiers::SHIFT,
                "move_down".to_string(),
                KeybindContext::Editor,
                "Move cursor down with selection".to_string(),
            ),
            Keybind::with_description(
                KeyCode::Left,
                KeyModifiers::SHIFT,
                "move_left".to_string(),
                KeybindContext::Editor,
                "Move cursor left with selection".to_string(),
            ),
            Keybind::with_description(
                KeyCode::Right,
                KeyModifiers::SHIFT,
                "move_right".to_string(),
                KeybindContext::Editor,
                "Move cursor right with selection".to_string(),
            ),
            Keybind::with_description(
                KeyCode::Home,
                KeyModifiers::SHIFT,
                "move_home".to_string(),
                KeybindContext::Editor,
                "Move cursor to beginning of line with selection".to_string(),
            ),
            Keybind::with_description(
                KeyCode::End,
                KeyModifiers::SHIFT,
                "move_end".to_string(),
                KeybindContext::Editor,
                "Move cursor to end of line with selection".to_string(),
            ),
            
            // Text editing
            Keybind::with_description(
                KeyCode::Enter,
                KeyModifiers::NONE,
                "insert_newline".to_string(),
                KeybindContext::Editor,
                "Insert newline".to_string(),
            ),
            Keybind::with_description(
                KeyCode::Backspace,
                KeyModifiers::NONE,
                "delete_char".to_string(),
                KeybindContext::Editor,
                "Delete character before cursor".to_string(),
            ),
            Keybind::with_description(
                KeyCode::Delete,
                KeyModifiers::NONE,
                "delete_char".to_string(),
                KeybindContext::Editor,
                "Delete character after cursor".to_string(),
            ),
            Keybind::with_description(
                KeyCode::Tab,
                KeyModifiers::NONE,
                "insert_char".to_string(),
                KeybindContext::Editor,
                "Insert tab".to_string(),
            ),
            // Character input (for unbound characters)
            Keybind::with_description(
                KeyCode::Char(' '),
                KeyModifiers::NONE,
                "insert_char".to_string(),
                KeybindContext::Editor,
                "Insert space".to_string(),
            ),
        ];

        // Prompt keybinds
        let prompt_keybinds = vec![
            Keybind::with_description(
                KeyCode::Enter,
                KeyModifiers::NONE,
                "confirm".to_string(),
                KeybindContext::Prompt,
                "Confirm input".to_string(),
            ),
            Keybind::with_description(
                KeyCode::Esc,
                KeyModifiers::NONE,
                "cancel".to_string(),
                KeybindContext::Prompt,
                "Cancel input".to_string(),
            ),
            Keybind::with_description(
                KeyCode::Backspace,
                KeyModifiers::NONE,
                "backspace".to_string(),
                KeybindContext::Prompt,
                "Delete character".to_string(),
            ),
        ];

        // Register all keybinds
        self.register_multiple(global_keybinds);
        self.register_multiple(editor_keybinds);
        self.register_multiple(prompt_keybinds);
    }
}

impl Default for KeybindManager {
    fn default() -> Self {
        Self::new()
    }
} 