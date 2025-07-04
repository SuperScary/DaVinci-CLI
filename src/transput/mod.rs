//! # Terminal Input/Output Module
//! 
//! This module handles all terminal input and output operations for the Ninja editor.
//! It provides the core text processing, rendering, and user interaction functionality.
//! 
//! ## Components
//! 
//! - **`Reader`**: Handles keyboard input and event processing
//! - **`Output`**: Manages screen rendering, text editing, and editor state
//! 
//! ## Features
//! 
//! - **Text Editing**: Insert, delete, and modify text with undo/redo support
//! - **Selection**: Visual text selection with copy/cut/paste operations
//! - **Search**: Find text with navigation and highlighting
//! - **Syntax Highlighting**: Real-time syntax highlighting for multiple languages
//! - **Screen Management**: Efficient terminal rendering and cursor positioning
//! - **Input Processing**: Keyboard event handling and command processing
//! 
//! ## Architecture
//! 
//! The transput module is the heart of the editor, coordinating between:
//! - User input from the terminal
//! - Text content management
//! - Screen rendering and display
//! - Editor state and configuration

pub mod transput;