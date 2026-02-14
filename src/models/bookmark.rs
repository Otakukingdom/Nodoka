//! Bookmark model for marking specific positions in audiobooks
//!
//! Bookmarks allow users to save specific positions in their audiobooks
//! for quick navigation to important points or reminders.

use chrono::{DateTime, Utc};

/// Represents a bookmark at a specific position in an audiobook
#[derive(Debug, Clone)]
pub struct Bookmark {
    /// Database ID (None if not yet persisted)
    pub id: Option<i64>,
    /// ID of the audiobook this bookmark belongs to
    pub audiobook_id: i64,
    /// Full path to the audio file
    pub file_path: String,
    /// Position in milliseconds
    pub position_ms: i64,
    /// User-provided label for the bookmark
    pub label: String,
    /// Optional note/description
    pub note: Option<String>,
    /// When the bookmark was created
    pub created_at: DateTime<Utc>,
}

impl Bookmark {
    /// Creates a new bookmark
    ///
    /// # Arguments
    ///
    /// * `audiobook_id` - ID of the audiobook
    /// * `file_path` - Full path to the audio file
    /// * `position_ms` - Position in milliseconds
    /// * `label` - User-friendly label
    #[must_use]
    pub fn new(audiobook_id: i64, file_path: String, position_ms: i64, label: String) -> Self {
        Self {
            id: None,
            audiobook_id,
            file_path,
            position_ms,
            label,
            note: None,
            created_at: Utc::now(),
        }
    }

    /// Creates a bookmark with an optional note
    #[must_use]
    pub fn with_note(mut self, note: Option<String>) -> Self {
        self.note = note;
        self
    }
}
