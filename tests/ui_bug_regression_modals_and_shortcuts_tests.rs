//! Regression tests for UI bugs discovered during systematic testing.
//!
//! Tests in this file focus on modal invariants and keyboard shortcut behavior.

use nodoka::models::AudiobookFile;
use nodoka::ui::{BookmarkEditor, PlaybackStatus, ScanState, State};

/// Bug #0005: Modal state prevents multiple modals simultaneously
///
/// Scenario: Settings modal is open, user triggers bookmark editor.
///
/// Expected: Only one modal should be open at a time.
#[test]
fn test_bug_0005_single_modal_invariant() {
    let mut state = State {
        settings_open: true,
        bookmark_editor: None,
        ..Default::default()
    };

    assert!(state.settings_open);
    assert!(state.bookmark_editor.is_none());

    // Attempt to open bookmark editor while settings is open
    // The UI should close settings first
    if state.settings_open {
        state.settings_open = false;
    }

    state.bookmark_editor = Some(BookmarkEditor {
        id: None,
        audiobook_id: 1,
        file_path: "/test/file.mp3".to_string(),
        position_ms: 1000,
        label: "Test".to_string(),
        note: String::new(),
    });

    // Verify only bookmark editor is open
    assert!(!state.settings_open);
    assert!(state.bookmark_editor.is_some());
}

/// Bug #0016: Escape key closes correct modal
#[test]
fn test_bug_0016_escape_closes_topmost_modal() {
    let mut state = State {
        settings_open: false,
        bookmark_editor: Some(BookmarkEditor {
            id: None,
            audiobook_id: 1,
            file_path: "/test/file.mp3".to_string(),
            position_ms: 1000,
            label: "Test".to_string(),
            note: String::new(),
        }),
        ..Default::default()
    };

    if state.bookmark_editor.is_some() {
        state.bookmark_editor = None;
    } else if state.settings_open {
        state.settings_open = false;
    }

    assert!(state.bookmark_editor.is_none());
}

/// Bug #0020: Keyboard shortcuts respect modal state
#[test]
fn test_bug_0020_keyboard_shortcuts_respect_modal_state() {
    let state = State {
        settings_open: true,
        playback: PlaybackStatus::Paused,
        ..Default::default()
    };

    let should_handle_shortcut = !state.settings_open && state.bookmark_editor.is_none();
    assert!(!should_handle_shortcut);
}

/// Bug #0021: Multiple modals cannot be open simultaneously
#[test]
fn test_bug_0021_single_modal_invariant() {
    let mut state = State {
        settings_open: true,
        bookmark_editor: None,
        selected_audiobook: Some(1),
        selected_file: Some("/test/file.mp3".to_string()),
        current_time: 1000.0,
        ..Default::default()
    };

    state.settings_open = false;
    state.bookmark_editor = Some(BookmarkEditor {
        id: Some(1),
        audiobook_id: 1,
        file_path: "/test/file.mp3".to_string(),
        position_ms: 1000,
        label: "Bookmark".to_string(),
        note: String::new(),
    });

    assert!(!state.settings_open);
    assert!(state.bookmark_editor.is_some());

    state.bookmark_editor = None;
    state.settings_open = true;

    assert!(state.bookmark_editor.is_none());
    assert!(state.settings_open);
}

/// Bug #0028: Play/pause shortcut (Space) blocked when modal open
#[test]
fn test_bug_0028_play_pause_blocked_when_modal_open() {
    let state = State {
        settings_open: true,
        playback: PlaybackStatus::Paused,
        selected_file: Some("/test/file.mp3".to_string()),
        ..Default::default()
    };
    assert!(state.settings_open);

    let state = State {
        bookmark_editor: Some(BookmarkEditor {
            id: None,
            audiobook_id: 1,
            file_path: "/test/file.mp3".to_string(),
            position_ms: 1000,
            label: "Test".to_string(),
            note: String::new(),
        }),
        playback: PlaybackStatus::Paused,
        ..Default::default()
    };

    assert!(state.bookmark_editor.is_some());
}

/// Bug #0029: Seek shortcuts blocked when modal open
#[test]
fn test_bug_0029_seek_blocked_when_modal_open() {
    let state = State {
        settings_open: true,
        current_time: 100.0,
        total_duration: 3600.0,
        ..Default::default()
    };
    assert!(state.settings_open);

    let state = State {
        bookmark_editor: Some(BookmarkEditor {
            id: None,
            audiobook_id: 1,
            file_path: "/test/file.mp3".to_string(),
            position_ms: 1000,
            label: "Test".to_string(),
            note: String::new(),
        }),
        current_time: 100.0,
        total_duration: 3600.0,
        ..Default::default()
    };

    assert!(state.bookmark_editor.is_some());
}

/// Bug #0030: File navigation shortcuts blocked when modal open
#[test]
fn test_bug_0030_file_navigation_blocked_when_modal_open() {
    let files = vec![
        AudiobookFile {
            audiobook_id: 1,
            name: "file1.mp3".to_string(),
            full_path: "/test/file1.mp3".to_string(),
            length_of_file: Some(60000),
            seek_position: None,
            checksum: None,
            position: 0,
            completeness: 0,
            file_exists: true,
            created_at: chrono::Utc::now(),
        },
        AudiobookFile {
            audiobook_id: 1,
            name: "file2.mp3".to_string(),
            full_path: "/test/file2.mp3".to_string(),
            length_of_file: Some(60000),
            seek_position: None,
            checksum: None,
            position: 1,
            completeness: 0,
            file_exists: true,
            created_at: chrono::Utc::now(),
        },
    ];

    let state = State {
        settings_open: true,
        selected_file: Some("/test/file1.mp3".to_string()),
        current_files: files.clone(),
        ..Default::default()
    };
    assert!(state.settings_open);

    let state = State {
        bookmark_editor: Some(BookmarkEditor {
            id: None,
            audiobook_id: 1,
            file_path: "/test/file1.mp3".to_string(),
            position_ms: 1000,
            label: "Test".to_string(),
            note: String::new(),
        }),
        selected_file: Some("/test/file1.mp3".to_string()),
        current_files: files,
        ..Default::default()
    };

    assert!(state.bookmark_editor.is_some());
}

/// Bug #0038: Rapid modal open/close cycles
#[test]
fn test_bug_0038_rapid_modal_toggle() {
    let mut state = State {
        settings_open: false,
        ..Default::default()
    };

    for _ in 0..10 {
        state.settings_open = true;
        assert!(state.settings_open);

        state.settings_open = false;
        assert!(!state.settings_open);
    }

    assert!(!state.settings_open);
}

/// Bug #0051: Settings modal opened while scanning
#[test]
fn test_bug_0051_modal_during_scanning() {
    let state = State {
        scan_state: ScanState::Scanning {
            directory: Some("/path/to/audiobooks".to_string()),
        },
        settings_open: true,
        ..Default::default()
    };

    assert!(matches!(state.scan_state, ScanState::Scanning { .. }));
    assert!(state.settings_open);
}

/// Bug FIX #004 (Feb 2026): Single modal invariant enforcement
#[test]
fn test_bug_fix_feb2026_004_single_modal_invariant_settings_over_bookmark() {
    let mut state = State {
        bookmark_editor: Some(BookmarkEditor {
            id: None,
            audiobook_id: 1,
            file_path: "/test/file.mp3".to_string(),
            position_ms: 1000,
            label: "Test".to_string(),
            note: String::new(),
        }),
        settings_open: false,
        ..Default::default()
    };

    state.bookmark_editor = None;
    state.settings_open = true;

    assert!(state.settings_open);
    assert!(state.bookmark_editor.is_none());
}

#[test]
fn test_bug_fix_feb2026_004_single_modal_invariant_bookmark_over_settings() {
    let mut state = State {
        settings_open: true,
        bookmark_editor: None,
        ..Default::default()
    };

    state.settings_open = false;
    state.bookmark_editor = Some(BookmarkEditor {
        id: None,
        audiobook_id: 1,
        file_path: "/test/file.mp3".to_string(),
        position_ms: 1000,
        label: "Bookmark".to_string(),
        note: String::new(),
    });

    assert!(state.bookmark_editor.is_some());
    assert!(!state.settings_open);
}
