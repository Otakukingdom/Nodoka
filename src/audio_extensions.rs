//! Canonical audio-file extension support for Nodoka.
//!
//! All modules that need to decide whether a path represents an audio file
//! MUST use the helpers in this module.  Do not copy the extension list.

/// Every file extension (without the leading dot, case-insensitive) that
/// Nodoka considers an audio file.
pub const AUDIO_EXTENSIONS: &[&str] = &[
    "mp3", "m4b", "m4a", "flac", "ogg", "opus", "wav", "aac", "wma", "aiff", "aif",
];

/// Returns `true` when `path` has an extension that is in [`AUDIO_EXTENSIONS`].
///
/// The comparison is ASCII-case-insensitive so `.MP3` and `.mp3` both match.
pub fn is_audio_file(path: &std::path::Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .is_some_and(|ext| {
            AUDIO_EXTENSIONS
                .iter()
                .any(|audio_ext| ext.eq_ignore_ascii_case(audio_ext))
        })
}

/// Returns `true` when `ext` (without the leading dot) is a known audio extension.
///
/// The comparison is ASCII-case-insensitive.
pub fn is_audio_extension(ext: &str) -> bool {
    AUDIO_EXTENSIONS
        .iter()
        .any(|audio_ext| ext.eq_ignore_ascii_case(audio_ext))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_is_audio_file_known_extensions() {
        assert!(is_audio_file(Path::new("book.mp3")));
        assert!(is_audio_file(Path::new("book.MP3")));
        assert!(is_audio_file(Path::new("chapter.m4b")));
        assert!(is_audio_file(Path::new("chapter.flac")));
        assert!(is_audio_file(Path::new("track.ogg")));
        assert!(is_audio_file(Path::new("track.opus")));
        assert!(is_audio_file(Path::new("track.wav")));
        assert!(is_audio_file(Path::new("track.aac")));
        assert!(is_audio_file(Path::new("track.wma")));
        assert!(is_audio_file(Path::new("track.m4a")));
        assert!(is_audio_file(Path::new("track.aiff")));
        assert!(is_audio_file(Path::new("track.aif")));
    }

    #[test]
    fn test_is_audio_file_unknown_extensions() {
        assert!(!is_audio_file(Path::new("cover.jpg")));
        assert!(!is_audio_file(Path::new("readme.txt")));
        assert!(!is_audio_file(Path::new("noextension")));
        assert!(!is_audio_file(Path::new("archive.zip")));
    }

    #[test]
    fn test_is_audio_extension_case_insensitive() {
        assert!(is_audio_extension("MP3"));
        assert!(is_audio_extension("mp3"));
        assert!(is_audio_extension("Mp3"));
        assert!(is_audio_extension("FLAC"));
        assert!(!is_audio_extension("jpg"));
        assert!(!is_audio_extension("txt"));
    }
}
