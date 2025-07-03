use crate::screens::editor::EditorContents;
use crate::modules::status::StatusMessage;
use crossterm::terminal::{self, ClearType};
use crossterm::queue;

pub struct MessageBar;

impl MessageBar {
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