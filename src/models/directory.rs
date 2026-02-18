use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A user-configured root directory to scan for audiobooks.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Directory {
    /// Absolute filesystem path.
    pub full_path: String,
    /// When this directory entry was created.
    pub created_at: DateTime<Utc>,
    /// Last scan timestamp (when a scan completed successfully).
    pub last_scanned: Option<DateTime<Utc>>,
}

impl Directory {
    /// Creates a new directory entry.
    #[must_use]
    pub fn new(full_path: String) -> Self {
        Self {
            full_path,
            created_at: Utc::now(),
            last_scanned: None,
        }
    }

    /// Marks the directory as scanned at the current time.
    pub fn mark_scanned(&mut self) {
        self.last_scanned = Some(Utc::now());
    }
}
