//! # Clipboard Management Module
//! 
//! This module provides multi-item clipboard functionality for the Ninja editor.
//! It implements a clipboard stack that can hold multiple copied items and
//! integrates with the system clipboard for external compatibility.
//! 
//! ## Features
//! 
//! - **Multi-item Clipboard**: Store multiple copied items in a stack
//! - **System Integration**: Syncs with the system clipboard
//! - **Thread Safety**: Safe for use across multiple threads
//! - **Persistent Storage**: Maintains clipboard history during editor session
//! - **Flexible Access**: Peek, paste, and remove items from any position
//! 
//! ## Usage
//! 
//! ```rust
//! use ninja::modules::clipboard::CLIPBOARD;
//! 
//! // Copy text to clipboard
//! CLIPBOARD.lock().unwrap().add("Hello, World!".to_string());
//! 
//! // Paste the most recent item
//! if let Some(text) = CLIPBOARD.lock().unwrap().get_top() {
//!     println!("Pasted: {}", text);
//! }
//! 
//! // Check clipboard size
//! let size = CLIPBOARD.lock().unwrap().size();
//! ```
//! 
//! ## Clipboard Stack Behavior
//! 
//! The clipboard operates as a stack where:
//! - New items are pushed to the top
//! - The most recent item is always accessible via `get_top()`
//! - Items can be removed from any position
//! - The system clipboard is updated with the most recent item

use std::sync::Mutex;
use cli_clipboard::{ClipboardContext, ClipboardProvider};
use once_cell::sync::Lazy;

/// A multi-item clipboard that maintains a stack of copied text items.
/// 
/// This struct provides clipboard functionality with the following features:
/// - Stores multiple copied items in a stack
/// - Integrates with the system clipboard
/// - Provides methods for adding, removing, and accessing items
/// - Maintains clipboard history during the editor session
/// 
/// # Thread Safety
/// 
/// This struct is not thread-safe by itself. Use the global `CLIPBOARD`
/// instance which is wrapped in a `Mutex` for thread-safe access.
/// 
/// # Example
/// 
/// ```rust
/// use ninja::modules::clipboard::Clipboard;
/// 
/// let mut clipboard = Clipboard::new();
/// clipboard.add("First item".to_string());
/// clipboard.add("Second item".to_string());
/// 
/// assert_eq!(clipboard.size(), 2);
/// assert_eq!(clipboard.get_top(), Some(&"Second item".to_string()));
/// ```
pub struct Clipboard {
    /// Internal stack of clipboard items
    stack: Vec<String>,
    /// System clipboard context for external integration
    ctx: ClipboardContext
}

impl Clipboard {
    /// Creates a new clipboard instance.
    /// 
    /// This method initializes a new clipboard and attempts to load
    /// the current system clipboard contents as the first item.
    /// 
    /// # Returns
    /// 
    /// Returns a new `Clipboard` instance with the system clipboard
    /// contents loaded if available.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ninja::modules::clipboard::Clipboard;
    /// 
    /// let clipboard = Clipboard::new();
    /// // Clipboard may contain system clipboard contents
    /// ```
    pub fn new() -> Self {
        //Clipboard { stack: Vec::new(), ctx: ClipboardContext::new().unwrap() }
        let mut cb = Clipboard {
            stack: Vec::new(),
            ctx: ClipboardContext::new().unwrap()
        };
        if let Ok(contents) = cb.ctx.get_contents() {
            cb.stack.push(contents);
        }
        cb
    }
    
    /// Initialize the clipboard with the current clipboard contents if they exist.
    /// 
    /// This method attempts to load the current system clipboard contents
    /// and adds them to the clipboard stack if successful.
    /// 
    /// # Returns
    /// 
    /// Returns `self` for method chaining.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ninja::modules::clipboard::Clipboard;
    /// 
    /// let clipboard = Clipboard::new().init();
    /// ```
    pub fn init(mut self) -> Self {
        if let Ok(stack) = self.ctx.get_contents() { 
            self.stack.push(stack);
        }
        self
    }

    /// Adds text to the clipboard stack and updates the system clipboard.
    /// 
    /// This method adds the provided text to the top of the clipboard stack
    /// and also updates the system clipboard with the new content. Empty
    /// strings are ignored.
    /// 
    /// # Arguments
    /// 
    /// * `text` - The text to add to the clipboard
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ninja::modules::clipboard::Clipboard;
    /// 
    /// let mut clipboard = Clipboard::new();
    /// clipboard.add("Hello, World!".to_string());
    /// assert_eq!(clipboard.size(), 1);
    /// ```
    pub fn add(&mut self, text: String) {
        if !text.is_empty() {
            self.stack.push(text.clone());
            self.ctx.set_contents(text.clone()).unwrap();
        }
    }

    /// Removes and returns the most recent item from the clipboard stack.
    /// 
    /// This method removes the top item from the stack and returns it.
    /// The system clipboard is not updated by this operation.
    /// 
    /// # Returns
    /// 
    /// Returns `Some(text)` if the clipboard is not empty, or `None` if empty.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ninja::modules::clipboard::Clipboard;
    /// 
    /// let mut clipboard = Clipboard::new();
    /// clipboard.add("Test".to_string());
    /// 
    /// if let Some(text) = clipboard.paste() {
    ///     println!("Pasted: {}", text);
    /// }
    /// assert!(clipboard.is_empty());
    /// ```
    pub fn paste(&mut self) -> Option<String> {
        if let Some(text) = self.stack.pop() {
            Some(text)
        } else {
            None
        }
    }

    /// Returns the most recent item without removing it from the stack.
    /// 
    /// This method allows you to peek at the top item without modifying
    /// the clipboard stack.
    /// 
    /// # Returns
    /// 
    /// Returns `Some(text)` if the clipboard is not empty, or `None` if empty.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ninja::modules::clipboard::Clipboard;
    /// 
    /// let mut clipboard = Clipboard::new();
    /// clipboard.add("Test".to_string());
    /// 
    /// if let Some(text) = clipboard.paste_peek() {
    ///     println!("Top item: {}", text);
    /// }
    /// assert_eq!(clipboard.size(), 1); // Item still in stack
    /// ```
    pub fn paste_peek(&self) -> Option<String> {
        if let Some(text) = self.stack.last() {
            Some(text.clone())
        } else {
            None
        }
    }

    /// Returns a reference to the most recent item without removing it.
    /// 
    /// This method provides a reference to the top item for efficient
    /// access without cloning the string.
    /// 
    /// # Returns
    /// 
    /// Returns `Some(&String)` if the clipboard is not empty, or `None` if empty.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ninja::modules::clipboard::Clipboard;
    /// 
    /// let mut clipboard = Clipboard::new();
    /// clipboard.add("Test".to_string());
    /// 
    /// if let Some(text) = clipboard.peek() {
    ///     println!("Top item: {}", text);
    /// }
    /// ```
    pub fn peek(&self) -> Option<&String> {
        self.stack.last()
    }

    /// Returns the number of items in the clipboard stack.
    /// 
    /// # Returns
    /// 
    /// Returns the number of items currently stored in the clipboard.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ninja::modules::clipboard::Clipboard;
    /// 
    /// let mut clipboard = Clipboard::new();
    /// assert_eq!(clipboard.size(), 0);
    /// 
    /// clipboard.add("Item 1".to_string());
    /// clipboard.add("Item 2".to_string());
    /// assert_eq!(clipboard.size(), 2);
    /// ```
    pub fn size(&self) -> usize {
        self.stack.len()
    }
    
    /// Removes all items from the clipboard stack.
    /// 
    /// This method clears the entire clipboard stack but does not
    /// affect the system clipboard.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ninja::modules::clipboard::Clipboard;
    /// 
    /// let mut clipboard = Clipboard::new();
    /// clipboard.add("Item 1".to_string());
    /// clipboard.add("Item 2".to_string());
    /// 
    /// clipboard.clear();
    /// assert!(clipboard.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.stack.clear();
    }
    
    /// Returns a reference to the item at the specified index.
    /// 
    /// # Arguments
    /// 
    /// * `index` - The index of the item to retrieve (0-based)
    /// 
    /// # Returns
    /// 
    /// Returns `Some(&String)` if the index is valid, or `None` if out of bounds.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ninja::modules::clipboard::Clipboard;
    /// 
    /// let mut clipboard = Clipboard::new();
    /// clipboard.add("First".to_string());
    /// clipboard.add("Second".to_string());
    /// 
    /// assert_eq!(clipboard.get(0), Some(&"First".to_string()));
    /// assert_eq!(clipboard.get(1), Some(&"Second".to_string()));
    /// assert_eq!(clipboard.get(2), None);
    /// ```
    pub fn get(&self, index: usize) -> Option<&String> {
        self.stack.get(index)
    }
    
    /// Returns a reference to the most recent item (top of stack).
    /// 
    /// This is a convenience method equivalent to `self.stack.last()`.
    /// 
    /// # Returns
    /// 
    /// Returns `Some(&String)` if the clipboard is not empty, or `None` if empty.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ninja::modules::clipboard::Clipboard;
    /// 
    /// let mut clipboard = Clipboard::new();
    /// clipboard.add("Latest item".to_string());
    /// 
    /// if let Some(text) = clipboard.get_top() {
    ///     println!("Most recent: {}", text);
    /// }
    /// ```
    pub fn get_top(&self) -> Option<&String> {
        self.stack.last()
    }
    
    /// Removes and returns the item at the specified index.
    /// 
    /// # Arguments
    /// 
    /// * `index` - The index of the item to remove (0-based)
    /// 
    /// # Returns
    /// 
    /// Returns `Some(String)` if the index is valid, or `None` if out of bounds.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ninja::modules::clipboard::Clipboard;
    /// 
    /// let mut clipboard = Clipboard::new();
    /// clipboard.add("First".to_string());
    /// clipboard.add("Second".to_string());
    /// 
    /// if let Some(text) = clipboard.remove(0) {
    ///     println!("Removed: {}", text);
    /// }
    /// assert_eq!(clipboard.size(), 1);
    /// ```
    pub fn remove(&mut self, index: usize) -> Option<String> {
        if index < self.stack.len() {
            Some(self.stack.remove(index))
        } else {
            None
        }
    }
    
    /// Returns a copy of all items in the clipboard stack.
    /// 
    /// # Returns
    /// 
    /// Returns a `Vec<String>` containing all clipboard items.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ninja::modules::clipboard::Clipboard;
    /// 
    /// let mut clipboard = Clipboard::new();
    /// clipboard.add("Item 1".to_string());
    /// clipboard.add("Item 2".to_string());
    /// 
    /// let contents = clipboard.get_contents();
    /// assert_eq!(contents.len(), 2);
    /// ```
    pub fn get_contents(&self) -> Vec<String> {
        self.stack.clone()
    }
    
    /// Checks if the clipboard stack is empty.
    /// 
    /// # Returns
    /// 
    /// Returns `true` if the clipboard contains no items, `false` otherwise.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ninja::modules::clipboard::Clipboard;
    /// 
    /// let mut clipboard = Clipboard::new();
    /// assert!(clipboard.is_empty());
    /// 
    /// clipboard.add("Item".to_string());
    /// assert!(!clipboard.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
}

/// Global thread-safe clipboard instance.
/// 
/// This is the main clipboard instance used throughout the editor.
/// It's wrapped in a `Mutex` to provide thread-safe access and is
/// initialized lazily when first accessed.
/// 
/// # Usage
/// 
/// ```rust
/// use ninja::modules::clipboard::CLIPBOARD;
/// 
/// // Add text to clipboard
/// CLIPBOARD.lock().unwrap().add("Hello".to_string());
/// 
/// // Get the most recent item
/// if let Some(text) = CLIPBOARD.lock().unwrap().get_top() {
///     println!("Clipboard: {}", text);
/// }
/// ```
pub static CLIPBOARD: Lazy<Mutex<Clipboard>> = Lazy::new(|| Mutex::new(Clipboard::new().init()));