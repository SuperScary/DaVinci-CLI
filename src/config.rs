use config::{Config, ConfigError, Environment, File};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct DaVinciConfig {
    pub(crate) editor: EditorConfig,
    pub(crate) display: DisplayConfig,
    pub(crate) behavior: BehaviorConfig,
    pub(crate) syntax: SyntaxConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct EditorConfig {
    pub(crate) tab_size: usize,
    pub(crate) soft_tabs: bool,
    pub(crate) auto_indent: bool,
    pub(crate) show_line_numbers: bool,
    pub(crate) gutter_width: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct DisplayConfig {
    pub(crate) theme: String,
    pub(crate) status_bar_style: String,
    pub(crate) welcome_message: String,
    pub(crate) show_file_info: bool,
    pub(crate) show_syntax_info: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct BehaviorConfig {
    pub(crate) quit_times: u8,
    pub(crate) auto_save: bool,
    pub(crate) backup_files: bool,
    pub(crate) search_case_sensitive: bool,
    pub(crate) search_wrap_around: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct SyntaxConfig {
    pub(crate) enable_syntax_highlighting: bool,
    pub(crate) default_file_type: String,
    pub(crate) auto_detect_file_type: bool,
}

impl Default for DaVinciConfig {
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
                welcome_message: "DaVinci CLI --- Version {}".to_string(),
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

impl DaVinciConfig {
    pub(crate) fn load() -> Result<Self, ConfigError> {
        let config_path = Self::get_config_path();
        
        let config = Config::builder()
            // Start with default values
            .add_source(File::from_str(
                &Self::default_config_toml(),
                config::FileFormat::Toml,
            ))
            // Add config file if it exists
            .add_source(File::from(config_path).required(false))
            // Add environment variables with prefix "DAVINCI_"
            .add_source(Environment::with_prefix("DAVINCI").separator("_"))
            .build()?;

        config.try_deserialize()
    }

    pub(crate) fn get_config_path() -> PathBuf {
        let mut config_path = if let Some(home) = dirs::home_dir() {
            home.join(".config").join("davinci")
        } else {
            PathBuf::from(".")
        };
        config_path.push("config.toml");
        config_path
    }

    pub(crate) fn create_default_config() -> Result<(), std::io::Error> {
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

    fn default_config_toml() -> String {
        r#"# DaVinci CLI Configuration File
# This file allows you to customize how DaVinci operates, interacts, and looks

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
welcome_message = "DaVinci CLI --- Version {}"
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