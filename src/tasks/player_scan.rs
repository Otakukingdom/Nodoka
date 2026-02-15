use crate::error::Result;
use crate::models::MediaProperty;
use crate::player::Scanner;
use std::path::PathBuf;

/// Scans a media file for its properties using VLC
///
/// # Errors
///
/// Returns an error if VLC cannot parse the media or duration is unavailable
pub async fn scan_media_properties(file_path: PathBuf) -> Result<MediaProperty> {
    tokio::task::spawn_blocking(move || {
        let scanner = Scanner::new()?;
        scanner.scan_media(&file_path)
    })
    .await
    .map_err(|e| {
        crate::error::Error::Io(std::io::Error::other(
            format!("Task join error: {e}"),
        ))
    })?
}
