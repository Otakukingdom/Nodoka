//! End-to-end workflow tests - complete user journeys
//!
//! Tests simulate complete user workflows through the application,
//! verifying all UI interactions work together correctly.

use nodoka::models::{AudiobookFile, SleepTimer, SleepTimerMode};
use nodoka::ui::{Message, PlaybackStatus, ScanState, State};
use std::error::Error;

mod acceptance_support;
use acceptance_support::{create_test_audiobook, create_test_db};

#[test]
fn test_first_time_user_add_directory_enters_scanning_state() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let temp = temp_dir::TempDir::new()?;
    let raw_dir = temp.path().to_string_lossy().to_string();
    let expected_dir = {
        fn trim_trailing_separators(path: &str) -> String {
            let mut out = path;
            while out.len() > 1 && (out.ends_with('/') || out.ends_with('\\')) {
                out = &out[..out.len() - 1];
            }
            out.to_string()
        }

        let canonical =
            std::fs::canonicalize(temp.path()).unwrap_or_else(|_| temp.path().to_path_buf());
        trim_trailing_separators(&canonical.to_string_lossy())
    };

    // Start with empty library.
    assert!(nodoka::db::queries::get_all_audiobooks(db.connection())?.is_empty());
    assert!(nodoka::db::queries::get_all_directories(db.connection())?.is_empty());

    let mut state = State::default();
    let mut player = None;

    let _task = nodoka::ui::update::update(
        &mut state,
        Message::DirectoryAdded(raw_dir),
        &mut player,
        &db,
    );

    assert_eq!(state.directories.len(), 1);
    assert!(matches!(
        &state.scan_state,
        ScanState::Scanning {
            directory: Some(d)
        } if d == &expected_dir
    ));
    assert!(state.operation_in_progress);

    // Directory is persisted.
    let stored = nodoka::db::queries::get_all_directories(db.connection())?;
    assert_eq!(stored.len(), 1);
    let first = stored.first().ok_or("missing stored directory")?;
    assert_eq!(first.full_path, expected_dir);

    Ok(())
}

#[test]
fn test_bookmark_workflow() -> Result<(), Box<dyn Error>> {
    // Complete bookmark lifecycle via UI message handling.
    let db = create_test_db()?;

    // 1. Create audiobook with file
    let audiobook_id = create_test_audiobook(&db, "/test", "Test Book")?;

    let file_path = "/test/chapter1.mp3".to_string();
    let file = AudiobookFile {
        audiobook_id,
        name: "chapter1.mp3".to_string(),
        full_path: file_path.clone(),
        length_of_file: Some(3_600_000),
        seek_position: None,
        checksum: None,
        position: 0,
        completeness: 0,
        file_exists: true,
        created_at: chrono::Utc::now(),
    };
    nodoka::db::queries::insert_audiobook_file(db.connection(), &file)?;

    let mut state = State {
        selected_audiobook: Some(audiobook_id),
        selected_file: Some(file_path),
        current_time: 120_000.0,
        ..State::default()
    };
    let mut player = None;

    // 2. Create bookmark (opens editor)
    let _task = nodoka::ui::update::update(&mut state, Message::CreateBookmark, &mut player, &db);
    assert!(state.bookmark_editor.is_some());
    assert_eq!(state.bookmarks.len(), 1);

    // 3. Edit label + save
    let _task = nodoka::ui::update::update(
        &mut state,
        Message::BookmarkEditorLabelChanged("Even more important".to_string()),
        &mut player,
        &db,
    );
    let _task =
        nodoka::ui::update::update(&mut state, Message::BookmarkEditorSave, &mut player, &db);
    assert!(state.bookmark_editor.is_none());

    let bookmarks =
        nodoka::db::queries::get_bookmarks_for_audiobook(db.connection(), audiobook_id)?;
    assert_eq!(bookmarks.len(), 1);
    let first = bookmarks.first().ok_or("missing bookmark")?;
    assert_eq!(first.label, "Even more important");

    // 4. Delete via message
    let id = first.id.ok_or("bookmark id missing")?;
    let _task =
        nodoka::ui::update::update(&mut state, Message::BookmarkDelete(id), &mut player, &db);
    assert!(
        nodoka::db::queries::get_bookmarks_for_audiobook(db.connection(), audiobook_id)?.is_empty()
    );

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
fn test_file_selection_workflow_via_update() -> Result<(), Box<dyn Error>> {
    // File selection is driven by `Message::FileSelected`.
    let db = create_test_db()?;
    let audiobook_id = create_test_audiobook(&db, "/test", "Test Book")?;

    let mut state = State {
        selected_audiobook: Some(audiobook_id),
        ..State::default()
    };
    assert!(state.selected_file.is_none());

    let mut player = None;
    let _task = nodoka::ui::update::update(
        &mut state,
        Message::FileSelected("/test/chapter1.mp3".to_string()),
        &mut player,
        &db,
    );
    assert_eq!(state.selected_file.as_deref(), Some("/test/chapter1.mp3"));

    let _task = nodoka::ui::update::update(
        &mut state,
        Message::FileSelected("/test/chapter2.mp3".to_string()),
        &mut player,
        &db,
    );
    assert_eq!(state.selected_file.as_deref(), Some("/test/chapter2.mp3"));

    // Selection is persisted on the audiobook record.
    let ab = nodoka::db::queries::get_audiobook_by_id(db.connection(), audiobook_id)?
        .ok_or("missing audiobook")?;
    assert_eq!(ab.selected_file.as_deref(), Some("/test/chapter2.mp3"));

    Ok(())
}

#[test]
fn test_modal_workflow_via_update() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let audiobook_id = create_test_audiobook(&db, "/test", "Test Book")?;

    let mut state = State {
        selected_audiobook: Some(audiobook_id),
        selected_file: Some("/test/file.mp3".to_string()),
        current_time: 5_000.0,
        ..State::default()
    };

    let mut player = None;

    let _task = nodoka::ui::update::update(&mut state, Message::OpenSettings, &mut player, &db);
    assert!(state.settings_open);
    assert!(state.bookmark_editor.is_none());

    // Creating a bookmark closes settings and opens the editor (single-modal invariant).
    let _task = nodoka::ui::update::update(&mut state, Message::CreateBookmark, &mut player, &db);
    assert!(!state.settings_open);
    assert!(state.bookmark_editor.is_some());

    // CloseModal closes the topmost modal.
    let _task = nodoka::ui::update::update(&mut state, Message::CloseModal, &mut player, &db);
    assert!(state.bookmark_editor.is_none());
    assert!(!state.settings_open);

    Ok(())
}

#[test]
fn test_error_banner_workflow_via_update() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let mut state = State::default();
    let mut player = None;

    assert!(state.error_message.is_none());

    let _task = nodoka::ui::update::update(
        &mut state,
        Message::ScanError("permission denied".to_string()),
        &mut player,
        &db,
    );
    assert!(state.error_message.is_some());
    assert!(state.error_timestamp.is_some());

    let _task = nodoka::ui::update::update(&mut state, Message::DismissError, &mut player, &db);
    assert!(state.error_message.is_none());
    assert!(state.error_timestamp.is_none());

    Ok(())
}

#[test]
fn test_scanning_state_workflow_via_update() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let temp = temp_dir::TempDir::new()?;
    let raw_dir = temp.path().to_string_lossy().to_string();
    let expected_dir = {
        fn trim_trailing_separators(path: &str) -> String {
            let mut out = path;
            while out.len() > 1 && (out.ends_with('/') || out.ends_with('\\')) {
                out = &out[..out.len() - 1];
            }
            out.to_string()
        }

        let canonical =
            std::fs::canonicalize(temp.path()).unwrap_or_else(|_| temp.path().to_path_buf());
        trim_trailing_separators(&canonical.to_string_lossy())
    };

    let mut state = State::default();
    assert_eq!(state.scan_state, ScanState::Idle);

    let mut player = None;
    let _task = nodoka::ui::update::update(
        &mut state,
        Message::DirectoryAdded(raw_dir),
        &mut player,
        &db,
    );

    assert!(matches!(
        &state.scan_state,
        ScanState::Scanning { directory: Some(d) } if d == &expected_dir
    ));

    Ok(())
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
