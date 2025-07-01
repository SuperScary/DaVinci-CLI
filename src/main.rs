mod config;
mod highlighting;
mod search;
mod d_io;
mod d_cursor;
mod status;
mod clipboard;
mod screens;
mod keybinds;

use crate::config::DaVinciConfig;
use crate::d_io::Output;
use crossterm::{event, terminal};
use screens::editor::EditorContents;
use std::cmp;
use crate::screens::ScreenManager;

const VERSION: &str = "0.0.1-pre-alpha";
const TAB_STOP: usize = 8;

struct CleanUp;

impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Unable to disable raw mode");
        Output::clear_screen().expect("error");
    }
}

#[macro_export]
macro_rules! prompt {
    ($output:expr,$args:tt) => {
        prompt!($output, $args, callback = |&_, _, _| {})
    };
    ($output:expr,$args:tt, callback = $callback:expr) => {{
        let output: &mut Output = $output;
        let mut input = String::with_capacity(32);
        loop {
            output.status_message.set_message(format!($args, input));
            output.refresh_screen()?;
            let key_event = Reader.read_key()?;
            match key_event {
                KeyEvent {
                    code: KeyCode::Enter,
                    modifiers: KeyModifiers::NONE,
                } => {
                    if !input.is_empty() {
                        output.status_message.set_message(String::new());
                        $callback(output, &input, KeyCode::Enter);
                        break;
                    }
                }
                KeyEvent {
                    code: KeyCode::Esc, ..
                } => {
                    output.status_message.set_message(String::new());
                    input.clear();
                    $callback(output, &input, KeyCode::Esc);
                    break;
                }
                KeyEvent {
                    code: KeyCode::Backspace | KeyCode::Delete,
                    modifiers: KeyModifiers::NONE,
                } => {
                    input.pop();
                }
                KeyEvent {
                    code: code @ (KeyCode::Char(..) | KeyCode::Tab),
                    modifiers: KeyModifiers::NONE | KeyModifiers::SHIFT,
                } => {
                    input.push(match code {
                        KeyCode::Tab => '\t',
                        KeyCode::Char(ch) => ch,
                        _ => unreachable!(),
                    });
                }
                _ => {}
            }
            $callback(output, &input, key_event.code);
        }
        if input.is_empty() {
            None
        } else {
            Some(input)
        }
    }};
}

fn main() -> crossterm::Result<()> {
    let _clean_up = CleanUp;
    
    // Load configuration
    let config = DaVinciConfig::load().unwrap_or_else(|e| {
        eprintln!("Failed to load configuration: {}", e);
        eprintln!("Using default configuration...");
        DaVinciConfig::default()
    });
    
    // Create default config file if it doesn't exist
    if let Err(e) = DaVinciConfig::create_default_config() {
        eprintln!("Warning: Could not create default config file: {}", e);
    }
    
    terminal::enable_raw_mode()?;
    let mut screen_manager = ScreenManager::new();
    screen_manager.show_editor_screen(config);
    screen_manager.run_active();
    Ok(())
}