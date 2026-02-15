use crate::models::Audiobook;
use chrono::Utc;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use super::archive_handling;

#[derive(Debug, Clone)]
pub struct DiscoveredAudiobook {
    pub path: PathBuf,
    pub name: String,
    pub files: Vec<DiscoveredFile>,
}

#[derive(Debug, Clone)]
pub struct DiscoveredFile {
    /// File identifier stored in the database.
    ///
    /// For filesystem files this is an absolute path string.
    /// For ZIP entries this is the `zip://...::...` virtual path created by
    /// [`crate::tasks::to_zip_virtual_path`].
    pub full_path: String,
    /// Display name (typically the filename within the audiobook).
    pub name: String,
    /// Stable sort key for natural ordering within an audiobook.
    pub sort_key: String,
    /// Optional checksum for detecting content changes.
    pub checksum: Option<String>,
}

/// Scans a directory for audiobook folders
///
/// # Errors
///
/// Returns an error if the directory cannot be read
pub async fn scan_directory(dir_path: PathBuf) -> Result<Vec<DiscoveredAudiobook>, std::io::Error> {
    tokio::task::spawn_blocking(move || {
        let metadata = std::fs::metadata(&dir_path)?;
        if !metadata.is_dir() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Scan root must be a directory",
            ));
        }

        let mut audiobooks = Vec::new();

        // The scan root itself can be an audiobook when it contains audio files.
        if let Some(root_audiobook) = discover_audiobook(&dir_path) {
            audiobooks.push(root_audiobook);
        }

        // Follow directory symlinks so users can link external audiobook roots.
        // walkdir detects cycles and reports them as errors which we skip.
        for entry_result in WalkDir::new(&dir_path).min_depth(1).follow_links(true) {
            let entry = match entry_result {
                Ok(entry) => entry,
                Err(e) => {
                    tracing::warn!(
                        "Skipping unreadable path during scan of {}: {e}",
                        dir_path.display()
                    );
                    continue;
                }
            };

            // Filter out hidden files and directories (starting with '.')
            if entry
                .file_name()
                .to_str()
                .is_some_and(|s| s.starts_with('.'))
            {
                continue;
            }

            if entry.file_type().is_file() {
                if let Some(zip_book) = discover_zip_audiobook(entry.path()) {
                    audiobooks.push(zip_book);
                }
            }

            if entry.file_type().is_dir() {
                if let Some(audiobook) = discover_audiobook(entry.path()) {
                    audiobooks.push(audiobook);
                }
            }
        }

        Ok(audiobooks)
    })
    .await
    .map_err(std::io::Error::other)?
}

fn discover_zip_audiobook(zip_path: &Path) -> Option<DiscoveredAudiobook> {
    if !archive_handling::is_zip_archive(zip_path) {
        return None;
    }

    let entries = match archive_handling::list_zip_audio_entries(zip_path) {
        Ok(entries) => entries,
        Err(e) => {
            tracing::warn!("Failed to read ZIP archive {}: {e}", zip_path.display());
            return None;
        }
    };

    if entries.is_empty() {
        return None;
    }

    let files: Vec<DiscoveredFile> = entries
        .into_iter()
        .filter_map(|entry| {
            let full_path = archive_handling::to_zip_virtual_path(zip_path, &entry).ok()?;
            let name = entry
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("Unknown")
                .to_string();
            let sort_key = entry.to_string_lossy().to_string();
            Some(DiscoveredFile {
                full_path,
                name,
                sort_key,
                checksum: None,
            })
        })
        .collect();

    if files.is_empty() {
        return None;
    }

    let name = zip_path
        .file_stem()
        .or_else(|| zip_path.file_name())?
        .to_string_lossy()
        .to_string();

    Some(DiscoveredAudiobook {
        path: zip_path.to_path_buf(),
        name,
        files,
    })
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

    let files: Vec<DiscoveredFile> = audio_files
        .into_iter()
        .map(|p| {
            let name = p
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("Unknown")
                .to_string();
            let sort_key = name.clone();
            let full_path = p.display().to_string();
            let checksum = sha256_file(&p).ok();
            DiscoveredFile {
                full_path,
                name,
                sort_key,
                checksum,
            }
        })
        .collect();

    Some(DiscoveredAudiobook {
        path: path.to_path_buf(),
        name: path.file_name()?.to_string_lossy().to_string(),
        files,
    })
}

fn sha256_file(path: &Path) -> std::io::Result<String> {
    use sha2::Digest as _;
    use std::io::Read as _;

    let file = std::fs::File::open(path)?;
    let mut reader = std::io::BufReader::new(file);
    let mut hasher = sha2::Sha256::new();
    let mut buf = [0u8; 8192];
    loop {
        let n = reader.read(&mut buf)?;
        if n == 0 {
            break;
        }
        if let Some(slice) = buf.get(..n) {
            hasher.update(slice);
        }
    }
    let out = hasher.finalize();
    Ok(format!("{out:x}"))
}

fn is_audio_file(path: &Path) -> bool {
    if let Some(ext) = path.extension() {
        if let Some(ext_str) = ext.to_str() {
            return matches!(
                ext_str.to_lowercase().as_str(),
                "mp3" | "m4a" | "m4b" | "ogg" | "flac" | "opus" | "aac" | "wma" | "wav"
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
