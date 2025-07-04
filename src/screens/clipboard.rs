//! # Clipboard Screen Module
//! 
//! This module provides the clipboard management screen for the Ninja editor.
//! It allows users to view and manage their clipboard history, including
//! copying, cutting, and pasting operations.
//! 
//! ## Features
//! 
//! - **Clipboard History**: View all items in the clipboard history
//! - **Item Management**: Select, copy, and delete clipboard items
//! - **Visual Interface**: Clean display of clipboard contents
//! - **Integration**: Seamless integration with the main editor
//! 
//! ## Usage
//! 
//! ```rust
//! use ninja::screens::clipboard::ClipboardScreen;
//! use ninja::modules::clipboard::Clipboard;
//! 
//! let clipboard = Clipboard::new();
//! ```

use crate::modules::clipboard;

/// Represents the clipboard management screen.
/// 
/// This struct provides the interface for managing clipboard operations
/// in the editor. It integrates with the clipboard module to provide
/// a user-friendly way to view and manage clipboard history.
/// 
/// # Features
/// 
/// - **History Display**: Shows all items in the clipboard history
/// - **Item Selection**: Allows users to select specific clipboard items
/// - **Copy Operations**: Copy items back to the active clipboard
/// - **Delete Operations**: Remove items from the clipboard history
/// - **Navigation**: Navigate through clipboard items
/// 
/// # Integration
/// 
/// The clipboard screen integrates with:
/// - **Main Editor**: Seamless transition between editor and clipboard views
/// - **Clipboard Module**: Direct access to clipboard functionality
/// - **Keybind System**: Custom keybinds for clipboard operations
/// 
/// # Example
/// 
/// ```rust
/// use ninja::screens::clipboard::ClipboardScreen;
/// use ninja::modules::clipboard::Clipboard;
/// 
/// let clipboard = Clipboard::new();
/// 
/// // The clipboard screen can be used to display and manage
/// // clipboard history in a user-friendly interface
/// ```
pub struct ClipboardScreen {
    /// The clipboard instance that manages the clipboard history
    pub clipboard: clipboard::Clipboard
}