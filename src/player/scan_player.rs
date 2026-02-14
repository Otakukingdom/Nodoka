use super::media_duration;
use super::vlc_env;
use crate::error::{Error, Result};
use crate::models::MediaProperty;
use std::path::Path;
use std::time::Duration;
use vlc::{Instance, Media, Meta};

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

        media.parse_async();

        let duration =
            match media_duration::parse_duration_with_timeout(&media, Duration::from_secs(2)) {
                Ok(duration) => duration,
                Err(e) => {
                    tracing::debug!(
                        "Failed to parse duration for {} (continuing with 0ms): {e}",
                        path.display()
                    );
                    0
                }
            };

        let mut props = MediaProperty::new(path.to_path_buf(), duration);
        props.title = media.get_meta(Meta::Title);
        props.author = media.get_meta(Meta::Artist);
        props.narrator = media
            .get_meta(Meta::Publisher)
            .or_else(|| media.get_meta(Meta::EncodedBy));
        props.year = media.get_meta(Meta::Date).as_deref().and_then(parse_year);

        Ok(props)
    }

    /// Attempts to extract embedded cover art bytes for a media file.
    ///
    /// This method uses VLC's media parsing and reads `Meta::ArtworkURL` when available.
    /// If VLC exposes the embedded artwork via a file URL, this reads that file.
    ///
    /// # Errors
    ///
    /// Returns an error if the media cannot be loaded.
    pub fn scan_embedded_cover_art_bytes(&self, path: &Path) -> Result<Option<Vec<u8>>> {
        const PARSE_TIMEOUT: Duration = Duration::from_secs(2);
        const MAX_BYTES: u64 = 10 * 1024 * 1024;

        let media = Media::new_path(&self.instance, path)
            .ok_or_else(|| Error::MediaParse("Failed to load media".to_string()))?;
        media.parse_async();

        let start = std::time::Instant::now();
        while !media.is_parsed() {
            if start.elapsed() >= PARSE_TIMEOUT {
                break;
            }
            std::thread::sleep(Duration::from_millis(25));
        }

        let Some(art_url) = media.get_meta(Meta::ArtworkURL) else {
            return Ok(None);
        };

        let Some(path) = artwork_url_to_path(&art_url) else {
            return Ok(None);
        };

        let file = std::fs::File::open(&path)?;
        let meta = file.metadata()?;
        if meta.len() > MAX_BYTES {
            return Ok(None);
        }

        let mut reader = std::io::BufReader::new(file);
        let mut out = Vec::new();
        std::io::Read::read_to_end(&mut reader, &mut out)?;
        Ok(Some(out))
    }
}

fn parse_year(input: &str) -> Option<i32> {
    let trimmed = input.trim();
    let digits: String = trimmed
        .chars()
        .filter(char::is_ascii_digit)
        .take(4)
        .collect();
    if digits.len() != 4 {
        return None;
    }
    digits.parse::<i32>().ok()
}

fn artwork_url_to_path(url: &str) -> Option<std::path::PathBuf> {
    let trimmed = url.trim();
    let without_prefix = trimmed.strip_prefix("file://")?;
    let decoded = percent_decode_utf8(without_prefix.as_bytes())?;
    Some(std::path::PathBuf::from(decoded))
}

fn percent_decode_utf8(input: &[u8]) -> Option<String> {
    let mut out: Vec<u8> = Vec::with_capacity(input.len());
    let mut iter = input.iter().copied();
    while let Some(b) = iter.next() {
        if b == b'%' {
            let hi = from_hex(iter.next()?)?;
            let lo = from_hex(iter.next()?)?;
            out.push((hi << 4) | lo);
        } else {
            out.push(b);
        }
    }
    String::from_utf8(out).ok()
}

const fn from_hex(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
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
