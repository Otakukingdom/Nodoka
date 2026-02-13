use crate::error::{Error, Result};
use crate::models::MediaProperty;
use std::path::Path;
use vlc::{Instance, Media};

/// A VLC-based media scanner for extracting metadata without playback
pub struct Scanner {
    instance: Instance,
}

impl Scanner {
    /// Creates a new scan player
    ///
    /// # Errors
    ///
    /// Returns an error if VLC instance cannot be created
    pub fn new() -> Result<Self> {
        let instance = Instance::new()
            .ok_or_else(|| Error::Vlc("Failed to create VLC instance".to_string()))?;
        Ok(Self { instance })
    }

    /// Scans a media file and extracts its duration
    ///
    /// # Errors
    ///
    /// Returns an error if the media cannot be parsed or duration is unavailable
    pub fn scan_media(&self, path: &Path) -> Result<MediaProperty> {
        let media = Media::new_path(&self.instance, path)
            .ok_or_else(|| Error::MediaParse("Failed to load media".to_string()))?;

        // Parse the media to extract metadata
        media.parse();

        // Wait for parsing to complete
        let duration = media
            .duration()
            .ok_or_else(|| Error::MediaParse("Duration not available".to_string()))?;

        Ok(MediaProperty::new(path.to_path_buf(), duration))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn skip_if_vlc_unavailable() -> Option<Scanner> {
        Scanner::new().ok()
    }

    #[test]
    fn test_scanner_creation() {
        if let Some(_scanner) = skip_if_vlc_unavailable() {
            // Scanner created successfully
        }
    }

    #[test]
    fn test_scan_nonexistent_file() {
        if let Some(scanner) = skip_if_vlc_unavailable() {
            let nonexistent_path = PathBuf::from("/nonexistent/file/path.mp3");
            let result = scanner.scan_media(&nonexistent_path);
            assert!(result.is_err(), "Scanning nonexistent file should fail");
        }
    }

    #[test]
    fn test_scan_invalid_media_file() {
        if let Some(scanner) = skip_if_vlc_unavailable() {
            let invalid_path = PathBuf::from("/dev/null");
            let result = scanner.scan_media(&invalid_path);
            assert!(
                result.is_err(),
                "Scanning invalid media file should fail or return error"
            );
        }
    }

    #[test]
    fn test_scan_unsupported_format() {
        if let Some(scanner) = skip_if_vlc_unavailable() {
            let txt_path = PathBuf::from("/tmp/test.txt");
            if txt_path.exists() {
                let result = scanner.scan_media(&txt_path);
                assert!(
                    result.is_err(),
                    "Scanning unsupported format should fail or return error"
                );
            }
        }
    }
}
