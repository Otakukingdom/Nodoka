//! Persistent cover thumbnail cache.
//!
//! This module bridges [`crate::cover_art`] selection and UI display by generating a resized
//! thumbnail in the application data directory.

use crate::error::Result;
use std::path::{Path, PathBuf};

/// Ensures a cached cover thumbnail exists for the provided audiobook.
///
/// The thumbnail is stored in the application data directory and can be loaded by the UI via
/// `iced::widget::image`.
///
/// # Errors
///
/// Returns an error if the application data directory cannot be determined or if I/O fails while
/// creating the cache directory.
pub fn ensure_cover_thumbnail(audiobook_id: i64, audiobook_path: &Path) -> Result<Option<PathBuf>> {
    ensure_cover_thumbnail_with_embedded(audiobook_id, audiobook_path, None)
}

/// Like [`ensure_cover_thumbnail`], but allows callers to supply embedded image bytes.
///
/// This is used to support cover art embedded in container formats (M4B/MP4) and tag formats
/// (MP3 ID3) without requiring this module to parse media metadata itself.
///
/// # Errors
///
/// Returns an error if cache directory creation fails or if thumbnail generation fails.
pub fn ensure_cover_thumbnail_with_embedded(
    audiobook_id: i64,
    audiobook_path: &Path,
    embedded_image: Option<&[u8]>,
) -> Result<Option<PathBuf>> {
    use directories::ProjectDirs;

    let selection_root: &Path = if audiobook_path.is_dir() {
        audiobook_path
    } else {
        audiobook_path.parent().unwrap_or(audiobook_path)
    };

    let probed_embedded: Option<Vec<u8>> = if embedded_image.is_some() {
        None
    } else {
        probe_embedded_cover_bytes(audiobook_path)
    };

    let embedded_image = embedded_image.or(probed_embedded.as_deref());

    let selected = crate::cover_art::select(selection_root, embedded_image)?;
    let Some(selection) = selected else {
        return Ok(None);
    };

    let proj_dirs = ProjectDirs::from("com", "Otakukingdom", "Nodoka")
        .ok_or(crate::error::Error::ProjectDirNotFound)?;
    let cache_dir = proj_dirs.data_dir().join("cover-cache");
    std::fs::create_dir_all(&cache_dir)?;
    let out_path = cache_dir.join(format!("{audiobook_id}.png"));

    let img = match selection.source {
        crate::cover_art::Source::Embedded => {
            let Some(bytes) = embedded_image else {
                return Ok(None);
            };
            image::load_from_memory(bytes)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?
        }
        crate::cover_art::Source::File(path) => image::open(&path)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?,
    };
    let thumb = img.thumbnail(200, 200);
    thumb
        .save_with_format(&out_path, image::ImageFormat::Png)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    Ok(Some(out_path))
}

fn probe_embedded_cover_bytes(audiobook_path: &Path) -> Option<Vec<u8>> {
    let media_path = first_audio_file_candidate(audiobook_path)?;

    let scanner = match crate::player::Scanner::new() {
        Ok(scanner) => scanner,
        Err(e) => {
            tracing::debug!("Skipping embedded cover art extraction; VLC scanner unavailable: {e}");
            return None;
        }
    };

    match scanner.scan_embedded_cover_art_bytes(&media_path) {
        Ok(bytes) => bytes,
        Err(e) => {
            tracing::debug!(
                "Skipping embedded cover art extraction for {}: {e}",
                media_path.display()
            );
            None
        }
    }
}

fn first_audio_file_candidate(audiobook_path: &Path) -> Option<std::path::PathBuf> {
    if audiobook_path.is_file() {
        let ext = audiobook_path.extension().and_then(|e| e.to_str())?;
        if is_audio_extension(ext) {
            return Some(audiobook_path.to_path_buf());
        }
        return None;
    }

    if !audiobook_path.is_dir() {
        return None;
    }

    let mut candidates: Vec<std::path::PathBuf> = std::fs::read_dir(audiobook_path)
        .ok()?
        .filter_map(std::result::Result::ok)
        .map(|e| e.path())
        .filter(|p| {
            if !p.is_file() {
                return false;
            }
            let Some(name) = p.file_name().and_then(|n| n.to_str()) else {
                return false;
            };
            if name.starts_with('.') {
                return false;
            }
            let Some(ext) = p.extension().and_then(|e| e.to_str()) else {
                return false;
            };
            is_audio_extension(ext)
        })
        .collect();

    candidates.sort();
    candidates.into_iter().next()
}

fn is_audio_extension(ext: &str) -> bool {
    ext.eq_ignore_ascii_case("mp3")
        || ext.eq_ignore_ascii_case("m4a")
        || ext.eq_ignore_ascii_case("m4b")
        || ext.eq_ignore_ascii_case("ogg")
        || ext.eq_ignore_ascii_case("flac")
        || ext.eq_ignore_ascii_case("opus")
        || ext.eq_ignore_ascii_case("aac")
        || ext.eq_ignore_ascii_case("wma")
        || ext.eq_ignore_ascii_case("wav")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_ensure_cover_thumbnail_creates_file_for_folder_jpg(
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let temp = temp_dir::TempDir::new()?;
        let book_dir = temp.path().join("Book");
        fs::create_dir_all(&book_dir)?;

        // Create a minimal valid JPEG using the image crate.
        let img = image::RgbImage::from_pixel(2, 2, image::Rgb([255, 0, 0]));
        let dyn_img = image::DynamicImage::ImageRgb8(img);
        let mut bytes = Vec::new();
        let mut encoder = image::codecs::jpeg::JpegEncoder::new(&mut bytes);
        encoder.encode_image(&dyn_img)?;
        fs::write(book_dir.join("folder.jpg"), bytes)?;

        let thumb = ensure_cover_thumbnail(42, &book_dir)?;
        assert!(thumb.is_some(), "expected a thumbnail to be generated");
        Ok(())
    }

    fn write_test_png(path: &Path) -> std::result::Result<Vec<u8>, Box<dyn std::error::Error>> {
        let img = image::RgbImage::from_pixel(2, 2, image::Rgb([0, 255, 0]));
        let dyn_img = image::DynamicImage::ImageRgb8(img);

        let mut bytes = Vec::new();
        dyn_img.write_to(
            &mut std::io::Cursor::new(&mut bytes),
            image::ImageOutputFormat::Png,
        )?;
        fs::write(path, &bytes)?;
        Ok(bytes)
    }

    #[test]
    fn test_ensure_cover_thumbnail_creates_file_for_embedded_cover(
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let temp = temp_dir::TempDir::new()?;
        let book_dir = temp.path().join("Book");
        fs::create_dir_all(&book_dir)?;

        let cover_path = book_dir.join("embedded.png");
        let cover_bytes = write_test_png(&cover_path)?;
        fs::remove_file(&cover_path)?;

        let thumb = ensure_cover_thumbnail_with_embedded(7, &book_dir, Some(&cover_bytes))?;
        assert!(thumb.is_some(), "expected thumbnail from embedded cover");
        Ok(())
    }
}
