//! # Search Functionality Module
//! 
//! This module provides search and find functionality for the Ninja editor.
//! It manages search state, navigation between matches, and highlighting
//! of search results.
//! 
//! ## Features
//! 
//! - **Search State Management**: Tracks current search position and direction
//! - **Bidirectional Search**: Search forward and backward through the document
//! - **Match Highlighting**: Visual highlighting of search matches
//! - **Navigation**: Move between search matches with arrow keys
//! - **State Persistence**: Maintains search state during navigation
//! 
//! ## Components
//! 
//! - **`SearchDirection`**: Enumeration of search directions
//! - **`SearchIndex`**: State management for search operations
//! 
//! ## Usage
//! 
//! ```rust
//! use ninja::modules::search::SearchIndex;
//! 
//! let mut search_index = SearchIndex::new();
//! search_index.reset(); // Clear search state
//! ```

use crate::modules::highlighting::HighlightType;

/// Represents the direction of search operations.
/// 
/// This enum is used to specify whether a search should proceed
/// forward or backward through the document.
/// 
/// # Variants
/// 
/// - **`Forward`**: Search from current position toward the end of the document
/// - **`Backward`**: Search from current position toward the beginning of the document
/// 
/// # Example
/// 
/// ```rust
/// use ninja::modules::search::SearchDirection;
/// 
/// let forward_search = SearchDirection::Forward;
/// let backward_search = SearchDirection::Backward;
/// ```
pub enum SearchDirection {
    /// Search from current position toward the end of the document
    Forward,
    /// Search from current position toward the beginning of the document
    Backward,
}

/// Manages the state of search operations in the editor.
/// 
/// This struct tracks the current search position, direction, and
/// highlighting state to provide seamless search and navigation
/// functionality.
/// 
/// # Fields
/// 
/// - **`x_index`**: Current horizontal position in the search
/// - **`y_index`**: Current vertical position (row) in the search
/// - **`x_direction`**: Horizontal search direction (if specified)
/// - **`y_direction`**: Vertical search direction (if specified)
/// - **`previous_highlight`**: Previous highlighting state to restore
/// 
/// # Search Behavior
/// 
/// The search index maintains state to support:
/// - **Incremental Search**: Continue searching from the last match
/// - **Directional Navigation**: Move forward/backward through matches
/// - **Highlight Preservation**: Restore previous highlighting when moving
/// - **Position Tracking**: Remember exact position within matches
/// 
/// # Example
/// 
/// ```rust
/// use ninja::modules::search::SearchIndex;
/// 
/// let mut search_index = SearchIndex::new();
/// 
/// // Set search position
/// search_index.x_index = 10;
/// search_index.y_index = 5;
/// 
/// // Clear search state
/// search_index.reset();
/// ```
pub struct SearchIndex {
    /// Current horizontal position in the search (character index)
    pub x_index: usize,
    /// Current vertical position in the search (row index)
    pub y_index: usize,
    /// Horizontal search direction (if specified)
    pub x_direction: Option<SearchDirection>,
    /// Vertical search direction (if specified)
    pub y_direction: Option<SearchDirection>,
    /// Previous highlighting state to restore when moving between matches
    pub previous_highlight: Option<(usize, Vec<HighlightType>)>,
}

impl SearchIndex {
    /// Creates a new search index with default state.
    /// 
    /// The search index starts at position (0, 0) with no direction
    /// specified and no previous highlighting to restore.
    /// 
    /// # Returns
    /// 
    /// Returns a new `SearchIndex` instance with default values.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ninja::modules::search::SearchIndex;
    /// 
    /// let search_index = SearchIndex::new();
    /// assert_eq!(search_index.x_index, 0);
    /// assert_eq!(search_index.y_index, 0);
    /// assert!(search_index.x_direction.is_none());
    /// assert!(search_index.y_direction.is_none());
    /// ```
    pub fn new() -> Self {
        Self {
            x_index: 0,
            y_index: 0,
            x_direction: None,
            y_direction: None,
            previous_highlight: None,
        }
    }

    /// Resets the search index to its initial state.
    /// 
    /// This method clears all search state, including position,
    /// direction, and previous highlighting information.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ninja::modules::search::SearchIndex;
    /// 
    /// let mut search_index = SearchIndex::new();
    /// search_index.x_index = 10;
    /// search_index.y_index = 5;
    /// 
    /// search_index.reset();
    /// assert_eq!(search_index.x_index, 0);
    /// assert_eq!(search_index.y_index, 0);
    /// ```
    pub fn reset(&mut self) {
        self.y_index = 0;
        self.x_index = 0;
        self.y_direction = None;
        self.x_direction = None;
        self.previous_highlight = None
    }
}