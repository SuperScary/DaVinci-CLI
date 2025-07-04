//! # Configuration Module
//! 
//! This module handles all configuration management for the Ninja editor.
//! It provides a structured way to configure editor behavior, appearance,
//! and functionality through TOML configuration files and environment variables.
//! 
//! ## Configuration Sources
//! 
//! The configuration is loaded from multiple sources in order of precedence:
//! 
//! 1. **Environment Variables**: Variables prefixed with `NINJA_` (e.g., `NINJA_EDITOR_TAB_SIZE`)
//! 2. **Configuration File**: `~/.config/ninja/config.toml`
//! 3. **Default Values**: Built-in defaults if no other configuration is found
//! 
//! ## Configuration Structure
//! 
//! The configuration is organized into four main sections:
//! 
//! - **`editor`**: Text editing behavior and appearance
//! - **`display`**: Visual appearance and UI settings
//! - **`behavior`**: Editor behavior and interaction settings
//! - **`syntax`**: Syntax highlighting configuration
//! 
//! ## Example Configuration
//! 
//! ```toml
//! [editor]
//! tab_size = 4
//! soft_tabs = true
//! auto_indent = true
//! show_line_numbers = true
//! gutter_width = 6
//! 
//! [display]
//! theme = "default"
//! status_bar_style = "reverse"
//! welcome_message = "Ninja --- Version {}"
//! 
//! [behavior]
//! quit_times = 3
//! search_case_sensitive = false
//! search_wrap_around = true
//! 
//! [syntax]
//! enable_syntax_highlighting = true
//! auto_detect_file_type = true
//! ```

use config::{Config, ConfigError, Environment, File};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Main configuration structure for the Ninja editor.
/// 
/// This struct contains all configurable settings organized into logical groups.
/// It can be loaded from files, environment variables, or created with default values.
/// 
/// # Example
/// 
/// ```rust
/// use ninja::config::NinjaConfig;
/// 
/// // Load configuration from file and environment
/// let config = NinjaConfig::load()?;
/// 
/// // Use default configuration
/// let config = NinjaConfig::default();
/// ```
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NinjaConfig {
    /// Text editing behavior and appearance settings
    pub editor: EditorConfig,
    /// Visual appearance and UI settings
    pub display: DisplayConfig,
    /// Editor behavior and interaction settings
    pub behavior: BehaviorConfig,
    /// Syntax highlighting configuration
    pub syntax: SyntaxConfig,
}

/// Configuration for text editing behavior and appearance.
/// 
/// Controls how the editor handles text input, indentation, and basic display
/// of the text content.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EditorConfig {
    /// Number of spaces to use for indentation
    pub tab_size: usize,
    /// Whether to use spaces instead of tab characters
    pub soft_tabs: bool,
    /// Automatically indent new lines based on the previous line
    pub auto_indent: bool,
    /// Show line numbers in the left gutter
    pub show_line_numbers: bool,
    /// Width of the line number gutter in characters
    pub gutter_width: usize,
}

/// Configuration for visual appearance and UI elements.
/// 
/// Controls the visual styling and display of various UI components
/// in the editor interface.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DisplayConfig {
    /// Theme name for the editor (future feature)
    pub theme: String,
    /// Style for the status bar ("reverse", "bold", etc.)
    pub status_bar_style: String,
    /// Welcome message shown when no file is open
    pub welcome_message: String,
    /// Show file information in the status bar
    pub show_file_info: bool,
    /// Show syntax highlighting information in the status bar
    pub show_syntax_info: bool,
}

/// Configuration for editor behavior and interaction patterns.
/// 
/// Controls how the editor responds to user input and handles various
/// operations like quitting, searching, and file management.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BehaviorConfig {
    /// Number of Ctrl-Q presses required to quit when file is modified
    pub quit_times: u8,
    /// Automatically save files (future feature)
    pub auto_save: bool,
    /// Create backup files (future feature)
    pub backup_files: bool,
    /// Whether search operations are case sensitive
    pub search_case_sensitive: bool,
    /// Whether search wraps around to the beginning/end of the file
    pub search_wrap_around: bool,
}

/// Configuration for syntax highlighting behavior.
/// 
/// Controls how the editor handles syntax highlighting, including
/// which languages are supported and how file types are detected.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SyntaxConfig {
    /// Whether syntax highlighting is enabled
    pub enable_syntax_highlighting: bool,
    /// Default file type when syntax cannot be detected
    pub default_file_type: String,
    /// Automatically detect file type from file extension
    pub auto_detect_file_type: bool,
}

impl Default for NinjaConfig {
    /// Creates a new configuration with sensible default values.
    /// 
    /// These defaults provide a good starting point for most users
    /// and can be overridden through configuration files or environment variables.
    fn default() -> Self {
        Self {
            editor: EditorConfig {
                tab_size: 4,
                soft_tabs: true,
                auto_indent: true,
                show_line_numbers: true,
                gutter_width: 6,
            },
            display: DisplayConfig {
                theme: "default".to_string(),
                status_bar_style: "reverse".to_string(),
                welcome_message: "Ninja --- v{}".to_string(),
                show_file_info: true,
                show_syntax_info: true,
            },
            behavior: BehaviorConfig {
                quit_times: 3,
                auto_save: false,
                backup_files: false,
                search_case_sensitive: false,
                search_wrap_around: true,
            },
            syntax: SyntaxConfig {
                enable_syntax_highlighting: true,
                default_file_type: "text".to_string(),
                auto_detect_file_type: true,
            },
        }
    }
}

impl NinjaConfig {
    /// Loads configuration from all available sources.
    /// 
    /// This method loads configuration from multiple sources in order of precedence:
    /// 1. Environment variables (prefixed with `NINJA_`)
    /// 2. Configuration file (`~/.config/ninja/config.toml`)
    /// 3. Default values
    /// 
    /// # Returns
    /// 
    /// Returns a `NinjaConfig` instance with the loaded configuration,
    /// or a `ConfigError` if the configuration cannot be loaded.
    /// 
    /// # Errors
    /// 
    /// This function will return an error if:
    /// - The configuration file exists but is malformed
    /// - Environment variables contain invalid values
    /// - The configuration system encounters an internal error
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ninja::config::NinjaConfig;
    /// 
    /// match NinjaConfig::load() {
    ///     Ok(config) => println!("Configuration loaded successfully"),
    ///     Err(e) => eprintln!("Failed to load configuration: {}", e),
    /// }
    /// ```
    pub fn load() -> Result<Self, ConfigError> {
        let config_path = Self::get_config_path();
        
        let config = Config::builder()
            // Start with default values
            .add_source(File::from_str(
                &Self::default_config_toml(),
                config::FileFormat::Toml,
            ))
            // Add config file if it exists
            .add_source(File::from(config_path).required(false))
            // Add environment variables with prefix "NINJA_"
            .add_source(Environment::with_prefix("NINJA").separator("_"))
            .build()?;

        config.try_deserialize()
    }

    /// Gets the path to the configuration file.
    /// 
    /// The configuration file is typically located at `~/.config/ninja/config.toml`.
    /// If the home directory cannot be determined, it defaults to `./config.toml`.
    /// 
    /// # Returns
    /// 
    /// Returns the `PathBuf` to the configuration file location.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ninja::config::NinjaConfig;
    /// 
    /// let config_path = NinjaConfig::get_config_path();
    /// println!("Configuration file location: {:?}", config_path);
    /// ```
    pub fn get_config_path() -> PathBuf {
        let mut config_path = if let Some(home) = dirs::home_dir() {
            home.join(".config").join("ninja")
        } else {
            PathBuf::from(".")
        };
        config_path.push("config.toml");
        config_path
    }

    /// Creates the default configuration file if it doesn't exist.
    /// 
    /// This method creates the configuration directory and writes a default
    /// configuration file with comments explaining each option. This is useful
    /// for first-time setup or when the configuration file has been deleted.
    /// 
    /// # Returns
    /// 
    /// Returns `Ok(())` if the configuration file was created successfully
    /// or already exists, or an `std::io::Error` if the operation fails.
    /// 
    /// # Errors
    /// 
    /// This function will return an error if:
    /// - The configuration directory cannot be created
    /// - The configuration file cannot be written
    /// - Insufficient permissions to create the file or directory
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ninja::config::NinjaConfig;
    /// 
    /// if let Err(e) = NinjaConfig::create_default_config() {
    ///     eprintln!("Failed to create default config: {}", e);
    /// }
    /// ```
    pub fn create_default_config() -> Result<(), std::io::Error> {
        let config_path = Self::get_config_path();
        
        // Create directory if it doesn't exist
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        // Write default config if file doesn't exist
        if !config_path.exists() {
            std::fs::write(&config_path, Self::default_config_toml())?;
            println!("Created default configuration file at: {:?}", config_path);
        }
        
        Ok(())
    }

    /// Returns the default configuration as a TOML string.
    /// 
    /// This method provides a well-formatted TOML configuration with comments
    /// explaining each option. It's used both for creating default configuration
    /// files and as a fallback when no configuration is available.
    /// 
    /// # Returns
    /// 
    /// Returns a `String` containing the default configuration in TOML format.
    fn default_config_toml() -> String {
        r#"# Ninja Configuration File
# This file allows you to customize how Ninja operates, interacts, and looks

[editor]
# Number of spaces for indentation
tab_size = 4
# Use spaces instead of tabs
soft_tabs = true
# Automatically indent new lines
auto_indent = true
# Show line numbers in the gutter
show_line_numbers = true
# Width of the line number gutter
gutter_width = 6

[display]
# Theme for the editor (future feature)
theme = "default"
# Style for the status bar ("reverse", "bold", etc.)
status_bar_style = "reverse"
# Welcome message shown when no file is open
welcome_message = "Ninja --- Version {}"
# Show file information in status bar
show_file_info = true
# Show syntax highlighting information in status bar
show_syntax_info = true

[behavior]
# Number of Ctrl-Q presses required to quit when file is modified
quit_times = 3
# Automatically save files (future feature)
auto_save = false
# Create backup files (future feature)
backup_files = false
# Case sensitive search
search_case_sensitive = false
# Wrap around when searching
search_wrap_around = true

[syntax]
# Enable syntax highlighting
enable_syntax_highlighting = true
# Default file type when syntax cannot be detected
default_file_type = "text"
# Automatically detect file type from extension
auto_detect_file_type = true
"#.to_string()
    }
}