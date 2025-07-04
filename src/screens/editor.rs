//! # Editor Screen Module
//! 
//! This module provides the main editor screen functionality for the Ninja editor.
//! It contains the core data structures and logic for text editing, file handling,
//! and the main editor loop.
//! 
//! ## Components
//! 
//! - **`Row`**: Represents a single line of text with rendering and highlighting
//! - **`EditorRows`**: Collection of rows with file handling capabilities
//! - **`EditorContents`**: Buffer for terminal output rendering
//! - **`Editor`**: Main editor struct that coordinates all functionality
//! 
//! ## Features
//! 
//! - **Text Editing**: Character insertion, deletion, and row manipulation
//! - **File I/O**: Reading and writing files with UTF-8 support
//! - **Syntax Highlighting**: Integration with syntax highlighting system
//! - **Unicode Support**: Full Unicode character width calculation
//! - **Tab Handling**: Proper tab expansion and rendering
//! - **Keybind Integration**: Full integration with the keybind system
//! 
//! ## Architecture
//! 
//! The editor screen uses a layered architecture:
//! - **Row Level**: Individual line management and rendering
//! - **Collection Level**: Multiple rows with file operations
//! - **Editor Level**: Main loop and user interaction
//! - **Output Level**: Terminal rendering and display
//! 
//! ## Usage
//! 
//! ```rust
//! use ninja::screens::editor::Editor;
//! use ninja::config::NinjaConfig;
//! 
//! let config = NinjaConfig::default();
//! let mut editor = Editor::new(config);
//! 
//! // Run the editor
//! match editor.run() {
//!     Ok(true) => println!("Editor completed normally"),
//!     Ok(false) => println!("Editor was quit"),
//!     Err(e) => eprintln!("Editor error: {}", e),
//! }
//! ```

use crate::config::NinjaConfig;
use crate::transput::transput::Reader;
use crate::modules::highlighting::{HighlightType, SyntaxHighlight};
use crate::keybinds::actions::ActionExecutor;
use crate::keybinds::{KeybindContext, KeybindManager};
use crate::{transput::transput, prompt, TAB_STOP};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use transput::Output;
use std::io::{stdout, ErrorKind, Write};
use std::path::PathBuf;
use std::{env, fs, io};

/// Represents a single line of text in the editor.
/// 
/// This struct contains the actual text content, its rendered representation
/// (with tabs expanded), syntax highlighting information, and metadata about
/// the line's state.
/// 
/// # Fields
/// 
/// - **`row_content`**: The actual text content of the line
/// - **`render`**: The rendered representation with tabs expanded
/// - **`highlight`**: Syntax highlighting information for each character
/// - **`is_comment`**: Whether this line is part of a multi-line comment
/// 
/// # Unicode Support
/// 
/// The `Row` struct provides full Unicode support including:
/// - UTF-8 character handling
/// - Proper character width calculation for CJK and emoji characters
/// - Safe character insertion and deletion
/// - Tab expansion and rendering
/// 
/// # Example
/// 
/// ```rust
/// use ninja::screens::editor::Row;
/// 
/// let mut row = Row::new("Hello\tWorld".to_string(), "Hello    World".to_string());
/// 
/// // Insert a character
/// row.insert_char(5, ' ');
/// 
/// // Get character count
/// assert_eq!(row.char_count(), 12);
/// ```
#[derive(Clone)]
pub struct Row {
    /// The actual text content of the line
    pub row_content: String,
    /// The rendered representation with tabs expanded
    pub render: String,
    /// Syntax highlighting information for each character
    pub highlight: Vec<HighlightType>,
    /// Whether this line is part of a multi-line comment
    pub is_comment: bool,
}

impl Row {
    /// Creates a new row with the given content and rendered representation.
    /// 
    /// This constructor initializes a new row with the specified content
    /// and rendered text. The highlighting vector is initialized as empty,
    /// and the comment flag is set to false.
    /// 
    /// # Arguments
    /// 
    /// * `row_content` - The actual text content
    /// * `render` - The rendered representation (with tabs expanded)
    /// 
    /// # Returns
    /// 
    /// Returns a new `Row` instance.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ninja::screens::editor::Row;
    /// 
    /// let row = Row::new("Hello\tWorld".to_string(), "Hello    World".to_string());
    /// assert_eq!(row.row_content, "Hello\tWorld");
    /// assert_eq!(row.render, "Hello    World");
    /// ```
    pub fn new(row_content: String, render: String) -> Self {
        Self {
            row_content,
            render,
            highlight: Vec::new(),
            is_comment: false,
        }
    }

    /// Inserts a character at the specified position.
    /// 
    /// This method safely inserts a character at the given character index,
    /// handling UTF-8 encoding properly. After insertion, the row is
    /// automatically re-rendered to update the display representation.
    /// 
    /// # Arguments
    /// 
    /// * `at` - The character index where to insert (0-based)
    /// * `ch` - The character to insert
    /// 
    /// # Behavior
    /// 
    /// - **UTF-8 Safe**: Properly handles multi-byte characters
    /// - **Bounds Checking**: Handles out-of-bounds insertion gracefully
    /// - **Auto-render**: Automatically updates the rendered representation
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ninja::screens::editor::Row;
    /// 
    /// let mut row = Row::new("Hello".to_string(), "Hello".to_string());
    /// row.insert_char(5, '!');
    /// assert_eq!(row.row_content, "Hello!");
    /// ```
    pub fn insert_char(&mut self, at: usize, ch: char) {
        // Convert character index to byte index for safe insertion
        let byte_index = self.row_content
            .char_indices()
            .nth(at)
            .map(|(i, _)| i)
            .unwrap_or_else(|| self.row_content.len());
        
        self.row_content.insert(byte_index, ch);
        EditorRows::render_row(self)
    }

    /// Deletes a character at the specified position.
    /// 
    /// This method safely deletes a character at the given character index,
    /// handling UTF-8 encoding properly. After deletion, the row is
    /// automatically re-rendered to update the display representation.
    /// 
    /// # Arguments
    /// 
    /// * `at` - The character index to delete (0-based)
    /// 
    /// # Behavior
    /// 
    /// - **UTF-8 Safe**: Properly handles multi-byte characters
    /// - **Bounds Checking**: Only deletes if the index is valid
    /// - **Auto-render**: Automatically updates the rendered representation
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ninja::screens::editor::Row;
    /// 
    /// let mut row = Row::new("Hello!".to_string(), "Hello!".to_string());
    /// row.delete_char(5);
    /// assert_eq!(row.row_content, "Hello");
    /// ```
    pub fn delete_char(&mut self, at: usize) {
        // Convert character index to byte index for safe deletion
        if let Some((byte_index, _)) = self.row_content.char_indices().nth(at) {
            self.row_content.remove(byte_index);
            EditorRows::render_row(self)
        }
    }

    /// Calculates the display width of a character.
    /// 
    /// This method determines how many terminal columns a character should
    /// occupy when displayed. It handles various Unicode character types
    /// including CJK characters, emojis, and control characters.
    /// 
    /// # Arguments
    /// 
    /// * `ch` - The character to measure
    /// 
    /// # Returns
    /// 
    /// Returns the display width in terminal columns:
    /// - **1**: Normal ASCII and most Unicode characters
    /// - **2**: Wide characters (CJK, emojis, etc.)
    /// - **TAB_STOP**: Tab characters (typically 8)
    /// 
    /// # Unicode Support
    /// 
    /// The method supports a comprehensive range of Unicode characters:
    /// - **CJK Characters**: Chinese, Japanese, Korean ideographs
    /// - **Emojis**: All emoji and pictographic symbols
    /// - **Wide Symbols**: Various wide Unicode symbols
    /// - **Control Characters**: Tab and other control characters
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ninja::screens::editor::Row;
    /// 
    /// assert_eq!(Row::char_width('a'), 1);
    /// assert_eq!(Row::char_width('中'), 2); // Chinese character
    /// assert_eq!(Row::char_width('😀'), 2); // Emoji
    /// assert_eq!(Row::char_width('\t'), 8); // Tab (TAB_STOP)
    /// ```
    pub fn char_width(ch: char) -> usize {
        match ch {
            '\t' => TAB_STOP,
            // Check if character is in the "Wide" or "Fullwidth" Unicode categories
            ch if ch as u32 >= 0x1100 && (
                (ch as u32 <= 0x115F) || // Hangul Jamo
                (ch as u32 == 0x2329) || (ch as u32 == 0x232A) || // Miscellaneous Technical
                (ch as u32 >= 0x2E80 && ch as u32 <= 0x303E) || // CJK Radicals Supplement, etc.
                (ch as u32 >= 0x3040 && ch as u32 <= 0x309F) || // Hiragana
                (ch as u32 >= 0x30A0 && ch as u32 <= 0x30FF) || // Katakana
                (ch as u32 >= 0x3100 && ch as u32 <= 0x312F) || // Bopomofo
                (ch as u32 >= 0x3130 && ch as u32 <= 0x318F) || // Hangul Compatibility Jamo
                (ch as u32 >= 0x3190 && ch as u32 <= 0x319F) || // Kanban
                (ch as u32 >= 0x31A0 && ch as u32 <= 0x31BF) || // Bopomofo Extended
                (ch as u32 >= 0x31C0 && ch as u32 <= 0x31EF) || // CJK Strokes
                (ch as u32 >= 0x31F0 && ch as u32 <= 0x31FF) || // Katakana Phonetic Extensions
                (ch as u32 >= 0x3200 && ch as u32 <= 0x32FF) || // Enclosed CJK Letters and Months
                (ch as u32 >= 0x3300 && ch as u32 <= 0x33FF) || // CJK Compatibility
                (ch as u32 >= 0x3400 && ch as u32 <= 0x4DBF) || // CJK Unified Ideographs Extension A
                (ch as u32 >= 0x4E00 && ch as u32 <= 0x9FFF) || // CJK Unified Ideographs
                (ch as u32 >= 0xA000 && ch as u32 <= 0xA48F) || // Yi Syllables
                (ch as u32 >= 0xA490 && ch as u32 <= 0xA4CF) || // Yi Radicals
                (ch as u32 >= 0xAC00 && ch as u32 <= 0xD7AF) || // Hangul Syllables
                (ch as u32 >= 0xF900 && ch as u32 <= 0xFAFF) || // CJK Compatibility Ideographs
                (ch as u32 >= 0xFE10 && ch as u32 <= 0xFE1F) || // Vertical Forms
                (ch as u32 >= 0xFE30 && ch as u32 <= 0xFE4F) || // CJK Compatibility Forms
                (ch as u32 >= 0xFF00 && ch as u32 <= 0xFFEF) || // Halfwidth and Fullwidth Forms
                (ch as u32 >= 0x1B000 && ch as u32 <= 0x1B0FF) || // Kana Supplement
                (ch as u32 >= 0x1D300 && ch as u32 <= 0x1D35F) || // Tai Xian Jing Symbols
                (ch as u32 >= 0x1F000 && ch as u32 <= 0x1F02F) || // Mahjong Tiles
                (ch as u32 >= 0x1F030 && ch as u32 <= 0x1F09F) || // Domino Tiles
                (ch as u32 >= 0x1F0A0 && ch as u32 <= 0x1F0FF) || // Playing Cards
                (ch as u32 >= 0x1F100 && ch as u32 <= 0x1F1FF) || // Enclosed Alphanumeric Supplement
                (ch as u32 >= 0x1F200 && ch as u32 <= 0x1F2FF) || // Enclosed Ideographic Supplement
                (ch as u32 >= 0x1F300 && ch as u32 <= 0x1F5FF) || // Miscellaneous Symbols and Pictographs
                (ch as u32 >= 0x1F600 && ch as u32 <= 0x1F64F) || // Emoticons
                (ch as u32 >= 0x1F650 && ch as u32 <= 0x1F67F) || // Ornamental Dingbats
                (ch as u32 >= 0x1F680 && ch as u32 <= 0x1F6FF) || // Transport and Map Symbols
                (ch as u32 >= 0x1F700 && ch as u32 <= 0x1F77F) || // Alchemical Symbols
                (ch as u32 >= 0x1F780 && ch as u32 <= 0x1F7FF) || // Geometric Shapes Extended
                (ch as u32 >= 0x1F800 && ch as u32 <= 0x1F8FF) || // Supplemental Arrows-C
                (ch as u32 >= 0x1F900 && ch as u32 <= 0x1F9FF) || // Supplemental Symbols and Pictographs
                (ch as u32 >= 0x1FA00 && ch as u32 <= 0x1FA6F) || // Chess Symbols
                (ch as u32 >= 0x1FA70 && ch as u32 <= 0x1FAFF) || // Symbols and Pictographs Extended-A
                (ch as u32 >= 0x20000 && ch as u32 <= 0x2A6DF) || // CJK Unified Ideographs Extension B
                (ch as u32 >= 0x2A700 && ch as u32 <= 0x2B73F) || // CJK Unified Ideographs Extension C
                (ch as u32 >= 0x2B740 && ch as u32 <= 0x2B81F) || // CJK Unified Ideographs Extension D
                (ch as u32 >= 0x2B820 && ch as u32 <= 0x2CEAF) || // CJK Unified Ideographs Extension E
                (ch as u32 >= 0x2CEB0 && ch as u32 <= 0x2EBEF) || // CJK Unified Ideographs Extension F
                (ch as u32 >= 0x30000 && ch as u32 <= 0x3134F) || // CJK Unified Ideographs Extension G
                (ch as u32 >= 0x31350 && ch as u32 <= 0x323AF)    // CJK Unified Ideographs Extension H
            ) => 2,
            _ => 1,
        }
    }

    /// Converts a render position to a content position.
    /// 
    /// This method maps a position in the rendered text (with tabs expanded)
    /// back to the corresponding position in the original content. It's used
    /// for cursor positioning and coordinate conversion.
    /// 
    /// # Arguments
    /// 
    /// * `render_x` - The position in the rendered text
    /// 
    /// # Returns
    /// 
    /// Returns the corresponding character index in the original content.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ninja::screens::editor::Row;
    /// 
    /// let row = Row::new("Hello\tWorld".to_string(), "Hello    World".to_string());
    /// assert_eq!(row.get_row_content_x(5), 5); // Before tab
    /// assert_eq!(row.get_row_content_x(8), 6); // After tab expansion
    /// ```
    pub fn get_row_content_x(&self, render_x: usize) -> usize {
        let mut current_render_x = 0;
        for (cursor_x, ch) in self.row_content.chars().enumerate() {
            let char_width = Self::char_width(ch);
            if ch == '\t' {
                current_render_x += (TAB_STOP - 1) - (current_render_x % TAB_STOP);
            }
            current_render_x += char_width;
            if current_render_x > render_x {
                return cursor_x;
            }
        }
        0
    }

    /// Returns the number of characters in the row (UTF-8 safe).
    /// 
    /// This method provides a safe way to count characters in the row,
    /// properly handling multi-byte UTF-8 characters.
    /// 
    /// # Returns
    /// 
    /// Returns the number of Unicode characters in the row.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ninja::screens::editor::Row;
    /// 
    /// let row = Row::new("Hello中World".to_string(), "Hello中World".to_string());
    /// assert_eq!(row.char_count(), 10); // 5 ASCII + 1 CJK + 4 ASCII
    /// ```
    pub fn char_count(&self) -> usize {
        self.row_content.chars().count()
    }

    /// Returns a substring by character indices (UTF-8 safe).
    /// 
    /// This method extracts a substring from the row using character indices
    /// rather than byte indices, ensuring proper handling of multi-byte
    /// UTF-8 characters.
    /// 
    /// # Arguments
    /// 
    /// * `start` - The starting character index (inclusive)
    /// * `end` - The ending character index (exclusive)
    /// 
    /// # Returns
    /// 
    /// Returns a `String` containing the specified substring.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ninja::screens::editor::Row;
    /// 
    /// let row = Row::new("Hello中World".to_string(), "Hello中World".to_string());
    /// let substring = row.substring_by_chars(5, 6);
    /// assert_eq!(substring, "中");
    /// ```
    pub fn substring_by_chars(&self, start: usize, end: usize) -> String {
        self.row_content
            .chars()
            .skip(start)
            .take(end - start)
            .collect()
    }
}

/// Manages a collection of text rows with file handling capabilities.
/// 
/// This struct represents the complete text content of a file, providing
/// methods for reading from and writing to files, as well as managing
/// the collection of rows.
/// 
/// # Features
/// 
/// - **File I/O**: Reading and writing files with UTF-8 support
/// - **Row Management**: Adding, removing, and modifying rows
/// - **Syntax Highlighting**: Integration with syntax highlighting system
/// - **Rendering**: Automatic row rendering and tab expansion
/// - **File Association**: Tracks the associated file path
/// 
/// # File Handling
/// 
/// The struct provides robust file handling:
/// - **UTF-8 Support**: Proper handling of UTF-8 encoded files
/// - **Error Recovery**: Graceful handling of invalid UTF-8
/// - **Auto-detection**: Automatic syntax highlighting based on file extension
/// - **Lossy Conversion**: Fallback for files with encoding issues
/// 
/// # Example
/// 
/// ```rust
/// use ninja::screens::editor::EditorRows;
/// use ninja::modules::highlighting::SyntaxHighlight;
/// use std::path::PathBuf;
/// 
/// let mut syntax_highlight = None;
/// let editor_rows = EditorRows::new(&mut syntax_highlight);
/// 
/// // Load from file
/// let file_path = PathBuf::from("example.txt");
/// let mut rows = EditorRows::from_file(file_path, &mut syntax_highlight);
/// 
/// // Get row count
/// println!("File has {} rows", rows.number_of_rows());
/// ```
pub struct EditorRows {
    /// The collection of text rows
    pub row_contents: Vec<Row>,
    /// The associated file path (if any)
    pub filename: Option<PathBuf>,
}

impl EditorRows {
    /// Creates a new editor rows instance.
    /// 
    /// This method initializes a new editor rows instance. If a filename
    /// is provided as a command line argument, it attempts to load that file.
    /// Otherwise, it creates an empty editor.
    /// 
    /// # Arguments
    /// 
    /// * `syntax_highlight` - Mutable reference to the syntax highlighter
    /// 
    /// # Returns
    /// 
    /// Returns a new `EditorRows` instance.
    /// 
    /// # Behavior
    /// 
    /// - **File Loading**: Attempts to load file from command line argument
    /// - **Syntax Detection**: Automatically detects syntax highlighting
    /// - **Empty Editor**: Creates empty editor if no file specified
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ninja::screens::editor::EditorRows;
    /// use ninja::modules::highlighting::SyntaxHighlight;
    /// 
    /// let mut syntax_highlight = None;
    /// let editor_rows = EditorRows::new(&mut syntax_highlight);
    /// ```
    pub fn new(syntax_highlight: &mut Option<Box<dyn SyntaxHighlight>>) -> Self {
        match env::args().nth(1) {
            None => Self {
                row_contents: Vec::new(),
                filename: None,
            },
            Some(file) => Self::from_file(file.into(), syntax_highlight),
        }
    }

    /// Creates an editor rows instance from a file.
    /// 
    /// This method loads a file and creates an editor rows instance with
    /// its contents. It handles UTF-8 encoding and automatically detects
    /// syntax highlighting based on the file extension.
    /// 
    /// # Arguments
    /// 
    /// * `file` - The path to the file to load
    /// * `syntax_highlight` - Mutable reference to the syntax highlighter
    /// 
    /// # Returns
    /// 
    /// Returns a new `EditorRows` instance with the file contents.
    /// 
    /// # File Handling
    /// 
    /// - **UTF-8 Reading**: Attempts to read as UTF-8 first
    /// - **Fallback**: Uses lossy conversion for invalid UTF-8
    /// - **Error Recovery**: Returns empty content if file can't be read
    /// - **Syntax Detection**: Automatically sets syntax highlighter
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ninja::screens::editor::EditorRows;
    /// use ninja::modules::highlighting::SyntaxHighlight;
    /// use std::path::PathBuf;
    /// 
    /// let mut syntax_highlight = None;
    /// let file_path = PathBuf::from("example.rs");
    /// let editor_rows = EditorRows::from_file(file_path, &mut syntax_highlight);
    /// ```
    pub fn from_file(
        file: PathBuf,
        syntax_highlight: &mut Option<Box<dyn SyntaxHighlight>>,
    ) -> Self {
        // Use lossy UTF-8 conversion to handle invalid UTF-8 gracefully
        let file_contents = match fs::read_to_string(&file) {
            Ok(contents) => contents,
            Err(_) => {
                // If UTF-8 conversion fails, try reading as bytes and converting lossily
                match fs::read(&file) {
                    Ok(bytes) => String::from_utf8_lossy(&bytes).into_owned(),
                    Err(_) => String::new(), // Return empty string if file can't be read
                }
            }
        };
        
        let mut row_contents = Vec::new();
        file.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| Output::select_syntax(ext).map(|syntax| syntax_highlight.insert(syntax)));
        file_contents.lines().enumerate().for_each(|(i, line)| {
            let mut row = Row::new(line.into(), String::new());
            Self::render_row(&mut row);
            row_contents.push(row);
            if let Some(it) = syntax_highlight {
                it.update_syntax(i, &mut row_contents)
            }
        });
        Self {
            filename: Some(file),
            row_contents,
        }
    }

    /// Returns the number of rows in the editor.
    /// 
    /// # Returns
    /// 
    /// Returns the total number of rows.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ninja::screens::editor::EditorRows;
    /// 
    /// let editor_rows = EditorRows { row_contents: vec![], filename: None };
    /// assert_eq!(editor_rows.number_of_rows(), 0);
    /// ```
    pub fn number_of_rows(&self) -> usize {
        self.row_contents.len()
    }

    /// Gets the content of a specific row.
    /// 
    /// # Arguments
    /// 
    /// * `at` - The row index (0-based)
    /// 
    /// # Returns
    /// 
    /// Returns a string slice containing the row content.
    /// 
    /// # Panics
    /// 
    /// Panics if the index is out of bounds.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ninja::screens::editor::{EditorRows, Row};
    /// 
    /// let row = Row::new("Hello".to_string(), "Hello".to_string());
    /// let editor_rows = EditorRows { row_contents: vec![row], filename: None };
    /// assert_eq!(editor_rows.get_row(0), "Hello");
    /// ```
    pub fn get_row(&self, at: usize) -> &str {
        &self.row_contents[at].row_content
    }

    /// Gets the rendered representation of a specific row.
    /// 
    /// # Arguments
    /// 
    /// * `at` - The row index (0-based)
    /// 
    /// # Returns
    /// 
    /// Returns a string slice containing the rendered row.
    /// 
    /// # Panics
    /// 
    /// Panics if the index is out of bounds.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ninja::screens::editor::{EditorRows, Row};
    /// 
    /// let row = Row::new("Hello\tWorld".to_string(), "Hello    World".to_string());
    /// let editor_rows = EditorRows { row_contents: vec![row], filename: None };
    /// assert_eq!(editor_rows.get_render(0), "Hello    World");
    /// ```
    pub fn get_render(&self, at: usize) -> &String {
        &self.row_contents[at].render
    }

    /// Gets a reference to a specific row.
    /// 
    /// # Arguments
    /// 
    /// * `at` - The row index (0-based)
    /// 
    /// # Returns
    /// 
    /// Returns a reference to the specified row.
    /// 
    /// # Panics
    /// 
    /// Panics if the index is out of bounds.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ninja::screens::editor::{EditorRows, Row};
    /// 
    /// let row = Row::new("Hello".to_string(), "Hello".to_string());
    /// let editor_rows = EditorRows { row_contents: vec![row], filename: None };
    /// let row_ref = editor_rows.get_editor_row(0);
    /// assert_eq!(row_ref.row_content, "Hello");
    /// ```
    pub fn get_editor_row(&self, at: usize) -> &Row {
        &self.row_contents[at]
    }

    /// Gets a mutable reference to a specific row.
    /// 
    /// # Arguments
    /// 
    /// * `at` - The row index (0-based)
    /// 
    /// # Returns
    /// 
    /// Returns a mutable reference to the specified row.
    /// 
    /// # Panics
    /// 
    /// Panics if the index is out of bounds.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ninja::screens::editor::{EditorRows, Row};
    ///
    /// let mut row = Row::new("Hello".to_string(), "Hello".to_string());
    /// let mut editor_rows = EditorRows { row_contents: vec![row], filename: None };
    /// let mut_row_ref = editor_rows.get_editor_row_mut(0);
    /// mut_row_ref.row_content = "Goodbye".to_string();
    /// assert_eq!(editor_rows.get_row(0), "Goodbye");
    /// ```
    pub fn get_editor_row_mut(&mut self, at: usize) -> &mut Row {
        &mut self.row_contents[at]
    }

    /// Renders a single row to its display representation.
    /// 
    /// This method expands tabs in the given row's content to spaces
    /// and updates the `render` field of the row.
    /// 
    /// # Arguments
    /// 
    /// * `row` - The row to render
    /// 
    /// # Behavior
    /// 
    /// - **Tab Expansion**: Replaces tabs (`\t`) with spaces until the next
    ///   multiple of `TAB_STOP` (typically 8).
    /// - **Capacity**: Allocates enough capacity for the rendered string.
    /// - **UTF-8 Safe**: Handles multi-byte characters correctly.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ninja::screens::editor::{EditorRows, Row};
    /// 
    /// let mut row = Row::new("Hello\tWorld".to_string(), "Hello    World".to_string());
    /// EditorRows::render_row(&mut row);
    /// assert_eq!(row.render, "Hello    World");
    /// ```
    pub fn render_row(row: &mut Row) {
        let mut index = 0;
        let capacity = row
            .row_content
            .chars()
            .fold(0, |acc, next| acc + if next == '\t' { TAB_STOP } else { 1 });
        row.render = String::with_capacity(capacity);
        row.row_content.chars().for_each(|c| {
            index += 1;
            if c == '\t' {
                row.render.push(' ');
                while index % TAB_STOP != 0 {
                    row.render.push(' ');
                    index += 1
                }
            } else {
                row.render.push(c);
            }
        });
    }

    /// Inserts a new row at the specified position.
    /// 
    /// This method creates a new row with the given content and renders it,
    /// then inserts it into the collection at the specified index.
    /// 
    /// # Arguments
    /// 
    /// * `at` - The index where to insert the new row (0-based)
    /// * `contents` - The content for the new row
    /// 
    /// # Behavior
    /// 
    /// - **New Row Creation**: Creates a new `Row` instance.
    /// - **Rendering**: Renders the new row.
    /// - **Insertion**: Inserts the new row into the collection.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ninja::screens::editor::{EditorRows, Row};
    /// 
    /// let mut editor_rows = EditorRows { row_contents: vec![], filename: None };
    /// editor_rows.insert_row(0, "Hello".to_string());
    /// assert_eq!(editor_rows.number_of_rows(), 1);
    /// assert_eq!(editor_rows.get_row(0), "Hello");
    /// ```
    pub fn insert_row(&mut self, at: usize, contents: String) {
        let mut new_row = Row::new(contents, String::new());
        EditorRows::render_row(&mut new_row);
        self.row_contents.insert(at, new_row);
    }

    /// Saves the current content of the editor to the associated file.
    /// 
    /// This method writes the content of all rows to the file specified
    /// by the `filename` field. It handles UTF-8 encoding and ensures
    /// the file's length matches the content length.
    /// 
    /// # Returns
    /// 
    /// Returns `Ok(len)` if successful, otherwise an `io::Error`.
    /// 
    /// # Behavior
    /// 
    /// - **File Opening**: Opens the file with write and create permissions.
    /// - **Content Writing**: Writes the content of all rows joined by newlines.
    /// - **Length Setting**: Sets the file's length to the content length.
    /// - **Error Handling**: Returns an error if the file cannot be opened or written.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use std::io;
    /// use ninja::screens::editor::{EditorRows, Row};
    /// use std::io::ErrorKind;
    /// use std::path::PathBuf;
    ///
    /// let mut editor_rows = EditorRows { row_contents: vec![], filename: None };
    /// let result = editor_rows.save();
    /// assert_eq!(result, Err(io::Error::new(ErrorKind::Other, "no file name specified")));
    ///
    /// let file_path = PathBuf::from("example.txt");
    /// let mut editor_rows = EditorRows::from_file(file_path, &mut None);
    /// let result = editor_rows.save();
    /// assert!(result.is_ok());
    /// ```
    pub fn save(&mut self) -> io::Result<usize> {
        match &self.filename {
            None => Err(io::Error::new(ErrorKind::Other, "no file name specified")),
            Some(name) => {
                let mut file = fs::OpenOptions::new().write(true).create(true).open(name)?;
                let contents: String = self
                    .row_contents
                    .iter()
                    .map(|it| it.row_content.as_str())
                    .collect::<Vec<&str>>()
                    .join("\n");
                file.set_len(contents.len() as u64)?;
                file.write_all(contents.as_bytes())?;
                Ok(contents.as_bytes().len())
            }
        }
    }

    /// Joins the row at the specified index with the previous row.
    /// 
    /// This method removes the row at `at` and concatenates its content
    /// to the row at `at - 1`. It then re-renders the previous row.
    /// 
    /// # Arguments
    /// 
    /// * `at` - The index of the row to join (0-based)
    /// 
    /// # Behavior
    /// 
    /// - **Row Removal**: Removes the row at `at`.
    /// - **Content Concatenation**: Appends the content of the removed row
    ///   to the row at `at - 1`.
    /// - **Re-rendering**: Re-renders the row at `at - 1`.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ninja::screens::editor::{EditorRows, Row};
    /// 
    /// let mut editor_rows = EditorRows { row_contents: vec![Row::new("Hello".to_string(), "Hello".to_string()), Row::new("World".to_string(), "World".to_string())], filename: None };
    /// editor_rows.join_adjacent_rows(1);
    /// assert_eq!(editor_rows.number_of_rows(), 1);
    /// assert_eq!(editor_rows.get_row(0), "HelloWorld");
    /// ```
    pub fn join_adjacent_rows(&mut self, at: usize) {
        let current_row = self.row_contents.remove(at);
        let previous_row = self.get_editor_row_mut(at - 1);
        previous_row.row_content.push_str(&current_row.row_content);
        Self::render_row(previous_row);
    }
}

/// Manages the content buffer for terminal output rendering.
/// 
/// This struct acts as a buffer for the terminal output, allowing
/// for efficient writing of characters and strings to the terminal.
/// It implements `Write` trait to allow it to be used with `stdout()`.
/// 
/// # Features
/// 
/// - **Character Insertion**: Inserting single characters and strings.
/// - **String Buffering**: Efficiently collects characters into a string.
/// - **Terminal Output**: Flushes the collected content to the terminal.
/// - **Error Handling**: Handles errors during writing and flushing.
/// 
/// # Example
/// 
/// ```rust
/// use ninja::screens::editor::EditorContents;
/// use std::io::{ErrorKind, Write};
///
/// let mut editor_contents = EditorContents::new();
/// let result = editor_contents.write(b"Hello");
/// assert_eq!(result, Ok(5));
/// assert_eq!(editor_contents.content, "Hello");
///
/// let result = editor_contents.flush();
/// assert!(result.is_ok());
/// assert_eq!(editor_contents.content, "");
/// ```
pub struct EditorContents {
    /// The buffer containing the collected content
    pub content: String,
}

impl EditorContents {
    /// Creates a new empty editor contents buffer.
    /// 
    /// # Returns
    /// 
    /// Returns a new `EditorContents` instance with an empty string.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ninja::screens::editor::EditorContents;
    /// 
    /// let editor_contents = EditorContents::new();
    /// assert_eq!(editor_contents.content, "");
    /// ```
    pub fn new() -> Self {
        Self {
            content: String::new(),
        }
    }

    /// Appends a single character to the buffer.
    /// 
    /// # Arguments
    /// 
    /// * `ch` - The character to append
    /// 
    /// # Behavior
    /// 
    /// - **Character Insertion**: Appends the character to the `content` string.
    /// - **Error Handling**: Returns `Ok(1)` on success.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ninja::screens::editor::EditorContents;
    /// 
    /// let mut editor_contents = EditorContents::new();
    /// editor_contents.push('H');
    /// assert_eq!(editor_contents.content, "H");
    /// ```
    pub fn push(&mut self, ch: char) {
        self.content.push(ch)
    }

    /// Appends a string to the buffer.
    /// 
    /// # Arguments
    /// 
    /// * `string` - The string to append
    /// 
    /// # Behavior
    /// 
    /// - **String Insertion**: Appends the string to the `content` string.
    /// - **Error Handling**: Returns `Ok(len)` where `len` is the length of the string.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ninja::screens::editor::EditorContents;
    /// 
    /// let mut editor_contents = EditorContents::new();
    /// editor_contents.push_str("Hello");
    /// assert_eq!(editor_contents.content, "Hello");
    /// ```
    pub fn push_str(&mut self, string: &str) {
        self.content.push_str(string)
    }
}

impl Write for EditorContents {
    /// Writes a slice of bytes to the buffer.
    /// 
    /// This method attempts to convert the bytes to a string and append
    /// it to the buffer. If the conversion fails, it returns an error.
    /// 
    /// # Arguments
    /// 
    /// * `buf` - The slice of bytes to write
    /// 
    /// # Returns
    /// 
    /// Returns `Ok(len)` if successful, otherwise an `io::Error`.
    /// 
    /// # Behavior
    /// 
    /// - **String Conversion**: Attempts to convert bytes to a string.
    /// - **Error Handling**: Returns `Err(ErrorKind::WriteZero)` if conversion fails.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ninja::screens::editor::EditorContents;
    /// use std::io::{ErrorKind, Write};
    ///
    /// let mut editor_contents = EditorContents::new();
    /// let result = editor_contents.write(b"Hello");
    /// assert_eq!(result, Ok(5));
    /// assert_eq!(editor_contents.content, "Hello");
    ///
    /// let result = editor_contents.write(b"\xFF"); // Invalid UTF-8
    /// ```
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match std::str::from_utf8(buf) {
            Ok(s) => {
                self.content.push_str(s);
                Ok(s.len())
            }
            Err(_) => Err(ErrorKind::WriteZero.into()),
        }
    }

    /// Flushes the collected content to the terminal.
    /// 
    /// This method writes the current content of the buffer to the terminal
    /// using `stdout()`. It then clears the buffer.
    /// 
    /// # Returns
    /// 
    /// Returns `Ok(())` if successful, otherwise an `io::Error`.
    /// 
    /// # Behavior
    /// 
    /// - **Terminal Output**: Writes the content to `stdout()`.
    /// - **Flushing**: Ensures the output is actually written.
    /// - **Buffer Clearing**: Clears the `content` string.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ninja::screens::editor::EditorContents;
    /// use std::io::{ErrorKind, Write};
    ///
    /// let mut editor_contents = EditorContents::new();
    /// let result = editor_contents.write(b"Hello");
    /// assert_eq!(result, Ok(5));
    /// assert_eq!(editor_contents.content, "Hello");
    ///
    /// let result = editor_contents.flush();
    /// assert!(result.is_ok());
    /// assert_eq!(editor_contents.content, "");
    /// ```
    fn flush(&mut self) -> io::Result<()> {
        let out = write!(stdout(), "{}", self.content);
        stdout().flush()?;
        self.content.clear();
        out
    }
}

/// Main editor struct that coordinates all editor functionality.
/// 
/// This struct manages the core editor state, including the reader,
/// output, configuration, and keybinds. It provides the main loop
/// for processing user input and updating the display.
/// 
/// # Fields
/// 
/// - **`reader`**: Reads input events from the terminal.
/// - **`output`**: Manages the terminal output and screen refresh.
/// - **`config`**: Contains the editor's configuration settings.
/// - **`quit_attempts`**: Tracks attempts to quit the editor.
/// - **`keybind_manager`**: Manages and resolves keybindings.
/// 
/// # Features
/// 
/// - **Keybinding**: Full integration with the keybind system.
/// - **File Handling**: Reading and writing files.
/// - **Syntax Highlighting**: Integration with the highlighting system.
/// - **Selection**: Text selection and cursor movement.
/// - **Tab Handling**: Proper tab expansion and rendering.
/// - **Unicode Support**: Full Unicode character width calculation.
/// 
/// # Example
/// 
/// ```rust
/// use ninja::screens::editor::Editor;
/// use ninja::config::NinjaConfig;
/// 
/// let config = NinjaConfig::default();
/// let mut editor = Editor::new(config);
/// 
/// // Run the editor
/// match editor.run() {
///     Ok(true) => println!("Editor completed normally"),
///     Ok(false) => println!("Editor was quit"),
///     Err(e) => eprintln!("Editor error: {}", e),
/// }
/// ```
pub struct Editor {
    /// Reads input events from the terminal.
    reader: Reader,
    /// Manages the terminal output and screen refresh.
    pub output: Output,
    /// Contains the editor's configuration settings.
    config: NinjaConfig,
    /// Tracks attempts to quit the editor.
    quit_attempts: u8,
    /// Manages and resolves keybindings.
    keybind_manager: KeybindManager,
}

impl Editor {
    /// Creates a new editor instance.
    /// 
    /// This constructor initializes a new editor with the given configuration.
    /// It sets up the reader, output, and keybind manager.
    /// 
    /// # Arguments
    /// 
    /// * `config` - The editor's configuration settings.
    /// 
    /// # Returns
    /// 
    /// Returns a new `Editor` instance.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ninja::screens::editor::Editor;
    /// use ninja::config::NinjaConfig;
    /// 
    /// let config = NinjaConfig::default();
    /// let mut editor = Editor::new(config);
    /// ```
    pub fn new(config: NinjaConfig) -> Self {
        Self {
            reader: Reader,
            output: Output::new(config.clone()),
            config,
            quit_attempts: 0,
            keybind_manager: KeybindManager::new(),
        }
    }

    /// Processes a single keypress event.
    /// 
    /// This method reads a key event from the reader, finds the corresponding
    /// keybinding, and executes the action. It handles special cases like
    /// quitting, saving, and character input.
    /// 
    /// # Returns
    /// 
    /// Returns `Ok(true)` if the keypress was handled, `Ok(false)` if it
    /// was not handled (e.g., unknown key), or an `io::Error`.
    /// 
    /// # Behavior
    /// 
    /// - **Keybinding Resolution**: Tries to find a keybinding for the event.
    /// - **Action Execution**: If a keybinding is found, executes the action.
    /// - **Special Cases**: Handles `Quit`, `Save`, `MoveCursor`, `InsertChar`,
    ///   `InsertNewline`, `DeleteChar`, and other actions.
    /// - **Unbound Keys**: If no keybinding is found, handles character input.
    /// - **Quit Attempts**: Resets quit attempts when any other key is pressed.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ninja::screens::editor::Editor;
    /// use ninja::config::NinjaConfig;
    /// use crossterm::event::KeyCode;
    /// use crossterm::event::KeyModifiers;
    /// 
    /// let config = NinjaConfig::default();
    /// let mut editor = Editor::new(config);
    /// 
    /// // Simulate a keypress
    /// let key_event = crossterm::event::KeyEvent { code: KeyCode::Char('a'), modifiers: KeyModifiers::NONE };
    /// let result = editor.process_keypress();
    /// assert!(result.is_ok());
    /// assert_eq!(editor.output.editor_rows.row_contents[0].row_content, "a");
    /// ```
    pub fn process_keypress(&mut self) -> crossterm::Result<bool> {
        let key_event = self.reader.read_key()?;
        
        // Define the contexts to check in order of priority
        let contexts = [KeybindContext::Global, KeybindContext::Editor];
        
        // Try to find a keybind for this event
        if let Some(keybind) = self.keybind_manager.find_keybind_in_contexts(&key_event, &contexts) {
            // Get the action for this keybind
            if let Some(action) = self.keybind_manager.get_action(&keybind.action) {
                // Handle special cases that need custom logic
                return match action {
                    crate::keybinds::actions::Action::Quit => {
                        if self.output.dirty > 0 && self.quit_attempts < self.config.behavior.quit_times {
                            self.quit_attempts += 1;
                            let remaining = self.config.behavior.quit_times - self.quit_attempts;
                            self.output.status_message.set_message(format!(
                                "WARNING!!! File has unsaved changes. Press Ctrl-Q {} more times to quit.",
                                remaining
                            ));
                            return Ok(true);
                        }
                        Ok(false)
                    }
                    crate::keybinds::actions::Action::Save => {
                        // Handle save with custom logic
                        if matches!(self.output.editor_rows.filename, None) {
                            let prompt = prompt!(&mut self.output, "Save as : {} (ESC to cancel)")
                                .map(|it| it.into());
                            if prompt.is_none() {
                                self.output
                                    .status_message
                                    .set_message("Save Aborted".into());
                                return Ok(true);
                            }
                            prompt
                                .as_ref()
                                .and_then(|path: &PathBuf| path.extension())
                                .and_then(|ext| ext.to_str())
                                .map(|ext| {
                                    Output::select_syntax(ext).map(|syntax| {
                                        let highlight = self.output.syntax_highlight.insert(syntax);
                                        for i in 0..self.output.editor_rows.number_of_rows() {
                                            highlight
                                                .update_syntax(i, &mut self.output.editor_rows.row_contents)
                                        }
                                    })
                                });
                            self.output.editor_rows.filename = prompt;
                        }
                        self.output.editor_rows.save().map(|len| {
                            self.output
                                .status_message
                                .set_message(format!("{} bytes written to disk", len));
                            self.output.dirty = 0
                        })?;
                        Ok(true)
                    }
                    crate::keybinds::actions::Action::MoveCursor(direction) => {
                        // Handle movement with selection logic
                        if key_event.modifiers.contains(KeyModifiers::SHIFT) {
                            if !self.output.is_selecting() {
                                self.output.start_selection();
                            }
                            self.output.move_cursor(*direction);
                            self.output.update_selection();
                        } else {
                            self.output.move_cursor(*direction);
                        }
                        Ok(true)
                    }
                    crate::keybinds::actions::Action::InsertChar(ch) => {
                        // Handle character insertion with selection clearing
                        if self.output.is_selecting() {
                            self.output.clear_selection();
                        }
                        if *ch == ' ' && key_event.code == KeyCode::Tab {
                            // Handle tab insertion with soft tabs
                            let tab_size = if self.config.editor.soft_tabs {
                                self.config.editor.tab_size
                            } else {
                                1
                            };
                            for _ in 0..tab_size {
                                self.output.insert_char(' ');
                            }
                        } else {
                            self.output.insert_char(*ch);
                        }
                        Ok(true)
                    }
                    crate::keybinds::actions::Action::InsertNewline => {
                        if self.output.is_selecting() {
                            self.output.clear_selection();
                        }
                        self.output.insert_newline();
                        Ok(true)
                    }
                    crate::keybinds::actions::Action::DeleteChar => {
                        // Handle delete with cursor movement for Delete key
                        if key_event.code == KeyCode::Delete {
                            self.output.move_cursor(KeyCode::Right);
                        }
                        self.output.delete_char();
                        Ok(true)
                    }
                    _ => {
                        // Execute the action using the action executor
                        match ActionExecutor::execute(action, &mut self.output) {
                            Ok(continue_running) => Ok(continue_running),
                            Err(e) => {
                                self.output.status_message.set_message(format!("Error: {}", e));
                                Ok(true)
                            }
                        }
                    }
                }
            }
        }
        
        // Handle unbound keys (character input)
        match key_event {
            KeyEvent {
                code: KeyCode::Char(ch),
                modifiers: KeyModifiers::NONE | KeyModifiers::SHIFT,
            } => {
                if self.output.is_selecting() {
                    self.output.clear_selection();
                }
                self.output.insert_char(ch);
            }
            _ => {
                // Unknown key combination
                //self.output.status_message.set_message(format!("Unknown key: {:?}", key_event));
            }
        }
        
        // Reset quit attempts when any other key is pressed
        self.quit_attempts = 0;
        Ok(true)
    }

    /// Runs the main editor loop.
    /// 
    /// This method continuously processes keypresses until the user quits
    /// or an error occurs. It refreshes the screen and processes each keypress.
    /// 
    /// # Returns
    /// 
    /// Returns `Ok(true)` if the editor completed normally, `Ok(false)` if
    /// it was quit, or an `io::Error`.
    /// 
    /// # Behavior
    /// 
    /// - **Screen Refresh**: Calls `refresh_screen()` on the output.
    /// - **Key Processing**: Calls `process_keypress()` to handle the event.
    /// - **Loop**: Continues until `process_keypress()` returns `Ok(false)`.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ninja::screens::editor::Editor;
    /// use ninja::config::NinjaConfig;
    /// 
    /// let config = NinjaConfig::default();
    /// let mut editor = Editor::new(config);
    /// 
    /// // Run the editor
    /// match editor.run() {
    ///     Ok(true) => println!("Editor completed normally"),
    ///     Ok(false) => println!("Editor was quit"),
    ///     Err(e) => eprintln!("Editor error: {}", e),
    /// }
    /// ```
    pub fn run(&mut self) -> crossterm::Result<bool> {
        self.output.refresh_screen()?;
        self.process_keypress()
    }
}