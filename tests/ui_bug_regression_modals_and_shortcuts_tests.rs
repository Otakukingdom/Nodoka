//! Regression tests for UI bugs discovered during systematic testing.
//!
//! Tests in this file focus on modal invariants and keyboard shortcut behavior.

use nodoka::models::AudiobookFile;
use nodoka::ui::{BookmarkEditor, PlaybackStatus, ScanState, State};

mod acceptance_support;
use acceptance_support::{create_test_audiobook, create_test_db, insert_test_file};

/// Bug #0005: Modal state prevents multiple modals simultaneously
///
/// Scenario: Settings modal is open, user triggers bookmark editor.
///
/// Expected: Only one modal should be open at a time.
#[test]
fn test_bug_0005_single_modal_invariant() {
    let db = create_test_db().expect("create test db");
    let audiobook_id = create_test_audiobook(&db, "/test", "Book").expect("create audiobook");
    let file_path = "/test/Book/ch1.mp3";
    insert_test_file(&db, audiobook_id, file_path).expect("insert file");

    let mut state = State {
        settings_open: true,
        selected_audiobook: Some(audiobook_id),
        selected_file: Some(file_path.to_string()),
        current_time: 1.0,
        ..Default::default()
    };
    let mut player = None;

    let _ = nodoka::ui::update::update(
        &mut state,
        nodoka::ui::Message::CreateBookmark,
        &mut player,
        &db,
    );

    assert!(
        !state.settings_open,
        "CreateBookmark should close settings to keep a single modal"
    );
    assert!(
        state.bookmark_editor.is_some(),
        "CreateBookmark should open the bookmark editor"
    );
}

/// Bug #0016: Escape key closes correct modal
#[test]
fn test_bug_0016_escape_closes_topmost_modal() {
    let db = create_test_db().expect("create test db");
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
    let mut player = None;

    let _ = nodoka::ui::update::update(
        &mut state,
        nodoka::ui::Message::CloseModal,
        &mut player,
        &db,
    );

    assert!(
        state.bookmark_editor.is_none(),
        "Escape should close the editor"
    );
}

/// Bug #0020: Keyboard shortcuts respect modal state
#[test]
fn test_bug_0020_keyboard_shortcuts_respect_modal_state() {
    let db = create_test_db().expect("create test db");
    let mut state = State {
        settings_open: true,
        playback: PlaybackStatus::Playing,
        current_time: 123.0,
        ..Default::default()
    };
    let mut player = None;

    let _ =
        nodoka::ui::update::update(&mut state, nodoka::ui::Message::PlayPause, &mut player, &db);

    assert_eq!(
        state.playback,
        PlaybackStatus::Playing,
        "PlayPause shortcut should be ignored while settings modal is open"
    );
    assert!(
        (state.current_time - 123.0).abs() < f64::EPSILON,
        "Shortcut should not mutate unrelated state"
    );
}

/// Bug #0021: Multiple modals cannot be open simultaneously
#[test]
fn test_bug_0021_single_modal_invariant() {
    let db = create_test_db().expect("create test db");
    let audiobook_id = create_test_audiobook(&db, "/test", "Book").expect("create audiobook");
    let file_path = "/test/Book/ch1.mp3";
    insert_test_file(&db, audiobook_id, file_path).expect("insert file");

    let mut state = State {
        settings_open: true,
        selected_audiobook: Some(audiobook_id),
        selected_file: Some(file_path.to_string()),
        current_time: 1.0,
        ..Default::default()
    };
    let mut player = None;

    let _ = nodoka::ui::update::update(
        &mut state,
        nodoka::ui::Message::CreateBookmark,
        &mut player,
        &db,
    );

    assert!(!state.settings_open);
    assert!(state.bookmark_editor.is_some());

    let _ = nodoka::ui::update::update(
        &mut state,
        nodoka::ui::Message::OpenSettings,
        &mut player,
        &db,
    );

    assert!(state.settings_open);
    assert!(state.bookmark_editor.is_none());
}

/// Bug #0028: Play/pause shortcut (Space) blocked when modal open
#[test]
fn test_bug_0028_play_pause_blocked_when_modal_open() {
    let db = create_test_db().expect("create test db");
    let mut player = None;

    let mut state = State {
        settings_open: true,
        playback: PlaybackStatus::Playing,
        ..Default::default()
    };

    let _ =
        nodoka::ui::update::update(&mut state, nodoka::ui::Message::PlayPause, &mut player, &db);

    assert_eq!(
        state.playback,
        PlaybackStatus::Playing,
        "PlayPause should be blocked when settings is open"
    );

    let mut state = State {
        bookmark_editor: Some(BookmarkEditor {
            id: None,
            audiobook_id: 1,
            file_path: "/test/file.mp3".to_string(),
            position_ms: 1000,
            label: "Test".to_string(),
            note: String::new(),
        }),
        playback: PlaybackStatus::Playing,
        ..Default::default()
    };

    let _ =
        nodoka::ui::update::update(&mut state, nodoka::ui::Message::PlayPause, &mut player, &db);

    assert_eq!(
        state.playback,
        PlaybackStatus::Playing,
        "PlayPause should be blocked when bookmark editor is open"
    );
}

/// Bug #0029: Seek shortcuts blocked when modal open
#[test]
fn test_bug_0029_seek_blocked_when_modal_open() {
    let db = create_test_db().expect("create test db");
    let mut player = None;

    let mut state = State {
        settings_open: true,
        current_time: 100.0,
        total_duration: 3600.0,
        ..Default::default()
    };
    let _ = nodoka::ui::update::update(
        &mut state,
        nodoka::ui::Message::SeekForward(5),
        &mut player,
        &db,
    );
    assert!((state.current_time - 100.0).abs() < f64::EPSILON);

    let mut state = State {
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
    let _ = nodoka::ui::update::update(
        &mut state,
        nodoka::ui::Message::SeekBackward(5),
        &mut player,
        &db,
    );
    assert!((state.current_time - 100.0).abs() < f64::EPSILON);
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

    let db = create_test_db().expect("create test db");
    let mut player = None;

    let mut state = State {
        settings_open: true,
        selected_file: Some("/test/file1.mp3".to_string()),
        current_files: files.clone(),
        ..Default::default()
    };
    let _ = nodoka::ui::update::update(&mut state, nodoka::ui::Message::NextFile, &mut player, &db);
    assert_eq!(state.selected_file.as_deref(), Some("/test/file1.mp3"));

    let mut state = State {
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

    let _ = nodoka::ui::update::update(
        &mut state,
        nodoka::ui::Message::PreviousFile,
        &mut player,
        &db,
    );
    assert_eq!(state.selected_file.as_deref(), Some("/test/file1.mp3"));
}

/// Bug #0038: Rapid modal open/close cycles
#[test]
fn test_bug_0038_rapid_modal_toggle() {
    let db = create_test_db().expect("create test db");
    let mut player = None;
    let mut state = State::default();

    for _ in 0..10 {
        let _ = nodoka::ui::update::update(
            &mut state,
            nodoka::ui::Message::OpenSettings,
            &mut player,
            &db,
        );
        assert!(state.settings_open);

        let _ = nodoka::ui::update::update(
            &mut state,
            nodoka::ui::Message::CloseSettings,
            &mut player,
            &db,
        );
        assert!(!state.settings_open);
    }
}

/// Bug #0051: Settings modal opened while scanning
#[test]
fn test_bug_0051_modal_during_scanning() {
    let db = create_test_db().expect("create test db");
    let mut player = None;
    let mut state = State {
        scan_state: ScanState::Scanning {
            directory: Some("/path/to/audiobooks".to_string()),
        },
        settings_open: false,
        ..Default::default()
    };

    let _ = nodoka::ui::update::update(
        &mut state,
        nodoka::ui::Message::OpenSettings,
        &mut player,
        &db,
    );

    assert!(matches!(state.scan_state, ScanState::Scanning { .. }));
    assert!(state.settings_open);
}

/// Bug FIX #004 (Feb 2026): Single modal invariant enforcement
#[test]
fn test_bug_fix_feb2026_004_single_modal_invariant_settings_over_bookmark() {
    let db = create_test_db().expect("create test db");
    let mut player = None;
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

    let _ = nodoka::ui::update::update(
        &mut state,
        nodoka::ui::Message::OpenSettings,
        &mut player,
        &db,
    );

    assert!(state.settings_open);
    assert!(state.bookmark_editor.is_none());
}

#[test]
fn test_bug_fix_feb2026_004_single_modal_invariant_bookmark_over_settings() {
    let db = create_test_db().expect("create test db");
    let audiobook_id = create_test_audiobook(&db, "/test", "Book").expect("create audiobook");
    let file_path = "/test/Book/ch1.mp3";
    insert_test_file(&db, audiobook_id, file_path).expect("insert file");

    let mut player = None;
    let mut state = State {
        settings_open: true,
        selected_audiobook: Some(audiobook_id),
        selected_file: Some(file_path.to_string()),
        current_time: 1.0,
        ..Default::default()
    };

    let _ = nodoka::ui::update::update(
        &mut state,
        nodoka::ui::Message::CreateBookmark,
        &mut player,
        &db,
    );

    assert!(state.bookmark_editor.is_some());
    assert!(!state.settings_open);
}
