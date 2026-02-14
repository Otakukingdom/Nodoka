use crate::models::Audiobook;
use chrono::Utc;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub struct DiscoveredAudiobook {
    pub path: PathBuf,
    pub name: String,
    pub files: Vec<PathBuf>,
}

/// Scans a directory for audiobook folders
///
/// # Errors
///
/// Returns an error if the directory cannot be read
pub async fn scan_directory(dir_path: PathBuf) -> Result<Vec<DiscoveredAudiobook>, std::io::Error> {
    tokio::task::spawn_blocking(move || {
        let mut audiobooks = Vec::new();

        for entry in WalkDir::new(&dir_path)
            .min_depth(1)
            .follow_links(false)
            .into_iter()
            .filter_entry(|e| {
                // Filter out hidden files and directories (starting with '.')
                e.file_name()
                    .to_str()
                    .is_some_and(|s| !s.starts_with('.'))
            })
            .filter_map(std::result::Result::ok)
        {
            if entry.file_type().is_dir() {
                if let Some(audiobook) = discover_audiobook(entry.path()) {
                    audiobooks.push(audiobook);
                }
            }
        }

        audiobooks
    })
    .await
    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
}

fn discover_audiobook(path: &Path) -> Option<DiscoveredAudiobook> {
    let mut audio_files: Vec<PathBuf> = std::fs::read_dir(path)
        .ok()?
        .filter_map(std::result::Result::ok)
        .map(|e| e.path())
        .filter(|p| {
            // Filter out hidden files (starting with '.')
            if let Some(file_name) = p.file_name().and_then(|n| n.to_str()) {
                if file_name.starts_with('.') {
                    return false;
                }
            }
            is_audio_file(p)
        })
        .collect();

    if audio_files.is_empty() {
        return None;
    }

    // Sort files using natural ordering
    audio_files.sort_by(|a, b| {
        let a_name = a.file_name().and_then(|n| n.to_str()).unwrap_or("");
        let b_name = b.file_name().and_then(|n| n.to_str()).unwrap_or("");
        natord::compare(a_name, b_name)
    });

    Some(DiscoveredAudiobook {
        path: path.to_path_buf(),
        name: path.file_name()?.to_string_lossy().to_string(),
        files: audio_files,
    })
}

fn is_audio_file(path: &Path) -> bool {
    if let Some(ext) = path.extension() {
        if let Some(ext_str) = ext.to_str() {
            return matches!(
                ext_str.to_lowercase().as_str(),
                "mp3" | "m4a" | "m4b" | "ogg" | "flac" | "opus" | "aac" | "wma"
            );
        }
    }
    false
}

/// Converts discovered audiobooks to domain models
#[must_use]
pub fn convert_to_audiobooks(
    discovered: Vec<DiscoveredAudiobook>,
    directory: &str,
) -> Vec<Audiobook> {
    discovered
        .into_iter()
        .enumerate()
        .map(|(idx, disc)| Audiobook {
            id: None,
            directory: directory.to_string(),
            name: disc.name,
            full_path: disc.path.display().to_string(),
            completeness: 0,
            // Safe cast: we cap at i32::MAX for extremely large collections
            default_order: i32::try_from(idx).unwrap_or(i32::MAX),
            selected_file: None,
            created_at: Utc::now(),
        })
        .collect()
}
