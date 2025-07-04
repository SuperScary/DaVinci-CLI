//! # Core Editor Modules
//! 
//! This module contains the core functionality components of the Ninja editor.
//! Each submodule provides a specific aspect of the editor's functionality.
//! 
//! ## Module Overview
//! 
//! - **`cursor`**: Cursor positioning and movement logic
//! - **`clipboard`**: Multi-item clipboard management
//! - **`highlighting`**: Syntax highlighting for various programming languages
//! - **`message_bar`**: User message display and management
//! - **`search`**: Text search functionality and state management
//! - **`status`**: Status message handling and display
//! - **`statusbar`**: Status bar rendering and information display
//! 
//! ## Architecture
//! 
//! These modules are designed to be loosely coupled, with each module
//! handling a specific concern. They communicate through well-defined
//! interfaces and can be used independently or in combination.

pub mod statusbar;
pub mod message_bar;
pub mod search;
pub mod highlighting;
pub mod clipboard;
pub mod status;
pub mod cursor; 