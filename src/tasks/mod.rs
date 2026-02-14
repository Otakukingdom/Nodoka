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
//! - `.opus` - Opus audio
//! - `.aac` - Advanced Audio Coding
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
//! Each file may involve:
//! 1. SHA-256 checksum calculation (I/O bound; returned in [`DiscoveredAudiobook`])
//! 2. Optional VLC metadata extraction (CPU bound; performed by callers via [`crate::player::Scanner`])
//! 3. Database writes (I/O bound; performed by the UI update layer)

mod archive_handling;
mod checksum;
mod player_scan;
mod scan_directory;

pub use archive_handling::{
    cleanup_temp_files, extract_zip_entry_for_playback, extract_zip_for_playback, is_zip_archive,
    list_zip_audio_entries, materialize_zip_virtual_path, parse_zip_virtual_path,
    to_zip_virtual_path, zip_temp_dir, zip_temp_root,
};
pub use checksum::sha256;

#[deprecated(
    since = "0.2.0",
    note = "Renamed: use `nodoka::tasks::sha256` instead of `calculate_checksum`"
)]
pub use checksum::sha256 as calculate_checksum;
pub use player_scan::scan_media_properties;
pub use scan_directory::{convert_to_audiobooks, scan_directory, DiscoveredAudiobook};
