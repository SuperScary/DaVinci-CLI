use crate::d_cursor::CursorController;
use crate::editor::{EditorContents, EditorRows, Row};
use crate::event::KeyModifiers;
use crate::highlighting::{CHighlight, CSSHighlight, GoHighlight, HTMLHighlight, HighlightType, JavaHighlight, JavaScriptHighlight, PythonHighlight, RustHighlight, SyntaxHighlight, TypeScriptHighlight};
use crate::search::{SearchDirection, SearchIndex};
use crate::status::StatusMessage;
use crate::config::DaVinciConfig;
use crate::{prompt, VERSION};
use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::terminal::ClearType;
use crossterm::{cursor, event, execute, queue, style, terminal};
use crossterm::style::Color;
use std::io::{stdout, Write};
use std::time::Duration;
use std::{cmp, io};

pub(crate) struct Output {
    pub(crate) win_size: (usize, usize),
    editor_contents: EditorContents,
    pub(crate) cursor_controller: CursorController,
    pub(crate) editor_rows: EditorRows,
    pub(crate) status_message: StatusMessage,
    pub(crate) dirty: u64,
    search_index: SearchIndex,
    pub(crate) syntax_highlight: Option<Box<dyn SyntaxHighlight>>,
    pub(crate) config: DaVinciConfig,
    // Clipboard and selection state
    clipboard: String,
    selection_start: Option<(usize, usize)>, // (row, col)
    selection_end: Option<(usize, usize)>,   // (row, col)
    is_selecting: bool,
    // Undo stack
    undo_stack: Vec<(Vec<Row>, CursorController, u64)>,
    pending_edit: bool,
}

impl Output {
    pub(crate) fn select_syntax(extension: &str) -> Option<Box<dyn SyntaxHighlight>> {
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
            Box::new(SwiftHightlight::new()),
            Box::new(KotlinHighlight::new()),
            Box::new(DartHighlight::new()),
            Box::new(RubyHighlight::new()),*/
            Box::new(HTMLHighlight::new()),
            Box::new(CSSHighlight::new()),
        ];
        list.into_iter()
            .find(|it| it.extensions().contains(&extension))
    }

    pub(crate) fn new(config: DaVinciConfig) -> Self {
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
            clipboard: String::new(),
            selection_start: None,
            selection_end: None,
            is_selecting: false,
            undo_stack: Vec::new(),
            pending_edit: false,
        }
    }

    pub(crate) fn clear_screen() -> crossterm::Result<()> {
        execute!(stdout(), terminal::Clear(ClearType::All))?;
        execute!(stdout(), cursor::MoveTo(0, 0))
    }

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
                                let start_byte = row.render
                                    .char_indices()
                                    .nth(start_char)
                                    .map(|(i, _)| i)
                                    .unwrap_or_else(|| row.render.len());
                                
                                row.render[start_byte..]
                                    .find(&keyword)
                                    .map(|index| {
                                        // Convert back to character index
                                        let byte_index = start_byte + index;
                                        row.render[..byte_index].chars().count()
                                    })
                            } else {
                                // Convert character index to byte index for safe slicing
                                let end_byte = row.render
                                    .char_indices()
                                    .nth(output.search_index.x_index)
                                    .map(|(i, _)| i)
                                    .unwrap_or_else(|| row.render.len());
                                
                                row.render[..end_byte].rfind(&keyword)
                                    .map(|byte_index| {
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

    pub(crate) fn find(&mut self) -> io::Result<()> {
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
    pub(crate) fn start_selection(&mut self) {
        self.is_selecting = true;
        self.selection_start = Some((self.cursor_controller.cursor_y, self.cursor_controller.cursor_x));
        self.selection_end = Some((self.cursor_controller.cursor_y, self.cursor_controller.cursor_x));
    }

    pub(crate) fn update_selection(&mut self) {
        if self.is_selecting {
            self.selection_end = Some((self.cursor_controller.cursor_y, self.cursor_controller.cursor_x));
        }
    }

    pub(crate) fn clear_selection(&mut self) {
        self.is_selecting = false;
        self.selection_start = None;
        self.selection_end = None;
    }

    pub(crate) fn has_selection(&self) -> bool {
        if let (Some(start), Some(end)) = (self.selection_start, self.selection_end) {
            start != end
        } else {
            false
        }
    }

    pub(crate) fn is_selecting(&self) -> bool {
        self.is_selecting
    }

    pub(crate) fn get_selection_bounds(&self) -> Option<((usize, usize), (usize, usize))> {
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

    pub(crate) fn copy_selection(&mut self) {
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
            
            self.clipboard = selected_text;
            self.status_message.set_message(format!("Copied {} characters", self.clipboard.len()));
        }
    }

    pub(crate) fn cut_selection(&mut self) {
        if let Some(((start_row, start_col), (end_row, end_col))) = self.get_selection_bounds() {
            self.push_undo();
            self.copy_selection();
            
            // Remove the selected text
            if start_row == end_row {
                // Single line selection
                let row = self.editor_rows.get_editor_row_mut(start_row);
                let mut chars: Vec<char> = row.row_content.chars().collect();
                chars.drain(start_col..end_col);
                row.row_content = chars.into_iter().collect();
                EditorRows::render_row(row);
            } else {
                // Multi-line selection
                // First, handle the end row (remove from beginning to end_col)
                let end_row_ref = self.editor_rows.get_editor_row_mut(end_row);
                let mut end_chars: Vec<char> = end_row_ref.row_content.chars().collect();
                end_chars.drain(..end_col);
                end_row_ref.row_content = end_chars.into_iter().collect();
                EditorRows::render_row(end_row_ref);
                
                // Handle the start row (remove from start_col to end)
                let start_row_ref = self.editor_rows.get_editor_row_mut(start_row);
                let mut start_chars: Vec<char> = start_row_ref.row_content.chars().collect();
                start_chars.drain(start_col..);
                start_row_ref.row_content = start_chars.into_iter().collect();
                EditorRows::render_row(start_row_ref);
                
                // Remove all rows in between
                let rows_to_remove = end_row - start_row - 1;
                for _ in 0..rows_to_remove {
                    self.editor_rows.row_contents.remove(start_row + 1);
                }
                
                // Join the start and end rows if they're now adjacent
                if start_row + 1 < self.editor_rows.number_of_rows() {
                    let end_row_content = self.editor_rows.get_editor_row(start_row + 1).row_content.clone();
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

    pub(crate) fn paste_clipboard(&mut self) {
        if !self.clipboard.is_empty() {
            self.push_undo();
            let clipboard_content = self.clipboard.clone();
            let mut chars = clipboard_content.chars().peekable();
            
            while let Some(ch) = chars.next() {
                if ch == '\n' {
                    self.insert_newline_without_undo();
                } else {
                    self.insert_char_without_undo(ch);
                }
            }
            self.status_message.set_message(format!("Pasted {} characters", clipboard_content.len()));
            self.pending_edit = false;
        }
    }

    // Helper methods that don't push undo (used by paste operation)
    fn insert_char_without_undo(&mut self, ch: char) {
        if self.cursor_controller.cursor_y == self.editor_rows.number_of_rows() {
            self.editor_rows
                .insert_row(self.editor_rows.number_of_rows(), String::new());
            self.dirty += 1;
        }
        self.editor_rows
            .get_editor_row_mut(self.cursor_controller.cursor_y)
            .insert_char(self.cursor_controller.cursor_x, ch);
        if let Some(it) = self.syntax_highlight.as_ref() {
            it.update_syntax(
                self.cursor_controller.cursor_y,
                &mut self.editor_rows.row_contents,
            )
        }
        self.cursor_controller.cursor_x += 1;
        self.dirty += 1;
    }

    fn insert_newline_without_undo(&mut self) {
        if self.cursor_controller.cursor_x == 0 {
            // If cursor is at the beginning, check previous line for indentation
            let indent_level = if self.config.editor.auto_indent && self.cursor_controller.cursor_y > 0 {
                let previous_row = self.editor_rows.get_row(self.cursor_controller.cursor_y - 1);
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
            let current_row = self.editor_rows.get_editor_row(self.cursor_controller.cursor_y);
            let indent_level = if self.config.editor.auto_indent {
                self.get_indentation_level(&current_row.row_content)
            } else {
                0
            };
            
            let current_row = self
                .editor_rows
                .get_editor_row_mut(self.cursor_controller.cursor_y);
            
            // Use character-based substring operation for UTF-8 safety
            let new_row_content = current_row.substring_by_chars(
                self.cursor_controller.cursor_x,
                current_row.char_count()
            );
            
            // Truncate the current row at the cursor position
            let truncated_content = current_row.substring_by_chars(0, self.cursor_controller.cursor_x);
            current_row.row_content = truncated_content;
            EditorRows::render_row(current_row);
            
            // Create new line with proper indentation
            let indent_spaces = " ".repeat(indent_level);
            let mut new_line_content = indent_spaces;
            new_line_content.push_str(&new_row_content);
            
            self.editor_rows
                .insert_row(self.cursor_controller.cursor_y + 1, new_line_content);
            if let Some(it) = self.syntax_highlight.as_ref() {
                it.update_syntax(
                    self.cursor_controller.cursor_y,
                    &mut self.editor_rows.row_contents,
                );
                it.update_syntax(
                    self.cursor_controller.cursor_y + 1,
                    &mut self.editor_rows.row_contents,
                )
            }
            self.cursor_controller.cursor_x = indent_level;
        }
        self.cursor_controller.cursor_y += 1;
        self.dirty += 1;
    }

    pub(crate) fn is_position_selected(&self, row: usize, col: usize) -> bool {
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

    fn draw_message_bar(&mut self) {
        queue!(
            self.editor_contents,
            terminal::Clear(ClearType::UntilNewLine)
        )
            .unwrap();
        if let Some(msg) = self.status_message.message() {
            let msg_chars: Vec<char> = msg.chars().collect();
            let truncated_msg = if msg_chars.len() > self.win_size.0 {
                msg_chars[..self.win_size.0].iter().collect::<String>()
            } else {
                msg.clone()
            };
            self.editor_contents.push_str(&truncated_msg);
        }
    }

    pub(crate) fn delete_char(&mut self) {
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
            let previous_row = self.editor_rows.get_editor_row(self.cursor_controller.cursor_y - 1);
            self.cursor_controller.cursor_x = previous_row.char_count();
            self.editor_rows
                .join_adjacent_rows(self.cursor_controller.cursor_y);
            self.cursor_controller.cursor_y -= 1;
        }
        if let Some(it) = self.syntax_highlight.as_ref() {
            it.update_syntax(
                self.cursor_controller.cursor_y,
                &mut self.editor_rows.row_contents,
            );
        }
        self.dirty += 1;
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

    pub(crate) fn insert_newline(&mut self) {
        if !self.pending_edit {
            self.push_undo();
            self.pending_edit = true;
        }
        if self.cursor_controller.cursor_x == 0 {
            // If cursor is at the beginning, check previous line for indentation
            let indent_level = if self.config.editor.auto_indent && self.cursor_controller.cursor_y > 0 {
                let previous_row = self.editor_rows.get_row(self.cursor_controller.cursor_y - 1);
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
            let current_row = self.editor_rows.get_editor_row(self.cursor_controller.cursor_y);
            let indent_level = if self.config.editor.auto_indent {
                self.get_indentation_level(&current_row.row_content)
            } else {
                0
            };
            
            let current_row = self
                .editor_rows
                .get_editor_row_mut(self.cursor_controller.cursor_y);
            
            // Use character-based substring operation for UTF-8 safety
            let new_row_content = current_row.substring_by_chars(
                self.cursor_controller.cursor_x,
                current_row.char_count()
            );
            
            // Truncate the current row at the cursor position
            let truncated_content = current_row.substring_by_chars(0, self.cursor_controller.cursor_x);
            current_row.row_content = truncated_content;
            EditorRows::render_row(current_row);
            
            // Create new line with proper indentation
            let indent_spaces = " ".repeat(indent_level);
            let mut new_line_content = indent_spaces;
            new_line_content.push_str(&new_row_content);
            
            self.editor_rows
                .insert_row(self.cursor_controller.cursor_y + 1, new_line_content);
            if let Some(it) = self.syntax_highlight.as_ref() {
                it.update_syntax(
                    self.cursor_controller.cursor_y,
                    &mut self.editor_rows.row_contents,
                );
                it.update_syntax(
                    self.cursor_controller.cursor_y + 1,
                    &mut self.editor_rows.row_contents,
                )
            }
            self.cursor_controller.cursor_x = indent_level;
        }
        self.cursor_controller.cursor_y += 1;
        self.dirty += 1;
    }

    pub(crate) fn insert_char(&mut self, ch: char) {
        if !self.pending_edit {
            self.push_undo();
            self.pending_edit = true;
        }
        if self.cursor_controller.cursor_y == self.editor_rows.number_of_rows() {
            self.editor_rows
                .insert_row(self.editor_rows.number_of_rows(), String::new());
            self.dirty += 1;
        }
        self.editor_rows
            .get_editor_row_mut(self.cursor_controller.cursor_y)
            .insert_char(self.cursor_controller.cursor_x, ch);
        if let Some(it) = self.syntax_highlight.as_ref() {
            it.update_syntax(
                self.cursor_controller.cursor_y,
                &mut self.editor_rows.row_contents,
            )
        }
        self.cursor_controller.cursor_x += 1;
        self.dirty += 1;
    }

    fn draw_status_bar(&mut self) {
        self.editor_contents
            .push_str(&style::Attribute::Reverse.to_string());
        let info = format!(
            "{} {} -- {} lines",
            self.editor_rows
                .filename
                .as_ref()
                .and_then(|path| path.file_name())
                .and_then(|name| name.to_str())
                .unwrap_or("[No Name]"),
            if self.dirty > 0 { "(modified)" } else { "" },
            self.editor_rows.number_of_rows()
        );
        let info_len = cmp::min(info.len(), self.win_size.0);
        /* LINES AND COLUMNS */
        let line_info = format!(
            "{} | {}:{}",
            self.syntax_highlight
                .as_ref()
                .map(|highlight| highlight.file_type())
                .unwrap_or("no ft"),
            self.cursor_controller.cursor_y + 1,
            self.cursor_controller.cursor_x + 1
        );
        self.editor_contents.push_str(&info[..info_len]);
        for i in info_len..self.win_size.0 {
            if self.win_size.0 - i == line_info.len() {
                self.editor_contents.push_str(&line_info);
                break;
            } else {
                self.editor_contents.push(' ')
            }
        }
        self.editor_contents
            .push_str(&style::Attribute::Reset.to_string());
        self.editor_contents.push_str("\r\n");
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
                let len = cmp::min(render_chars.len().saturating_sub(column_offset), content_width);
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
                        let highlight_slice = if start < row.highlight.len() && end <= row.highlight.len() {
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
                                let _ = queue!(self.editor_contents, style::SetForegroundColor(color));
                            }
                            self.editor_contents.push(c);
                        }
                        let _ = queue!(self.editor_contents, style::SetForegroundColor(Color::Reset));
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

    pub(crate) fn move_cursor(&mut self, direction: KeyCode) {
        self.cursor_controller
            .move_cursor(direction, &self.editor_rows);
        self.pending_edit = false;
    }

    pub(crate) fn refresh_screen(&mut self) -> crossterm::Result<()> {
        let gutter_width = if self.config.editor.show_line_numbers {
            self.config.editor.gutter_width
        } else {
            0
        };
        self.cursor_controller.scroll(&self.editor_rows, gutter_width);
        queue!(self.editor_contents, cursor::Hide, cursor::MoveTo(0, 0))?;
        self.draw_rows();
        self.draw_status_bar();
        self.draw_message_bar();
        let cursor_x = self.cursor_controller.render_x - self.cursor_controller.column_offset + gutter_width;
        let cursor_y = self.cursor_controller.cursor_y - self.cursor_controller.row_offset;
        queue!(
            self.editor_contents,
            cursor::MoveTo(cursor_x as u16, cursor_y as u16),
            cursor::Show
        )?;
        self.editor_contents.flush()
    }

    // Undo stack methods
    pub(crate) fn push_undo(&mut self) {
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

    pub(crate) fn pop_undo(&mut self) {
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
}

pub(crate) struct Reader;

impl Reader {
    pub(crate) fn read_key(&self) -> crossterm::Result<KeyEvent> {
        loop {
            if event::poll(Duration::from_millis(500))? {
                if let Event::Key(event) = event::read()? {
                    return Ok(event);
                }
            }
        }
    }
}