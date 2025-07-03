# Ninja

![Ninja Icon](icon.ico)

*A modern, lightweight terminal text editor designed for the 21st century*

## Overview

Ninja is a sophisticated terminal-based text editor written in Rust, designed to bridge the gap between the power of traditional editors like Vim and Emacs, and the simplicity of modern text editors. Named after the legendary polymath Leonardo da Vinci, this editor embodies the spirit of innovation and versatility.

Unlike its predecessors, Ninja offers an intuitive learning curve while maintaining the efficiency and power that experienced developers expect. It's built with modern development workflows in mind, featuring syntax highlighting, intelligent search, clipboard integration, and a highly configurable interface.

## Key Features

### **Modern Design Philosophy**
- **Intuitive Interface**: Clean, uncluttered design that doesn't overwhelm new users
- **Progressive Disclosure**: Advanced features are available but don't interfere with basic operations
- **Consistent Behavior**: Predictable keybindings and operations across all modes

### **Core Editor Features**
- **Syntax Highlighting**: Automatic language detection with customizable themes
- **Multi-line Editing**: Full support for complex text manipulation
- **Search & Replace**: Powerful search functionality with case sensitivity options
- **Clipboard Integration**: Seamless copy, cut, and paste operations
- **File Management**: Open, edit, and save files with automatic backup options

### **Advanced Capabilities**
- **Customizable Keybindings**: Remap any key to any action with context-aware bindings
- **Multiple Screen Modes**: Editor, Debug, and Clipboard screens for different workflows
- **Configuration System**: TOML-based configuration with environment variable support
- **Cross-platform**: Works on Windows, macOS, and Linux with consistent behavior

### **User Experience**
- **Status Bar**: Real-time file information and syntax details
- **Line Numbers**: Optional gutter with customizable width
- **Tab Support**: Configurable tab size with soft/hard tab options
- **Auto-indentation**: Intelligent indentation for clean, readable code

## Comparison with Traditional Editors

### vs. Vim
| Feature | Vim | Ninja |
|---------|-----|-------------|
| **Learning Curve** | Steep (modal editing) | Gentle (intuitive shortcuts) |
| **Configuration** | Vimscript (complex) | TOML (simple, readable) |
| **Modern Features** | Requires plugins | Built-in |
| **Cross-platform** | Good | Excellent |
| **Performance** | Excellent | Excellent (Rust) |

**Why choose Ninja over Vim?**
- No need to learn modal editing modes
- Modern configuration format that's easier to understand
- Built-in features that would require multiple Vim plugins
- More intuitive for developers coming from modern IDEs

### vs. Emacs
| Feature | Emacs | Ninja |
|---------|-------|-------------|
| **Resource Usage** | High (full Lisp environment) | Low (lightweight Rust binary) |
| **Startup Time** | Slow | Instant |
| **Configuration** | Emacs Lisp (powerful but complex) | TOML (simple and declarative) |
| **Learning Curve** | Very steep | Moderate |
| **Memory Footprint** | Large | Minimal |

**Why choose Ninja over Emacs?**
- Much faster startup and lower resource usage
- Simpler configuration without learning a new language
- Focused on text editing rather than being an entire operating system
- Better performance for large files

## Getting Started

### Prerequisites
- Rust 1.70+ (for building from source)
- Windows, macOS, or Linux

### Installation

#### From Source
```bash
git clone https://github.com/SuperScary/Ninja.git
cd Ninja
cargo build --release
```

#### Binary Installation
*Coming soon - pre-built binaries will be available for all platforms*

### Basic Usage

#### Opening Files
```bash
# Open a specific file
ninja filename.txt

# Open Ninja without a file (creates new document)
ninja
```

#### Essential Commands
- **Ctrl+S**: Save current file
- **Ctrl+Q**: Quit (requires confirmation if unsaved changes)
- **Ctrl+F**: Find/search in current file
- **Ctrl+C**: Copy selected text
- **Ctrl+X**: Cut selected text
- **Ctrl+V**: Paste from clipboard
- **Ctrl+Z**: Undo
- **Ctrl+Y**: Redo

## Configuration

Ninja uses a TOML configuration file located at `~/.config/ninja/config.toml`. The editor automatically creates a default configuration file on first run.

### Example Configuration
```toml
[editor]
tab_size = 4
soft_tabs = true
auto_indent = true
show_line_numbers = true
gutter_width = 6

[display]
theme = "default"
status_bar_style = "reverse"
welcome_message = "Ninja --- Version {}"
show_file_info = true
show_syntax_info = true

[behavior]
quit_times = 3
auto_save = false
backup_files = false
search_case_sensitive = false
search_wrap_around = true

[syntax]
enable_syntax_highlighting = true
default_file_type = "text"
auto_detect_file_type = true
```

### Environment Variables
You can override configuration values using environment variables with the `NINJA_` prefix:
```bash
export NINJA_EDITOR_TAB_SIZE=2
export NINJA_DISPLAY_THEME=dark
```

## Use Cases

### Perfect For:
- **Quick File Edits**: Fast startup and intuitive interface for quick changes
- **Configuration Files**: Syntax highlighting for various config formats
- **Code Reviews**: Clean interface for reviewing code without distractions
- **Remote Development**: Lightweight enough for SSH sessions
- **Learning Programming**: Simple interface for beginners
- **System Administration**: Reliable editing on any system

### Ideal Users:
- **Developers** who want a modern terminal editor
- **System Administrators** who need a reliable text editor
- **Students** learning programming and text editing
- **Power Users** who appreciate customization without complexity
- **Teams** who want consistent editing experience across platforms

## Development

### Project Structure
```
Ninja/
├── src/
│   ├── main.rs              # Application entry point
│   ├── config.rs            # Configuration management
│   ├── screens/             # Different editor screens
│   │   ├── editor.rs        # Main editor functionality
│   │   ├── clipboard.rs     # Clipboard management
│   │   └── debug.rs         # Debug screen
│   ├── keybinds/            # Keybinding system
│   │   ├── manager.rs       # Keybind management
│   │   ├── bindings.rs      # Keybind definitions
│   │   └── actions.rs       # Action implementations
│   ├── highlighting.rs      # Syntax highlighting
│   ├── search.rs            # Search functionality
│   ├── status.rs            # Status bar management
│   ├── clipboard.rs         # Clipboard operations
│   ├── d_io.rs              # Input/output handling
│   └── d_cursor.rs          # Cursor management
├── Cargo.toml               # Rust dependencies
└── README.md               # This file
```

### Building for Development
```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run tests
cargo test

# Check code quality
cargo clippy
```

## Contributing

We welcome contributions! Please see our [Contributing Guidelines](CONTRIBUTING.md) for details.

### Areas for Contribution
- **Syntax Highlighting**: Add support for new programming languages
- **Themes**: Create new color schemes and themes
- **Keybindings**: Suggest new default keybindings or improvements
- **Documentation**: Improve documentation and examples
- **Testing**: Add tests for new features or existing functionality
- **Performance**: Optimize editor performance for large files

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- **Crossterm**: For excellent cross-platform terminal handling
- **Rust Community**: For the amazing ecosystem and tools
- **Vim & Emacs**: For inspiration and proving the value of powerful text editors
- **Modern Text Editors**: For showing how good UX can enhance productivity

## Roadmap

### Short Term (v0.1.0)
- [ ] Plugin system for extensibility
- [ ] Multiple file tabs
- [ ] Split window support
- [ ] More syntax highlighting languages
- [ ] Custom themes support

### Medium Term (v0.2.0)
- [ ] LSP (Language Server Protocol) integration
- [ ] Git integration
- [ ] Fuzzy file finder
- [ ] Command palette
- [ ] Macros and automation

### Long Term (v0.3.0)
- [ ] Collaborative editing
- [ ] Remote editing capabilities
- [ ] Advanced debugging integration
- [ ] Custom language support
- [ ] Performance profiling tools

## Support

- **Issues**: [GitHub Issues](https://github.com/SuperScary/Ninja/issues)
- **Discussions**: [GitHub Discussions](https://github.com/SuperScary/Ninja/discussions)
- **Documentation**: [Wiki](https://github.com/SuperScary/Ninja/wiki)

---

*Built with ❤️* 