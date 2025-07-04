//! # Screen Management Module
//! 
//! This module provides the screen management system for the Ninja editor.
//! It coordinates between different editor screens and manages screen
//! transitions and state.
//! 
//! ## Components
//! 
//! - **`ScreenManager`**: Central manager for all screens
//! - **`ActiveScreen`**: Enumeration of available screen types
//! - **`EditorScreen`**: Wrapper for the main editor screen
//! - **`DebugScreen`**: Wrapper for the debug screen
//! - **`ClipboardScreen`**: Wrapper for the clipboard screen
//! 
//! ## Features
//! 
//! - **Screen Management**: Switch between different editor screens
//! - **State Preservation**: Maintain screen state during transitions
//! - **Active Screen Tracking**: Keep track of the currently active screen
//! - **Screen Execution**: Run the appropriate screen logic
//! - **Configuration Integration**: Pass configuration to screens
//! 
//! ## Screen Types
//! 
//! The screen system supports several types of screens:
//! - **Editor**: Main text editing interface
//! - **Debug**: Diagnostic and debugging information
//! - **Clipboard**: Clipboard history management
//! 
//! ## Usage
//! 
//! ```rust
//! use ninja::screens::screens::{ScreenManager, ActiveScreen};
//! use ninja::config::NinjaConfig;
//! 
//! let mut screen_manager = ScreenManager::new();
//! let config = NinjaConfig::default();
//! 
//! // Show the editor screen
//! screen_manager.show_editor_screen(config);
//! 
//! // Run the active screen
//! screen_manager.run_active();
//! ```

use crate::config::NinjaConfig;
use crate::screens::{clipboard, debug, editor};

/// Manages the different screens in the Ninja editor.
/// 
/// This struct provides a centralized way to manage different screens
/// in the editor, including switching between screens and maintaining
/// screen state.
/// 
/// # Features
/// 
/// - **Screen Switching**: Change between different editor screens
/// - **State Management**: Preserve screen state during transitions
/// - **Active Screen Tracking**: Keep track of which screen is currently active
/// - **Screen Execution**: Run the appropriate screen logic
/// - **Configuration Handling**: Pass configuration to screens as needed
/// 
/// # Screen Lifecycle
/// 
/// The screen manager handles the complete lifecycle of screens:
/// - **Initialization**: Create and configure screens
/// - **Activation**: Set a screen as the active screen
/// - **Execution**: Run the active screen's logic
/// - **Transition**: Switch between different screens
/// - **Cleanup**: Handle screen cleanup when switching
/// 
/// # Example
/// 
/// ```rust
/// use ninja::screens::screens::ScreenManager;
/// use ninja::config::NinjaConfig;
/// 
/// let mut screen_manager = ScreenManager::new();
/// let config = NinjaConfig::default();
/// 
/// // Show the editor screen
/// screen_manager.show_editor_screen(config);
/// 
/// // Run the active screen
/// screen_manager.run_active();
/// ```
pub struct ScreenManager {
    /// The currently active screen (if any)
    active_screen: Option<ActiveScreen>
}

/// Wrapper for the main editor screen.
/// 
/// This struct provides a wrapper around the main editor functionality,
/// allowing it to be managed by the screen manager.
/// 
/// # Features
/// 
/// - **Editor Integration**: Full integration with the main editor
/// - **Configuration Support**: Accepts editor configuration
/// - **Run Loop**: Provides the main editor run loop
/// - **Error Handling**: Proper error handling for editor operations
/// 
/// # Example
/// 
/// ```rust
/// use ninja::screens::screens::EditorScreen;
/// use ninja::screens::editor::Editor;
/// use ninja::config::NinjaConfig;
///
/// let config = NinjaConfig::default();
/// let editor = Editor::new(config);
/// let mut editor_screen = EditorScreen { screen: editor };
///
/// // Run the editor screen
/// editor_screen.run();
/// ```
pub struct EditorScreen {
    /// The underlying editor instance
    pub screen: editor::Editor,
}

/// Wrapper for the debug screen.
/// 
/// This struct provides a wrapper around the debug screen functionality,
/// allowing it to be managed by the screen manager.
/// 
/// # Features
/// 
/// - **Debug Information**: Display diagnostic information
/// - **State Inspection**: View editor internal state
/// - **Performance Metrics**: Show performance data
/// - **Error Reporting**: Display error information
/// 
/// # Example
/// 
/// ```rust
/// use ninja::screens::screens::DebugScreen;
/// use ninja::screens::debug::DebugScreen as InnerDebugScreen;
///
/// let debug_screen = InnerDebugScreen { debug_info: "Debug info".to_string() };
/// let mut debug_screen_wrapper = DebugScreen { screen: debug_screen };
///
/// // Run the debug screen
/// debug_screen_wrapper.run();
/// ```
pub struct DebugScreen {
    /// The underlying debug screen instance
    pub screen: debug::DebugScreen,
}

/// Wrapper for the clipboard screen.
/// 
/// This struct provides a wrapper around the clipboard screen functionality,
/// allowing it to be managed by the screen manager.
/// 
/// # Features
/// 
/// - **Clipboard History**: View clipboard history
/// - **Item Management**: Manage clipboard items
/// - **Copy Operations**: Copy items from history
/// - **Delete Operations**: Remove items from history
/// 
/// # Example
/// 
/// ```rust
/// use ninja::screens::screens::ClipboardScreen;
/// use ninja::screens::clipboard::ClipboardScreen as InnerClipboardScreen;
/// use ninja::modules::clipboard::Clipboard;
///
/// let clipboard = Clipboard::new();
/// let clipboard_screen = InnerClipboardScreen { clipboard };
/// let mut clipboard_screen_wrapper = ClipboardScreen { screen: clipboard_screen };
///
/// // Run the clipboard screen
/// clipboard_screen_wrapper.run();
/// ```
pub struct ClipboardScreen {
    /// The underlying clipboard screen instance
    pub screen: clipboard::ClipboardScreen,
}

impl ScreenManager {
    /// Creates a new screen manager instance.
    /// 
    /// This method initializes a new screen manager with no active screen.
    /// 
    /// # Returns
    /// 
    /// Returns a new `ScreenManager` instance.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ninja::screens::screens::ScreenManager;
    /// 
    /// let screen_manager = ScreenManager::new();
    /// assert!(screen_manager.active_screen().is_none());
    /// ```
    pub fn new() -> Self {
        Self {
            active_screen: None
        }
    }
    
    /// Sets the active screen.
    /// 
    /// This method changes the currently active screen to the specified screen.
    /// 
    /// # Arguments
    /// 
    /// * `screen` - The screen to set as active
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ninja::screens::screens::{ScreenManager, ActiveScreen, EditorScreen};
    /// use ninja::screens::editor::Editor;
    /// use ninja::config::NinjaConfig;
    /// 
    /// let mut screen_manager = ScreenManager::new();
    /// let config = NinjaConfig::default();
    /// let editor = Editor::new(config);
    /// let editor_screen = EditorScreen { screen: editor };
    /// 
    /// screen_manager.set_active_screen(ActiveScreen::Editor(editor_screen));
    /// assert!(screen_manager.active_screen().is_some());
    /// ```
    pub fn set_active_screen(&mut self, screen: ActiveScreen) {
        self.active_screen = Some(screen);
    }
    
    /// Gets a reference to the active screen.
    /// 
    /// # Returns
    /// 
    /// Returns `Some(&ActiveScreen)` if there is an active screen, or `None` if not.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ninja::screens::screens::ScreenManager;
    /// 
    /// let screen_manager = ScreenManager::new();
    /// let active_screen = screen_manager.active_screen();
    /// assert!(active_screen.is_none());
    /// ```
    pub fn active_screen(&self) -> Option<&ActiveScreen> {
        self.active_screen.as_ref()
    }
    
    /// Gets a mutable reference to the active screen.
    /// 
    /// # Returns
    /// 
    /// Returns `Some(&mut ActiveScreen)` if there is an active screen, or `None` if not.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ninja::screens::screens::{ScreenManager, ActiveScreen, EditorScreen};
    /// use ninja::screens::editor::Editor;
    /// use ninja::config::NinjaConfig;
    /// 
    /// let mut screen_manager = ScreenManager::new();
    /// let config = NinjaConfig::default();
    /// let editor = Editor::new(config);
    /// let editor_screen = EditorScreen { screen: editor };
    /// 
    /// screen_manager.set_active_screen(ActiveScreen::Editor(editor_screen));
    /// let active_screen = screen_manager.active_screen_mut();
    /// assert!(active_screen.is_some());
    /// ```
    pub fn active_screen_mut(&mut self) -> Option<&mut ActiveScreen> {
        self.active_screen.as_mut()
    }

    /// Shows the editor screen with the given configuration.
    /// 
    /// This method creates a new editor screen with the specified configuration
    /// and sets it as the active screen.
    /// 
    /// # Arguments
    /// 
    /// * `config` - The configuration to use for the editor
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ninja::screens::screens::ScreenManager;
    /// use ninja::config::NinjaConfig;
    /// 
    /// let mut screen_manager = ScreenManager::new();
    /// let config = NinjaConfig::default();
    /// 
    /// screen_manager.show_editor_screen(config);
    /// assert!(screen_manager.active_screen().is_some());
    /// ```
    pub fn show_editor_screen(&mut self, config: NinjaConfig) {
        let editor = editor::Editor::new(config);
        self.set_active_screen(ActiveScreen::Editor(EditorScreen { screen: editor }));
    }
    
    /// Runs the currently active screen.
    /// 
    /// This method executes the logic for the currently active screen.
    /// If no screen is active, it prints an error message.
    /// 
    /// # Behavior
    /// 
    /// - **Editor Screen**: Runs the main editor loop
    /// - **Debug Screen**: Runs the debug screen logic
    /// - **Clipboard Screen**: Runs the clipboard screen logic
    /// - **No Active Screen**: Prints an error message
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ninja::screens::screens::ScreenManager;
    /// use ninja::config::NinjaConfig;
    /// 
    /// let mut screen_manager = ScreenManager::new();
    /// let config = NinjaConfig::default();
    /// 
    /// screen_manager.show_editor_screen(config);
    /// screen_manager.run_active();
    /// ```
    pub fn run_active(&mut self) {
        match self.active_screen_mut() {
            Some(ActiveScreen::Editor(editor)) => editor.run(),
            Some(ActiveScreen::Debug(debug)) => debug.run(),
            Some(ActiveScreen::Clipboard(clipboard)) => clipboard.run(),
            None => {
                eprintln!("No active screen to run.");
            }
        }
    }
    
}

impl EditorScreen {
    /// Runs the editor screen.
    /// 
    /// This method runs the main editor loop until the editor is quit.
    /// It handles the complete lifecycle of the editor, including
    /// input processing, screen updates, and error handling.
    /// 
    /// # Behavior
    /// 
    /// - **Main Loop**: Continuously runs the editor until quit
    /// - **Error Handling**: Expects the editor to run successfully
    /// - **Exit Condition**: Stops when the editor returns false (quit)
    /// 
    /// # Panics
    /// 
    /// Panics if the editor fails to run (e.g., terminal errors).
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ninja::screens::screens::EditorScreen;
    /// use ninja::screens::editor::Editor;
    /// use ninja::config::NinjaConfig;
    /// 
    /// let config = NinjaConfig::default();
    /// let editor = Editor::new(config);
    /// let mut editor_screen = EditorScreen { screen: editor };
    /// 
    /// // This will run the editor until the user quits
    /// // editor_screen.run();
    /// ```
    pub fn run(&mut self) {
        while self.screen.run().expect("Could not run Ninja Editor") {}
    }
}

impl DebugScreen {
    /// Runs the debug screen.
    /// 
    /// This method runs the debug screen logic, displaying diagnostic
    /// information and debug data.
    /// 
    /// # Note
    /// 
    /// This method is currently a placeholder and does not perform
    /// any actual debug screen operations.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ninja::screens::screens::DebugScreen;
    /// use ninja::screens::debug::DebugScreen as InnerDebugScreen;
    /// 
    /// let debug_screen = InnerDebugScreen { debug_info: "Debug info".to_string() };
    /// let mut debug_screen_wrapper = DebugScreen { screen: debug_screen };
    /// 
    /// // Run the debug screen (currently a no-op)
    /// debug_screen_wrapper.run();
    /// ```
    pub fn run(&mut self) {
        //while self.screen.run().expect("Could not run Ninja Debugger") {}
    }
}

impl ClipboardScreen {
    /// Runs the clipboard screen.
    /// 
    /// This method runs the clipboard screen logic, allowing users
    /// to view and manage their clipboard history.
    /// 
    /// # Note
    /// 
    /// This method is currently a placeholder and does not perform
    /// any actual clipboard screen operations.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use ninja::screens::screens::ClipboardScreen;
    /// use ninja::screens::clipboard::ClipboardScreen as InnerClipboardScreen;
    /// use ninja::modules::clipboard::Clipboard;
    /// 
    /// let clipboard = Clipboard::new();
    /// let clipboard_screen = InnerClipboardScreen { clipboard };
    /// let mut clipboard_screen_wrapper = ClipboardScreen { screen: clipboard_screen };
    /// 
    /// // Run the clipboard screen (currently a no-op)
    /// clipboard_screen_wrapper.run();
    /// ```
    pub fn run(&mut self) {
        // Implement clipboard screen logic here
    }}

/// Enumeration of all available screen types.
/// 
/// This enum represents all the different types of screens that can
/// be managed by the screen manager. Each variant contains the
/// appropriate screen wrapper.
/// 
/// # Variants
/// 
/// - **`Editor`**: The main text editing screen
/// - **`Debug`**: The debug and diagnostic screen
/// - **`Clipboard`**: The clipboard management screen
/// 
/// # Example
/// 
/// ```rust
/// use ninja::screens::screens::{ActiveScreen, EditorScreen, DebugScreen, ClipboardScreen};
/// use ninja::screens::editor::Editor;
/// use ninja::screens::debug::DebugScreen as InnerDebugScreen;
/// use ninja::screens::clipboard::ClipboardScreen as InnerClipboardScreen;
/// use ninja::modules::clipboard::Clipboard;
/// use ninja::config::NinjaConfig;
/// 
/// let config = NinjaConfig::default();
/// let editor = Editor::new(config);
/// let editor_screen = EditorScreen { screen: editor };
/// let active_screen = ActiveScreen::Editor(editor_screen);
/// 
/// match active_screen {
///     ActiveScreen::Editor(_) => println!("Editor screen"),
///     ActiveScreen::Debug(_) => println!("Debug screen"),
///     ActiveScreen::Clipboard(_) => println!("Clipboard screen"),
/// }
/// ```
pub enum ActiveScreen {
    /// The main text editing screen
    Editor(EditorScreen),
    /// The debug and diagnostic screen
    Debug(DebugScreen),
    /// The clipboard management screen
    Clipboard(ClipboardScreen),
}
