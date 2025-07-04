//! # Terminal Input/Output Implementation
//! 
//! This module contains the core implementation of the Ninja editor's terminal
//! input/output functionality. It provides the `Reader` and `Output` structs
//! that handle all user interaction and screen rendering.
//! 
//! ## Key Components
//! 
//! - **`Reader`**: Processes keyboard input and provides a clean interface for key events
//! - **`Output`**: Manages the editor state, text content, and screen rendering
//! 
//! ## Text Editing Features
//! 
//! - Character insertion and deletion with undo/redo
//! - Multi-line text selection and manipulation
//! - Clipboard integration with copy/cut/paste
//! - Search functionality with highlighting
//! - Syntax highlighting for multiple programming languages
//! 
//! ## Screen Management
//! 
//! - Efficient terminal rendering with minimal screen updates
//! - Smart scrolling to keep cursor visible
//! - Status bar and message bar display
//! - Line number gutter support
//! 
//! ## State Management
//! 
//! - Undo/redo stack for all text operations
//! - Selection state tracking
//! - Search state and highlighting
//! - Configuration integration

use crate::modules::clipboard::CLIPBOARD;
use crate::config::NinjaConfig;
use crate::modules::cursor::CursorController;
use crate::screens::editor::{EditorContents, EditorRows, Row};
use crossterm::event::KeyModifiers;
use crate::modules::highlighting::{
    CHighlight, CSSHighlight, GoHighlight, HTMLHighlight, HighlightType, JavaHighlight,
    JavaScriptHighlight, PythonHighlight, RustHighlight, SyntaxHighlight, TypeScriptHighlight, TOMLHighlight,
};
use crate::modules::search::{SearchDirection, SearchIndex};
use crate::modules::status::StatusMessage;
use crate::modules::statusbar::StatusBar;
use crate::modules::message_bar::MessageBar;
use crate::{prompt, VERSION};
use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::style::Color;
use crossterm::terminal::ClearType;
use crossterm::{cursor, event, execute, queue, style, terminal};
use std::io::{stdout, Write};
use std::time::Duration;
use std::{cmp, io};

/// Handles keyboard input and event processing for the editor.
/// 
/// This struct provides a clean interface for reading keyboard events
/// from the terminal with proper error handling and event polling.
/// 
/// # Example
/// 
/// ```rust
/// use ninja::transput::transput::Reader;
/// 
/// let reader = Reader;
/// let key_event = reader.read_key()?;
/// ```
pub struct Reader;

/// Manages the editor's output, state, and text processing.
/// 
/// This is the main struct that coordinates all editor functionality including:
/// - Text content management and editing
/// - Screen rendering and display
/// - Cursor positioning and movement
/// - Selection and clipboard operations
/// - Search functionality
/// - Syntax highlighting
/// - Undo/redo operations
/// 
/// # State Management
/// 
/// The `Output` struct maintains several types of state:
/// - **Text Content**: The actual file content being edited
/// - **Cursor State**: Current cursor position and scroll offsets
/// - **Selection State**: Text selection boundaries and highlighting
/// - **Search State**: Current search term and highlighting
/// - **Undo State**: Stack of previous states for undo/redo
/// - **Configuration**: Editor settings and preferences
/// 
/// # Example
/// 
/// ```rust
/// use ninja::transput::transput::Output;
/// use ninja::config::NinjaConfig;
/// 
/// let config = NinjaConfig::default();
/// let mut output = Output::new(config);
/// 
/// // Insert text
/// output.insert_char('H');
/// output.insert_char('i');
/// 
/// // Refresh the screen
/// output.refresh_screen()?;
/// ```
pub struct Output {
    pub win_size: (usize, usize),
    editor_contents: EditorContents,
    pub cursor_controller: CursorController,
    pub editor_rows: EditorRows,
    pub status_message: StatusMessage,
    pub dirty: u64,
    search_index: SearchIndex,
    pub syntax_highlight: Option<Box<dyn SyntaxHighlight>>,
    pub config: NinjaConfig,
    // Clipboard and selection state
    //clipboard: Clipboard,
    selection_start: Option<(usize, usize)>, // (row, col)
    selection_end: Option<(usize, usize)>,   // (row, col)
    is_selecting: bool,
    // Undo stack
    undo_stack: Vec<(Vec<Row>, CursorController, u64)>,
    pending_edit: bool,
}

impl Output {
    /// Selects the appropriate syntax highlighting based on the file extension.
    /// 
    /// This method determines which syntax highlighter to use based on the
    /// file extension. It supports multiple programming languages including
    /// Rust, C, Java, Python, Go, JavaScript, TypeScript, HTML, CSS, and TOML.
    /// 
    /// # Arguments
    /// 
    /// * `extension` - The file extension (without the dot) to match
    /// 
    /// # Returns
    /// 
    /// Returns `Some(Box<dyn SyntaxHighlight>)` if a matching highlighter is found,
    /// or `None` if no highlighter supports the given extension.
    /// 
    /// # Supported Extensions
    /// 
    /// - **Rust**: `rs`
    /// - **C/C++**: `c`, `cpp`, `h`, `hpp`
    /// - **Java**: `java`
    /// - **Python**: `py`
    /// - **Go**: `go`
    /// - **JavaScript**: `js`
    /// - **TypeScript**: `ts`, `tsx`
    /// - **HTML**: `html`, `htm`
    /// - **CSS**: `css`
    /// - **TOML**: `toml`
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ninja::transput::transput::Output;
    /// 
    /// let highlighter = Output::select_syntax("rs");
    /// assert!(highlighter.is_some());
    /// 
    /// let highlighter = Output::select_syntax("unknown");
    /// assert!(highlighter.is_none());
    /// ```
    pub fn select_syntax(extension: &str) -> Option<Box<dyn SyntaxHighlight>> {
        let list: Vec<Box<dyn SyntaxHighlight>> = vec![
            Box::new(RustHighlight::new()),
            Box::new(CHighlight::new()),
            Box::new(JavaHighlight::new()),
            Box::new(PythonHighlight::new()),
            Box::new(GoHighlight::new()),
            Box::new(JavaScriptHighlight::new()),
            Box::new(TypeScriptHighlight::new()),
            /*Box::new(CSharpHighlight::new()),
            Box::new(RHighlight::new()),
            Box::new(PHPHighlight::new()),
            Box::new(ObjectiveCHighlight::new()),
            Box::new(SwiftHighlight::new()),
            Box::new(KotlinHighlight::new()),
            Box::new(DartHighlight::new()),
            Box::new(RubyHighlight::new()),*/
            Box::new(HTMLHighlight::new()),
            Box::new(CSSHighlight::new()),
            Box::new(TOMLHighlight::new()),
        ];
        list.into_iter()
            .find(|it| it.extensions().contains(&extension))
    }

    /// Creates a new Output instance with the given configuration.
    /// 
    /// This method initializes all the components needed for the editor to function:
    /// - Terminal window size detection
    /// - Editor content buffers
    /// - Cursor controller with proper positioning
    /// - Editor rows for text content management
    /// - Status message system
    /// - Search index for find functionality
    /// - Selection state tracking
    /// - Undo stack for text operations
    /// 
    /// # Arguments
    /// 
    /// * `config` - The editor configuration containing all settings and preferences
    /// 
    /// # Returns
    /// 
    /// Returns a new `Output` instance ready for text editing and screen rendering.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ninja::transput::transput::Output;
    /// use ninja::config::NinjaConfig;
    /// 
    /// let config = NinjaConfig::default();
    /// let output = Output::new(config);
    /// ```
    pub fn new(config: NinjaConfig) -> Self {
        let win_size = terminal::size()
            .map(|(x, y)| (x as usize, y as usize - 2))
            .unwrap();
        let mut syntax_highlight = None;
        Self {
            win_size,
            editor_contents: EditorContents::new(),
            cursor_controller: CursorController::new(win_size),
            editor_rows: EditorRows::new(&mut syntax_highlight),
            status_message: StatusMessage::new(
                "HELP: Ctrl-S = Save | Ctrl-Q = Quit | Ctrl-F = Find | Ctrl-C = Copy | Ctrl-V = Paste".into(),
            ),
            dirty: 0,
            search_index: SearchIndex::new(),
            syntax_highlight,
            config,
            //clipboard: CLIPBOARD.lock().unwrap(), //Clipboard::new().init(),
            selection_start: None,
            selection_end: None,
            is_selecting: false,
            undo_stack: Vec::new(),
            pending_edit: false,
        }
    }

    /// Clears the terminal screen and moves the cursor to the top-left corner.
    /// 
    /// This method provides a clean slate for rendering by clearing all
    /// terminal content and positioning the cursor at the origin.
    /// 
    /// # Returns
    /// 
    /// Returns `Ok(())` on success, or a `crossterm::Error` if the operation fails.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ninja::transput::transput::Output;
    /// 
    /// Output::clear_screen()?;
    /// ```
    pub fn clear_screen() -> crossterm::Result<()> {
        execute!(stdout(), terminal::Clear(ClearType::All))?;
        execute!(stdout(), cursor::MoveTo(0, 0))
    }

    /// Callback function for handling search key events.
    /// 
    /// This function is called by the search prompt to handle navigation
    /// and search functionality. It supports:
    /// - Arrow key navigation through search results
    /// - Enter to confirm search
    /// - Escape to cancel search
    /// - Real-time search highlighting
    /// 
    /// # Arguments
    /// 
    /// * `output` - The editor output instance
    /// * `keyword` - The current search term
    /// * `key_code` - The key that was pressed
    /// 
    /// # Key Handling
    /// 
    /// - **Enter/Escape**: Exit search mode
    /// - **Arrow Keys**: Navigate through search results
    /// - **Other Keys**: Continue search with updated term
    fn find_callback(output: &mut Output, keyword: &str, key_code: KeyCode) {
        if let Some((index, highlight)) = output.search_index.previous_highlight.take() {
            output.editor_rows.get_editor_row_mut(index).highlight = highlight;
        }
        match key_code {
            KeyCode::Esc | KeyCode::Enter => {
                output.search_index.reset();
            }
            _ => {
                output.search_index.y_direction = None;
                output.search_index.x_direction = None;
                match key_code {
                    KeyCode::Down => {
                        output.search_index.y_direction = SearchDirection::Forward.into()
                    }
                    KeyCode::Up => {
                        output.search_index.y_direction = SearchDirection::Backward.into()
                    }
                    KeyCode::Left => {
                        output.search_index.x_direction = SearchDirection::Backward.into()
                    }
                    KeyCode::Right => {
                        output.search_index.x_direction = SearchDirection::Forward.into()
                    }
                    _ => {}
                }
                for i in 0..output.editor_rows.number_of_rows() {
                    let row_index = match output.search_index.y_direction.as_ref() {
                        None => {
                            if output.search_index.x_direction.is_none() {
                                output.search_index.y_index = i;
                            }
                            output.search_index.y_index
                        }
                        Some(dir) => {
                            if matches!(dir, SearchDirection::Forward) {
                                output.search_index.y_index + i + 1
                            } else {
                                let res = output.search_index.y_index.saturating_sub(i);
                                if res == 0 {
                                    break;
                                }
                                res - 1
                            }
                        }
                    };
                    if row_index > output.editor_rows.number_of_rows() - 1 {
                        break;
                    }
                    let row = output.editor_rows.get_editor_row_mut(row_index);
                    let index = match output.search_index.x_direction.as_ref() {
                        None => row.render.find(&keyword),
                        Some(dir) => {
                            let index = if matches!(dir, SearchDirection::Forward) {
                                // Convert character index to byte index for safe slicing
                                let start_char = output.search_index.x_index + 1;
                                let start_byte = row
                                    .render
                                    .char_indices()
                                    .nth(start_char)
                                    .map(|(i, _)| i)
                                    .unwrap_or_else(|| row.render.len());

                                row.render[start_byte..].find(&keyword).map(|index| {
                                    // Convert back to character index
                                    let byte_index = start_byte + index;
                                    row.render[..byte_index].chars().count()
                                })
                            } else {
                                // Convert character index to byte index for safe slicing
                                let end_byte = row
                                    .render
                                    .char_indices()
                                    .nth(output.search_index.x_index)
                                    .map(|(i, _)| i)
                                    .unwrap_or_else(|| row.render.len());

                                row.render[..end_byte].rfind(&keyword).map(|byte_index| {
                                    // Convert back to character index
                                    row.render[..byte_index].chars().count()
                                })
                            };
                            if index.is_none() {
                                break;
                            }
                            index
                        }
                    };
                    if let Some(index) = index {
                        output.search_index.previous_highlight =
                            Some((row_index, row.highlight.clone()));
                        (index..index + keyword.len())
                            .for_each(|index| row.highlight[index] = HighlightType::SearchMatch);
                        output.cursor_controller.cursor_y = row_index;
                        output.search_index.y_index = row_index;
                        output.search_index.x_index = index;
                        output.cursor_controller.cursor_x = row.get_row_content_x(index);
                        output.cursor_controller.row_offset = output.editor_rows.number_of_rows();
                        break;
                    }
                }
            }
        }
    }

    /// Initiates a search for text in the editor content.
    /// 
    /// This method displays a search prompt at the bottom of the screen
    /// and allows the user to enter a search term. The search supports:
    /// - Real-time highlighting of matches
    /// - Navigation between matches with arrow keys
    /// - Case-sensitive and case-insensitive search (configurable)
    /// - Wrap-around search (configurable)
    /// 
    /// # Returns
    /// 
    /// Returns `Ok(())` on successful completion, or an `io::Error` if
    /// the search operation fails.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ninja::transput::transput::Output;
    /// 
    /// // Start a search
    /// output.find()?;
    /// ```
    pub fn find(&mut self) -> io::Result<()> {
        let cursor_controller = self.cursor_controller;
        if prompt!(
            self,
            "Search: {} (Use ESC / Arrows / Enter)",
            callback = Output::find_callback
        )
        .is_none()
        {
            self.cursor_controller = cursor_controller
        }
        Ok(())
    }

    // Selection and clipboard methods
    pub fn start_selection(&mut self) {
        self.is_selecting = true;
        self.selection_start = Some((
            self.cursor_controller.cursor_y,
            self.cursor_controller.cursor_x,
        ));
        self.selection_end = Some((
            self.cursor_controller.cursor_y,
            self.cursor_controller.cursor_x,
        ));
    }

    pub fn update_selection(&mut self) {
        if self.is_selecting {
            self.selection_end = Some((
                self.cursor_controller.cursor_y,
                self.cursor_controller.cursor_x,
            ));
        }
    }

    pub fn clear_selection(&mut self) {
        self.is_selecting = false;
        self.selection_start = None;
        self.selection_end = None;
    }

    pub fn has_selection(&self) -> bool {
        if let (Some(start), Some(end)) = (self.selection_start, self.selection_end) {
            start != end
        } else {
            false
        }
    }

    pub fn is_selecting(&self) -> bool {
        self.is_selecting
    }

    pub fn get_selection_bounds(&self) -> Option<((usize, usize), (usize, usize))> {
        if let (Some(start), Some(end)) = (self.selection_start, self.selection_end) {
            // Ensure start is before end
            if (start.0 < end.0) || (start.0 == end.0 && start.1 <= end.1) {
                Some((start, end))
            } else {
                Some((end, start))
            }
        } else {
            None
        }
    }

    pub fn copy_selection(&mut self) {
        if let Some(((start_row, start_col), (end_row, end_col))) = self.get_selection_bounds() {
            let mut selected_text = String::new();

            if start_row == end_row {
                // Single line selection
                let row = self.editor_rows.get_editor_row(start_row);
                let content = &row.row_content[start_col..end_col];
                selected_text.push_str(content);
            } else {
                // Multi-line selection
                for row_idx in start_row..=end_row {
                    let row = self.editor_rows.get_editor_row(row_idx);
                    if row_idx == start_row {
                        // First line: from start_col to end
                        selected_text.push_str(&row.row_content[start_col..]);
                    } else if row_idx == end_row {
                        // Last line: from beginning to end_col
                        selected_text.push_str(&row.row_content[..end_col]);
                    } else {
                        // Middle lines: entire line
                        selected_text.push_str(&row.row_content);
                    }

                    if row_idx < end_row {
                        selected_text.push('\n');
                    }
                }
            }

            CLIPBOARD.lock().unwrap().add(selected_text);
            self.status_message
                .set_message(format!("Copied {} characters", CLIPBOARD.lock().unwrap().get_top().unwrap().len()));
        }
    }

    pub fn cut_selection(&mut self) {
        if let Some(((start_row, start_col), (end_row, end_col))) = self.get_selection_bounds() {
            self.push_undo();
            self.copy_selection();

            // Remove the selected text
            if start_row == end_row {
                // Single line selection
                self.remove_chars_from_row(start_row, start_col, end_col);
            } else {
                // Multi-line selection
                // Handle the end row (remove from beginning to end_col)
                self.remove_chars_from_row(end_row, 0, end_col);

                // Handle the start row (remove from start_col to end)
                self.remove_chars_from_row(start_row, start_col, self.editor_rows.get_editor_row(start_row).char_count());

                // Remove all rows in between
                let rows_to_remove = end_row - start_row - 1;
                for _ in 0..rows_to_remove {
                    self.editor_rows.row_contents.remove(start_row + 1);
                }

                // Join the start and end rows if they're now adjacent
                if start_row + 1 < self.editor_rows.number_of_rows() {
                    let end_row_content = self
                        .editor_rows
                        .get_editor_row(start_row + 1)
                        .row_content
                        .clone();
                    let start_row_ref = self.editor_rows.get_editor_row_mut(start_row);
                    start_row_ref.row_content.push_str(&end_row_content);
                    EditorRows::render_row(start_row_ref);

                    // Remove the end row since we joined it
                    self.editor_rows.row_contents.remove(start_row + 1);
                }
            }

            self.clear_selection();
            self.cursor_controller.cursor_y = start_row;
            self.cursor_controller.cursor_x = start_col;
            self.dirty += 1;
            self.pending_edit = false;
        }
    }

    pub fn paste_clipboard(&mut self) {
        if !CLIPBOARD.lock().unwrap().is_empty() {
            self.push_undo();
            let clipboard_content = CLIPBOARD.lock().unwrap().get_top().unwrap().clone();
            let mut chars = clipboard_content.chars().peekable();

            while let Some(ch) = chars.next() {
                if ch == '\n' {
                    self.insert_newline_without_undo();
                } else {
                    self.insert_char_without_undo(ch);
                }
            }
            self.status_message
                .set_message(format!("Pasted {} characters", clipboard_content.len()));
            self.pending_edit = false;
        }
    }

    /// Checks if the cursor is at the bottom of the file
    fn cursor_at_bottom(&mut self) -> bool {
        self.cursor_controller.cursor_y == self.editor_rows.number_of_rows()
    }
    
    fn cursor_at_start(&mut self) -> bool {
        self.cursor_controller.cursor_x == 0
    }
    
    // Helper methods that don't push undo (used by paste operation)
    fn insert_char_core(&mut self, ch: char) {
        if self.cursor_at_bottom() {
            self.editor_rows
                .insert_row(self.editor_rows.number_of_rows(), String::new());
            self.dirty += 1;
        }
        self.editor_rows
            .get_editor_row_mut(self.cursor_controller.cursor_y)
            .insert_char(self.cursor_controller.cursor_x, ch);
        self.update_syntax_highlighting(self.cursor_controller.cursor_y);
        self.cursor_controller.cursor_x += 1;
        self.dirty += 1;
    }

    fn insert_newline_core(&mut self) {
        if self.cursor_at_start() {
            // If cursor is at the beginning, check previous line for indentation
            let indent_level =
                if self.config.editor.auto_indent && self.cursor_controller.cursor_y > 0 {
                    let previous_row = self
                        .editor_rows
                        .get_row(self.cursor_controller.cursor_y - 1);
                    self.get_indentation_level(previous_row)
                } else {
                    0
                };

            // Create new row with same indentation
            let indent_spaces = " ".repeat(indent_level);
            self.editor_rows
                .insert_row(self.cursor_controller.cursor_y, indent_spaces);
            self.cursor_controller.cursor_x = indent_level;
        } else {
            // Get the current row content and calculate indentation before any mutable operations
            let current_row = self
                .editor_rows
                .get_editor_row(self.cursor_controller.cursor_y);
            let indent_level = if self.config.editor.auto_indent {
                self.get_indentation_level(&current_row.row_content)
            } else {
                0
            };

            let current_row = self
                .editor_rows
                .get_editor_row_mut(self.cursor_controller.cursor_y);

            // Use character-based substring operation for UTF-8 safety
            let new_row_content = current_row
                .substring_by_chars(self.cursor_controller.cursor_x, current_row.char_count());

            // Truncate the current row at the cursor position
            let truncated_content =
                current_row.substring_by_chars(0, self.cursor_controller.cursor_x);
            current_row.row_content = truncated_content;
            EditorRows::render_row(current_row);

            // Create new line with proper indentation
            let indent_spaces = " ".repeat(indent_level);
            let mut new_line_content = indent_spaces;
            new_line_content.push_str(&new_row_content);

            self.editor_rows
                .insert_row(self.cursor_controller.cursor_y + 1, new_line_content);
            self.update_syntax_highlighting(self.cursor_controller.cursor_y);
            self.update_syntax_highlighting(self.cursor_controller.cursor_y + 1);
            self.cursor_controller.cursor_x = indent_level;
        }
        self.cursor_controller.cursor_y += 1;
        self.dirty += 1;
    }

    // Helper methods that don't push undo (used by paste operation)
    fn insert_char_without_undo(&mut self, ch: char) {
        self.insert_char_core(ch);
    }

    fn insert_newline_without_undo(&mut self) {
        self.insert_newline_core();
    }

    pub fn is_position_selected(&self, row: usize, col: usize) -> bool {
        if let Some(((start_row, start_col), (end_row, end_col))) = self.get_selection_bounds() {
            if row == start_row && row == end_row {
                // Single line selection
                col >= start_col && col < end_col
            } else if row == start_row {
                // First line of multi-line selection
                col >= start_col
            } else if row == end_row {
                // Last line of multi-line selection
                col < end_col
            } else {
                // Middle line of multi-line selection
                row > start_row && row < end_row
            }
        } else {
            false
        }
    }

    fn draw_rows(&mut self) {
        let screen_rows = self.win_size.1;
        let screen_columns = self.win_size.0;
        let gutter_width = if self.config.editor.show_line_numbers {
            self.config.editor.gutter_width
        } else {
            0
        };
        let content_width = screen_columns.saturating_sub(gutter_width);

        for i in 0..screen_rows {
            let file_row = i + self.cursor_controller.row_offset;
            if file_row >= self.editor_rows.number_of_rows() {
                if self.editor_rows.number_of_rows() == 0 && i == screen_rows / 3 {
                    let welcome = self.config.display.welcome_message.replace("{}", VERSION);
                    let welcome_chars: Vec<char> = welcome.chars().collect();
                    let welcome = if welcome_chars.len() > content_width {
                        welcome_chars[..content_width].iter().collect::<String>()
                    } else {
                        welcome
                    };
                    let mut padding = (content_width - welcome.chars().count()) / 2;
                    // Add gutter padding
                    if self.config.editor.show_line_numbers {
                        (0..gutter_width).for_each(|_| self.editor_contents.push(' '));
                    }
                    if padding != 0 {
                        padding -= 1
                    }
                    (0..padding).for_each(|_| self.editor_contents.push(' '));
                    self.editor_contents.push_str(&welcome);
                } else {
                    // Display empty gutter for empty lines
                    if self.config.editor.show_line_numbers {
                        (0..gutter_width).for_each(|_| self.editor_contents.push(' '));
                    }
                }
            } else {
                let row = self.editor_rows.get_editor_row(file_row);
                let render = &row.render;
                let column_offset = self.cursor_controller.column_offset;

                // Use character-based operations for UTF-8 safety
                let render_chars: Vec<char> = render.chars().collect();
                let len = cmp::min(
                    render_chars.len().saturating_sub(column_offset),
                    content_width,
                );
                let start = if len == 0 { 0 } else { column_offset };
                let end = start + len;

                // Create the rendered string using character operations
                let render = render_chars[start..end].iter().collect::<String>();

                // Draw line number in gutter
                if self.config.editor.show_line_numbers {
                    let line_num = format!("{:>5} ", file_row + 1);
                    self.editor_contents.push_str(&line_num);
                }

                // Draw the actual content with syntax highlighting
                if self.config.syntax.enable_syntax_highlighting {
                    if let Some(syntax_highlight) = &self.syntax_highlight {
                        // Ensure highlight array has enough elements
                        let highlight_slice =
                            if start < row.highlight.len() && end <= row.highlight.len() {
                                &row.highlight[start..end]
                            } else {
                                // Fallback to normal highlighting if indices are out of bounds
                                &[]
                            };

                        // Apply selection highlighting over syntax highlighting
                        let mut final_highlights = highlight_slice.to_vec();
                        if self.has_selection() {
                            for (char_idx, _) in render.char_indices().enumerate() {
                                let actual_char_idx = start + char_idx;
                                if self.is_position_selected(file_row, actual_char_idx) {
                                    if char_idx < final_highlights.len() {
                                        final_highlights[char_idx] = HighlightType::Selection;
                                    } else {
                                        final_highlights.push(HighlightType::Selection);
                                    }
                                }
                            }
                        }

                        syntax_highlight.color_row(
                            &render,
                            &final_highlights,
                            &mut self.editor_contents,
                        );
                    } else {
                        self.editor_contents.push_str(&render);
                    }
                } else {
                    // No syntax highlighting, but still apply selection highlighting
                    if self.has_selection() {
                        let mut current_color = Color::Reset;
                        for (char_idx, c) in render.char_indices() {
                            let actual_char_idx = start + char_idx;
                            let color = if self.is_position_selected(file_row, actual_char_idx) {
                                Color::White
                            } else {
                                Color::Reset
                            };

                            if current_color != color {
                                current_color = color;
                                let _ =
                                    queue!(self.editor_contents, style::SetForegroundColor(color));
                            }
                            self.editor_contents.push(c);
                        }
                        let _ = queue!(
                            self.editor_contents,
                            style::SetForegroundColor(Color::Reset)
                        );
                    } else {
                        self.editor_contents.push_str(&render);
                    }
                }
            }
            queue!(
                self.editor_contents,
                terminal::Clear(ClearType::UntilNewLine)
            )
            .unwrap();
            self.editor_contents.push_str("\r\n");
        }
    }

    pub fn move_cursor(&mut self, direction: KeyCode) {
        self.cursor_controller
            .move_cursor(direction, &self.editor_rows);
        self.pending_edit = false;
    }

    pub fn refresh_screen(&mut self) -> crossterm::Result<()> {
        let gutter_width = if self.config.editor.show_line_numbers {
            self.config.editor.gutter_width
        } else {
            0
        };
        self.cursor_controller
            .scroll(&self.editor_rows, gutter_width);
        queue!(self.editor_contents, cursor::Hide, cursor::MoveTo(0, 0))?;
        self.draw_rows();
        StatusBar::draw_status_bar(
            &mut self.editor_contents,
            self.win_size,
            &self.editor_rows.filename,
            self.dirty,
            &self.syntax_highlight,
            &self.cursor_controller,
        );
        MessageBar::draw_message_bar(
            &mut self.editor_contents,
            self.win_size,
            &mut self.status_message,
        );
        let cursor_x =
            self.cursor_controller.render_x - self.cursor_controller.column_offset + gutter_width;
        let cursor_y = self.cursor_controller.cursor_y - self.cursor_controller.row_offset;
        queue!(
            self.editor_contents,
            cursor::MoveTo(cursor_x as u16, cursor_y as u16),
            cursor::Show
        )?;
        self.editor_contents.flush()
    }

    // Undo stack methods
    pub fn push_undo(&mut self) {
        // Store a deep copy of the editor state
        self.undo_stack.push((
            self.editor_rows.row_contents.clone(),
            self.cursor_controller,
            self.dirty,
        ));
        // Limit undo stack size if desired
        if self.undo_stack.len() > 100 {
            self.undo_stack.remove(0);
        }
    }

    pub fn pop_undo(&mut self) {
        if let Some((rows, cursor, dirty)) = self.undo_stack.pop() {
            self.editor_rows.row_contents = rows;
            self.cursor_controller = cursor;
            self.dirty = dirty;
            // Re-render all rows for syntax highlighting
            if let Some(it) = self.syntax_highlight.as_ref() {
                for i in 0..self.editor_rows.number_of_rows() {
                    it.update_syntax(i, &mut self.editor_rows.row_contents);
                }
            }
        }
        self.pending_edit = false;
    }

    fn get_indentation_level(&self, row_content: &str) -> usize {
        let mut indent_level = 0;
        for ch in row_content.chars() {
            if ch == ' ' {
                indent_level += 1;
            } else {
                break;
            }
        }
        indent_level
    }

    fn update_syntax_highlighting(&mut self, row_index: usize) {
        if let Some(it) = self.syntax_highlight.as_ref() {
            it.update_syntax(row_index, &mut self.editor_rows.row_contents);
        }
    }

    fn remove_chars_from_row(&mut self, row_index: usize, start: usize, end: usize) {
        let row = self.editor_rows.get_editor_row_mut(row_index);
        let mut chars: Vec<char> = row.row_content.chars().collect();
        chars.drain(start..end);
        row.row_content = chars.into_iter().collect();
        EditorRows::render_row(row);
    }

    pub fn insert_char(&mut self, ch: char) {
        if !self.pending_edit {
            self.push_undo();
            self.pending_edit = true;
        }
        self.insert_char_core(ch);
    }

    pub fn insert_newline(&mut self) {
        if !self.pending_edit {
            self.push_undo();
            self.pending_edit = true;
        }
        self.insert_newline_core();
    }

    pub fn delete_char(&mut self) {
        self.push_undo();
        if self.cursor_controller.cursor_y == self.editor_rows.number_of_rows() {
            return;
        }
        if self.cursor_controller.cursor_y == 0 && self.cursor_controller.cursor_x == 0 {
            return;
        }
        if self.cursor_controller.cursor_x > 0 {
            self.editor_rows
                .get_editor_row_mut(self.cursor_controller.cursor_y)
                .delete_char(self.cursor_controller.cursor_x - 1);
            self.cursor_controller.cursor_x -= 1;
        } else {
            let previous_row = self
                .editor_rows
                .get_editor_row(self.cursor_controller.cursor_y - 1);
            self.cursor_controller.cursor_x = previous_row.char_count();
            self.editor_rows
                .join_adjacent_rows(self.cursor_controller.cursor_y);
            self.cursor_controller.cursor_y -= 1;
        }
        self.update_syntax_highlighting(self.cursor_controller.cursor_y);
        self.dirty += 1;
        self.pending_edit = false;
    }
}

impl Reader {
    /// Reads a single key event from the terminal.
    /// 
    /// This method polls for keyboard input with a timeout and returns
    /// the next key event when available. It handles all types of key
    /// events including regular characters, function keys, and modifier
    /// combinations.
    /// 
    /// # Returns
    /// 
    /// Returns `Ok(KeyEvent)` when a key is pressed, or a `crossterm::Error`
    /// if the input operation fails.
    /// 
    /// # Polling Behavior
    /// 
    /// The method polls for input every 500ms to avoid blocking the
    /// editor indefinitely. This allows for responsive UI updates
    /// while waiting for user input.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ninja::transput::transput::Reader;
    /// 
    /// let reader = Reader;
    /// let key_event = reader.read_key()?;
    /// 
    /// match key_event.code {
    ///     KeyCode::Char('q') => println!("User pressed 'q'"),
    ///     KeyCode::Ctrl('s') => println!("User pressed Ctrl+S"),
    ///     _ => {}
    /// }
    /// ```
    pub fn read_key(&self) -> crossterm::Result<KeyEvent> {
        loop {
            if event::poll(Duration::from_millis(500))? {
                if let Event::Key(event) = event::read()? {
                    return Ok(event);
                }
            }
        }
    }
}
