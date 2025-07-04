//! # Screen Management Module
//! 
//! This module handles different screens and UI states in the Ninja editor.
//! It provides a screen management system that allows switching between
//! different editor modes and views.
//! 
//! ## Screen Types
//! 
//! - **`editor`**: The main text editing screen with full functionality
//! - **`clipboard`**: Clipboard management and history view
//! - **`debug`**: Debug information and diagnostics display
//! - **`screens`**: Screen manager that coordinates between different screens
//! 
//! ## Architecture
//! 
//! The screen system uses a state machine pattern where:
//! - Each screen implements a common interface
//! - The screen manager handles transitions between screens
//! - Screens can be paused and resumed without losing state
//! - The active screen controls all input and output

pub mod screens;
pub mod clipboard;
pub mod debug;
pub mod editor;