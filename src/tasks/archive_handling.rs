use crate::error::Result;
use std::fs;
use std::path::{Path, PathBuf};
use zip::ZipArchive;

/// Checks if a path is a ZIP archive
pub fn is_zip_archive(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.eq_ignore_ascii_case("zip"))
        .unwrap_or(false)
}

/// Extract ZIP to temporary location for playback
/// Returns list of extracted audio file paths
pub fn extract_zip_for_playback(zip_path: &Path, temp_dir: &Path) -> Result<Vec<PathBuf>> {
    let file = fs::File::open(zip_path)?;
    let mut archive = ZipArchive::new(file)?;

    let mut audio_files = Vec::new();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let file_path = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        // Only extract audio files
        if !is_audio_file(&file_path) {
            continue;
        }

        let output_path = temp_dir.join(&file_path);

        // Create parent directories
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)?;
        }

        // Extract file
        let mut output_file = fs::File::create(&output_path)?;
        std::io::copy(&mut file, &mut output_file)?;

        audio_files.push(output_path);
    }

    Ok(audio_files)
}

/// Cleanup temporary extracted files
pub fn cleanup_temp_files(temp_dir: &Path) -> Result<()> {
    if temp_dir.exists() {
        fs::remove_dir_all(temp_dir)?;
    }
    Ok(())
}

/// Check if a file is an audio file based on extension
fn is_audio_file(path: &Path) -> bool {
    let audio_extensions = [
        "mp3", "m4a", "m4b", "ogg", "flac", "opus", "aac", "wma", "wav",
    ];

    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| {
            audio_extensions
                .iter()
                .any(|audio_ext| ext.eq_ignore_ascii_case(audio_ext))
        })
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_zip_archive_true() {
        assert!(is_zip_archive(Path::new("test.zip")));
        assert!(is_zip_archive(Path::new("test.ZIP")));
        assert!(is_zip_archive(Path::new("/path/to/archive.zip")));
    }

    #[test]
    fn test_is_zip_archive_false() {
        assert!(!is_zip_archive(Path::new("test.mp3")));
        assert!(!is_zip_archive(Path::new("test.tar.gz")));
        assert!(!is_zip_archive(Path::new("noextension")));
    }

    #[test]
    fn test_is_audio_file_true() {
        assert!(is_audio_file(Path::new("file.mp3")));
        assert!(is_audio_file(Path::new("file.MP3")));
        assert!(is_audio_file(Path::new("file.m4b")));
        assert!(is_audio_file(Path::new("file.flac")));
        assert!(is_audio_file(Path::new("file.ogg")));
    }

    #[test]
    fn test_is_audio_file_false() {
        assert!(!is_audio_file(Path::new("file.txt")));
        assert!(!is_audio_file(Path::new("file.jpg")));
        assert!(!is_audio_file(Path::new("noext")));
    }
}
