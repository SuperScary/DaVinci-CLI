use super::super::screens::editor::EditorContents;
use crate::modules::highlighting::SyntaxHighlight;
use crate::modules::cursor::CursorController;
use crossterm::style;
use std::cmp;

pub struct StatusBar;

impl StatusBar {
    /// Draws the status bar with file information, modification status, and cursor position
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
