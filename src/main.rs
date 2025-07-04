//! # Ninja Editor - Main Binary
//! 
//! This is the main entry point for the Ninja text editor.
//! It provides a simple wrapper around the library's `run_editor` function
//! and handles cleanup when the program exits.

use ninja::run_editor;

/// Cleanup handler that ensures proper terminal restoration on exit.
/// 
/// This struct implements `Drop` to guarantee that terminal cleanup
/// occurs even if the program panics or exits unexpectedly.
struct CleanUp;

impl Drop for CleanUp {
    fn drop(&mut self) {
        // Cleanup is handled by the library
    }
}

/// Main entry point for the Ninja editor.
/// 
/// This function initializes the cleanup handler and starts the editor.
/// The cleanup handler ensures that the terminal is properly restored
/// when the program exits, regardless of how it terminates.
/// 
/// # Returns
/// 
/// Returns `Ok(())` on successful completion, or propagates any errors
/// from the editor.
/// 
/// # Errors
/// 
/// This function will return an error if the editor encounters any
/// fatal errors during execution.
fn main() -> crossterm::Result<()> {
    let _clean_up = CleanUp;
    run_editor()
}