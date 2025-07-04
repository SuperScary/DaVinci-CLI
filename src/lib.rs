//! # Ninja Editor
//! 
//! A fast, lightweight text editor written in Rust with syntax highlighting,
//! clipboard support, and a modern terminal interface.
//! 
//! ## Features
//! 
//! - **Syntax Highlighting**: Support for multiple programming languages including Rust, C, Java, Python, Go, JavaScript, TypeScript, HTML, CSS, and TOML
//! - **Clipboard Operations**: Copy, cut, and paste functionality with multi-item clipboard history
//! - **Text Selection**: Visual text selection with mouse and keyboard support
//! - **Undo/Redo**: Full undo/redo functionality for all text operations
//! - **Search**: Find text with navigation support
//! - **Configuration**: Customizable settings via TOML configuration files
//! - **Cross-platform**: Works on Windows, macOS, and Linux
//! 
//! ## Usage
//! 
//! ```rust
//! use ninja::run_editor;
//! 
//! fn main() -> crossterm::Result<()> {
//!     run_editor()
//! }
//! ```
//! 
//! ## Architecture
//! 
//! The editor is organized into several modules:
//! 
//! - **`config`**: Configuration management and settings
//! - **`modules`**: Core editor functionality (cursor, highlighting, search, etc.)
//! - **`screens`**: Different editor screens and UI management
//! - **`transput`**: Terminal input/output handling and text processing
//! - **`keybinds`**: Keyboard shortcuts and input handling
//! 
//! ## Configuration
//! 
//! The editor can be configured via a TOML file. See the `config` module for details
//! on available options.

pub mod modules;
pub mod config;
pub mod transput;
pub mod screens;
pub mod keybinds;

/// Current version of the Ninja editor
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Default tab stop width in spaces
pub const TAB_STOP: usize = 8;

/// Runs the main editor application.
/// 
/// This function initializes the editor, loads configuration, sets up the terminal,
/// and starts the main editor loop. It handles the complete lifecycle of the editor
/// from startup to shutdown.
/// 
/// # Returns
/// 
/// Returns `Ok(())` on successful completion, or an error if the editor
/// encounters a fatal error during execution.
/// 
/// # Errors
/// 
/// This function will return an error if:
/// - Terminal raw mode cannot be enabled
/// - Configuration cannot be loaded
/// - The editor encounters an unrecoverable error
/// 
/// # Example
/// 
/// ```rust
/// use ninja::run_editor;
/// 
/// fn main() -> crossterm::Result<()> {
///     run_editor()
/// }
/// ```
pub fn run_editor() -> crossterm::Result<()> {
    use crate::config::NinjaConfig;
    use crate::screens::screens::ScreenManager;
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

/// Macro for creating interactive prompts in the editor.
/// 
/// This macro provides a convenient way to create user input prompts with
/// customizable callbacks for handling different key events.
/// 
/// # Syntax
/// 
/// Basic usage:
/// ```rust
/// use ninja::prompt;
///
/// prompt!(output, "Search: {}");
/// ```
/// 
/// With callback:
/// ```rust
/// use ninja::prompt;
///
/// prompt!(output, "Search: {}", callback = |output, input, key| {
///     // Handle key events
/// });
/// ```
/// 
/// # Parameters
/// 
/// - `output`: A mutable reference to the editor output
/// - `args`: A format string for the prompt message
/// - `callback`: Optional callback function for handling key events
/// 
/// # Returns
/// 
/// Returns `Some(String)` containing the user input if Enter was pressed,
/// or `None` if Escape was pressed or the input was empty.
/// 
/// # Key Handling
/// 
/// The macro handles the following keys:
/// - **Enter**: Submits the input
/// - **Escape**: Cancels the input
/// - **Backspace/Delete**: Removes the last character
/// - **Character keys**: Adds characters to the input
/// - **Tab**: Adds a tab character
/// 
/// # Example
/// 
/// ```rust
/// let result = prompt!(output, "Enter filename: {}", callback = |output, input, key| {
///     match key {
///         KeyCode::Down => { /* handle down arrow */ }
///         KeyCode::Up => { /* handle up arrow */ }
///         _ => {}
///     }
/// });
/// 
/// match result {
///     Some(filename) => println!("User entered: {}", filename),
///     None => println!("User cancelled"),
/// }
/// ```
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