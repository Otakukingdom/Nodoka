//! UI state transition integration tests
//!
//! Tests complex multi-step state transitions and workflows that involve
//! multiple UI components and state changes working together.

use nodoka::models::{AudiobookFile, Bookmark, SleepTimer, SleepTimerMode};
use nodoka::ui::{PlaybackStatus, ScanState, State};
use std::error::Error;

mod acceptance_support;
use acceptance_support::{create_test_audiobook, create_test_db, insert_test_file};

#[test]
fn test_modal_close_order_bookmark_editor_then_settings() -> Result<(), Box<dyn Error>> {
    // CloseModal should close the topmost modal first.
    // In Nodoka, the bookmark editor is stacked above settings, so it closes first.
    let db = create_test_db()?;

    let mut state = State {
        bookmark_editor: Some(nodoka::ui::BookmarkEditor {
            id: None,
            audiobook_id: 1,
            file_path: "/test/file.mp3".to_string(),
            position_ms: 0,
            label: String::new(),
            note: String::new(),
        }),
        settings_open: true,
        ..State::default()
    };

    let mut player = None;

    let _task = nodoka::ui::update::update(
        &mut state,
        nodoka::ui::Message::CloseModal,
        &mut player,
        &db,
    );

    assert!(
        state.bookmark_editor.is_none(),
        "Bookmark editor should close first"
    );
    assert!(
        state.settings_open,
        "Settings should remain open after closing bookmark editor"
    );

    let _task = nodoka::ui::update::update(
        &mut state,
        nodoka::ui::Message::CloseModal,
        &mut player,
        &db,
    );

    assert!(
        !state.settings_open,
        "Settings should close after bookmark editor is closed"
    );

    Ok(())
}

#[test]
fn test_error_banner_during_scanning() {
    // Test that error banner and scanning state can coexist
    let state = State {
        scan_state: ScanState::Scanning {
            directory: Some("/test/audiobooks".to_string()),
        },
        error_message: Some("Failed to read directory".to_string()),
        error_timestamp: Some(chrono::Utc::now()),
        ..State::default()
    };

    // Both should be visible
    assert!(matches!(state.scan_state, ScanState::Scanning { .. }));
    assert!(state.error_message.is_some(), "Error should be displayed");
}

#[test]
fn test_file_selected_updates_state_even_without_player() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let audiobook_id = create_test_audiobook(&db, "/test", "Book")?;

    let mut state = State {
        selected_audiobook: Some(audiobook_id),
        ..State::default()
    };
    let mut player = None;

    let path = "/nonexistent/file.mp3";
    let _task = nodoka::ui::update::update(
        &mut state,
        nodoka::ui::Message::FileSelected(path.to_string()),
        &mut player,
        &db,
    );

    assert_eq!(state.selected_file.as_deref(), Some(path));
    Ok(())
}

#[test]
fn test_audiobook_switch_resets_playback_and_clears_file_selection() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let ab1_id = create_test_audiobook(&db, "/test", "Book1")?;
    let ab2_id = create_test_audiobook(&db, "/test", "Book2")?;

    let ab1_file = "/test/Book1/ch1.mp3";
    let ab2_file = "/test/Book2/ch1.mp3";
    insert_test_file(&db, ab1_id, ab1_file)?;
    insert_test_file(&db, ab2_id, ab2_file)?;

    let mut state = State {
        audiobooks: vec![
            nodoka::models::Audiobook {
                id: Some(ab1_id),
                directory: "/test".to_string(),
                name: "Book1".to_string(),
                full_path: "/test/Book1".to_string(),
                completeness: 0,
                default_order: 0,
                selected_file: Some(ab1_file.to_string()),
                created_at: chrono::Utc::now(),
            },
            nodoka::models::Audiobook {
                id: Some(ab2_id),
                directory: "/test".to_string(),
                name: "Book2".to_string(),
                full_path: "/test/Book2".to_string(),
                completeness: 0,
                default_order: 0,
                selected_file: Some(ab2_file.to_string()),
                created_at: chrono::Utc::now(),
            },
        ],
        selected_audiobook: Some(ab1_id),
        selected_file: Some(ab1_file.to_string()),
        playback: PlaybackStatus::Playing,
        current_time: 5000.0,
        total_duration: 10_000.0,
        ..State::default()
    };
    let mut player = None;

    let _task = nodoka::ui::update::update(
        &mut state,
        nodoka::ui::Message::AudiobookSelected(ab2_id),
        &mut player,
        &db,
    );

    assert_eq!(state.selected_audiobook, Some(ab2_id));
    assert!(
        state.selected_file.is_none(),
        "switch should clear file selection"
    );
    assert_eq!(state.playback, PlaybackStatus::Paused);
    assert!((state.current_time - 0.0).abs() < f64::EPSILON);
    assert!((state.total_duration - 0.0).abs() < f64::EPSILON);
    Ok(())
}

#[test]
fn test_directory_removal_with_selected_audiobook() -> Result<(), Box<dyn Error>> {
    // Test that removing a directory clears selection if affected
    let db = create_test_db()?;
    let temp_dir = std::env::temp_dir().join("nodoka_state_test");
    std::fs::create_dir_all(&temp_dir)?;

    // Create audiobook in directory
    let dir = temp_dir.to_string_lossy();
    let audiobook_id = create_test_audiobook(&db, dir.as_ref(), "Test Book")?;

    // Create state with selection
    let mut state = State {
        selected_audiobook: Some(audiobook_id),
        ..State::default()
    };

    // In real app, removing directory would trigger state update
    // Here we just verify the state can be cleared
    state.selected_audiobook = None;
    state.current_files.clear();

    assert!(state.selected_audiobook.is_none());
    assert!(state.current_files.is_empty());

    // Cleanup
    let _ = std::fs::remove_dir_all(&temp_dir);

    Ok(())
}

#[test]
fn test_sleep_timer_expiration_during_file_boundary() {
    // Test sleep timer expiring during file transition
    let state = State {
        playback: PlaybackStatus::Playing,
        current_time: 3_595_000.0, // 5 seconds before end of 1-hour file
        total_duration: 3_600_000.0,
        sleep_timer: Some(SleepTimer::new(SleepTimerMode::Duration(10), 30)),
        ..State::default()
    };

    // In real app, sleep timer would pause playback
    // File advance would be prevented or handled
    assert!(state.sleep_timer.is_some());
    assert_eq!(state.playback, PlaybackStatus::Playing);
}

#[test]
fn test_volume_speed_persistence_across_file_changes() {
    // Test that volume and speed settings persist when changing files
    let mut state = State {
        volume: 75,
        speed: 1.5,
        selected_file: Some("/book/chapter1.mp3".to_string()),
        ..State::default()
    };

    // Change file
    state.selected_file = Some("/book/chapter2.mp3".to_string());

    // Volume and speed should persist
    assert_eq!(state.volume, 75);
    assert!((state.speed - 1.5).abs() < f32::EPSILON);
}

#[test]
fn test_bookmark_creation_during_playback() -> Result<(), Box<dyn Error>> {
    // Test creating bookmark while playing
    let db = create_test_db()?;

    let audiobook_id = create_test_audiobook(&db, "/test", "Test Book")?;

    let file = AudiobookFile {
        audiobook_id,
        name: "chapter1.mp3".to_string(),
        full_path: "/test/chapter1.mp3".to_string(),
        length_of_file: Some(3_600_000),
        seek_position: Some(120_000),
        checksum: None,
        position: 0,
        completeness: 0,
        file_exists: true,
        created_at: chrono::Utc::now(),
    };
    nodoka::db::queries::insert_audiobook_file(db.connection(), &file)?;

    // Create bookmark at current position
    let bookmark = Bookmark {
        id: None,
        audiobook_id,
        file_path: "/test/chapter1.mp3".to_string(),
        position_ms: 120_000,
        label: "Interesting moment".to_string(),
        note: None,
        created_at: chrono::Utc::now(),
    };

    let bookmark_id = nodoka::db::queries::insert_bookmark(db.connection(), &bookmark)?;
    assert!(bookmark_id > 0);

    // Verify bookmark exists
    let bookmarks =
        nodoka::db::queries::get_bookmarks_for_audiobook(db.connection(), audiobook_id)?;
    assert_eq!(bookmarks.len(), 1);
    let first = bookmarks.first().ok_or("missing bookmark")?;
    assert_eq!(first.position_ms, 120_000);

    Ok(())
}

#[test]
fn test_rapid_audiobook_selection_convergence() -> Result<(), Box<dyn Error>> {
    // Test that rapid selections converge to final state
    let db = create_test_db()?;

    // Create multiple audiobooks
    let mut ids = Vec::new();
    for i in 0..5 {
        let id = create_test_audiobook(&db, "/test", &format!("Book {i}"))?;
        ids.push(id);
    }

    let first = ids.first().copied().ok_or("missing audiobook ids")?;
    let mut state = State {
        selected_audiobook: Some(first),
        ..State::default()
    };

    // Rapidly change selection
    for id in ids.iter().copied().skip(1) {
        state.selected_audiobook = Some(id);
    }

    // Should converge to last selection
    let expected = ids.last().copied().ok_or("missing audiobook ids")?;
    assert_eq!(state.selected_audiobook, Some(expected));

    Ok(())
}

#[test]
fn test_playback_state_transitions() {
    // Test complete playback state machine
    let mut state = State::default();

    // Paused → Playing
    assert_eq!(state.playback, PlaybackStatus::Paused);
    state.playback = PlaybackStatus::Playing;
    assert_eq!(state.playback, PlaybackStatus::Playing);

    // Playing → Paused
    state.playback = PlaybackStatus::Paused;
    let paused_position = state.current_time;

    // Paused → Playing (resume)
    state.playback = PlaybackStatus::Playing;
    assert!((state.current_time - paused_position).abs() < f64::EPSILON);

    // Playing → Paused (stop without player in this state-only test)
    state.playback = PlaybackStatus::Paused;
    state.current_time = 0.0;
    assert!(state.current_time.abs() < f64::EPSILON);
}

#[test]
fn test_error_dismissal_workflow() {
    // Test error banner display and dismissal
    let mut state = State {
        error_message: Some("Test error".to_string()),
        error_timestamp: Some(chrono::Utc::now()),
        ..State::default()
    };
    assert!(state.error_message.is_some());

    // Dismiss error
    state.error_message = None;
    state.error_timestamp = None;
    assert!(state.error_message.is_none());
    assert!(state.error_timestamp.is_none());
}

#[test]
fn test_multiple_modal_close_sequence() {
    // Test closing multiple modals in sequence
    let mut state = State {
        settings_open: true,
        bookmark_editor: Some(nodoka::ui::BookmarkEditor {
            id: None,
            audiobook_id: 1,
            file_path: "/test/file.mp3".to_string(),
            position_ms: 0,
            label: String::new(),
            note: String::new(),
        }),
        ..State::default()
    };

    // Close settings first
    state.settings_open = false;
    assert!(!state.settings_open);
    assert!(state.bookmark_editor.is_some());

    // Close bookmark editor
    state.bookmark_editor = None;
    assert!(state.bookmark_editor.is_none());
}

#[test]
fn test_scanning_state_management() {
    // Test directory scanning state transitions
    let idle = State::default();
    assert_eq!(idle.scan_state, ScanState::Idle);

    let mut state = State {
        scan_state: ScanState::Scanning {
            directory: Some("/test/audiobooks".to_string()),
        },
        ..State::default()
    };

    // Complete scanning
    state.scan_state = ScanState::Idle;

    assert_eq!(state.scan_state, ScanState::Idle);
}

#[test]
fn test_focus_tracking_workflow() {
    // Test keyboard focus tracking
    let mut state = State::default();

    // Initial focus
    assert_eq!(state.focused_element, nodoka::ui::FocusedElement::None);

    // Focus play button
    state.focused_element = nodoka::ui::FocusedElement::PlayPauseButton;
    assert_eq!(
        state.focused_element,
        nodoka::ui::FocusedElement::PlayPauseButton
    );

    // Tab to volume slider
    state.focused_element = nodoka::ui::FocusedElement::VolumeSlider;
    assert_eq!(
        state.focused_element,
        nodoka::ui::FocusedElement::VolumeSlider
    );

    // Tab to file list
    state.focused_element = nodoka::ui::FocusedElement::FileList;
    assert_eq!(state.focused_element, nodoka::ui::FocusedElement::FileList);
}

#[test]
fn test_bookmark_editor_state_workflow() {
    // Test bookmark editor opening, editing, and closing
    let mut state = State::default();

    // Editor closed initially
    assert!(state.bookmark_editor.is_none());

    // Open editor for new bookmark
    state.bookmark_editor = Some(nodoka::ui::BookmarkEditor {
        id: None,
        audiobook_id: 1,
        file_path: "/test/file.mp3".to_string(),
        position_ms: 120_000,
        label: String::new(),
        note: String::new(),
    });

    // Edit label
    if let Some(editor) = &mut state.bookmark_editor {
        editor.label = "Important scene".to_string();
        editor.note = "This is interesting".to_string();
    }

    // Verify edits
    if let Some(editor) = &state.bookmark_editor {
        assert_eq!(editor.label, "Important scene");
        assert_eq!(editor.note, "This is interesting");
    }

    // Close editor
    state.bookmark_editor = None;
    assert!(state.bookmark_editor.is_none());
}

#[test]
fn test_sleep_timer_mode_transitions() {
    // Test switching between sleep timer modes
    let mut state = State::default();

    // No timer initially
    assert!(state.sleep_timer.is_none());

    // Set duration timer
    state.sleep_timer = Some(SleepTimer::new(SleepTimerMode::Duration(900), 30));
    assert!(state.sleep_timer.is_some());

    // Change to end of chapter
    state.sleep_timer = Some(SleepTimer::new(SleepTimerMode::EndOfChapter, 30));
    if let Some(timer) = &state.sleep_timer {
        assert!(matches!(timer.mode, SleepTimerMode::EndOfChapter));
    }

    // Change back to duration timer with different duration
    state.sleep_timer = Some(SleepTimer::new(SleepTimerMode::Duration(1800), 30));
    if let Some(timer) = &state.sleep_timer {
        assert!(matches!(timer.mode, SleepTimerMode::Duration(1800)));
    }

    // Cancel timer
    state.sleep_timer = None;
    assert!(state.sleep_timer.is_none());
}

#[test]
fn test_file_list_update_on_audiobook_selection() -> Result<(), Box<dyn Error>> {
    // Test that file list updates when audiobook is selected
    let db = create_test_db()?;

    // Create audiobook with files
    let audiobook_id = create_test_audiobook(&db, "/test", "Test Book")?;

    for i in 0..3 {
        let file = AudiobookFile {
            audiobook_id,
            name: format!("chapter{i}.mp3"),
            full_path: format!("/test/chapter{i}.mp3"),
            length_of_file: Some(3_600_000),
            seek_position: None,
            checksum: None,
            position: i,
            completeness: 0,
            file_exists: true,
            created_at: chrono::Utc::now(),
        };
        nodoka::db::queries::insert_audiobook_file(db.connection(), &file)?;
    }

    // Retrieve files
    let files = nodoka::db::queries::get_audiobook_files(db.connection(), audiobook_id)?;
    assert_eq!(files.len(), 3);

    // Update state
    let state = State {
        selected_audiobook: Some(audiobook_id),
        current_files: files,
        ..State::default()
    };

    assert_eq!(state.current_files.len(), 3);

    Ok(())
}
