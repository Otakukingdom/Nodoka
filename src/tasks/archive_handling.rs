use crate::error::Result;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};
use zip::read::ZipFile;
use zip::ZipArchive;

const ZIP_VIRTUAL_PREFIX: &str = "zip://";
const ZIP_VIRTUAL_DELIM: &str = "::";

#[derive(Clone, Copy, Debug)]
struct ZipExtractionLimits {
    entries: usize,
    total_bytes: u64,
    entry_bytes: u64,
    path_depth: usize,
}

const DEFAULT_ZIP_LIMITS: ZipExtractionLimits = ZipExtractionLimits {
    // Defensive defaults: large enough for real libraries, bounded for safety.
    entries: 10_000,
    total_bytes: 4 * 1024 * 1024 * 1024,
    entry_bytes: 1024 * 1024 * 1024,
    path_depth: 32,
};

fn map_zip_error(err: zip::result::ZipError) -> crate::error::Error {
    match err {
        zip::result::ZipError::UnsupportedArchive(msg)
            if msg == zip::result::ZipError::PASSWORD_REQUIRED =>
        {
            crate::error::Error::ZipPasswordProtected
        }
        other => crate::error::Error::Zip(other),
    }
}

/// Checks if a path is a ZIP archive
#[must_use]
pub fn is_zip_archive(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .is_some_and(|ext| ext.eq_ignore_ascii_case("zip"))
}

/// Extract ZIP to temporary location for playback
///
/// Returns list of extracted audio file paths
///
/// # Errors
///
/// Returns an error if the ZIP file cannot be opened or extracted
pub fn extract_zip_for_playback(zip_path: &Path, temp_dir: &Path) -> Result<Vec<PathBuf>> {
    extract_zip_for_playback_with_limits(zip_path, temp_dir, DEFAULT_ZIP_LIMITS)
}

fn extract_zip_for_playback_with_limits(
    zip_path: &Path,
    temp_dir: &Path,
    limits: ZipExtractionLimits,
) -> Result<Vec<PathBuf>> {
    let file = fs::File::open(zip_path)?;
    let mut archive = ZipArchive::new(file).map_err(map_zip_error)?;

    let archive_len = archive.len();
    if archive_len > limits.entries {
        return Err(crate::error::Error::InvalidInput(format!(
            "ZIP archive has too many entries ({archive_len} > {})",
            limits.entries
        )));
    }

    let mut audio_files = Vec::new();
    let mut total_written: u64 = 0;

    for i in 0..archive_len {
        let mut entry = archive.by_index(i).map_err(map_zip_error)?;
        let file_path = match entry.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        if file_path.components().count() > limits.path_depth {
            return Err(crate::error::Error::InvalidInput(
                "ZIP entry path is too deep".to_string(),
            ));
        }

        // Only extract audio files
        if !is_audio_file(&file_path) {
            continue;
        }

        let entry_size = entry.size();
        if entry_size > limits.entry_bytes {
            return Err(crate::error::Error::InvalidInput(
                "ZIP entry exceeds maximum allowed size".to_string(),
            ));
        }

        let remaining_total = limits.total_bytes.saturating_sub(total_written);
        if remaining_total == 0 {
            return Err(crate::error::Error::InvalidInput(
                "ZIP extraction exceeds maximum allowed total size".to_string(),
            ));
        }

        let output_path = temp_dir.join(&file_path);

        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let mut output_file = fs::File::create(&output_path)?;
        let max_for_entry = limits.entry_bytes.min(remaining_total);
        let written = copy_zip_entry_limited(&mut entry, &mut output_file, max_for_entry)?;
        total_written = total_written.saturating_add(written);

        audio_files.push(output_path);
    }

    Ok(audio_files)
}

/// Lists audio entries contained in a ZIP archive.
///
/// Entries are returned as safe, relative paths (as provided by `ZipFile::enclosed_name()`).
///
/// # Errors
///
/// Returns an error if the ZIP cannot be opened or read.
pub fn list_zip_audio_entries(zip_path: &Path) -> Result<Vec<PathBuf>> {
    let file = fs::File::open(zip_path)?;
    let mut archive = ZipArchive::new(file).map_err(map_zip_error)?;

    let archive_len = archive.len();
    if archive_len > DEFAULT_ZIP_LIMITS.entries {
        return Err(crate::error::Error::InvalidInput(format!(
            "ZIP archive has too many entries ({archive_len} > {})",
            DEFAULT_ZIP_LIMITS.entries
        )));
    }

    let mut out = Vec::new();
    for i in 0..archive_len {
        let file = archive.by_index(i).map_err(map_zip_error)?;
        let Some(enclosed) = file.enclosed_name() else {
            continue;
        };
        if enclosed.components().count() > DEFAULT_ZIP_LIMITS.path_depth {
            continue;
        }
        if is_audio_file(enclosed) {
            out.push(enclosed.to_path_buf());
        }
    }

    out.sort_by(|a, b| {
        let a_name = a.to_string_lossy();
        let b_name = b.to_string_lossy();
        natord::compare(&a_name, &b_name)
    });

    Ok(out)
}

/// Creates a stable virtual path for an audio entry inside a ZIP archive.
///
/// The value is intended for database persistence and UI selection; it is not a real filesystem
/// path. Use [`materialize_zip_virtual_path`] to extract it for playback.
///
/// Format: `zip://<percent-encoded zip path>::<percent-encoded entry path>`
///
/// # Errors
///
/// Returns an error if `zip_path` cannot be canonicalized.
pub fn to_zip_virtual_path(zip_path: &Path, entry_path: &Path) -> Result<String> {
    let zip_abs = zip_path.canonicalize()?;
    let zip_enc = percent_encode_component(zip_abs.to_string_lossy().as_ref());
    let entry_enc = percent_encode_component(entry_path.to_string_lossy().as_ref());
    Ok(format!(
        "{ZIP_VIRTUAL_PREFIX}{zip_enc}{ZIP_VIRTUAL_DELIM}{entry_enc}"
    ))
}

/// Parses a ZIP virtual path created by [`to_zip_virtual_path`].
#[must_use]
pub fn parse_zip_virtual_path(value: &str) -> Option<(PathBuf, PathBuf)> {
    let remainder = value.strip_prefix(ZIP_VIRTUAL_PREFIX)?;
    let (zip_enc, entry_enc) = remainder.split_once(ZIP_VIRTUAL_DELIM)?;
    let zip = percent_decode_component(zip_enc)?;
    let entry = percent_decode_component(entry_enc)?;
    Some((PathBuf::from(zip), PathBuf::from(entry)))
}

/// Materializes a ZIP virtual path by extracting the referenced entry to a stable temp location.
///
/// # Errors
///
/// Returns an error if the virtual path is invalid, extraction fails, or the project data
/// directory cannot be determined.
pub fn materialize_zip_virtual_path(value: &str) -> Result<PathBuf> {
    let (zip_path, entry_path) = parse_zip_virtual_path(value)
        .ok_or_else(|| crate::error::Error::InvalidInput("Invalid ZIP virtual path".to_string()))?;
    let temp_dir = zip_temp_dir(&zip_path)?;
    extract_zip_entry_for_playback(&zip_path, &entry_path, &temp_dir)
}

/// Returns the stable per-archive temp directory used for ZIP playback extraction.
///
/// The directory is deterministic based on the ZIP absolute path to keep extracted paths stable
/// across sessions (progress persistence) while still allowing full cleanup on exit.
///
/// # Errors
///
/// Returns an error if the project data directory cannot be determined or created.
pub fn zip_temp_dir(zip_path: &Path) -> Result<PathBuf> {
    let root = zip_temp_root()?;
    let zip_abs = zip_path.canonicalize()?;
    let hash = sha256_hex(&zip_abs.to_string_lossy());
    Ok(root.join(hash))
}

/// Returns the shared ZIP extraction root directory.
///
/// This directory is used for extracting ZIP entries for playback. It is safe to delete on
/// application shutdown.
///
/// # Errors
///
/// Returns an error if the project data directory cannot be determined or created.
pub fn zip_temp_root() -> Result<PathBuf> {
    use directories::ProjectDirs;
    let proj_dirs = ProjectDirs::from("com", "Otakukingdom", "Nodoka")
        .ok_or(crate::error::Error::ProjectDirNotFound)?;
    let root = proj_dirs.data_dir().join("zip-temp");
    std::fs::create_dir_all(&root)?;
    Ok(root)
}

fn sha256_hex(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let bytes = hasher.finalize();
    let mut out = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        use std::fmt::Write as _;
        let _ = write!(&mut out, "{b:02x}");
    }
    out
}

fn percent_encode_component(input: &str) -> String {
    let mut out = String::new();
    for b in input.as_bytes() {
        match b {
            b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(char::from(*b));
            }
            _ => {
                use std::fmt::Write;
                let _ = write!(out, "%{b:02X}");
            }
        }
    }
    out
}

fn percent_decode_component(input: &str) -> Option<String> {
    let bytes = input.as_bytes();
    let mut out: Vec<u8> = Vec::with_capacity(bytes.len());
    let mut iter = bytes.iter().copied();

    while let Some(b) = iter.next() {
        if b == b'%' {
            let hi = iter.next()?;
            let lo = iter.next()?;
            let val = (hex_val(hi)? << 4) | hex_val(lo)?;
            out.push(val);
        } else {
            out.push(b);
        }
    }

    String::from_utf8(out).ok()
}

const fn hex_val(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}

/// Extracts a single ZIP entry for playback.
///
/// The entry path must match an `enclosed_name()` inside the archive; path traversal entries are
/// rejected.
///
/// # Errors
///
/// Returns an error if extraction fails.
pub fn extract_zip_entry_for_playback(
    zip_path: &Path,
    entry_path: &Path,
    temp_dir: &Path,
) -> Result<PathBuf> {
    extract_zip_entry_for_playback_with_limits(zip_path, entry_path, temp_dir, DEFAULT_ZIP_LIMITS)
}

fn extract_zip_entry_for_playback_with_limits(
    zip_path: &Path,
    entry_path: &Path,
    temp_dir: &Path,
    limits: ZipExtractionLimits,
) -> Result<PathBuf> {
    let file = fs::File::open(zip_path)?;
    let mut archive = ZipArchive::new(file).map_err(map_zip_error)?;

    let archive_len = archive.len();
    if archive_len > limits.entries {
        return Err(crate::error::Error::InvalidInput(format!(
            "ZIP archive has too many entries ({archive_len} > {})",
            limits.entries
        )));
    }

    let entry_str = entry_path.to_string_lossy();
    let mut entry = archive.by_name(&entry_str).map_err(map_zip_error)?;
    let Some(enclosed) = entry.enclosed_name() else {
        return Err(crate::error::Error::InvalidInput(
            "ZIP entry path is not enclosed".to_string(),
        ));
    };
    if enclosed != entry_path {
        return Err(crate::error::Error::InvalidInput(
            "ZIP entry path mismatch".to_string(),
        ));
    }

    if enclosed.components().count() > limits.path_depth {
        return Err(crate::error::Error::InvalidInput(
            "ZIP entry path is too deep".to_string(),
        ));
    }

    let entry_size = entry.size();
    if entry_size > limits.entry_bytes {
        return Err(crate::error::Error::InvalidInput(
            "ZIP entry exceeds maximum allowed size".to_string(),
        ));
    }

    let output_path = temp_dir.join(enclosed);
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut output_file = fs::File::create(&output_path)?;
    let _written = copy_zip_entry_limited(&mut entry, &mut output_file, limits.entry_bytes)?;
    Ok(output_path)
}

fn copy_zip_entry_limited(
    entry: &mut ZipFile<'_>,
    output: &mut fs::File,
    max_bytes: u64,
) -> Result<u64> {
    use std::io::Read as _;
    let mut limited = entry.take(max_bytes.saturating_add(1));
    let written = std::io::copy(&mut limited, output)?;
    if written > max_bytes {
        return Err(crate::error::Error::InvalidInput(
            "ZIP entry exceeds maximum allowed extracted bytes".to_string(),
        ));
    }
    Ok(written)
}

/// Cleanup temporary extracted files
///
/// # Errors
///
/// Returns an error if the directory cannot be removed
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
        .is_some_and(|ext| {
            audio_extensions
                .iter()
                .any(|audio_ext| ext.eq_ignore_ascii_case(audio_ext))
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

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

    #[test]
    fn test_zip_virtual_path_round_trip() -> Result<()> {
        let temp = temp_dir::TempDir::new()?;
        let zip = temp.path().join("a.zip");
        std::fs::write(&zip, b"PK\x03\x04")?;

        let entry = Path::new("disc1/track 01.mp3");
        let v = to_zip_virtual_path(&zip, entry)?;
        let (zip2, entry2) = parse_zip_virtual_path(&v).ok_or_else(|| {
            crate::error::Error::InvalidState("failed to parse virtual path".to_string())
        })?;

        assert_eq!(zip2, zip.canonicalize()?);
        assert_eq!(entry2, entry);
        Ok(())
    }

    #[test]
    fn test_materialize_zip_virtual_path_extracts_entry() -> Result<()> {
        let temp = temp_dir::TempDir::new()?;
        let zip_path = temp.path().join("book.zip");

        let mut zip = zip::ZipWriter::new(std::fs::File::create(&zip_path)?);
        zip.start_file("disc1/chapter1.mp3", zip::write::FileOptions::default())?;
        zip.write_all(b"fake mp3")?;
        zip.finish()?;

        let entries = list_zip_audio_entries(&zip_path)?;
        let first = entries
            .first()
            .ok_or_else(|| crate::error::Error::InvalidState("no entries".to_string()))?;
        let virtual_path = to_zip_virtual_path(&zip_path, first)?;

        let extracted = materialize_zip_virtual_path(&virtual_path)?;
        assert!(extracted.exists());
        assert!(extracted.is_file());
        Ok(())
    }

    #[test]
    fn test_extract_zip_for_playback_enforces_entry_count_limit() -> Result<()> {
        let temp = temp_dir::TempDir::new()?;
        let zip_path = temp.path().join("many.zip");

        let mut zip = zip::ZipWriter::new(std::fs::File::create(&zip_path)?);
        for i in 0..3 {
            zip.start_file(format!("file{i}.mp3"), zip::write::FileOptions::default())?;
            zip.write_all(b"audio")?;
        }
        zip.finish()?;

        let out_dir = temp.path().join("out");
        std::fs::create_dir_all(&out_dir)?;

        let limits = ZipExtractionLimits {
            entries: 2,
            total_bytes: 1024,
            entry_bytes: 1024,
            path_depth: 10,
        };

        let result = extract_zip_for_playback_with_limits(&zip_path, &out_dir, limits);
        assert!(result.is_err());
        Ok(())
    }
}
