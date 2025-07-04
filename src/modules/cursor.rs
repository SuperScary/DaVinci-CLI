//! # Cursor Management Module
//! 
//! This module handles cursor positioning, movement, and rendering calculations
//! for the Ninja editor. It provides a `CursorController` that manages the
//! cursor's position in both logical (character-based) and visual (screen-based)
//! coordinate systems.
//! 
//! ## Features
//! 
//! - **Dual Coordinate Systems**: Tracks both logical cursor position and visual render position
//! - **Smart Scrolling**: Automatically scrolls the viewport to keep the cursor visible
//! - **Tab Handling**: Properly calculates cursor position with tab characters
//! - **Unicode Support**: Handles multi-byte characters correctly
//! - **Boundary Checking**: Prevents cursor from moving to invalid positions
//! 
//! ## Coordinate Systems
//! 
//! The cursor controller maintains two coordinate systems:
//! 
//! - **Logical Position** (`cursor_x`, `cursor_y`): Position in the actual text content
//! - **Visual Position** (`render_x`): Position on the screen accounting for tabs and Unicode
//! 
//! ## Usage
//! 
//! ```rust
//! use ninja::modules::cursor::CursorController;
//! use ninja::screens::editor::EditorRows;
//! 
//! let mut cursor = CursorController::new((80, 24));
//! cursor.move_cursor(KeyCode::Right, &editor_rows);
//! cursor.scroll(&editor_rows, 6); // 6 is gutter width
//! ```

use std::cmp;
use std::cmp::Ordering;
use crossterm::event::KeyCode;
use crate::screens::editor::{EditorRows, Row};
use crate::TAB_STOP;

/// Controls cursor positioning and movement in the editor.
/// 
/// This struct manages the cursor's position in both logical and visual
/// coordinate systems, handles scrolling to keep the cursor visible,
/// and provides methods for cursor movement and position calculations.
/// 
/// # Fields
/// 
/// - `cursor_x`: Logical horizontal position in characters
/// - `cursor_y`: Logical vertical position in lines
/// - `screen_rows`: Number of visible screen rows
/// - `screen_columns`: Number of visible screen columns
/// - `row_offset`: Vertical scroll offset
/// - `column_offset`: Horizontal scroll offset
/// - `render_x`: Visual horizontal position accounting for tabs and Unicode
/// 
/// # Example
/// 
/// ```rust
/// use ninja::modules::cursor::CursorController;
/// 
/// // Create a cursor controller for an 80x24 terminal
/// let mut cursor = CursorController::new((80, 24));
/// 
/// // Move cursor and scroll to keep it visible
/// cursor.move_cursor(KeyCode::Down, &editor_rows);
/// cursor.scroll(&editor_rows, 6);
/// ```
#[derive(Copy, Clone)]
pub struct CursorController {
    /// Logical horizontal position in characters (0-based)
    pub cursor_x: usize,
    /// Logical vertical position in lines (0-based)
    pub cursor_y: usize,
    /// Number of visible screen rows
    screen_rows: usize,
    /// Number of visible screen columns
    screen_columns: usize,
    /// Vertical scroll offset (which row is at the top of the screen)
    pub row_offset: usize,
    /// Horizontal scroll offset (which column is at the left of the screen)
    pub column_offset: usize,
    /// Visual horizontal position accounting for tabs and Unicode characters
    pub render_x: usize,
}

impl CursorController {
    /// Creates a new cursor controller with the specified window size.
    /// 
    /// The cursor starts at position (0, 0) with no scroll offsets.
    /// 
    /// # Arguments
    /// 
    /// * `win_size` - A tuple of (columns, rows) representing the terminal window size
    /// 
    /// # Returns
    /// 
    /// Returns a new `CursorController` instance initialized at the top-left position.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ninja::modules::cursor::CursorController;
    /// 
    /// let cursor = CursorController::new((80, 24));
    /// assert_eq!(cursor.cursor_x, 0);
    /// assert_eq!(cursor.cursor_y, 0);
    /// ```
    pub fn new(win_size: (usize, usize)) -> CursorController {
        Self {
            cursor_x: 0,
            cursor_y: 0,
            screen_columns: win_size.0,
            screen_rows: win_size.1,
            row_offset: 0,
            column_offset: 0,
            render_x: 0,
        }
    }

    /// Calculates the visual render position for the current logical cursor position.
    /// 
    /// This method converts the logical cursor position to a visual position
    /// that accounts for tab characters and Unicode characters of varying widths.
    /// 
    /// # Arguments
    /// 
    /// * `row` - The row containing the cursor
    /// 
    /// # Returns
    /// 
    /// Returns the visual column position where the cursor should be rendered.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// let render_x = cursor.get_render_x(&row);
    /// // render_x accounts for tabs and Unicode characters
    /// ```
    fn get_render_x(&self, row: &Row) -> usize {
        row.row_content
            .chars()
            .take(self.cursor_x)
            .fold(0, |render_x, c| {
                if c == '\t' {
                    render_x + (TAB_STOP - 1) - (render_x % TAB_STOP) + 1
                } else {
                    // Use the shared Unicode width calculation
                    render_x + Row::char_width(c)
                }
            })
    }

    /// Updates scroll offsets to keep the cursor visible on screen.
    /// 
    /// This method calculates the visual render position and adjusts the
    /// row and column offsets to ensure the cursor remains visible within
    /// the terminal window bounds.
    /// 
    /// # Arguments
    /// 
    /// * `editor_rows` - The editor's row collection
    /// * `gutter_width` - Width of the line number gutter (if enabled)
    /// 
    /// # Example
    /// 
    /// ```rust
    /// // After moving the cursor, scroll to keep it visible
    /// cursor.move_cursor(KeyCode::Down, &editor_rows);
    /// cursor.scroll(&editor_rows, 6);
    /// ```
    pub fn scroll(&mut self, editor_rows: &EditorRows, gutter_width: usize) {
        self.render_x = 0;
        if self.cursor_y < editor_rows.number_of_rows() {
            self.render_x = self.get_render_x(editor_rows.get_editor_row(self.cursor_y));
        }
        self.row_offset = cmp::min(self.row_offset, self.cursor_y);
        if self.cursor_y >= self.row_offset + self.screen_rows {
            self.row_offset = self.cursor_y - self.screen_rows + 1;
        }
        let content_width = self.screen_columns.saturating_sub(gutter_width);
        self.column_offset = cmp::min(self.column_offset, self.render_x);
        if self.render_x >= self.column_offset + content_width {
            self.column_offset = self.render_x - content_width + 1;
        }
    }

    /// Moves the cursor in the specified direction.
    /// 
    /// This method handles cursor movement in all directions, including
    /// wrapping around line boundaries and respecting file boundaries.
    /// 
    /// # Arguments
    /// 
    /// * `direction` - The direction to move the cursor
    /// * `editor_rows` - The editor's row collection for boundary checking
    /// 
    /// # Supported Directions
    /// 
    /// - `KeyCode::Up`: Move cursor up one line
    /// - `KeyCode::Down`: Move cursor down one line
    /// - `KeyCode::Left`: Move cursor left one character, wrap to previous line
    /// - `KeyCode::Right`: Move cursor right one character, wrap to next line
    /// - `KeyCode::Home`: Move cursor to beginning of current line
    /// - `KeyCode::End`: Move cursor to end of current line
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use crossterm::event::KeyCode;
    /// 
    /// // Move cursor in various directions
    /// cursor.move_cursor(KeyCode::Right, &editor_rows);
    /// cursor.move_cursor(KeyCode::Down, &editor_rows);
    /// cursor.move_cursor(KeyCode::Home, &editor_rows);
    /// ```
    pub fn move_cursor(&mut self, direction: KeyCode, editor_rows: &EditorRows) {
        let number_of_rows = editor_rows.number_of_rows();

        match direction {
            KeyCode::Up => {
                self.cursor_y = self.cursor_y.saturating_sub(1);
            }
            KeyCode::Left => {
                if self.cursor_x != 0 {
                    self.cursor_x -= 1;
                } else if self.cursor_y > 0 {
                    self.cursor_y -= 1;
                    self.cursor_x = editor_rows.get_editor_row(self.cursor_y).char_count();
                }
            }
            KeyCode::Down => {
                if self.cursor_y < number_of_rows {
                    self.cursor_y += 1;
                }
            }
            KeyCode::Right => {
                if self.cursor_y < number_of_rows {
                    let row_char_count = editor_rows.get_editor_row(self.cursor_y).char_count();
                    match self.cursor_x.cmp(&row_char_count) {
                        Ordering::Less => self.cursor_x += 1,
                        Ordering::Equal => {
                            self.cursor_y += 1;
                            self.cursor_x = 0
                        }
                        _ => {}
                    }
                }
            }
            KeyCode::End => {
                if self.cursor_y < number_of_rows {
                    self.cursor_x = editor_rows.get_editor_row(self.cursor_y).char_count();
                }
            }
            KeyCode::Home => self.cursor_x = 0,
            _ => unimplemented!(),
        }
        let row_char_count = if self.cursor_y < number_of_rows {
            editor_rows.get_editor_row(self.cursor_y).char_count()
        } else {
            0
        };
        self.cursor_x = cmp::min(self.cursor_x, row_char_count);
    }
}