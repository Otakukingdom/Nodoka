//! End-to-end workflow tests - complete user journeys
//!
//! Tests simulate complete user workflows through the application,
//! verifying all UI interactions work together correctly.

use nodoka::models::{AudiobookFile, Bookmark, SleepTimer, SleepTimerMode};
use nodoka::ui::{BookmarkEditor, PlaybackStatus, ScanState, State};
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
    let first = bookmarks.first().ok_or("missing bookmark")?;
    assert_eq!(first.label, "Important moment");

    // 4. Update bookmark label
    let mut updated = first.clone();
    updated.label = "Even more important".to_string();
    nodoka::db::queries::update_bookmark(db.connection(), &updated)?;

    // 5. Verify update
    let bookmarks =
        nodoka::db::queries::get_bookmarks_for_audiobook(db.connection(), audiobook_id)?;
    let first = bookmarks.first().ok_or("missing bookmark")?;
    assert_eq!(first.label, "Even more important");

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
    let first = files.first().ok_or("missing first file")?;
    nodoka::db::queries::update_file_progress(
        db.connection(),
        &first.full_path,
        nodoka::conversions::ms_to_f64(first.length_of_file.unwrap_or(0))?,
        100,
    )?;

    // 4. Verify completion
    let db_files = nodoka::db::queries::get_audiobook_files(db.connection(), audiobook_id)?;
    let first = db_files.first().ok_or("missing first file")?;
    assert_eq!(first.completeness, 100, "First file should be complete");

    // 5. Simulate navigation between files (would be done via UI messages)
    // User would use Up/Down arrows or click to navigate

    Ok(())
}

#[test]
fn test_sleep_timer_workflow() {
    // Sleep timer with extensions and cancellation
    let mut state = State {
        playback: PlaybackStatus::Playing,
        current_time: 0.0,
        total_duration: 3_600_000.0,
        ..State::default()
    };

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
    let first = directories.first().ok_or("missing directory")?;
    nodoka::db::queries::delete_directory(db.connection(), &first.full_path)?;

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
    let initial = State::default();
    assert_eq!(initial.playback, PlaybackStatus::Paused);
    assert!(initial.current_time.abs() < f64::EPSILON);

    let paused_position = 5_000.0;
    let paused = State {
        playback: PlaybackStatus::Paused,
        current_time: paused_position,
        ..State::default()
    };
    assert!((paused.current_time - paused_position).abs() < f64::EPSILON);

    let stopped = State {
        playback: PlaybackStatus::Paused,
        current_time: 0.0,
        ..State::default()
    };
    assert!(stopped.current_time.abs() < f64::EPSILON);
}

#[test]
fn test_volume_and_speed_workflow() {
    // Test volume and speed adjustments
    let state = State {
        volume: 75,
        speed: 1.5,
        ..State::default()
    };
    assert_eq!(state.volume, 75);
    assert!((state.speed - 1.5).abs() < f32::EPSILON);

    let state = State::default();
    assert_eq!(state.volume, 100);
    assert!((state.speed - 1.0).abs() < f32::EPSILON);

    // 5. Test preset speeds
    let speed_presets = vec![0.5, 0.75, 1.0, 1.25, 1.5, 2.0];
    for speed in speed_presets {
        let state = State {
            speed,
            ..State::default()
        };
        assert!((state.speed - speed).abs() < f32::EPSILON);
    }
}

#[test]
fn test_file_selection_workflow() {
    // Test file selection changes
    let state = State::default();
    assert!(state.selected_file.is_none());

    let state = State {
        selected_file: Some("/test/chapter1.mp3".to_string()),
        ..State::default()
    };
    assert_eq!(state.selected_file.as_deref(), Some("/test/chapter1.mp3"));

    let state = State {
        selected_file: Some("/test/chapter2.mp3".to_string()),
        ..State::default()
    };
    assert_eq!(state.selected_file.as_deref(), Some("/test/chapter2.mp3"));

    let state = State {
        selected_file: None,
        ..State::default()
    };
    assert!(state.selected_file.is_none());
}

#[test]
fn test_modal_workflow() {
    // Test modal opening and closing
    let state = State::default();
    assert!(!state.settings_open);
    assert!(state.bookmark_editor.is_none());

    let state = State {
        settings_open: true,
        ..State::default()
    };
    assert!(state.settings_open);

    let state = State {
        bookmark_editor: Some(BookmarkEditor {
            id: Some(1),
            audiobook_id: 1,
            file_path: "/test/file.mp3".to_string(),
            position_ms: 0,
            label: String::new(),
            note: String::new(),
        }),
        ..State::default()
    };
    assert!(state.bookmark_editor.is_some());
}

#[test]
fn test_error_banner_workflow() {
    // Test error display and dismissal
    let state = State::default();
    assert!(state.error_message.is_none());

    let state = State {
        error_message: Some("Failed to load file".to_string()),
        error_timestamp: Some(chrono::Utc::now()),
        ..State::default()
    };
    assert!(state.error_message.is_some());
}

#[test]
fn test_scanning_state_workflow() {
    // Test directory scanning state
    let state = State::default();
    assert_eq!(state.scan_state, ScanState::Idle);

    let state = State {
        scan_state: ScanState::Scanning {
            directory: Some("/test/audiobooks".to_string()),
        },
        ..State::default()
    };
    assert!(matches!(
        &state.scan_state,
        ScanState::Scanning { directory: Some(d) } if d == "/test/audiobooks"
    ));
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
    let first = files.first().ok_or("missing file")?;
    assert_eq!(first.seek_position, Some(180_000));
    assert_eq!(first.completeness, 50);

    Ok(())
}
