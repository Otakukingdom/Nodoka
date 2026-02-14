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
    use directories::ProjectDirs;

    let selection_root: &Path = if audiobook_path.is_dir() {
        audiobook_path
    } else {
        audiobook_path.parent().unwrap_or(audiobook_path)
    };

    let selected = crate::cover_art::select(selection_root, None)?;
    let Some(selection) = selected else {
        return Ok(None);
    };

    let source_path = match selection.source {
        crate::cover_art::Source::Embedded => {
            // Embedded images require the caller to supply bytes.
            return Ok(None);
        }
        crate::cover_art::Source::File(path) => path,
    };

    let proj_dirs = ProjectDirs::from("com", "Otakukingdom", "Nodoka")
        .ok_or(crate::error::Error::ProjectDirNotFound)?;
    let cache_dir = proj_dirs.data_dir().join("cover-cache");
    std::fs::create_dir_all(&cache_dir)?;
    let out_path = cache_dir.join(format!("{audiobook_id}.png"));

    let img = image::open(&source_path)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    let thumb = img.thumbnail(200, 200);
    thumb
        .save_with_format(&out_path, image::ImageFormat::Png)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    Ok(Some(out_path))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

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
}
