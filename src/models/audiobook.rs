use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A discovered audiobook directory tracked by the application.
///
/// This is the top-level domain model used by the library UI and the database.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Audiobook {
    /// Database ID (None if not yet persisted).
    pub id: Option<i64>,
    /// Directory identifier used for grouping (typically the user-added root directory).
    pub directory: String,
    /// Display name derived from the folder name.
    pub name: String,
    /// Author name extracted from metadata or directory structure.
    pub author: Option<String>,
    /// Full path to the audiobook directory.
    pub full_path: String,
    /// Completion percentage in the range 0-100.
    pub completeness: i32,
    /// Default ordering key used for stable library sorting.
    pub default_order: i32,
    /// Selected file path (if the user chose a specific chapter/file to start from).
    pub selected_file: Option<String>,
    /// Total duration of all files in milliseconds.
    pub total_duration_ms: Option<i64>,
    /// Remaining duration in milliseconds computed from per-file progress when available.
    ///
    /// This value is `None` when the database cannot compute it reliably (for example,
    /// when any file is missing a duration).
    pub remaining_duration_ms: Option<i64>,
    /// When this audiobook entry was created.
    pub created_at: DateTime<Utc>,
    /// Last time this audiobook was played (for "recently played" sorting)
    pub last_listened: Option<DateTime<Utc>>,
}

impl Audiobook {
    /// Returns whether the audiobook is considered complete.
    #[must_use]
    pub const fn is_complete(&self) -> bool {
        self.completeness >= 100
    }

    /// Creates a new audiobook model with zero completeness.
    #[must_use]
    pub fn new(directory: String, name: String, full_path: String, default_order: i32) -> Self {
        Self {
            id: None,
            directory,
            name,
            author: None,
            full_path,
            completeness: 0,
            default_order,
            selected_file: None,
            total_duration_ms: None,
            remaining_duration_ms: None,
            created_at: Utc::now(),
            last_listened: None,
        }
    }
}
