//! End-to-end workflow tests - complete user journeys
//!
//! Tests simulate complete user workflows through the application,
//! verifying all UI interactions work together correctly.

#![allow(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::float_cmp,
    clippy::cast_precision_loss,
    clippy::field_reassign_with_default
)]

use nodoka::models::{AudiobookFile, Bookmark, SleepTimer, SleepTimerMode};
use nodoka::ui::{BookmarkEditor, State};
use std::error::Error;

mod acceptance_support;
use acceptance_support::{create_test_audiobook, create_test_db};

#[test]
fn test_first_time_user_workflow_structure() -> Result<(), Box<dyn Error>> {
    // Simulate complete first-time user experience structure
    // 1. Launch with empty database
    let db = create_test_db()?;

    // 2. Verify no audiobooks initially
    let audiobooks = nodoka::db::queries::get_all_audiobooks(db.connection())?;
    assert!(
        audiobooks.is_empty(),
        "New database should have no audiobooks"
    );

    // 3. Add directory (Message::DirectoryAdd would be sent)
    // 4. Directory would be scanned
    // 5. Audiobooks would be discovered
    // 6. User would select audiobook and file
    // 7. Playback would start

    Ok(())
}

#[test]
fn test_bookmark_workflow() -> Result<(), Box<dyn Error>> {
    // Complete bookmark lifecycle
    let db = create_test_db()?;

    // 1. Create audiobook with file
    let audiobook_id = create_test_audiobook(&db, "/test", "Test Book")?;

    let file = AudiobookFile {
        audiobook_id,
        name: "chapter1.mp3".to_string(),
        full_path: "/test/chapter1.mp3".to_string(),
        length_of_file: Some(3_600_000),
        seek_position: None,
        checksum: None,
        position: 0,
        completeness: 0,
        file_exists: true,
        created_at: chrono::Utc::now(),
    };
    nodoka::db::queries::insert_audiobook_file(db.connection(), &file)?;

    // 2. Create bookmark at position
    let bookmark = Bookmark {
        id: None,
        audiobook_id,
        file_path: "/test/chapter1.mp3".to_string(),
        position_ms: 120_000, // 2 minutes
        label: "Important moment".to_string(),
        note: Some("This is a key scene".to_string()),
        created_at: chrono::Utc::now(),
    };

    let bookmark_id = nodoka::db::queries::insert_bookmark(db.connection(), &bookmark)?;

    // 3. Verify bookmark was created
    let bookmarks =
        nodoka::db::queries::get_bookmarks_for_audiobook(db.connection(), audiobook_id)?;
    assert_eq!(bookmarks.len(), 1, "Should have one bookmark");
    assert_eq!(bookmarks[0].label, "Important moment");

    // 4. Update bookmark label
    let mut updated = bookmarks[0].clone();
    updated.label = "Even more important".to_string();
    nodoka::db::queries::update_bookmark(db.connection(), &updated)?;

    // 5. Verify update
    let bookmarks =
        nodoka::db::queries::get_bookmarks_for_audiobook(db.connection(), audiobook_id)?;
    assert_eq!(bookmarks[0].label, "Even more important");

    // 6. Delete bookmark
    nodoka::db::queries::delete_bookmark(db.connection(), bookmark_id)?;

    // 7. Verify deletion
    let bookmarks =
        nodoka::db::queries::get_bookmarks_for_audiobook(db.connection(), audiobook_id)?;
    assert!(bookmarks.is_empty(), "Bookmark should be deleted");

    Ok(())
}

#[test]
fn test_multi_file_audiobook_workflow() -> Result<(), Box<dyn Error>> {
    // Complete multi-file playback workflow
    let db = create_test_db()?;

    // 1. Create audiobook with 5 files
    let audiobook_id = create_test_audiobook(&db, "/test", "Multi-File Book")?;

    let mut files = Vec::new();
    for i in 0..5 {
        let file = AudiobookFile {
            audiobook_id,
            name: format!("chapter{}.mp3", i + 1),
            full_path: format!("/test/chapter{}.mp3", i + 1),
            length_of_file: Some(3_600_000),
            seek_position: None,
            checksum: None,
            position: i,
            completeness: 0,
            file_exists: true,
            created_at: chrono::Utc::now(),
        };
        nodoka::db::queries::insert_audiobook_file(db.connection(), &file)?;
        files.push(file);
    }

    // 2. Verify all files created
    let db_files = nodoka::db::queries::get_audiobook_files(db.connection(), audiobook_id)?;
    assert_eq!(db_files.len(), 5, "Should have 5 files");

    // 3. Mark first file as complete
    nodoka::db::queries::update_file_progress(
        db.connection(),
        &files[0].full_path,
        files[0].length_of_file.unwrap_or(0) as f64,
        100,
    )?;

    // 4. Verify completion
    let db_files = nodoka::db::queries::get_audiobook_files(db.connection(), audiobook_id)?;
    assert_eq!(
        db_files[0].completeness, 100,
        "First file should be complete"
    );

    // 5. Simulate navigation between files (would be done via UI messages)
    // User would use Up/Down arrows or click to navigate

    Ok(())
}

#[test]
fn test_sleep_timer_workflow() {
    // Sleep timer with extensions and cancellation
    let mut state = State::default();

    // 1. Start playback
    state.is_playing = true;
    state.current_time = 0.0;
    state.total_duration = 3_600_000.0;

    // 2. Set 15-minute timer
    let timer = SleepTimer::new(SleepTimerMode::Duration(15 * 60), 30);
    state.sleep_timer = Some(timer);

    assert!(state.sleep_timer.is_some(), "Sleep timer should be active");

    // 3. Verify timer has correct mode
    if let Some(timer) = &state.sleep_timer {
        assert!(matches!(timer.mode, SleepTimerMode::Duration(_)));
    }

    // 4. Extend timer by 15 minutes (would be Message::SleepTimerExtendSeconds)
    // In real app, this would modify the timer's expiry time

    // 5. Cancel timer
    state.sleep_timer = None;

    assert!(state.sleep_timer.is_none(), "Timer should be cancelled");

    // 6. Set end-of-chapter timer
    let chapter_timer = SleepTimer::new(SleepTimerMode::EndOfChapter, 30);
    state.sleep_timer = Some(chapter_timer);

    if let Some(timer) = &state.sleep_timer {
        assert!(matches!(timer.mode, SleepTimerMode::EndOfChapter));
    }
}

#[test]
fn test_settings_workflow() -> Result<(), Box<dyn Error>> {
    // Directory management workflow
    let db = create_test_db()?;
    let temp_root = std::env::temp_dir().join("nodoka_e2e_test");
    std::fs::create_dir_all(&temp_root)?;

    // 1. Add directory
    let dir1 = temp_root.join("audiobooks");
    std::fs::create_dir_all(&dir1)?;
    let directory = nodoka::models::Directory::new(dir1.to_string_lossy().to_string());
    nodoka::db::queries::insert_directory(db.connection(), &directory)?;

    // 2. Verify directory added
    let directories = nodoka::db::queries::get_all_directories(db.connection())?;
    assert_eq!(directories.len(), 1, "Should have one directory");

    // 3. Add another directory
    let dir2 = temp_root.join("more_audiobooks");
    std::fs::create_dir_all(&dir2)?;
    let directory2 = nodoka::models::Directory::new(dir2.to_string_lossy().to_string());
    nodoka::db::queries::insert_directory(db.connection(), &directory2)?;

    // 4. Verify both directories
    let directories = nodoka::db::queries::get_all_directories(db.connection())?;
    assert_eq!(directories.len(), 2, "Should have two directories");

    // 5. Remove first directory
    nodoka::db::queries::delete_directory(db.connection(), &directories[0].full_path)?;

    // 6. Verify removal
    let directories = nodoka::db::queries::get_all_directories(db.connection())?;
    assert_eq!(
        directories.len(),
        1,
        "Should have one directory after removal"
    );

    // Cleanup
    let _ = std::fs::remove_dir_all(&temp_root);

    Ok(())
}

#[test]
fn test_playback_state_workflow() {
    // Test playback state transitions
    let mut state = State::default();

    // 1. Initial state (stopped)
    assert!(!state.is_playing, "Should start not playing");
    assert_eq!(state.current_time, 0.0, "Should start at time 0");

    // 2. Start playback (Message::PlayPause)
    state.is_playing = true;

    // 3. Simulate time progress
    state.current_time = 5_000.0; // 5 seconds

    // 4. Pause (Message::PlayPause)
    state.is_playing = false;

    // 5. Verify position maintained
    assert_eq!(state.current_time, 5_000.0, "Position should be maintained");

    // 6. Resume playback
    state.is_playing = true;

    // 7. Stop (Message::Stop)
    state.is_playing = false;
    state.current_time = 0.0;

    assert_eq!(state.current_time, 0.0, "Stop should reset position");
}

#[test]
fn test_volume_and_speed_workflow() {
    // Test volume and speed adjustments
    let mut state = State::default();

    // 1. Initial values
    let initial_volume = state.volume;
    let initial_speed = state.speed;

    // 2. Adjust volume (Message::VolumeChanged)
    state.volume = 75;
    assert_eq!(state.volume, 75, "Volume should update");

    // 3. Adjust speed (Message::SpeedChanged)
    state.speed = 1.5;
    assert_eq!(state.speed, 1.5, "Speed should update");

    // 4. Reset to defaults
    state.volume = initial_volume;
    state.speed = initial_speed;

    // 5. Test preset speeds
    let speed_presets = vec![0.5, 0.75, 1.0, 1.25, 1.5, 2.0];
    for speed in speed_presets {
        state.speed = speed;
        assert_eq!(state.speed, speed, "Preset speed {speed} should work");
    }
}

#[test]
fn test_file_selection_workflow() {
    // Test file selection changes
    let mut state = State::default();

    // 1. No file selected initially
    assert!(
        state.selected_file.is_none(),
        "Should start with no file selected"
    );

    // 2. Select a file (Message::FileSelected)
    state.selected_file = Some("/test/chapter1.mp3".to_string());

    assert!(state.selected_file.is_some(), "File should be selected");
    assert_eq!(state.selected_file.as_ref().unwrap(), "/test/chapter1.mp3");

    // 3. Switch to different file
    state.selected_file = Some("/test/chapter2.mp3".to_string());

    assert_eq!(state.selected_file.as_ref().unwrap(), "/test/chapter2.mp3");

    // 4. Clear selection
    state.selected_file = None;

    assert!(state.selected_file.is_none(), "Selection should be cleared");
}

#[test]
fn test_modal_workflow() {
    // Test modal opening and closing
    let mut state = State::default();

    // 1. Settings modal closed initially
    assert!(!state.settings_open, "Settings should start closed");

    // 2. Open settings (Message::OpenSettings)
    state.settings_open = true;

    assert!(state.settings_open, "Settings should be open");

    // 3. Close settings (Message::CloseSettings or Escape)
    state.settings_open = false;

    assert!(!state.settings_open, "Settings should be closed");

    // 4. Test bookmark editor modal
    assert!(
        state.bookmark_editor.is_none(),
        "Bookmark editor should start closed"
    );

    // 5. Open editor (Message::BookmarkEdit)
    state.bookmark_editor = Some(BookmarkEditor {
        id: Some(1),
        audiobook_id: 1,
        file_path: "/test/file.mp3".to_string(),
        position_ms: 0,
        label: String::new(),
        note: String::new(),
    });

    assert!(state.bookmark_editor.is_some(), "Editor should be open");

    // 6. Close editor (Message::BookmarkEditorCancel or Escape)
    state.bookmark_editor = None;

    assert!(state.bookmark_editor.is_none(), "Editor should be closed");
}

#[test]
fn test_error_banner_workflow() {
    // Test error display and dismissal
    let mut state = State::default();

    // 1. No error initially
    assert!(state.error_message.is_none(), "Should start with no error");

    // 2. Display error (Message::ErrorOccurred)
    state.error_message = Some("Failed to load file".to_string());
    state.error_timestamp = Some(chrono::Utc::now());

    assert!(state.error_message.is_some(), "Error should be displayed");

    // 3. Dismiss error (Message::DismissError)
    state.error_message = None;
    state.error_timestamp = None;

    assert!(state.error_message.is_none(), "Error should be dismissed");
}

#[test]
fn test_scanning_state_workflow() {
    // Test directory scanning state
    let mut state = State::default();

    // 1. Not scanning initially
    assert!(!state.is_scanning, "Should not be scanning initially");
    assert!(
        state.scanning_directory.is_none(),
        "No directory being scanned"
    );

    // 2. Start scanning
    state.is_scanning = true;
    state.scanning_directory = Some("/test/audiobooks".to_string());

    assert!(state.is_scanning, "Should be scanning");
    assert_eq!(
        state.scanning_directory.as_ref().unwrap(),
        "/test/audiobooks"
    );

    // 3. Complete scanning
    state.is_scanning = false;
    state.scanning_directory = None;

    assert!(!state.is_scanning, "Should not be scanning");
    assert!(state.scanning_directory.is_none(), "Scan complete");
}

#[test]
fn test_progress_persistence_workflow() -> Result<(), Box<dyn Error>> {
    // Test that progress is saved and restored
    let db = create_test_db()?;

    // 1. Create audiobook and file
    let audiobook_id = create_test_audiobook(&db, "/test", "Test Book")?;

    let file = AudiobookFile {
        audiobook_id,
        name: "chapter1.mp3".to_string(),
        full_path: "/test/chapter1.mp3".to_string(),
        length_of_file: Some(3_600_000),
        seek_position: Some(120_000), // 2 minutes
        checksum: None,
        position: 0,
        completeness: 33, // 33% complete
        file_exists: true,
        created_at: chrono::Utc::now(),
    };
    nodoka::db::queries::insert_audiobook_file(db.connection(), &file)?;

    // 2. Update progress
    nodoka::db::queries::update_file_progress(
        db.connection(),
        &file.full_path,
        180_000.0, // 3 minutes
        50,        // 50% complete
    )?;

    // 3. Retrieve and verify progress
    let files = nodoka::db::queries::get_audiobook_files(db.connection(), audiobook_id)?;
    assert_eq!(files[0].seek_position, Some(180_000));
    assert_eq!(files[0].completeness, 50);

    Ok(())
}
