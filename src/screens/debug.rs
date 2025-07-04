//! # Debug Screen Module
//! 
//! This module provides the debug screen functionality for the Ninja editor.
//! It displays diagnostic information and debugging data to help developers
//! understand the editor's internal state and troubleshoot issues.
//! 
//! ## Features
//! 
//! - **Debug Information**: Display internal editor state and diagnostics
//! - **Performance Metrics**: Show performance-related information
//! - **Error Reporting**: Display error messages and stack traces
//! - **State Inspection**: View current editor configuration and settings
//! - **Development Tools**: Assist in debugging and development
//! 
//! ## Usage
//! 
//! ```rust
//! use ninja::screens::debug::DebugScreen;
//! 
//! let debug_info = "Editor state: Normal\nCursor position: (10, 5)\nFile: example.rs".to_string();
//! ```

/// Represents the debug screen for displaying diagnostic information.
/// 
/// This struct provides a dedicated screen for showing debug information
/// and diagnostic data about the editor's internal state. It's primarily
/// used during development and troubleshooting.
/// 
/// # Features
/// 
/// - **State Display**: Shows current editor state and configuration
/// - **Performance Data**: Displays performance metrics and timing information
/// - **Error Information**: Shows error messages and diagnostic details
/// - **Memory Usage**: Displays memory allocation and usage statistics
/// - **Thread Information**: Shows thread state and activity
/// 
/// # Debug Information
/// 
/// The debug screen can display various types of information:
/// - **Editor State**: Current mode, file information, cursor position
/// - **Configuration**: Loaded settings and configuration values
/// - **Performance**: Timing data, memory usage, operation counts
/// - **Errors**: Error messages, stack traces, and diagnostic codes
/// - **System Info**: Platform information, terminal capabilities
/// 
/// # Development Use
/// 
/// This screen is particularly useful for:
/// - **Development**: Understanding editor behavior during development
/// - **Troubleshooting**: Diagnosing issues and performance problems
/// - **Testing**: Verifying editor state during testing
/// - **Profiling**: Analyzing performance characteristics
/// 
/// # Example
/// 
/// ```rust
/// use ninja::screens::debug::DebugScreen;
/// 
/// let debug_info = format!(
///     "Editor State: {}\nCursor: ({}, {})\nFile: {}\nMemory: {}KB",
///     "Normal", 10, 5, "example.rs", 1024
/// );
/// 
/// // The debug screen can be displayed to show
/// // current editor state and diagnostic information
/// ```
pub struct DebugScreen {
    /// Debug information string containing diagnostic data
    pub debug_info: String,
}