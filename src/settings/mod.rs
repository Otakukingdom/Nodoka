//! User settings and preferences.
//!
//! This module handles loading and saving user preferences to the database.
//! Settings are stored as key-value pairs in the `metadata` table.
//!
//! ## Available Settings
//!
//! Currently supported preferences:
//! - Selected file path (resume playback)
//! - Last played audiobook
//! - Playback volume
//! - Playback speed
//!
//! ## Usage
//!
//! ```no_run
//! # use nodoka::{Database, settings::Settings};
//! # use nodoka::error::Result;
//! # fn example() -> Result<()> {
//! let db = Database::open()?;
//! let settings = Settings::new(db.connection());
//!
//! // Save settings
//! settings.set_volume(80)?;
//! settings.set_speed(1.25)?;
//!
//! // Load settings
//! let volume = settings.get_volume()?;
//! let speed = settings.get_speed()?;
//! println!("Volume: {volume}, Speed: {speed}");
//! # Ok(())
//! # }
//! ```

mod storage;

pub use storage::Settings;
