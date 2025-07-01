use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::collections::HashMap;

/// Represents a keybind with its associated action and context
#[derive(Debug, Clone)]
pub struct Keybind {
    pub key_code: KeyCode,
    pub modifiers: KeyModifiers,
    pub action: String,
    pub context: KeybindContext,
    pub description: Option<String>,
}

impl Keybind {
    /// Creates a new keybind
    pub fn new(
        key_code: KeyCode,
        modifiers: KeyModifiers,
        action: String,
        context: KeybindContext,
    ) -> Self {
        Self {
            key_code,
            modifiers,
            action,
            context,
            description: None,
        }
    }

    /// Creates a new keybind with a description
    pub fn with_description(
        key_code: KeyCode,
        modifiers: KeyModifiers,
        action: String,
        context: KeybindContext,
        description: String,
    ) -> Self {
        Self {
            key_code,
            modifiers,
            action,
            context,
            description: Some(description),
        }
    }

    /// Checks if this keybind matches the given KeyEvent
    pub fn matches(&self, event: &KeyEvent) -> bool {
        self.key_code == event.code && self.modifiers == event.modifiers
    }

    /// Returns a human-readable representation of the keybind
    pub fn to_string(&self) -> String {
        let mut parts = Vec::new();
        
        // Add modifiers
        if self.modifiers.contains(KeyModifiers::CONTROL) {
            parts.push("Ctrl".to_string());
        }
        if self.modifiers.contains(KeyModifiers::SHIFT) {
            parts.push("Shift".to_string());
        }
        if self.modifiers.contains(KeyModifiers::ALT) {
            parts.push("Alt".to_string());
        }
        // if self.modifiers.contains(KeyModifiers::SUPER) {
        //     parts.push("Super".to_string());
        // }
        
        // Add key code
        match self.key_code {
            KeyCode::Char(ch) => parts.push(ch.to_uppercase().to_string()),
            KeyCode::Up => parts.push("Up".to_string()),
            KeyCode::Down => parts.push("Down".to_string()),
            KeyCode::Left => parts.push("Left".to_string()),
            KeyCode::Right => parts.push("Right".to_string()),
            KeyCode::Home => parts.push("Home".to_string()),
            KeyCode::End => parts.push("End".to_string()),
            KeyCode::PageUp => parts.push("PageUp".to_string()),
            KeyCode::PageDown => parts.push("PageDown".to_string()),
            KeyCode::Enter => parts.push("Enter".to_string()),
            KeyCode::Esc => parts.push("Esc".to_string()),
            KeyCode::Backspace => parts.push("Backspace".to_string()),
            KeyCode::Delete => parts.push("Delete".to_string()),
            KeyCode::Tab => parts.push("Tab".to_string()),
            KeyCode::F(n) => parts.push(format!("F{}", n)),
            _ => parts.push(format!("{:?}", self.key_code)),
        }
        
        parts.join("-")
    }
}

/// Represents the context in which a keybind is active
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KeybindContext {
    /// Global keybinds that work everywhere
    Global,
    /// Editor-specific keybinds
    Editor,
    /// Search mode keybinds
    Search,
    /// Prompt mode keybinds
    Prompt,
    /// Debug mode keybinds
    Debug,
    /// Clipboard mode keybinds
    Clipboard,
    /// Custom context
    Custom(String),
}

impl KeybindContext {
    /// Returns a human-readable name for the context
    pub fn name(&self) -> &str {
        match self {
            KeybindContext::Global => "Global",
            KeybindContext::Editor => "Editor",
            KeybindContext::Search => "Search",
            KeybindContext::Prompt => "Prompt",
            KeybindContext::Debug => "Debug",
            KeybindContext::Clipboard => "Clipboard",
            KeybindContext::Custom(name) => name,
        }
    }
}

/// Builder for creating keybinds more easily
pub struct KeybindBuilder {
    key_code: Option<KeyCode>,
    modifiers: KeyModifiers,
    action: Option<String>,
    context: Option<KeybindContext>,
    description: Option<String>,
}

impl KeybindBuilder {
    pub fn new() -> Self {
        Self {
            key_code: None,
            modifiers: KeyModifiers::NONE,
            action: None,
            context: None,
            description: None,
        }
    }

    pub fn key(mut self, key_code: KeyCode) -> Self {
        self.key_code = Some(key_code);
        self
    }

    pub fn ctrl(mut self) -> Self {
        self.modifiers |= KeyModifiers::CONTROL;
        self
    }

    pub fn shift(mut self) -> Self {
        self.modifiers |= KeyModifiers::SHIFT;
        self
    }

    pub fn alt(mut self) -> Self {
        self.modifiers |= KeyModifiers::ALT;
        self
    }

    // pub fn super_key(mut self) -> Self {
    //     self.modifiers |= KeyModifiers::SUPER;
    //     self
    // }

    pub fn action(mut self, action: String) -> Self {
        self.action = Some(action);
        self
    }

    pub fn context(mut self, context: KeybindContext) -> Self {
        self.context = Some(context);
        self
    }

    pub fn description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn build(self) -> Result<Keybind, String> {
        let key_code = self.key_code.ok_or("Key code is required")?;
        let action = self.action.ok_or("Action is required")?;
        let context = self.context.ok_or("Context is required")?;

        let mut keybind = Keybind::new(key_code, self.modifiers, action, context);
        if let Some(description) = self.description {
            keybind.description = Some(description);
        }

        Ok(keybind)
    }
}

impl Default for KeybindBuilder {
    fn default() -> Self {
        Self::new()
    }
} 