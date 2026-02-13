use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaProperty {
    pub path: PathBuf,
    pub duration_ms: i64,
}

impl MediaProperty {
    #[must_use]
    pub const fn new(path: PathBuf, duration_ms: i64) -> Self {
        Self { path, duration_ms }
    }
}
