use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudiobookFile {
    pub audiobook_id: i64,
    pub name: String,
    pub full_path: String,
    pub length_of_file: Option<i64>,
    pub seek_position: Option<i64>,
    pub checksum: Option<String>,
    pub position: i32,
    pub completeness: i32,
    pub file_exists: bool,
    pub created_at: DateTime<Utc>,
}

impl AudiobookFile {
    #[must_use]
    pub const fn is_complete(&self) -> bool {
        self.completeness >= 100
    }

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

    /// Calculate completeness percentage based on seek position and file length
    ///
    /// Returns a value between 0 and 100
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
