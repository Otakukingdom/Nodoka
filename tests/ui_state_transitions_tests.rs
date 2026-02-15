//! UI state transition integration tests.
//!
//! These tests exercise real message-driven transitions by routing `Message` inputs through
//! `nodoka::ui::update::update` and asserting on resulting observable state/DB effects.

use nodoka::models::{Audiobook, Directory};
use nodoka::ui::{Message, PlaybackStatus, ScanState, State};
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
fn test_scan_error_sets_error_and_stops_scanning() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let mut player = None;
    let mut state = State {
        scan_state: ScanState::Scanning {
            directory: Some("/test/audiobooks".to_string()),
        },
        ..State::default()
    };

    let _task = nodoka::ui::update::update(
        &mut state,
        Message::ScanError("boom".to_string()),
        &mut player,
        &db,
    );

    assert_eq!(state.scan_state, ScanState::Idle);
    assert!(
        state
            .error_message
            .as_deref()
            .is_some_and(|m| m.contains("Failed to scan directory")),
        "ScanError should produce a user-facing error message"
    );
    assert!(state.error_timestamp.is_some());

    Ok(())
}

#[test]
fn test_dismiss_error_clears_message_and_timestamp() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let mut player = None;
    let now = chrono::Utc::now();
    let mut state = State {
        error_message: Some("Test error".to_string()),
        error_timestamp: Some(now),
        ..State::default()
    };

    let _task = nodoka::ui::update::update(&mut state, Message::DismissError, &mut player, &db);

    assert!(state.error_message.is_none());
    assert!(state.error_timestamp.is_none());

    Ok(())
}

#[test]
fn test_file_selected_updates_state_even_without_player() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let audiobook_id = create_test_audiobook(&db, "/test", "Book")?;

    let mut state = State {
        selected_audiobook: Some(audiobook_id),
        volume: 75,
        speed: 1.5,
        ..State::default()
    };
    let mut player = None;

    let path = "/nonexistent/file.mp3";
    let _task = nodoka::ui::update::update(
        &mut state,
        Message::FileSelected(path.to_string()),
        &mut player,
        &db,
    );

    assert_eq!(state.selected_file.as_deref(), Some(path));
    assert_eq!(state.volume, 75);
    assert!((state.speed - 1.5).abs() < f32::EPSILON);
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
fn test_directory_remove_clears_selection_when_in_directory() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let mut player = None;

    let temp_dir = temp_dir::TempDir::new()?;
    let dir_path = temp_dir.path().join("library");
    std::fs::create_dir_all(&dir_path)?;
    let dir_str = dir_path.to_string_lossy().to_string();

    // Directory must exist on disk for insert_directory validation.
    let directory = Directory {
        full_path: dir_str.clone(),
        created_at: chrono::Utc::now(),
        last_scanned: None,
    };
    nodoka::db::queries::insert_directory(db.connection(), &directory)?;

    let audiobook_id = create_test_audiobook(&db, &dir_str, "Test Book")?;
    let file_path = format!("{dir_str}/Test Book/ch1.mp3");
    insert_test_file(&db, audiobook_id, &file_path)?;

    let mut state = State {
        directories: vec![directory],
        audiobooks: vec![Audiobook {
            id: Some(audiobook_id),
            directory: dir_str.clone(),
            name: "Test Book".to_string(),
            full_path: format!("{dir_str}/Test Book"),
            completeness: 0,
            default_order: 0,
            selected_file: Some(file_path.clone()),
            created_at: chrono::Utc::now(),
        }],
        selected_audiobook: Some(audiobook_id),
        selected_file: Some(file_path),
        playback: PlaybackStatus::Playing,
        current_time: 123.0,
        total_duration: 456.0,
        ..State::default()
    };

    let _task = nodoka::ui::update::update(
        &mut state,
        Message::DirectoryRemove(dir_str),
        &mut player,
        &db,
    );

    assert!(state.directories.is_empty());
    assert!(state.audiobooks.is_empty());
    assert!(state.selected_audiobook.is_none());
    assert!(state.selected_file.is_none());
    assert!(state.current_files.is_empty());
    assert!(state.bookmarks.is_empty());
    assert!(state.bookmark_editor.is_none());
    assert_eq!(state.playback, PlaybackStatus::Paused);
    assert!((state.current_time - 0.0).abs() < f64::EPSILON);
    assert!((state.total_duration - 0.0).abs() < f64::EPSILON);

    Ok(())
}

#[test]
fn test_sleep_timer_duration_expires_on_tick_pauses_and_clears() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let mut player = None;
    let mut state = State {
        playback: PlaybackStatus::Playing,
        volume: 100,
        ..State::default()
    };

    let _task = nodoka::ui::update::update(
        &mut state,
        Message::SleepTimerSetDurationSeconds(0),
        &mut player,
        &db,
    );
    assert!(state.sleep_timer.is_some());
    assert!(state.sleep_timer_base_volume.is_some());

    let _task = nodoka::ui::update::update(&mut state, Message::PlayerTick, &mut player, &db);

    assert_eq!(state.playback, PlaybackStatus::Paused);
    assert!(state.sleep_timer.is_none());
    assert!(state.sleep_timer_base_volume.is_none());

    Ok(())
}

#[test]
fn test_volume_speed_persist_across_file_selected_message() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let audiobook_id = create_test_audiobook(&db, "/test", "Book")?;

    let mut player = None;
    let mut state = State {
        selected_audiobook: Some(audiobook_id),
        volume: 75,
        speed: 1.5,
        selected_file: Some("/book/chapter1.mp3".to_string()),
        ..State::default()
    };

    let _task = nodoka::ui::update::update(
        &mut state,
        Message::FileSelected("/book/chapter2.mp3".to_string()),
        &mut player,
        &db,
    );

    assert_eq!(state.volume, 75);
    assert!((state.speed - 1.5).abs() < f32::EPSILON);
    Ok(())
}

#[test]
fn test_create_bookmark_message_persists_and_opens_editor() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let audiobook_id = create_test_audiobook(&db, "/test", "Test Book")?;
    let file_path = "/test/Test Book/chapter1.mp3";
    insert_test_file(&db, audiobook_id, file_path)?;

    let mut player = None;
    let mut state = State {
        settings_open: true,
        selected_audiobook: Some(audiobook_id),
        selected_file: Some(file_path.to_string()),
        current_time: 12.0,
        ..State::default()
    };

    let _task = nodoka::ui::update::update(&mut state, Message::CreateBookmark, &mut player, &db);

    assert!(!state.settings_open);
    assert!(state.bookmark_editor.is_some());

    let bookmarks =
        nodoka::db::queries::get_bookmarks_for_audiobook(db.connection(), audiobook_id)?;
    assert_eq!(bookmarks.len(), 1);

    Ok(())
}

#[test]
fn test_rapid_audiobook_selected_messages_converge_to_last() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let mut player = None;

    let mut ids = Vec::new();
    let mut audiobooks = Vec::new();
    for i in 0..5 {
        let name = format!("Book {i}");
        let id = create_test_audiobook(&db, "/test", &name)?;
        ids.push(id);
        audiobooks.push(Audiobook {
            id: Some(id),
            directory: "/test".to_string(),
            name: name.clone(),
            full_path: format!("/test/{name}"),
            completeness: 0,
            default_order: i,
            selected_file: None,
            created_at: chrono::Utc::now(),
        });
    }

    // Seed state with list so update can find items for cleanup/thumbnail paths.
    let first = ids.first().copied().ok_or("missing audiobook ids")?;
    let mut state = State {
        audiobooks,
        selected_audiobook: Some(first),
        ..State::default()
    };

    for id in ids.iter().copied().skip(1) {
        let _task = nodoka::ui::update::update(
            &mut state,
            Message::AudiobookSelected(id),
            &mut player,
            &db,
        );
    }

    let expected = ids.last().copied().ok_or("missing audiobook ids")?;
    assert_eq!(state.selected_audiobook, Some(expected));
    Ok(())
}

#[test]
fn test_stop_resets_playback_state_without_player() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let mut player = None;
    let mut state = State {
        playback: PlaybackStatus::Playing,
        current_time: 123.0,
        total_duration: 456.0,
        ..State::default()
    };

    let _task = nodoka::ui::update::update(&mut state, Message::Stop, &mut player, &db);

    assert_eq!(state.playback, PlaybackStatus::Paused);
    assert!((state.current_time - 0.0).abs() < f64::EPSILON);

    Ok(())
}

#[test]
fn test_bookmark_editor_edit_and_save_via_messages_defaults_label() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let audiobook_id = create_test_audiobook(&db, "/test", "Test Book")?;
    let file_path = "/test/Test Book/chapter1.mp3";
    insert_test_file(&db, audiobook_id, file_path)?;

    let mut player = None;
    let mut state = State {
        selected_audiobook: Some(audiobook_id),
        selected_file: Some(file_path.to_string()),
        current_time: 1.0,
        ..State::default()
    };

    let _task = nodoka::ui::update::update(&mut state, Message::CreateBookmark, &mut player, &db);
    assert!(state.bookmark_editor.is_some());

    let _task = nodoka::ui::update::update(
        &mut state,
        Message::BookmarkEditorLabelChanged("   ".to_string()),
        &mut player,
        &db,
    );
    let _task = nodoka::ui::update::update(
        &mut state,
        Message::BookmarkEditorNoteChanged("note".to_string()),
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
    assert_eq!(first.label, "Bookmark");
    assert_eq!(first.note.as_deref(), Some("note"));

    Ok(())
}
