use crate::d_io::Reader;
use crate::highlighting::{HighlightType, SyntaxHighlight};
use crate::{TAB_STOP, d_io, QUIT_TIMES, prompt};
use d_io::Output;
use std::io::{ErrorKind, Write, stdout};
use std::path::PathBuf;
use std::{cmp, env, fs, io};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

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
        self.row_content.insert(at, ch);
        EditorRows::render_row(self)
    }

    pub(crate) fn delete_char(&mut self, at: usize) {
        self.row_content.remove(at);
        EditorRows::render_row(self)
    }

    pub(crate) fn get_row_content_x(&self, render_x: usize) -> usize {
        let mut current_render_x = 0;
        for (cursor_x, ch) in self.row_content.chars().enumerate() {
            if ch == '\t' {
                current_render_x += (TAB_STOP - 1) - (current_render_x % TAB_STOP);
            }
            current_render_x += 1;
            if current_render_x > render_x {
                return cursor_x;
            }
        }
        0
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
        let file_contents = fs::read_to_string(&file).expect("Unable to read file");
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
    quit_times: u8,
}

impl Editor {
    pub(crate) fn new() -> Self {
        Self {
            reader: Reader,
            output: Output::new(),
            quit_times: QUIT_TIMES,
        }
    }

    fn process_keypress(&mut self) -> crossterm::Result<bool> {
        match self.reader.read_key()? {
            KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::CONTROL,
            } => {
                if self.output.dirty > 0 && self.quit_times > 0 {
                    self.output.status_message.set_message(format!(
                        "WARNING!!! File has unsaved changes. Press Ctrl-Q {} more times to quit.",
                        self.quit_times
                    ));
                    self.quit_times -= 1;
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
            } => self.output.insert_newline(),
            KeyEvent {
                code: code @ (KeyCode::Char(..) | KeyCode::Tab),
                modifiers: KeyModifiers::NONE | KeyModifiers::SHIFT,
            } => self.output.insert_char(match code {
                KeyCode::Tab => '\t',
                KeyCode::Char(ch) => ch,
                _ => unreachable!(),
            }),
            _ => {}
        }
        self.quit_times = QUIT_TIMES;
        Ok(true)
    }

    pub(crate) fn run(&mut self) -> crossterm::Result<bool> {
        self.output.refresh_screen()?;
        self.process_keypress()
    }
}