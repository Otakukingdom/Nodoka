use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Extracted media metadata for a single audio file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaProperty {
    /// Filesystem path of the media file.
    pub path: PathBuf,
    /// Media duration in milliseconds.
    pub duration_ms: i64,
    /// Optional title tag.
    pub title: Option<String>,
    /// Optional author tag.
    pub author: Option<String>,
    /// Optional narrator tag.
    pub narrator: Option<String>,
    /// Optional publication year.
    pub year: Option<i32>,
}

impl MediaProperty {
    /// Creates a new media-property instance with unset optional tags.
    #[must_use]
    pub const fn new(path: PathBuf, duration_ms: i64) -> Self {
        Self {
            path,
            duration_ms,
            title: None,
            author: None,
            narrator: None,
            year: None,
        }
    }
}
