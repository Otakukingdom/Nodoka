use crate::error::{NodokaError, Result};
use crate::models::MediaProperty;
use std::path::Path;
use vlc::{Instance, Media};

/// A VLC-based media scanner for extracting metadata without playback
pub struct ScanPlayer {
    instance: Instance,
}

impl ScanPlayer {
    /// Creates a new scan player
    ///
    /// # Errors
    ///
    /// Returns an error if VLC instance cannot be created
    pub fn new() -> Result<Self> {
        let instance = Instance::new()
            .ok_or_else(|| NodokaError::Vlc("Failed to create VLC instance".to_string()))?;
        Ok(Self { instance })
    }

    /// Scans a media file and extracts its duration
    ///
    /// # Errors
    ///
    /// Returns an error if the media cannot be parsed or duration is unavailable
    pub fn scan_media(&self, path: &Path) -> Result<MediaProperty> {
        let media = Media::new_path(&self.instance, path)
            .ok_or_else(|| NodokaError::MediaParse("Failed to load media".to_string()))?;

        // Parse the media to extract metadata
        media.parse();

        // Wait for parsing to complete
        let duration = media
            .duration()
            .ok_or_else(|| NodokaError::MediaParse("Duration not available".to_string()))?;

        Ok(MediaProperty::new(path.to_path_buf(), duration))
    }
}
