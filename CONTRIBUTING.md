# Contributing to Ninja

Thank you for your interest in contributing to Ninja! 🥷 This document provides guidelines and information for contributors.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Project Structure](#project-structure)
- [Coding Standards](#coding-standards)
- [Making Changes](#making-changes)
- [Testing](#testing)
- [Submitting Changes](#submitting-changes)
- [Areas for Contribution](#areas-for-contribution)
- [Questions and Help](#questions-and-help)

## Code of Conduct

By participating in this project, you agree to abide by our Code of Conduct. We are committed to providing a welcoming and inspiring community for all.

## Getting Started

### Prerequisites

- **Rust 1.70+**: [Install Rust](https://rustup.rs/)
- **Git**: For version control
- **A terminal**: For running the editor
- **Basic knowledge of Rust**: Familiarity with Rust syntax and concepts

### Fork and Clone

1. **Fork the repository** on GitHub
2. **Clone your fork**:
   ```bash
   git clone https://github.com/YOUR_USERNAME/Ninja.git
   cd Ninja
   ```
3. **Add the upstream remote**:
   ```bash
   git remote add upstream https://github.com/SuperScary/Ninja.git
   ```

## Development Setup

### Building from Source

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run the editor
cargo run

# Run with a specific file
cargo run -- path/to/file.txt
```

### Development Tools

We recommend installing these tools for development:

```bash
# Rust analyzer for IDE support
rustup component add rust-analyzer

# Clippy for linting
rustup component add clippy

# Rustfmt for code formatting
rustup component add rustfmt
```

### IDE Setup

**VS Code** (Recommended):
- Install the "rust-analyzer" extension
- Install the "Even Better TOML" extension for config files

**Other IDEs**:
- Any IDE with Rust support will work
- Ensure rust-analyzer is configured

## Project Structure

```
Ninja/
├── src/
│   ├── main.rs              # Binary entry point
│   ├── lib.rs               # Library crate definitions
│   ├── config.rs            # Configuration management
│   ├── modules/             # UI component modules
│   │   ├── mod.rs           # Module declarations
│   │   ├── statusbar.rs     # Status bar component
│   │   └── messagebar.rs    # Message bar component
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
├── build.rs                 # Build script
├── icon.ico                 # Application icon
└── README.md               # Project documentation
```

## Coding Standards

### Rust Style Guidelines

- **Follow Rust conventions**: Use `rustfmt` for consistent formatting
- **Use meaningful names**: Variables, functions, and types should be descriptive
- **Document public APIs**: Use `///` for documentation comments
- **Handle errors properly**: Use `Result` and `Option` appropriately
- **Write idiomatic Rust**: Follow Rust best practices and patterns

### Code Formatting

```bash
# Format all code
cargo fmt

# Check formatting without changing files
cargo fmt -- --check
```

### Linting

```bash
# Run clippy for linting
cargo clippy

# Run clippy with all warnings
cargo clippy -- -W clippy::all
```

### Documentation

- **Public APIs**: Must have documentation comments
- **Complex functions**: Include examples in documentation
- **Modules**: Document the purpose of each module
- **Configuration**: Document all configuration options

Example:
```rust
/// Draws the status bar with file information and cursor position.
/// 
/// # Arguments
/// * `editor_contents` - The editor's content buffer
/// * `win_size` - Terminal window size (width, height)
/// * `filename` - Current file path
/// * `dirty` - Number of unsaved changes
/// * `syntax_highlight` - Current syntax highlighter
/// * `cursor_controller` - Cursor position and state
pub fn draw_status_bar(
    editor_contents: &mut EditorContents,
    win_size: (usize, usize),
    filename: &Option<std::path::PathBuf>,
    dirty: u64,
    syntax_highlight: &Option<Box<dyn SyntaxHighlight>>,
    cursor_controller: &CursorController,
) {
    // Implementation...
}
```

## Making Changes

### Branch Strategy

1. **Create a feature branch** from `main`:
   ```bash
   git checkout main
   git pull upstream main
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes** following the coding standards

3. **Test your changes** thoroughly

4. **Commit your changes** with clear, descriptive messages

### Commit Messages

Follow conventional commit format:

```
type(scope): description

[optional body]

[optional footer]
```

Examples:
```
feat(editor): add multi-cursor support
fix(statusbar): resolve display issue on small terminals
docs(readme): update installation instructions
test(search): add unit tests for search functionality
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

### Testing Your Changes

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run integration tests
cargo test --test integration_test_name
```

## Testing

### Unit Tests

- **Test all new functionality**: Every new feature should have tests
- **Test edge cases**: Consider boundary conditions and error cases
- **Use descriptive test names**: Test names should explain what they test

Example:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_bar_draws_correctly() {
        // Test implementation
    }

    #[test]
    fn test_status_bar_handles_empty_filename() {
        // Test edge case
    }
}
```

### Integration Tests

- **Test user workflows**: Test complete user interactions
- **Test configuration**: Verify configuration loading and validation
- **Test cross-platform**: Ensure features work on different platforms

### Manual Testing

- **Test on different terminals**: Test on various terminal emulators
- **Test with different file types**: Verify syntax highlighting works
- **Test keyboard shortcuts**: Ensure all shortcuts work as expected
- **Test error handling**: Verify graceful handling of errors

## Submitting Changes

### Pull Request Process

1. **Push your branch** to your fork:
   ```bash
   git push origin feature/your-feature-name
   ```

2. **Create a Pull Request** on GitHub:
   - Use the PR template if available
   - Provide a clear description of your changes
   - Include any relevant issue numbers
   - Add screenshots for UI changes

3. **Wait for review**: Maintainers will review your PR

4. **Address feedback**: Make requested changes and push updates

5. **Merge**: Once approved, your PR will be merged

### PR Checklist

Before submitting a PR, ensure:

- [ ] Code follows Rust conventions
- [ ] All tests pass
- [ ] Documentation is updated
- [ ] No warnings from clippy
- [ ] Code is formatted with rustfmt
- [ ] Changes are tested manually
- [ ] Commit messages are clear and descriptive

## Areas for Contribution

### High Priority

- **Bug fixes**: Any issues labeled as "bug"
- **Performance improvements**: Optimize editor performance
- **Cross-platform compatibility**: Ensure features work on all platforms
- **Documentation**: Improve existing documentation

### Features

- **Syntax highlighting**: Add support for new languages
- **Themes**: Create new color schemes
- **Keybindings**: Add new shortcuts or improve existing ones
- **Configuration**: Add new configuration options
- **UI improvements**: Enhance the user interface

### Infrastructure

- **Testing**: Add more tests
- **CI/CD**: Improve build and deployment processes
- **Documentation**: Write tutorials and guides
- **Examples**: Create example configurations

### Getting Started Issues

Look for issues labeled:
- `good first issue`: Suitable for newcomers
- `help wanted`: General help needed
- `documentation`: Documentation improvements

## Questions and Help

### Getting Help

- **GitHub Issues**: For bug reports and feature requests
- **GitHub Discussions**: For questions and general discussion
- **Discord/Slack**: If available, join our community channels

### Resources

- **Rust Book**: [https://doc.rust-lang.org/book/](https://doc.rust-lang.org/book/)
- **Rust Reference**: [https://doc.rust-lang.org/reference/](https://doc.rust-lang.org/reference/)
- **Crossterm Documentation**: [https://docs.rs/crossterm/](https://docs.rs/crossterm/)

### Communication Guidelines

- **Be respectful**: Treat all contributors with respect
- **Be patient**: Maintainers are volunteers
- **Be constructive**: Provide helpful, actionable feedback
- **Ask questions**: Don't hesitate to ask for clarification

## Recognition

Contributors will be recognized in:
- **README.md**: For significant contributions
- **Release notes**: For each release
- **GitHub contributors**: Automatic recognition on GitHub

---

Thank you for contributing to Ninja! Your contributions help make this editor better for everyone. 🥷

*"The ninja who masters the art of contribution becomes invisible to failure."* 