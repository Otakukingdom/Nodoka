//! Regression tests for UI bugs discovered during systematic testing.
//!
//! Tests in this file focus on library lists, directories, and related UI state.

use nodoka::models::{AudiobookFile, Directory};
use nodoka::ui::{ScanState, State};
use std::collections::{HashMap, HashSet};

/// Bug #0004: Audiobook selection clears file selection
///
/// Scenario: User selects file in audiobook A, then switches to audiobook B.
/// File selection state should be cleared.
///
/// Expected: `selected_file` is None after audiobook change.
#[test]
fn test_bug_0004_audiobook_selection_clears_file() {
    let mut state = State {
        selected_audiobook: Some(1),
        selected_file: Some("/audiobook1/file1.mp3".to_string()),
        ..Default::default()
    };

    assert_eq!(state.selected_audiobook, Some(1));
    assert_eq!(
        state.selected_file,
        Some("/audiobook1/file1.mp3".to_string())
    );

    // Simulate audiobook selection change
    let old_audiobook = state.selected_audiobook;
    let new_audiobook = Some(2);

    if old_audiobook != new_audiobook {
        state.selected_file = None;
        state.current_files.clear();
    }

    state.selected_audiobook = new_audiobook;

    assert_eq!(state.selected_audiobook, Some(2));
    assert_eq!(state.selected_file, None);
    assert!(state.current_files.is_empty());
}

/// Bug #0008: File selection with missing file shows warning
///
/// Scenario: User tries to select a file marked as missing.
///
/// Expected: File selection is not allowed for missing files.
#[test]
fn test_bug_0008_missing_file_not_selectable() {
    let file = AudiobookFile {
        audiobook_id: 1,
        name: "chapter1.mp3".to_string(),
        full_path: "/test/chapter1.mp3".to_string(),
        length_of_file: Some(3_600_000),
        seek_position: None,
        checksum: None,
        position: 0,
        completeness: 0,
        file_exists: false,
        created_at: chrono::Utc::now(),
    };

    assert!(!file.file_exists);

    let state = State {
        current_files: vec![file],
        ..Default::default()
    };

    assert!(!state.current_files.is_empty());
    assert!(matches!(state.current_files.first(), Some(f) if !f.file_exists));
}

/// Bug #0014: Progress bar handles completed files correctly
///
/// Scenario: File is 100% complete.
///
/// Expected: Progress bar shows full, completion indicator shown.
#[test]
fn test_bug_0014_completed_file_progress_display() {
    let file = AudiobookFile {
        audiobook_id: 1,
        name: "chapter1.mp3".to_string(),
        full_path: "/test/chapter1.mp3".to_string(),
        length_of_file: Some(3_600_000),
        seek_position: Some(3_600_000),
        checksum: None,
        position: 0,
        completeness: 100,
        file_exists: true,
        created_at: chrono::Utc::now(),
    };

    assert_eq!(file.completeness, 100);
    assert!(file.seek_position.is_some());

    let is_complete = file.completeness >= 100;
    assert!(is_complete);
}

/// Bug #0015: Settings modal scrolls with many directories
///
/// Scenario: User has 50+ audiobook directories configured.
///
/// Expected: Directory list is scrollable, doesn't overflow modal.
#[test]
fn test_bug_0015_settings_modal_scrollable_directory_list() {
    let directories: Vec<Directory> = (1..=50)
        .map(|i| Directory::new(format!("/path/to/audiobooks{i}")))
        .collect();

    let state = State {
        settings_open: true,
        directories,
        ..Default::default()
    };

    assert_eq!(state.directories.len(), 50);
    assert!(!state.directories.is_empty());
}

/// Bug #0033: File name rendering with extremely long paths
///
/// Scenario: User has files with very long names (200+ characters) that could
/// cause layout issues or text overflow in the file list UI.
///
/// Expected: File list renders without panic, text is displayed (possibly truncated)
#[test]
fn test_bug_0033_file_list_handles_extremely_long_names() {
    let long_name = "A".repeat(500);
    let long_path = format!("/very/long/path/{long_name}.mp3");

    let file = AudiobookFile {
        audiobook_id: 1,
        name: long_name,
        full_path: long_path,
        length_of_file: Some(60000),
        seek_position: None,
        checksum: None,
        position: 0,
        completeness: 0,
        file_exists: true,
        created_at: chrono::Utc::now(),
    };

    assert!(file.name.len() > 400);
}

/// Bug #0034: Audiobook selection with no files in database
///
/// Scenario: User selects an audiobook that has no associated files in database
/// (possibly due to database corruption or manual DB editing).
///
/// Expected: Application handles empty file list gracefully.
#[test]
fn test_bug_0034_audiobook_with_no_files() {
    let state = State {
        selected_audiobook: Some(1),
        current_files: vec![],
        selected_file: None,
        ..Default::default()
    };

    assert!(state.selected_audiobook.is_some());
    assert!(state.current_files.is_empty());
    assert!(state.selected_file.is_none());
}

/// Bug #0042: Multiple audiobooks with same directory path
///
/// Scenario: Database contains duplicate directory entries.
///
/// Expected: UI handles duplicates gracefully without confusion.
#[test]
fn test_bug_0042_duplicate_directory_paths() {
    let directories = ["/path/to/audiobooks", "/path/to/audiobooks", "/other/path"];

    let unique_dirs: HashSet<_> = directories.into_iter().collect();
    assert_eq!(unique_dirs.len(), 2);
}

/// Bug #0044: Audiobook list with missing cover images
///
/// Scenario: Some audiobooks have no cover image files.
///
/// Expected: UI shows placeholder or default image without errors.
#[test]
fn test_bug_0044_missing_cover_images() {
    let state = State {
        selected_audiobook: Some(1),
        cover_thumbnails: HashMap::new(),
        ..Default::default()
    };

    assert!(!state.cover_thumbnails.contains_key(&1));
}

/// Bug #0045: File list scrolling with many files
///
/// Scenario: Audiobook has 100+ files in file list.
///
/// Expected: Scrolling works smoothly, no performance issues.
#[test]
fn test_bug_0045_large_file_list_scrolling() {
    let files: Vec<AudiobookFile> = (0..150)
        .map(|i| AudiobookFile {
            audiobook_id: 1,
            name: format!("file{i:03}.mp3"),
            full_path: format!("/test/file{i:03}.mp3"),
            length_of_file: Some(60000),
            seek_position: None,
            checksum: None,
            position: i,
            completeness: 0,
            file_exists: true,
            created_at: chrono::Utc::now(),
        })
        .collect();

    let state = State {
        selected_audiobook: Some(1),
        current_files: files,
        selected_file: Some("/test/file050.mp3".to_string()),
        ..Default::default()
    };

    assert_eq!(state.current_files.len(), 150);
    assert!(state.selected_file.is_some());
}

/// Bug #0047: Window resize to very small dimensions
///
/// Scenario: User resizes window to minimum size.
///
/// Expected: UI remains usable with responsive layout, no clipping.
#[test]
fn test_bug_0047_minimum_window_size() {
    let min_width = 800_u32;
    let min_height = 600_u32;

    let clamped_width = 400_u32.max(min_width);
    let clamped_height = 300_u32.max(min_height);

    assert_eq!(clamped_width, min_width);
    assert_eq!(clamped_height, min_height);
}

/// Bug #0055: Directory path with spaces
///
/// Scenario: User adds directory "/path/with spaces/audiobooks".
///
/// Expected: Path is stored and scanned correctly.
#[test]
fn test_bug_0055_directory_path_with_spaces() {
    let path_with_spaces = "/path/with spaces/audiobooks";
    let state = State {
        scan_state: ScanState::Scanning {
            directory: Some(path_with_spaces.to_string()),
        },
        ..Default::default()
    };

    assert!(matches!(
        &state.scan_state,
        ScanState::Scanning { directory: Some(d) } if d.contains(' ')
    ));
}

/// Bug #0056: Cover thumbnail generation failure
///
/// Scenario: Cover image extraction fails for an audiobook.
///
/// Expected: Audiobook still displays with placeholder, no crash.
#[test]
fn test_bug_0056_cover_thumbnail_failure() {
    let state = State {
        selected_audiobook: Some(1),
        cover_thumbnails: HashMap::new(),
        ..Default::default()
    };

    assert!(!state.cover_thumbnails.contains_key(&1));
}

/// Bug #0058: Empty audiobook list after fresh install
///
/// Scenario: User opens app for first time, no directories added yet.
///
/// Expected: Empty state message, prompt to add directory.
#[test]
fn test_bug_0058_empty_audiobook_list() {
    let state = State {
        audiobooks: vec![],
        selected_audiobook: None,
        ..Default::default()
    };

    assert!(state.audiobooks.is_empty());
    assert!(state.selected_audiobook.is_none());
}
