use super::{
    CountingLoadPlayer, FailingLoadPlayer, FailingPlayPlayer, RecordingPlayer, RecordingSeekPlayer,
};
use crate::db::{self, Database};
use crate::error::Error;
use crate::models::{Audiobook, AudiobookFile};
use crate::ui::{Message, PlaybackStatus, State};
use std::cell::Cell;
use std::rc::Rc;

#[test]
fn test_handle_file_selected_does_not_change_selection_or_db_on_load_failure(
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    use super::super::handle_file_selected;

    let db = Database::new_in_memory()?;
    db::initialize(db.connection())?;

    let mut audiobook = Audiobook::new(
        "/dir".to_string(),
        "Test".to_string(),
        "/dir/book".to_string(),
        0,
    );
    let old_path = "/dir/book/old.mp3";
    audiobook.selected_file = Some(old_path.to_string());
    let audiobook_id = crate::db::queries::insert_audiobook(db.connection(), &audiobook)?;

    let old_file = AudiobookFile::new(audiobook_id, "old".to_string(), old_path.to_string(), 0);
    crate::db::queries::insert_audiobook_file(db.connection(), &old_file)?;

    let new_path = "/dir/book/new.mp3";
    let new_file = AudiobookFile::new(audiobook_id, "new".to_string(), new_path.to_string(), 1);
    crate::db::queries::insert_audiobook_file(db.connection(), &new_file)?;

    let mut state = State {
        selected_audiobook: Some(audiobook_id),
        selected_file: Some(old_path.to_string()),
        ..Default::default()
    };

    let mut player = Some(FailingLoadPlayer);

    let _cmd = handle_file_selected(&mut state, &mut player, &db, new_path);

    assert_eq!(
        state.selected_file.as_deref(),
        Some(old_path),
        "selection should remain unchanged when load fails"
    );

    let saved = crate::db::queries::get_audiobook_by_id(db.connection(), audiobook_id)?
        .ok_or_else(|| Error::AudiobookNotFound(audiobook_id))?;
    assert_eq!(
        saved.selected_file.as_deref(),
        Some(old_path),
        "db selected_file should not change when load fails"
    );
    Ok(())
}

#[test]
fn test_handle_file_selected_clears_error_on_successful_play(
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    use super::super::handle_file_selected;

    let db = Database::new_in_memory()?;
    db::initialize(db.connection())?;

    let audiobook = Audiobook::new(
        "/dir".to_string(),
        "Test".to_string(),
        "/dir/book".to_string(),
        0,
    );
    let audiobook_id = crate::db::queries::insert_audiobook(db.connection(), &audiobook)?;

    let mut state = State {
        selected_audiobook: Some(audiobook_id),
        error_message: Some("Stale error".to_string()),
        error_timestamp: Some(chrono::Utc::now()),
        ..Default::default()
    };

    let load_calls = Rc::new(Cell::new(0));
    let play_calls = Rc::new(Cell::new(0));
    let mut player = Some(RecordingPlayer {
        load_calls: load_calls.clone(),
        play_calls: play_calls.clone(),
    });

    let path = "/dir/book/new.mp3";
    let _cmd = handle_file_selected(&mut state, &mut player, &db, path);

    assert_eq!(load_calls.get(), 1, "media should be loaded");
    assert_eq!(play_calls.get(), 1, "play should be attempted");
    assert_eq!(state.error_message, None, "stale error should clear");
    assert_eq!(
        state.error_timestamp, None,
        "stale error timestamp should clear"
    );
    assert_eq!(state.playback, PlaybackStatus::Playing);
    Ok(())
}

#[test]
fn test_handle_file_selected_sets_paused_on_play_failure(
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    use super::super::handle_file_selected;

    let db = Database::new_in_memory()?;
    db::initialize(db.connection())?;

    let audiobook = Audiobook::new(
        "/dir".to_string(),
        "Test".to_string(),
        "/dir/book".to_string(),
        0,
    );
    let audiobook_id = crate::db::queries::insert_audiobook(db.connection(), &audiobook)?;

    let file_path = "/dir/book/ch1.mp3";
    let file = AudiobookFile::new(audiobook_id, "ch1".to_string(), file_path.to_string(), 0);
    crate::db::queries::insert_audiobook_file(db.connection(), &file)?;

    let mut state = State {
        selected_audiobook: Some(audiobook_id),
        playback: PlaybackStatus::Playing,
        ..Default::default()
    };

    let mut player = Some(FailingPlayPlayer);
    let _cmd = handle_file_selected(&mut state, &mut player, &db, file_path);

    assert!(
        state.error_message.is_some(),
        "play failure should surface error"
    );
    assert_eq!(
        state.playback,
        PlaybackStatus::Paused,
        "play failure should force playback state to Paused"
    );
    Ok(())
}

#[test]
fn test_handle_file_selected_rejects_malformed_zip_virtual_paths(
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    use super::super::handle_file_selected;

    let db = Database::new_in_memory()?;
    db::initialize(db.connection())?;

    let mut state = State::default();
    let load_calls = Rc::new(Cell::new(0));
    let mut player = Some(CountingLoadPlayer {
        load_calls: load_calls.clone(),
    });

    let invalid_zip_virtual = "zip://not-a-valid-virtual-path";
    let _cmd = handle_file_selected(&mut state, &mut player, &db, invalid_zip_virtual);

    assert_eq!(
        load_calls.get(),
        0,
        "malformed ZIP virtual path should not call load_media"
    );

    let msg = state.error_message.as_deref().ok_or("missing error")?;
    assert!(
        msg.to_lowercase().contains("zip") && msg.to_lowercase().contains("invalid"),
        "error should mention invalid ZIP virtual path"
    );
    assert!(
        state.error_timestamp.is_some(),
        "error should set timestamp"
    );
    assert!(
        state.selected_file.is_none(),
        "invalid ZIP virtual path should not persist selection"
    );
    Ok(())
}

#[test]
fn test_seek_to_clamps_to_total_duration_when_known() {
    let mut state = State {
        total_duration: 10_000.0,
        current_time: 123.0,
        ..Default::default()
    };

    let last_set_time_ms = Rc::new(Cell::new(-1));
    let mut player = Some(RecordingSeekPlayer {
        last_set_time_ms: last_set_time_ms.clone(),
        length_ms: 0,
    });

    let _ = super::super::handle_seek_to_media_control(&mut state, &mut player, 50_000.0);

    assert_eq!(last_set_time_ms.get(), 10_000);
    assert!((state.current_time - 10_000.0).abs() < 0.0001);
}

#[test]
fn test_seek_to_clamps_negative_to_zero_when_duration_known() {
    let mut state = State {
        total_duration: 10_000.0,
        current_time: 123.0,
        ..Default::default()
    };

    let last_set_time_ms = Rc::new(Cell::new(-1));
    let mut player = Some(RecordingSeekPlayer {
        last_set_time_ms: last_set_time_ms.clone(),
        length_ms: 0,
    });

    let _ = super::super::handle_seek_to_media_control(&mut state, &mut player, -100.0);

    assert_eq!(last_set_time_ms.get(), 0);
    assert!(state.current_time.abs() < 0.0001);
}

#[test]
fn test_audiobook_selected_loads_files() -> std::result::Result<(), Box<dyn std::error::Error>> {
    use super::super::update;

    let db = Database::new_in_memory()?;
    db::initialize(db.connection())?;

    let audiobook = Audiobook::new(
        "/dir".to_string(),
        "Test".to_string(),
        "/dir/book".to_string(),
        0,
    );
    let id = crate::db::queries::insert_audiobook(db.connection(), &audiobook)?;

    let file = AudiobookFile::new(
        id,
        "ch1.mp3".to_string(),
        "/dir/book/ch1.mp3".to_string(),
        0,
    );
    crate::db::queries::insert_audiobook_file(db.connection(), &file)?;

    let mut state = State::default();
    let mut player = None;

    let _cmd = update(&mut state, Message::AudiobookSelected(id), &mut player, &db);

    assert_eq!(state.selected_audiobook, Some(id));
    assert_eq!(state.current_files.len(), 1);

    Ok(())
}
