use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Directory {
    pub full_path: String,
    pub created_at: DateTime<Utc>,
    pub last_scanned: Option<DateTime<Utc>>,
}

impl Directory {
    #[must_use]
    pub fn new(full_path: String) -> Self {
        Self {
            full_path,
            created_at: Utc::now(),
            last_scanned: None,
        }
    }

    pub fn mark_scanned(&mut self) {
        self.last_scanned = Some(Utc::now());
    }
}
