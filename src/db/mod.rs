//! Database layer for audiobook metadata and progress tracking.
//!
//! This module provides SQLite-based persistence for user settings, tracked
//! audiobook directories, audiobook metadata, and playback progress.
//!
//! ## Database Schema
//!
//! The database consists of four tables:
//!
//! - **metadata**: Key-value settings storage (user preferences)
//! - **directories**: Tracked audiobook directories with scan timestamps
//! - **audiobooks**: Audiobook metadata (title, path, completion percentage)
//! - **`audiobook_file`**: Individual audio file tracking (position, duration, checksum)
//!
//! ## Thread Safety
//!
//! The [`Database`] connection is wrapped in `Arc<Mutex<Connection>>` to allow
//! safe concurrent access from multiple threads in the iced async runtime.
//!
//! ## Usage
//!
//! ```no_run
//! # use nodoka::Database;
//! # use nodoka::error::Result;
//! # fn example() -> Result<()> {
//! // Open database (creates if doesn't exist)
//! let db = Database::open()?;
//!
//! // Initialize schema on first run
//! nodoka::db::initialize(db.connection())?;
//!
//! // Use query functions
//! let directories = nodoka::db::queries::get_all_directories(db.connection())?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Data Location
//!
//! Database file is stored in platform-specific application data directory:
//! - **Windows**: `%APPDATA%\Otakukingdom\Nodoka\nodoka.db`
//! - **macOS**: `~/Library/Application Support/com.Otakukingdom.Nodoka/nodoka.db`
//! - **Linux**: `~/.local/share/com/Otakukingdom/Nodoka/nodoka.db`

mod connection;
pub mod queries;
mod schema;

pub use connection::Database;
pub use schema::initialize;
