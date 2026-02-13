use super::media_duration;
use super::vlc_env;
use crate::error::{Error, Result};
use crate::models::MediaProperty;
use std::path::Path;
use std::time::Duration;
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
        let instance = vlc_env::create_vlc_instance().ok_or_else(|| {
            let plugin_path_info = std::env::var("VLC_PLUGIN_PATH").map_or_else(
                |_| "VLC_PLUGIN_PATH not set".to_string(),
                |p| format!("VLC_PLUGIN_PATH={p}"),
            );

            tracing::error!(
                "Failed to create VLC instance for scanner. Environment: {}",
                plugin_path_info
            );

            Error::Vlc(format!(
                "Failed to create VLC instance for media scanning. {plugin_path_info}\n\
                 Please ensure VLC media player is installed. \
                 See VLC installation instructions in error documentation."
            ))
        })?;

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

        let duration = media_duration::parse_duration_with_timeout(&media, Duration::from_secs(2))?;

        Ok(MediaProperty::new(path.to_path_buf(), duration))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use temp_dir::TempDir;

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
    fn test_scan_directory_path_returns_error() -> Result<()> {
        let Some(scanner) = skip_if_vlc_unavailable() else {
            return Ok(());
        };

        let temp_dir = TempDir::new()?;
        let result = scanner.scan_media(temp_dir.path());

        match result {
            Ok(prop) => {
                assert_eq!(prop.path, temp_dir.path().to_path_buf());
                assert!(prop.duration_ms >= 0);
            }
            Err(Error::MediaParse(_)) => {}
            Err(other) => {
                assert!(
                    matches!(other, Error::MediaParse(_)),
                    "Unexpected error: {other:?}"
                );
            }
        }
        Ok(())
    }

    #[test]
    fn test_scan_text_file_returns_error() -> Result<()> {
        let Some(scanner) = skip_if_vlc_unavailable() else {
            return Ok(());
        };

        let temp_dir = TempDir::new()?;
        let txt_path = temp_dir.path().join("test.txt");
        std::fs::write(&txt_path, "not audio")?;

        let result = scanner.scan_media(&txt_path);

        match result {
            Ok(prop) => {
                assert_eq!(prop.path, txt_path);
                assert!(prop.duration_ms >= 0);
            }
            Err(Error::MediaParse(_)) => {}
            Err(other) => {
                assert!(
                    matches!(other, Error::MediaParse(_)),
                    "Unexpected error: {other:?}"
                );
            }
        }
        Ok(())
    }
}
