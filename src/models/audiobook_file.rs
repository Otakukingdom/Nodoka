use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A single audio file within an [`crate::models::Audiobook`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudiobookFile {
    /// Parent audiobook database ID.
    pub audiobook_id: i64,
    /// Display name (typically the filename).
    pub name: String,
    /// Full path to the audio file.
    pub full_path: String,
    /// File duration in milliseconds.
    pub length_of_file: Option<i64>,
    /// Last saved seek position in milliseconds.
    pub seek_position: Option<i64>,
    /// Optional content checksum (when available).
    pub checksum: Option<String>,
    /// File ordering position within the audiobook.
    pub position: i32,
    /// Completion percentage in the range 0-100.
    pub completeness: i32,
    /// Whether the file currently exists on disk.
    pub file_exists: bool,
    /// When this audiobook-file entry was created.
    pub created_at: DateTime<Utc>,
}

impl AudiobookFile {
    /// Returns whether the file is considered complete.
    #[must_use]
    pub const fn is_complete(&self) -> bool {
        self.completeness >= 100
    }

    /// Creates a new audiobook-file model with zero completeness.
    #[must_use]
    pub fn new(audiobook_id: i64, name: String, full_path: String, position: i32) -> Self {
        Self {
            audiobook_id,
            name,
            full_path,
            length_of_file: None,
            seek_position: None,
            checksum: None,
            position,
            completeness: 0,
            file_exists: true,
            created_at: Utc::now(),
        }
    }

    /// Calculates completeness percentage based on seek position and file length.
    ///
    /// Returns a value between 0 and 100.
    #[must_use]
    pub fn calculate_completeness(&self) -> i32 {
        if let (Some(length), Some(seek)) = (self.length_of_file, self.seek_position) {
            if length > 0 {
                // Calculate percentage, clamped to 0-100 range
                let percentage = (seek * 100) / length;
                // Clamp to 0-100 range to ensure it fits in i32
                let clamped = percentage.clamp(0, 100);
                // Safe conversion: clamped is guaranteed to be in 0-100 range
                i32::try_from(clamped).unwrap_or(0)
            } else {
                0
            }
        } else {
            0
        }
    }
}
