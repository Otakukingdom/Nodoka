use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use crate::models::Audiobook;
use chrono::Utc;

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
            .max_depth(2)
            .into_iter()
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
    let audio_files: Vec<PathBuf> = std::fs::read_dir(path)
        .ok()?
        .filter_map(std::result::Result::ok)
        .map(|e| e.path())
        .filter(|p| is_audio_file(p))
        .collect();
    
    if audio_files.is_empty() {
        return None;
    }
    
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
pub fn convert_to_audiobooks(discovered: Vec<DiscoveredAudiobook>, directory: &str) -> Vec<Audiobook> {
    discovered
        .into_iter()
        .enumerate()
        .map(|(idx, disc)| Audiobook {
            id: None,
            directory: directory.to_string(),
            name: disc.name,
            full_path: disc.path.display().to_string(),
            completeness: 0,
            default_order: idx as i32,
            selected_file: None,
            created_at: Utc::now(),
        })
        .collect()
}
