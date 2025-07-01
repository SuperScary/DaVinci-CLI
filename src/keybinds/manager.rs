use crate::keybinds::{Keybind, KeybindContext, Action};
use crate::keybinds::actions::ActionRegistry;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::collections::HashMap;

/// Manages keybinds for the application
pub struct KeybindManager {
    /// Registry of available actions
    action_registry: ActionRegistry,
    /// Keybinds organized by context
    keybinds: HashMap<KeybindContext, Vec<Keybind>>,
    /// Quick lookup map for keybinds by context and key event
    lookup_map: HashMap<(KeybindContext, KeyCode, KeyModifiers), String>,
}

impl KeybindManager {
    /// Creates a new keybind manager with default keybinds
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

    /// Registers a keybind
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

    /// Registers multiple keybinds at once
    pub fn register_multiple(&mut self, keybinds: Vec<Keybind>) {
        for keybind in keybinds {
            self.register(keybind);
        }
    }

    /// Finds a keybind for the given key event in the specified context
    pub fn find_keybind(&self, event: &KeyEvent, context: &KeybindContext) -> Option<&Keybind> {
        if let Some(context_keybinds) = self.keybinds.get(context) {
            context_keybinds.iter().find(|kb| kb.matches(event))
        } else {
            None
        }
    }

    /// Finds a keybind across multiple contexts (in order of priority)
    pub fn find_keybind_in_contexts(&self, event: &KeyEvent, contexts: &[KeybindContext]) -> Option<&Keybind> {
        for context in contexts {
            if let Some(keybind) = self.find_keybind(event, context) {
                return Some(keybind);
            }
        }
        None
    }

    /// Gets the action for a keybind
    pub fn get_action(&self, action_name: &str) -> Option<&Action> {
        self.action_registry.get(action_name)
    }

    /// Registers a custom action
    pub fn register_action(&mut self, name: &str, action: Action) {
        self.action_registry.register(name, action);
    }

    /// Lists all keybinds for a specific context
    pub fn list_keybinds(&self, context: &KeybindContext) -> Vec<Keybind> {
        match self.keybinds.get(context) {
            Some(kb) => kb.clone(),
            None => Vec::new(),
        }
    }

    /// Lists all keybinds across all contexts
    pub fn list_all_keybinds(&self) -> Vec<(&KeybindContext, &Keybind)> {
        let mut result = Vec::new();
        for (context, keybinds) in &self.keybinds {
            for keybind in keybinds {
                result.push((context, keybind));
            }
        }
        result
    }

    /// Gets help text for keybinds in a specific context
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

    /// Registers the default keybinds for the application
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