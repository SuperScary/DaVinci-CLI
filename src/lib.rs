pub mod modules;
pub mod config;
pub mod highlighting;
pub mod search;
pub mod d_io;
pub mod d_cursor;
pub mod status;
pub mod clipboard;
pub mod screens;
pub mod keybinds;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const TAB_STOP: usize = 8;

pub fn run_editor() -> crossterm::Result<()> {
    use crate::config::NinjaConfig;
    use crate::screens::ScreenManager;
    use crossterm::terminal;
    
    // Load configuration
    let config = NinjaConfig::load().unwrap_or_else(|e| {
        eprintln!("Failed to load configuration: {}", e);
        eprintln!("Using default configuration...");
        NinjaConfig::default()
    });
    
    // Create default config file if it doesn't exist
    if let Err(e) = NinjaConfig::create_default_config() {
        eprintln!("Warning: Could not create default config file: {}", e);
    }
    
    terminal::enable_raw_mode()?;
    let mut screen_manager = ScreenManager::new();
    screen_manager.show_editor_screen(config);
    screen_manager.run_active();
    Ok(())
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