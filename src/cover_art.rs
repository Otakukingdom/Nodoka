//! Cover art selection and validation.
//!
//! Nodoka selects cover art using a strict priority order:
//!
//! 1. Embedded image data (when provided by the caller)
//! 2. `folder.jpg` / `folder.jpeg` in the audiobook directory
//! 3. `cover.jpg` / `cover.jpeg` in the audiobook directory
//! 4. `cover.png` in the audiobook directory
//!
//! Candidate images are validated by attempting to decode them. Corrupted or
//! unsupported images are ignored and selection falls back to the next option.

use crate::error::Result;
use image::GenericImageView;
use std::path::{Path, PathBuf};

/// Where the selected cover art came from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Source {
    /// Image bytes embedded in media metadata.
    Embedded,
    /// Image file in the audiobook directory.
    File(PathBuf),
}

/// A validated cover art selection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Selection {
    pub source: Source,
    pub width: u32,
    pub height: u32,
}

/// Selects cover art for an audiobook directory.
///
/// The `embedded_image` parameter allows callers (e.g. media parsers) to pass
/// embedded image data without coupling this module to a specific metadata
/// extraction backend.
///
/// # Errors
///
/// Returns an error if `audiobook_dir` is not a readable directory.
pub fn select(audiobook_dir: &Path, embedded_image: Option<&[u8]>) -> Result<Option<Selection>> {
    let metadata = std::fs::metadata(audiobook_dir)?;
    if !metadata.is_dir() {
        return Err(crate::error::Error::Io(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Cover art root must be a directory",
        )));
    }

    if let Some(bytes) = embedded_image {
        if let Ok(img) = image::load_from_memory(bytes) {
            let (width, height) = img.dimensions();
            return Ok(Some(Selection {
                source: Source::Embedded,
                width,
                height,
            }));
        }
    }

    let candidates = candidate_paths(audiobook_dir)?;
    for path in candidates {
        if let Ok(img) = image::open(&path) {
            let (width, height) = img.dimensions();
            return Ok(Some(Selection {
                source: Source::File(path),
                width,
                height,
            }));
        }
    }

    Ok(None)
}

fn candidate_paths(audiobook_dir: &Path) -> Result<Vec<PathBuf>> {
    let mut entries: Vec<(String, PathBuf)> = Vec::new();
    for entry in std::fs::read_dir(audiobook_dir)? {
        let entry = entry?;
        let path = entry.path();
        let file_type = entry.file_type()?;
        if !file_type.is_file() {
            continue;
        }

        let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
            continue;
        };
        if name.starts_with('.') {
            continue;
        }
        entries.push((name.to_ascii_lowercase(), path));
    }

    let mut out = Vec::new();
    push_first_match(&mut out, &entries, ["folder.jpg", "folder.jpeg"]);
    push_first_match(&mut out, &entries, ["cover.jpg", "cover.jpeg"]);
    push_first_match(&mut out, &entries, ["cover.png"]);
    Ok(out)
}

fn push_first_match<const N: usize>(
    out: &mut Vec<PathBuf>,
    entries: &[(String, PathBuf)],
    names: [&str; N],
) {
    for name in names {
        if let Some((_, path)) = entries.iter().find(|(lower, _)| lower == name) {
            out.push(path.clone());
            return;
        }
    }
}
