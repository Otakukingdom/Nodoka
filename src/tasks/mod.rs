//! Async tasks for file scanning and media processing.
//!
//! This module provides async operations for:
//! - Directory scanning to discover audiobooks
//! - File checksum calculation
//! - Media metadata extraction via VLC
//!
//! All tasks are designed to run in Tokio's async runtime without
//! blocking the UI thread.
//!
//! ## Directory Scanning
//!
//! The [`scan_directory()`] function recursively walks a directory tree
//! to find audio files, groups them by parent folder, and extracts metadata:
//!
//! ```no_run
//! # use nodoka::tasks::scan_directory;
//! # use std::path::PathBuf;
//! # async fn example() -> Result<(), std::io::Error> {
//! let discovered = scan_directory(PathBuf::from("/path/to/audiobooks")).await?;
//!
//! for audiobook in discovered {
//!     println!("Found: {} with {} files", audiobook.name, audiobook.files.len());
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Supported File Types
//!
//! The scanner looks for these audio formats:
//! - `.mp3` - MPEG audio
//! - `.m4a`, `.m4b` - Apple audio/audiobook
//! - `.ogg` - OGG Vorbis
//! - `.flac` - Free Lossless Audio Codec
//! - `.wav` - Waveform audio
//! - `.wma` - Windows Media Audio
//!
//! ## Performance
//!
//! Scanning performance depends on:
//! - Number of files (1000+ files may take several minutes)
//! - Disk speed (SSD vs HDD)
//! - Network latency (local vs network drives)
//!
//! Each file requires:
//! 1. SHA-256 checksum calculation (I/O bound)
//! 2. VLC metadata extraction (CPU bound)
//! 3. Database write (I/O bound)

mod archive_handling;
mod checksum;
mod player_scan;
mod scan_directory;

pub use archive_handling::{cleanup_temp_files, extract_zip_for_playback, is_zip_archive};
pub use checksum::sha256;

#[deprecated(
    since = "0.2.0",
    note = "Renamed: use `nodoka::tasks::sha256` instead of `calculate_checksum`"
)]
pub use checksum::sha256 as calculate_checksum;
pub use player_scan::scan_media_properties;
pub use scan_directory::{convert_to_audiobooks, scan_directory, DiscoveredAudiobook};
