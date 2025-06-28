use std::cmp;
use std::cmp::Ordering;
use crossterm::event::KeyCode;
use crate::editor::{EditorRows, Row};
use crate::TAB_STOP;

#[derive(Copy, Clone)]
pub(crate) struct CursorController {
    pub(crate) cursor_x: usize,
    pub(crate) cursor_y: usize,
    screen_rows: usize,
    screen_columns: usize,
    pub(crate) row_offset: usize,
    pub(crate) column_offset: usize,
    pub(crate) render_x: usize,
}

impl CursorController {
    pub(crate) fn new(win_size: (usize, usize)) -> CursorController {
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

    fn get_render_x(&self, row: &Row) -> usize {
        row.row_content
            .chars()
            .take(self.cursor_x)
            .fold(0, |render_x, c| {
                if c == '\t' {
                    render_x + (TAB_STOP - 1) - (render_x % TAB_STOP) + 1
                } else {
                    // Use the shared Unicode width calculation
                    render_x + crate::editor::Row::char_width(c)
                }
            })
    }

    pub(crate) fn scroll(&mut self, editor_rows: &EditorRows, gutter_width: usize) {
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

    pub(crate) fn move_cursor(&mut self, direction: KeyCode, editor_rows: &EditorRows) {
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