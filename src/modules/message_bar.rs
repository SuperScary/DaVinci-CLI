//! # Message Bar Module
//! 
//! This module provides the message bar rendering functionality for the Ninja editor.
//! The message bar displays status messages and user feedback at the bottom
//! of the editor interface.
//! 
//! ## Features
//! 
//! - **Status Display**: Shows temporary status messages to the user
//! - **Message Truncation**: Automatically truncates long messages to fit the screen
//! - **Clean Rendering**: Properly clears and redraws the message area
//! - **Terminal Integration**: Uses crossterm for terminal output
//! 
//! ## Usage
//! 
//! ```rust
//! use ninja::modules::message_bar::MessageBar;
//! use ninja::modules::status::StatusMessage;
//! use ninja::screens::editor::EditorContents;
//! 
//! let mut editor_contents = EditorContents::new();
//! let mut status_message = StatusMessage::new("Hello, World!".to_string());
//! let win_size = (80, 24);
//! 
//! MessageBar::draw_message_bar(&mut editor_contents, win_size, &mut status_message);
//! ```

use crate::screens::editor::EditorContents;
use crate::modules::status::StatusMessage;
use crossterm::terminal::{self, ClearType};
use crossterm::queue;

/// Provides functionality for rendering the message bar in the editor.
/// 
/// The message bar is displayed at the bottom of the editor interface
/// and shows status messages, warnings, and other user feedback.
/// 
/// # Rendering Behavior
/// 
/// - **Position**: Always rendered at the bottom of the terminal
/// - **Clearing**: Clears the current line before rendering
/// - **Truncation**: Long messages are truncated to fit the screen width
/// - **Status Integration**: Works with the `StatusMessage` system
/// 
/// # Example
/// 
/// ```rust
/// use ninja::modules::message_bar::MessageBar;
/// use ninja::modules::status::StatusMessage;
/// use ninja::screens::editor::EditorContents;
/// 
/// let mut editor_contents = EditorContents::new();
/// let mut status_message = StatusMessage::new("File saved".to_string());
/// let win_size = (80, 24);
/// 
/// MessageBar::draw_message_bar(&mut editor_contents, win_size, &mut status_message);
/// ```
pub struct MessageBar;

impl MessageBar {
    /// Renders the message bar with the current status message.
    /// 
    /// This method draws the message bar at the bottom of the editor
    /// interface. It clears the current line and displays the status
    /// message if one is active. Long messages are automatically
    /// truncated to fit the screen width.
    /// 
    /// # Arguments
    /// 
    /// * `editor_contents` - The editor's content buffer for rendering
    /// * `win_size` - The terminal window size as (width, height)
    /// * `status_message` - The status message to display
    /// 
    /// # Rendering Process
    /// 
    /// 1. **Clear Line**: Clears the current line to prepare for rendering
    /// 2. **Check Message**: Checks if there's an active status message
    /// 3. **Truncate**: Truncates the message if it's too long for the screen
    /// 4. **Render**: Writes the message to the editor contents buffer
    /// 
    /// # Message Truncation
    /// 
    /// If the status message is longer than the screen width, it's
    /// truncated to fit. The truncation preserves the beginning of
    /// the message and cuts off the end.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ninja::modules::message_bar::MessageBar;
    /// use ninja::modules::status::StatusMessage;
    /// use ninja::screens::editor::EditorContents;
    /// 
    /// let mut editor_contents = EditorContents::new();
    /// let mut status_message = StatusMessage::new("This is a very long message that will be truncated".to_string());
    /// let win_size = (20, 10); // Narrow window
    /// 
    /// MessageBar::draw_message_bar(&mut editor_contents, win_size, &mut status_message);
    /// // Message will be truncated to fit the 20-character width
    /// ```
    pub fn draw_message_bar(
        editor_contents: &mut EditorContents,
        win_size: (usize, usize),
        status_message: &mut StatusMessage,
    ) {
        queue!(editor_contents, terminal::Clear(ClearType::UntilNewLine)).unwrap();
        if let Some(msg) = status_message.message() {
            let msg_chars: Vec<char> = msg.chars().collect();
            let truncated_msg = if msg_chars.len() > win_size.0 {
                msg_chars[..win_size.0].iter().collect::<String>()
            } else {
                msg.clone()
            };
            editor_contents.push_str(&truncated_msg);
        }
    }
} 