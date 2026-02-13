use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Audiobook {
    pub id: Option<i64>,
    pub directory: String,
    pub name: String,
    pub full_path: String,
    pub completeness: i32,
    pub default_order: i32,
    pub selected_file: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl Audiobook {
    #[must_use]
    pub const fn is_complete(&self) -> bool {
        self.completeness >= 100
    }

    #[must_use]
    pub fn new(directory: String, name: String, full_path: String, default_order: i32) -> Self {
        Self {
            id: None,
            directory,
            name,
            full_path,
            completeness: 0,
            default_order,
            selected_file: None,
            created_at: Utc::now(),
        }
    }
}
