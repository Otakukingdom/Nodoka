//! Domain models for audiobooks, files, and directories.
//!
//! This module defines the core data structures used throughout Nodoka
//! to represent audiobooks, their constituent files, and tracked directories.
//!
//! ## Models
//!
//! - [`Audiobook`]: Represents a complete audiobook (collection of audio files)
//! - [`AudiobookFile`]: Individual audio file within an audiobook
//! - [`Directory`]: Tracked directory path with scan metadata
//! - [`MediaProperty`]: Media file metadata (duration, codec)
//!
//! ## Data Flow
//!
//! 1. User adds a [`Directory`] to track
//! 2. Scanning discovers audio files and creates [`Audiobook`] instances
//! 3. Each audiobook contains multiple [`AudiobookFile`] instances
//! 4. [`MediaProperty`] stores VLC-extracted metadata for each file
//! 5. Progress tracking updates `seek_position` and `completeness` fields
//!
//! ## Database Mapping
//!
//! These models map directly to database tables:
//! - `Audiobook` → `audiobooks` table
//! - `AudiobookFile` → `audiobook_file` table
//! - `Directory` → `directories` table
//!
//! See [`crate::db`] for persistence layer.

mod audiobook;
mod audiobook_file;
mod bookmark;
mod directory;
mod media_property;
mod sleep_timer;

pub use audiobook::Audiobook;
pub use audiobook_file::AudiobookFile;
pub use bookmark::Bookmark;
pub use directory::Directory;
pub use media_property::MediaProperty;
pub use sleep_timer::{SleepTimer, SleepTimerMode};
