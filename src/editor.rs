use crate::d_io::Reader;
use crate::highlighting::{HighlightType, SyntaxHighlight};
use crate::{TAB_STOP, d_io, prompt};
use crate::config::DaVinciConfig;
use d_io::Output;
use std::io::{ErrorKind, Write, stdout};
use std::path::PathBuf;
use std::{cmp, env, fs, io};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[derive(Clone)]
pub(crate) struct Row {
    pub(crate) row_content: String,
    pub(crate) render: String,
    pub(crate) highlight: Vec<HighlightType>,
    pub(crate) is_comment: bool, // add line
}

impl Row {
    pub(crate) fn new(row_content: String, render: String) -> Self {
        Self {
            row_content,
            render,
            highlight: Vec::new(),
            is_comment: false, // add line
        }
    }

    pub(crate) fn insert_char(&mut self, at: usize, ch: char) {
        // Convert character index to byte index for safe insertion
        let byte_index = self.row_content
            .char_indices()
            .nth(at)
            .map(|(i, _)| i)
            .unwrap_or_else(|| self.row_content.len());
        
        self.row_content.insert(byte_index, ch);
        EditorRows::render_row(self)
    }

    pub(crate) fn delete_char(&mut self, at: usize) {
        // Convert character index to byte index for safe deletion
        if let Some((byte_index, _)) = self.row_content.char_indices().nth(at) {
            self.row_content.remove(byte_index);
            EditorRows::render_row(self)
        }
    }

    // Calculate the display width of a character (1 for normal chars, 2 for wide chars like emojis)
    pub(crate) fn char_width(ch: char) -> usize {
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
                (ch as u32 >= 0x3190 && ch as u32 <= 0x319F) || // Kanbun
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
                (ch as u32 >= 0x1D300 && ch as u32 <= 0x1D35F) || // Tai Xuan Jing Symbols
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

    pub(crate) fn get_row_content_x(&self, render_x: usize) -> usize {
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

    // Helper method to get character count (UTF-8 safe)
    pub(crate) fn char_count(&self) -> usize {
        self.row_content.chars().count()
    }

    // Helper method to get a substring by character indices (UTF-8 safe)
    pub(crate) fn substring_by_chars(&self, start: usize, end: usize) -> String {
        self.row_content
            .chars()
            .skip(start)
            .take(end - start)
            .collect()
    }
}

pub(crate) struct EditorRows {
    pub(crate) row_contents: Vec<Row>,
    pub(crate) filename: Option<PathBuf>,
}

impl EditorRows {
    pub(crate) fn new(syntax_highlight: &mut Option<Box<dyn SyntaxHighlight>>) -> Self {
        match env::args().nth(1) {
            None => Self {
                row_contents: Vec::new(),
                filename: None,
            },
            Some(file) => Self::from_file(file.into(), syntax_highlight),
        }
    }

    pub(crate) fn from_file(
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

    pub(crate) fn number_of_rows(&self) -> usize {
        self.row_contents.len()
    }

    pub(crate) fn get_row(&self, at: usize) -> &str {
        &self.row_contents[at].row_content
    }

    pub(crate) fn get_render(&self, at: usize) -> &String {
        &self.row_contents[at].render
    }

    pub(crate) fn get_editor_row(&self, at: usize) -> &Row {
        &self.row_contents[at]
    }

    pub(crate) fn get_editor_row_mut(&mut self, at: usize) -> &mut Row {
        &mut self.row_contents[at]
    }

    pub(crate) fn render_row(row: &mut Row) {
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

    pub(crate) fn insert_row(&mut self, at: usize, contents: String) {
        let mut new_row = Row::new(contents, String::new());
        EditorRows::render_row(&mut new_row);
        self.row_contents.insert(at, new_row);
    }

    pub(crate) fn save(&mut self) -> io::Result<usize> {
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

    pub(crate) fn join_adjacent_rows(&mut self, at: usize) {
        let current_row = self.row_contents.remove(at);
        let previous_row = self.get_editor_row_mut(at - 1);
        previous_row.row_content.push_str(&current_row.row_content);
        Self::render_row(previous_row);
    }
}

pub(crate) struct EditorContents {
    content: String,
}

impl EditorContents {
    pub(crate) fn new() -> Self {
        Self {
            content: String::new(),
        }
    }

    pub(crate) fn push(&mut self, ch: char) {
        self.content.push(ch)
    }

    pub(crate) fn push_str(&mut self, string: &str) {
        self.content.push_str(string)
    }
}

impl Write for EditorContents {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match std::str::from_utf8(buf) {
            Ok(s) => {
                self.content.push_str(s);
                Ok(s.len())
            }
            Err(_) => Err(ErrorKind::WriteZero.into()),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        let out = write!(stdout(), "{}", self.content);
        stdout().flush()?;
        self.content.clear();
        out
    }
}

pub(crate) struct Editor {
    reader: Reader,
    output: Output,
    config: DaVinciConfig,
    quit_attempts: u8,
}

impl Editor {
    pub(crate) fn new(config: DaVinciConfig) -> Self {
        Self {
            reader: Reader,
            output: Output::new(config.clone()),
            config,
            quit_attempts: 0,
        }
    }

    fn process_keypress(&mut self) -> crossterm::Result<bool> {
        match self.reader.read_key()? {
            KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::CONTROL,
            } => {
                if self.output.dirty > 0 && self.quit_attempts < self.config.behavior.quit_times {
                    self.quit_attempts += 1;
                    let remaining = self.config.behavior.quit_times - self.quit_attempts;
                    self.output.status_message.set_message(format!(
                        "WARNING!!! File has unsaved changes. Press Ctrl-Q {} more times to quit.",
                        remaining
                    ));
                    return Ok(true);
                }
                return Ok(false);
            }
            KeyEvent {
                code:
                direction
                @
                (KeyCode::Up
                | KeyCode::Down
                | KeyCode::Left
                | KeyCode::Right
                | KeyCode::Home
                | KeyCode::End),
                modifiers: KeyModifiers::NONE,
            } => self.output.move_cursor(direction),
            KeyEvent {
                code:
                direction
                @
                (KeyCode::Up
                | KeyCode::Down
                | KeyCode::Left
                | KeyCode::Right
                | KeyCode::Home
                | KeyCode::End),
                modifiers: KeyModifiers::SHIFT,
            } => {
                // Start selection if not already selecting
                if !self.output.is_selecting() {
                    self.output.start_selection();
                }
                self.output.move_cursor(direction);
                self.output.update_selection();
            }
            KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
            } => {
                if self.output.has_selection() {
                    self.output.copy_selection();
                }
            }
            KeyEvent {
                code: KeyCode::Char('v'),
                modifiers: KeyModifiers::CONTROL,
            } => {
                self.output.paste_clipboard();
            }
            KeyEvent {
                code: val @ (KeyCode::PageUp | KeyCode::PageDown),
                modifiers: KeyModifiers::NONE,
            } => {
                if matches!(val, KeyCode::PageUp) {
                    self.output.cursor_controller.cursor_y =
                        self.output.cursor_controller.row_offset
                } else {
                    self.output.cursor_controller.cursor_y = cmp::min(
                        self.output.win_size.1 + self.output.cursor_controller.row_offset - 1,
                        self.output.editor_rows.number_of_rows(),
                    );
                }
                (0..self.output.win_size.1).for_each(|_| {
                    self.output.move_cursor(if matches!(val, KeyCode::PageUp) {
                        KeyCode::Up
                    } else {
                        KeyCode::Down
                    });
                })
            }
            KeyEvent {
                code: KeyCode::Char('s'),
                modifiers: KeyModifiers::CONTROL,
            } => {
                if matches!(self.output.editor_rows.filename, None) {
                    let prompt = prompt!(&mut self.output, "Save as : {} (ESC to cancel)")
                        .map(|it| it.into());
                    if prompt.is_none() {
                        self.output
                            .status_message
                            .set_message("Save Aborted".into());
                        return Ok(true);
                    }
                    /* add the following */
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

                    self.output.editor_rows.filename = prompt
                }
                self.output.editor_rows.save().map(|len| {
                    self.output
                        .status_message
                        .set_message(format!("{} bytes written to disk", len));
                    self.output.dirty = 0
                })?;
            }
            KeyEvent {
                code: KeyCode::Char('f'),
                modifiers: KeyModifiers::CONTROL,
            } => {
                self.output.find()?;
            }
            KeyEvent {
                code: key @ (KeyCode::Backspace | KeyCode::Delete),
                modifiers: KeyModifiers::NONE,
            } => {
                if matches!(key, KeyCode::Delete) {
                    self.output.move_cursor(KeyCode::Right)
                }
                self.output.delete_char()
            }
            KeyEvent {
                code: KeyCode::Enter,
                modifiers: KeyModifiers::NONE,
            } => {
                if self.output.is_selecting() {
                    self.output.clear_selection();
                }
                self.output.insert_newline()
            }
            KeyEvent {
                code: code @ (KeyCode::Char(..) | KeyCode::Tab),
                modifiers: KeyModifiers::NONE | KeyModifiers::SHIFT,
            } => {
                if self.output.is_selecting() {
                    self.output.clear_selection();
                }
                match code {
                    KeyCode::Tab => {
                        // Insert spaces based on configuration
                        let tab_size = if self.config.editor.soft_tabs {
                            self.config.editor.tab_size
                        } else {
                            1 // For hard tabs, just insert one character
                        };
                        for _ in 0..tab_size {
                            self.output.insert_char(' ');
                        }
                    }
                    KeyCode::Char(ch) => self.output.insert_char(ch),
                    _ => unreachable!(),
                }
            }
            KeyEvent {
                code: KeyCode::Char('x'),
                modifiers: KeyModifiers::CONTROL,
            } => {
                if self.output.has_selection() {
                    self.output.cut_selection();
                }
            }
            KeyEvent {
                code: KeyCode::Char('z'),
                modifiers: KeyModifiers::CONTROL,
            } => {
                self.output.pop_undo();
            }
            _ => {}
        }
        // Reset quit attempts when any other key is pressed
        self.quit_attempts = 0;
        Ok(true)
    }

    pub(crate) fn run(&mut self) -> crossterm::Result<bool> {
        self.output.refresh_screen()?;
        self.process_keypress()
    }
}