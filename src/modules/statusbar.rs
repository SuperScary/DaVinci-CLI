//! # Status Bar Module
//! 
//! This module provides the status bar rendering functionality for the Ninja editor.
//! The status bar displays file information, modification status, cursor position,
//! and syntax highlighting information at the top of the editor interface.
//! 
//! ## Features
//! 
//! - **File Information**: Shows current filename and modification status
//! - **Cursor Position**: Displays current line and column numbers
//! - **Syntax Information**: Shows detected file type and syntax highlighting
//! - **Visual Styling**: Uses reverse video for status bar appearance
//! - **Responsive Layout**: Adapts to different terminal window sizes
//! 
//! ## Layout
//! 
//! The status bar is divided into two sections:
//! - **Left side**: Filename and modification status
//! - **Right side**: File type, line number, and column number
//! 
//! ## Usage
//! 
//! ```rust
//! use ninja::modules::statusbar::StatusBar;
//! use ninja::modules::cursor::CursorController;
//! use ninja::screens::editor::EditorContents;
//! use std::path::PathBuf;
//! 
//! let mut editor_contents = EditorContents::new();
//! let win_size = (80, 24);
//! let filename = Some(PathBuf::from("example.rs"));
//! let dirty = 0; // File is not modified
//! let syntax_highlight = None;
//! let cursor_controller = CursorController::new((80, 24));
//! 
//! StatusBar::draw_status_bar(
//!     &mut editor_contents,
//!     win_size,
//!     &filename,
//!     dirty,
//!     &syntax_highlight,
//!     &cursor_controller,
//! );
//! ```

use super::super::screens::editor::EditorContents;
use crate::modules::highlighting::SyntaxHighlight;
use crate::modules::cursor::CursorController;
use crossterm::style;
use std::cmp;

/// Provides functionality for rendering the status bar in the editor.
/// 
/// The status bar is displayed at the top of the editor interface and
/// provides essential information about the current file and editor state.
/// 
/// # Status Bar Information
/// 
/// The status bar displays:
/// - **Filename**: Name of the current file (or "[No Name]" if none)
/// - **Modification Status**: "(modified)" indicator if file has unsaved changes
/// - **File Type**: Detected or configured syntax highlighting language
/// - **Cursor Position**: Current line and column numbers (1-based)
/// 
/// # Visual Appearance
/// 
/// - **Reverse Video**: The entire status bar uses reverse video styling
/// - **Responsive Layout**: Adapts to terminal window size
/// - **Information Distribution**: File info on left, position info on right
/// 
/// # Example
/// 
/// ```rust
/// use ninja::modules::statusbar::StatusBar;
/// use ninja::modules::cursor::CursorController;
/// use ninja::screens::editor::EditorContents;
/// use std::path::PathBuf;
/// 
/// let mut editor_contents = EditorContents::new();
/// let win_size = (80, 24);
/// let filename = Some(PathBuf::from("main.rs"));
/// let dirty = 1; // File is modified
/// let syntax_highlight = None;
/// let cursor_controller = CursorController::new((80, 24));
/// 
/// StatusBar::draw_status_bar(
///     &mut editor_contents,
///     win_size,
///     &filename,
///     dirty,
///     &syntax_highlight,
///     &cursor_controller,
/// );
/// ```
pub struct StatusBar;

impl StatusBar {
    /// Draws the status bar with file information, modification status, and cursor position.
    /// 
    /// This method renders the status bar at the top of the editor interface.
    /// It displays comprehensive information about the current file and
    /// editor state in a visually appealing format.
    /// 
    /// # Arguments
    /// 
    /// * `editor_contents` - The editor's content buffer for rendering
    /// * `win_size` - The terminal window size as (width, height)
    /// * `filename` - The current file path (optional)
    /// * `dirty` - Modification counter (0 = clean, >0 = modified)
    /// * `syntax_highlight` - The current syntax highlighter (optional)
    /// * `cursor_controller` - The cursor controller for position information
    /// 
    /// # Layout Algorithm
    /// 
    /// 1. **Apply Styling**: Sets reverse video attribute for status bar
    /// 2. **Build File Info**: Creates filename and modification status string
    /// 3. **Build Position Info**: Creates file type, line, and column string
    /// 4. **Layout Calculation**: Distributes information across available width
    /// 5. **Render**: Writes the formatted status bar to the buffer
    /// 
    /// # Information Display
    /// 
    /// - **Filename**: Extracted from the path, falls back to "[No Name]"
    /// - **Modification Status**: Shows "(modified)" if `dirty > 0`
    /// - **File Type**: From syntax highlighter or "Detecting..."
    /// - **Position**: Line and column numbers (1-based display)
    /// 
    /// # Responsive Behavior
    /// 
    /// - **Truncation**: Long filenames are truncated to fit
    /// - **Spacing**: Information is properly spaced across the width
    /// - **Alignment**: Position info is right-aligned when possible
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ninja::modules::statusbar::StatusBar;
    /// use ninja::modules::cursor::CursorController;
    /// use ninja::screens::editor::EditorContents;
    /// use std::path::PathBuf;
    /// 
    /// let mut editor_contents = EditorContents::new();
    /// let win_size = (80, 24);
    /// let filename = Some(PathBuf::from("/path/to/example.rs"));
    /// let dirty = 2; // File has been modified twice
    /// let syntax_highlight = None;
    /// let mut cursor_controller = CursorController::new((80, 24));
    /// cursor_controller.cursor_y = 4; // 5th line (0-based)
    /// cursor_controller.cursor_x = 15; // 16th column (0-based)
    /// 
    /// StatusBar::draw_status_bar(
    ///     &mut editor_contents,
    ///     win_size,
    ///     &filename,
    ///     dirty,
    ///     &syntax_highlight,
    ///     &cursor_controller,
    /// );
    /// // Status bar will show: "example.rs (modified)                    Detecting... | 5:16"
    /// ```
    pub fn draw_status_bar(
        editor_contents: &mut EditorContents,
        win_size: (usize, usize),
        filename: &Option<std::path::PathBuf>,
        dirty: u64,
        syntax_highlight: &Option<Box<dyn SyntaxHighlight>>,
        cursor_controller: &CursorController,
    ) {
        editor_contents.push_str(&style::Attribute::Reverse.to_string());
        
        let info = format!(
            "{} {}",
            filename
                .as_ref()
                .and_then(|path| path.file_name())
                .and_then(|name| name.to_str())
                .unwrap_or("[No Name]"),
            if dirty > 0 { "(modified)" } else { "" }
        );
        
        let info_len = cmp::min(info.len(), win_size.0);
        
        /* LINES AND COLUMNS */
        let line_info = format!(
            "{} | {}:{}",
            syntax_highlight
                .as_ref()
                .map(|highlight| highlight.file_type())
                .unwrap_or("Detecting..."),
            cursor_controller.cursor_y + 1,
            cursor_controller.cursor_x + 1
        );
        
        editor_contents.push_str(&info[..info_len]);
        
        for i in info_len..win_size.0 {
            if win_size.0 - i == line_info.len() {
                editor_contents.push_str(&line_info);
                break;
            } else {
                editor_contents.push(' ')
            }
        }
        
        editor_contents.push_str(&style::Attribute::Reset.to_string());
        editor_contents.push_str("\r\n");
    }
}
