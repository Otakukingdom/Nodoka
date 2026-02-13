//! Nodoka Audiobook Reader Library
//!
//! A cross-platform audiobook player with automatic progress tracking.
//! Built with iced UI framework and VLC for media playback.
//!
//! # Features
//!
//! - Cross-platform support (Windows, macOS, Linux)
//! - VLC-powered audio playback
//! - SQLite-based progress tracking
//! - Automatic directory scanning
//! - Speed and volume controls
//! - Resume playback across sessions
//!
//! # Example
//!
//! ```no_run
//! use nodoka::Database;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let db = Database::open()?;
//! nodoka::db::initialize_schema(db.connection())?;
//! nodoka::app::run(db)?;
//! # Ok(())
//! # }
//! ```

pub mod app;
pub mod db;
pub mod error;
pub mod models;
pub mod player;
pub mod proxy;
pub mod settings;
pub mod tasks;
pub mod ui;

pub use app::NodokaApp;
pub use db::Database;
pub use error::{NodokaError, Result};
