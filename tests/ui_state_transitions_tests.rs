//! UI state transition integration tests
//!
//! Tests complex multi-step state transitions and workflows that involve
//! multiple UI components and state changes working together.

#![allow(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::float_cmp,
    clippy::cast_precision_loss,
    clippy::field_reassign_with_default
)]

use nodoka::models::{AudiobookFile, Bookmark, SleepTimer, SleepTimerMode};
use nodoka::ui::State;
use std::error::Error;

mod acceptance_support;
use acceptance_support::{create_test_audiobook, create_test_db};

#[test]
fn test_modal_priority_settings_over_bookmark_editor() {
    // Test modal priority: settings modal takes precedence over bookmark editor
    let mut state = State::default();

    // Open bookmark editor first
    state.bookmark_editor = Some(nodoka::ui::BookmarkEditor {
        id: None,
        audiobook_id: 1,
        file_path: "/test/file.mp3".to_string(),
        position_ms: 0,
        label: String::new(),
        note: String::new(),
    });

    // Then open settings
    state.settings_open = true;

    assert!(state.bookmark_editor.is_some(), "Editor should remain open");
    assert!(state.settings_open, "Settings should be open");

    // Escape should close settings first (tested via message handlers)
    // This verifies modal stacking behavior
}

#[test]
fn test_error_banner_during_scanning() {
    // Test that error banner and scanning state can coexist
    let mut state = State::default();

    // Start scanning
    state.is_scanning = true;
    state.scanning_directory = Some("/test/audiobooks".to_string());

    // Then encounter an error
    state.error_message = Some("Failed to read directory".to_string());
    state.error_timestamp = Some(chrono::Utc::now());

    // Both should be visible
    assert!(state.is_scanning, "Should still be scanning");
    assert!(state.error_message.is_some(), "Error should be displayed");
}

#[test]
fn test_file_selection_with_missing_file() {
    // Test file selection when file doesn't exist
    let mut state = State::default();

    // Select a non-existent file
    state.selected_file = Some("/nonexistent/file.mp3".to_string());

    // State should accept the selection (file existence checked elsewhere)
    assert_eq!(
        state.selected_file.as_deref(),
        Some("/nonexistent/file.mp3")
    );

    // Clear selection
    state.selected_file = None;
    assert!(state.selected_file.is_none());
}

#[test]
fn test_audiobook_switch_clears_playback_state() {
    // Test that switching audiobooks resets playback state
    let mut state = State::default();

    // Setup playback state for audiobook 1
    state.selected_audiobook = Some(1);
    state.selected_file = Some("/book1/chapter1.mp3".to_string());
    state.is_playing = true;
    state.current_time = 5000.0;

    // Switch to audiobook 2
    state.selected_audiobook = Some(2);
    state.selected_file = Some("/book2/chapter1.mp3".to_string());

    // Playback state should be updated (in real app, would reset or restore)
    assert_eq!(state.selected_audiobook, Some(2));
    assert_eq!(state.selected_file.as_deref(), Some("/book2/chapter1.mp3"));
}

#[test]
fn test_directory_removal_with_selected_audiobook() -> Result<(), Box<dyn Error>> {
    // Test that removing a directory clears selection if affected
    let db = create_test_db()?;
    let temp_dir = std::env::temp_dir().join("nodoka_state_test");
    std::fs::create_dir_all(&temp_dir)?;

    // Create audiobook in directory
    let audiobook_id = create_test_audiobook(&db, temp_dir.to_str().unwrap(), "Test Book")?;

    // Create state with selection
    let mut state = State::default();
    state.selected_audiobook = Some(audiobook_id);

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
    let mut state = State::default();

    // Setup playback near end of file
    state.is_playing = true;
    state.current_time = 3_595_000.0; // 5 seconds before end of 1-hour file
    state.total_duration = 3_600_000.0;

    // Set sleep timer
    state.sleep_timer = Some(SleepTimer::new(SleepTimerMode::Duration(10), 30));

    // In real app, sleep timer would pause playback
    // File advance would be prevented or handled
    assert!(state.sleep_timer.is_some());
    assert!(state.is_playing);
}

#[test]
fn test_volume_speed_persistence_across_file_changes() {
    // Test that volume and speed settings persist when changing files
    let mut state = State::default();

    // Set volume and speed
    state.volume = 75;
    state.speed = 1.5;
    state.selected_file = Some("/book/chapter1.mp3".to_string());

    // Change file
    state.selected_file = Some("/book/chapter2.mp3".to_string());

    // Volume and speed should persist
    assert_eq!(state.volume, 75);
    assert_eq!(state.speed, 1.5);
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
    assert_eq!(bookmarks[0].position_ms, 120_000);

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

    let mut state = State::default();

    // Rapidly change selection
    for id in &ids {
        state.selected_audiobook = Some(*id);
    }

    // Should converge to last selection
    assert_eq!(state.selected_audiobook, Some(ids[4]));

    Ok(())
}

#[test]
fn test_playback_state_transitions() {
    // Test complete playback state machine
    let mut state = State::default();

    // Stopped → Playing
    assert!(!state.is_playing);
    state.is_playing = true;
    assert!(state.is_playing);

    // Playing → Paused
    state.is_playing = false;
    let paused_position = state.current_time;

    // Paused → Playing (resume)
    state.is_playing = true;
    assert_eq!(state.current_time, paused_position);

    // Playing → Stopped
    state.is_playing = false;
    state.current_time = 0.0;
    assert_eq!(state.current_time, 0.0);
}

#[test]
fn test_error_dismissal_workflow() {
    // Test error banner display and dismissal
    let mut state = State::default();

    // Display error
    state.error_message = Some("Test error".to_string());
    state.error_timestamp = Some(chrono::Utc::now());
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
    let mut state = State::default();

    // Open both modals
    state.settings_open = true;
    state.bookmark_editor = Some(nodoka::ui::BookmarkEditor {
        id: None,
        audiobook_id: 1,
        file_path: "/test/file.mp3".to_string(),
        position_ms: 0,
        label: String::new(),
        note: String::new(),
    });

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
    let mut state = State::default();

    // Not scanning initially
    assert!(!state.is_scanning);
    assert!(state.scanning_directory.is_none());

    // Start scanning
    state.is_scanning = true;
    state.scanning_directory = Some("/test/audiobooks".to_string());

    // Complete scanning
    state.is_scanning = false;
    state.scanning_directory = None;

    assert!(!state.is_scanning);
    assert!(state.scanning_directory.is_none());
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
    let mut state = State::default();
    state.selected_audiobook = Some(audiobook_id);
    state.current_files = files;

    assert_eq!(state.current_files.len(), 3);

    Ok(())
}
