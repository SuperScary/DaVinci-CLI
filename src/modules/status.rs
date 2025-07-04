//! # Status Message Module
//! 
//! This module provides status message functionality for the Ninja editor.
//! It manages temporary status messages that are displayed to the user
//! and automatically expire after a specified duration.
//! 
//! ## Features
//! 
//! - **Temporary Messages**: Status messages that automatically disappear
//! - **Timed Expiration**: Messages expire after 5 seconds by default
//! - **Automatic Cleanup**: Expired messages are automatically cleared
//! - **Thread-safe**: Safe for use across multiple threads
//! 
//! ## Usage
//! 
//! ```rust
//! use ninja::modules::status::StatusMessage;
//! 
//! let mut status = StatusMessage::new("Welcome to Ninja!".to_string());
//! 
//! // Set a new message
//! status.set_message("File saved successfully".to_string());
//! 
//! // Check if message is still active
//! if let Some(msg) = status.message() {
//!     println!("Status: {}", msg);
//! }
//! ```

use std::time::{Duration, Instant};

/// Manages temporary status messages displayed to the user.
/// 
/// This struct provides functionality for displaying status messages
/// that automatically expire after a specified duration. It's commonly
/// used for showing operation results, warnings, or informational
/// messages to the user.
/// 
/// # Behavior
/// 
/// - **Message Display**: Messages are displayed immediately when set
/// - **Automatic Expiration**: Messages expire after 5 seconds
/// - **Cleanup**: Expired messages are automatically removed
/// - **Overwrite**: New messages replace existing ones
/// 
/// # Example
/// 
/// ```rust
/// use ninja::modules::status::StatusMessage;
/// 
/// let mut status = StatusMessage::new("Initial message".to_string());
/// 
/// // Set a new status message
/// status.set_message("Operation completed".to_string());
/// 
/// // Check if message is still active
/// if let Some(message) = status.message() {
///     println!("Current status: {}", message);
/// }
/// ```
pub struct StatusMessage {
    /// The current status message text
    message: Option<String>,
    /// When the message was set (for expiration tracking)
    set_time: Option<Instant>,
}

impl StatusMessage {
    /// Creates a new status message with an initial message.
    /// 
    /// The initial message is set with the current timestamp and will
    /// expire after the default duration (5 seconds).
    /// 
    /// # Arguments
    /// 
    /// * `initial_message` - The initial status message to display
    /// 
    /// # Returns
    /// 
    /// Returns a new `StatusMessage` instance with the specified message.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ninja::modules::status::StatusMessage;
    /// 
    /// let status = StatusMessage::new("Welcome to Ninja!".to_string());
    /// ```
    pub fn new(initial_message: String) -> Self {
        Self {
            message: Some(initial_message),
            set_time: Some(Instant::now()),
        }
    }

    /// Sets a new status message.
    /// 
    /// This method replaces the current message with a new one and
    /// resets the expiration timer. The new message will expire after
    /// 5 seconds from when it was set.
    /// 
    /// # Arguments
    /// 
    /// * `message` - The new status message to display
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ninja::modules::status::StatusMessage;
    /// 
    /// let mut status = StatusMessage::new("Initial".to_string());
    /// status.set_message("File saved successfully".to_string());
    /// ```
    pub fn set_message(&mut self, message: String) {
        self.message = Some(message);
        self.set_time = Some(Instant::now())
    }

    /// Retrieves the current status message if it hasn't expired.
    /// 
    /// This method checks if the current message has expired (after 5 seconds)
    /// and returns it if it's still valid. If the message has expired,
    /// it's automatically cleared and `None` is returned.
    /// 
    /// # Returns
    /// 
    /// Returns `Some(&String)` if the message is still active and hasn't expired,
    /// or `None` if the message has expired or doesn't exist.
    /// 
    /// # Expiration Behavior
    /// 
    /// - Messages expire after 5 seconds
    /// - Expired messages are automatically cleared
    /// - Subsequent calls return `None` for expired messages
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ninja::modules::status::StatusMessage;
    /// 
    /// let mut status = StatusMessage::new("Temporary message".to_string());
    /// 
    /// // Message is still active
    /// if let Some(msg) = status.message() {
    ///     println!("Status: {}", msg);
    /// }
    /// 
    /// // After 5 seconds, message will be None
    /// // (in real usage, you'd wait for the time to pass)
    /// ```
    pub fn message(&mut self) -> Option<&String> {
        self.set_time.and_then(|time| {
            if time.elapsed() > Duration::from_secs(5) {
                self.message = None;
                self.set_time = None;
                None
            } else {
                Some(self.message.as_ref().unwrap())
            }
        })
    }
}