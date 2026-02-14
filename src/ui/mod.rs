//! User interface layer following the Elm architecture.
//!
//! This module implements the UI using the iced framework with the Elm pattern:
//! - **Model**: [`State`] contains all UI state
//! - **Update**: [`update`] module handles messages and updates state
//! - **View**: [`main_window::view()`] renders the UI
//!
//! ## Architecture
//!
//! ```text
//! User Interaction
//!       ↓
//!   [Message]
//!       ↓
//!   [Update] → Modify State → [Command]
//!       ↓                          ↓
//!   [State]                   Async Tasks
//!       ↓                          ↓
//!    [View] ← ← ← ← ← ← ← ← ← [Message]
//! ```
//!
//! ## Message Flow
//!
//! 1. User clicks button → generates [`Message`]
//! 2. [`update`] module receives message and current [`State`]
//! 3. Update function modifies state and returns [`iced::Command`]
//! 4. Command may spawn async tasks that generate new messages
//! 5. [`main_window::view()`] renders updated state
//!
//! ## Components
//!
//! UI is organized into reusable components:
//! - [`components::player_controls`]: Playback controls (play/pause, volume, speed)
//! - [`components::audiobook_list`]: List of discovered audiobooks
//! - [`components::file_list`]: Files within selected audiobook
//! - [`settings_form`]: Settings dialog for managing directories
//!
//! ## Usage
//!
//! The UI is typically not used directly but through [`crate::app::App`]
//! which implements the iced [`Application`](iced::Application) trait.

pub mod components;
pub mod main_window;
mod message;
pub mod settings_form;
pub mod shortcuts;
mod state;
mod styles;
pub mod update;

pub use message::Message;
pub use state::State;
pub use styles::*;
