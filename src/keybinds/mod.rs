//! # Keyboard Bindings Module
//! 
//! This module provides a flexible and extensible keyboard binding system
//! for the Ninja editor. It allows for customizable key combinations and
//! context-aware action execution.
//! 
//! ## Components
//! 
//! - **`manager`**: Central keybinding manager that handles registration and execution
//! - **`bindings`**: Key binding definitions and context management
//! - **`actions`**: Action definitions and execution logic
//! 
//! ## Features
//! 
//! - **Context-Aware Bindings**: Different key combinations in different contexts
//! - **Modifier Support**: Full support for Ctrl, Alt, Shift, and Meta modifiers
//! - **Action System**: Extensible action framework for editor operations
//! - **Dynamic Binding**: Runtime registration and modification of key bindings
//! - **Conflict Resolution**: Automatic handling of binding conflicts
//! 
//! ## Usage
//! 
//! ```rust
//! use ninja::keybinds::{KeybindManager, Keybind, Action};
//! 
//! let mut manager = KeybindManager::new();
//! 
//! // Register a key binding
//! manager.register(Keybind::new("Ctrl+S", Action::Save, "editor"));
//! 
//! // Execute an action
//! manager.execute("Ctrl+S", "editor");
//! ```
//! 
//! ## Contexts
//! 
//! The keybinding system supports multiple contexts:
//! - **`editor`**: Main text editing mode
//! - **`search`**: Search and replace mode
//! - **`command`**: Command line mode
//! - **`visual`**: Visual selection mode 

pub mod manager;
pub mod bindings;
pub mod actions;

pub use manager::KeybindManager;
pub use bindings::{Keybind, KeybindContext};
pub use actions::Action; 