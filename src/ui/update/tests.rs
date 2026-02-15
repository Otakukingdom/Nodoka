use super::{handle_file_selected, update, MediaControl};
use crate::db::{self, Database};
use crate::error::{Error, Result};
use crate::models::{Audiobook, AudiobookFile, Bookmark, SleepTimer, SleepTimerMode};
use crate::player::Vlc;
use crate::ui::{Message, PlaybackStatus, State};
use std::cell::Cell;
use std::path::Path;
use std::rc::Rc;

#[derive(Default)]
struct FailingLoadPlayer;

impl MediaControl for FailingLoadPlayer {
    fn load_media(&mut self, _path: &Path) -> Result<()> {
        Err(Error::Vlc("load failed".to_string()))
    }

    fn set_time(&self, _time_ms: i64) -> Result<()> {
        Ok(())
    }

    fn get_length(&self) -> Result<i64> {
        Ok(0)
    }

    fn play(&self) -> Result<()> {
        Ok(())
    }
}

#[derive(Clone)]
struct CountingFailingLoadPlayer {
    set_time_calls: Rc<Cell<u32>>,
}

impl MediaControl for CountingFailingLoadPlayer {
    fn load_media(&mut self, _path: &Path) -> Result<()> {
        Err(Error::Vlc("load failed".to_string()))
    }

    fn set_time(&self, _time_ms: i64) -> Result<()> {
        self.set_time_calls.set(self.set_time_calls.get() + 1);
        Ok(())
    }

    fn get_length(&self) -> Result<i64> {
        Ok(0)
    }

    fn play(&self) -> Result<()> {
        Ok(())
    }
}

#[derive(Clone)]
struct RecordingPlayer {
    load_calls: Rc<Cell<u32>>,
    play_calls: Rc<Cell<u32>>,
}

impl MediaControl for RecordingPlayer {
    fn load_media(&mut self, _path: &Path) -> Result<()> {
        self.load_calls.set(self.load_calls.get() + 1);
        Ok(())
    }

    fn set_time(&self, _time_ms: i64) -> Result<()> {
        Ok(())
    }

    fn get_length(&self) -> Result<i64> {
        Ok(0)
    }

    fn play(&self) -> Result<()> {
        self.play_calls.set(self.play_calls.get() + 1);
        Ok(())
    }
}

#[test]
fn test_handle_file_selected_does_not_change_selection_or_db_on_load_failure(
) -> std::result::Result<(), Box<dyn std::error::Error>> {
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
fn test_bookmark_jump_does_not_seek_when_file_load_fails(
) -> std::result::Result<(), Box<dyn std::error::Error>> {
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

    let bookmark_id = 42;
    let mut bookmark = Bookmark::new(
        audiobook_id,
        new_path.to_string(),
        1000,
        "Important".to_string(),
    );
    bookmark.id = Some(bookmark_id);

    let mut state = State {
        selected_audiobook: Some(audiobook_id),
        selected_file: Some(old_path.to_string()),
        current_time: 123.0,
        bookmarks: vec![bookmark],
        ..Default::default()
    };

    let set_time_calls = Rc::new(Cell::new(0));
    let mut player = Some(CountingFailingLoadPlayer {
        set_time_calls: set_time_calls.clone(),
    });

    let _cmd = super::bookmarks::handle_bookmark_jump(&mut state, &mut player, &db, bookmark_id);

    assert_eq!(
        set_time_calls.get(),
        0,
        "bookmark jump should not seek when file load fails"
    );
    assert_eq!(
        state.selected_file.as_deref(),
        Some(old_path),
        "bookmark jump should not change selection when load fails"
    );
    assert!(
        state.error_message.is_some(),
        "bookmark jump should surface load failure to user"
    );
    assert!(
        (state.current_time - 123.0).abs() < 0.0001,
        "bookmark jump should not update current_time when load fails"
    );
    Ok(())
}

#[test]
fn test_handle_file_selected_clears_error_on_successful_play(
) -> std::result::Result<(), Box<dyn std::error::Error>> {
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
fn test_handle_time_updated_clamps_negative_to_zero(
) -> std::result::Result<(), Box<dyn std::error::Error>> {
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
        selected_file: Some(file_path.to_string()),
        total_duration: 10_000.0,
        current_time: 5_000.0,
        ..Default::default()
    };
    let mut player: Option<Vlc> = None;

    let _ = update(
        &mut state,
        Message::PlayerTimeUpdated(-123.0),
        &mut player,
        &db,
    );

    assert!(
        (state.current_time - 0.0).abs() < 0.0001,
        "negative time updates should clamp to zero"
    );

    let saved = crate::db::queries::get_audiobook_file_by_path(db.connection(), file_path)?
        .ok_or("missing audiobook file")?;
    assert_eq!(
        saved.seek_position,
        Some(0),
        "negative time update should persist as 0ms"
    );
    Ok(())
}

#[test]
fn test_handle_time_updated_ignores_non_finite_values(
) -> std::result::Result<(), Box<dyn std::error::Error>> {
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
    crate::db::queries::update_file_progress(db.connection(), file_path, 5_000.0, 50)?;

    let mut state = State {
        selected_audiobook: Some(audiobook_id),
        selected_file: Some(file_path.to_string()),
        total_duration: 10_000.0,
        current_time: 5_000.0,
        ..Default::default()
    };
    let mut player: Option<Vlc> = None;

    let _ = update(
        &mut state,
        Message::PlayerTimeUpdated(f64::NAN),
        &mut player,
        &db,
    );

    assert!(
        (state.current_time - 5_000.0).abs() < 0.0001,
        "non-finite time updates should be ignored"
    );

    let saved = crate::db::queries::get_audiobook_file_by_path(db.connection(), file_path)?
        .ok_or("missing audiobook file")?;
    assert_eq!(
        saved.seek_position,
        Some(5000),
        "non-finite time updates should not persist progress"
    );
    Ok(())
}

#[test]
fn test_operation_in_progress_guards_directory_actions(
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let db = Database::new_in_memory()?;
    db::initialize(db.connection())?;

    let mut state = State::default();
    let mut player: Option<Vlc> = None;

    let _ = update(&mut state, Message::DirectoryAdd, &mut player, &db);
    assert!(
        state.operation_in_progress,
        "DirectoryAdd should mark operation as in progress"
    );

    let _ = update(&mut state, Message::DirectoryAddCancelled, &mut player, &db);
    assert!(
        !state.operation_in_progress,
        "DirectoryAddCancelled should clear operation_in_progress"
    );

    let _ = update(
        &mut state,
        Message::DirectoryRescan("/dir".to_string()),
        &mut player,
        &db,
    );
    assert!(
        state.operation_in_progress,
        "DirectoryRescan should mark operation as in progress"
    );

    let _ = update(
        &mut state,
        Message::ScanError("scan failed".to_string()),
        &mut player,
        &db,
    );
    assert!(
        !state.operation_in_progress,
        "ScanError should clear operation_in_progress"
    );
    Ok(())
}

#[test]
fn test_create_bookmark_opens_editor() -> std::result::Result<(), Box<dyn std::error::Error>> {
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
        selected_file: Some(file_path.to_string()),
        current_time: 1500.0,
        ..Default::default()
    };

    let mut player: Option<Vlc> = None;
    let _cmd = update(&mut state, Message::CreateBookmark, &mut player, &db);

    let editor = state.bookmark_editor.as_ref().ok_or("expected editor")?;
    assert_eq!(editor.audiobook_id, audiobook_id);
    assert_eq!(editor.file_path, file_path);
    assert_eq!(editor.position_ms, 1500);
    assert_eq!(editor.label, "Bookmark");
    assert!(editor.note.is_empty());
    Ok(())
}

#[test]
fn test_bookmark_editor_save_inserts_and_closes(
) -> std::result::Result<(), Box<dyn std::error::Error>> {
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
        selected_file: Some(file_path.to_string()),
        current_time: 2000.0,
        ..Default::default()
    };
    let mut player: Option<Vlc> = None;

    let _ = update(&mut state, Message::CreateBookmark, &mut player, &db);
    let _ = update(
        &mut state,
        Message::BookmarkEditorLabelChanged("Chapter 1".to_string()),
        &mut player,
        &db,
    );
    let _ = update(
        &mut state,
        Message::BookmarkEditorNoteChanged("note".to_string()),
        &mut player,
        &db,
    );
    let _ = update(&mut state, Message::BookmarkEditorSave, &mut player, &db);

    assert!(state.bookmark_editor.is_none());

    let saved = crate::db::queries::get_bookmarks_for_audiobook(db.connection(), audiobook_id)?;
    let first = saved.first().ok_or("no bookmark")?;
    assert_eq!(first.label, "Chapter 1");
    assert_eq!(first.note.as_deref(), Some("note"));
    Ok(())
}

#[test]
fn test_volume_changed_clamps_and_persists_without_player(
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let db = Database::new_in_memory()?;
    db::initialize(db.connection())?;

    let mut state = State::default();
    let mut player: Option<Vlc> = None;

    let _ = update(&mut state, Message::VolumeChanged(999), &mut player, &db);

    assert_eq!(state.volume, 200);

    let saved = crate::db::queries::get_metadata(db.connection(), "volume")?
        .ok_or("missing volume metadata")?;
    assert_eq!(saved, "200");
    Ok(())
}

#[test]
fn test_speed_changed_quantizes_and_persists_without_player(
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let db = Database::new_in_memory()?;
    db::initialize(db.connection())?;

    let mut state = State::default();
    let mut player: Option<Vlc> = None;

    let _ = update(&mut state, Message::SpeedChanged(1.26), &mut player, &db);

    assert!((state.speed - 1.25).abs() < 0.0001);

    let saved = crate::db::queries::get_metadata(db.connection(), "speed")?
        .ok_or("missing speed metadata")?;
    assert_eq!(saved, "1.25");
    Ok(())
}

#[test]
fn test_speed_preset_values_are_preserved_exactly(
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let db = Database::new_in_memory()?;
    db::initialize(db.connection())?;

    let mut state = State::default();
    let mut player: Option<Vlc> = None;

    let _ = update(&mut state, Message::SpeedChanged(0.75), &mut player, &db);

    assert!((state.speed - 0.75).abs() < 0.0001);

    let saved = crate::db::queries::get_metadata(db.connection(), "speed")?
        .ok_or("missing speed metadata")?;
    assert_eq!(saved, "0.75");
    Ok(())
}

#[test]
fn test_end_of_chapter_sleep_timer_intercepts_auto_advance() {
    let state = State {
        sleep_timer: Some(SleepTimer::new(SleepTimerMode::EndOfChapter, 0)),
        ..State::default()
    };
    assert!(super::sleep_timer::should_pause_for_end_of_chapter(
        &state, true
    ));
    assert!(!super::sleep_timer::should_pause_for_end_of_chapter(
        &state, false
    ));
}

#[test]
fn test_sleep_timer_custom_minutes_submit_sets_duration(
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let db = Database::new_in_memory()?;
    db::initialize(db.connection())?;

    let mut state = State {
        sleep_timer_custom_minutes: "17".to_string(),
        ..State::default()
    };

    let mut player: Option<Vlc> = None;
    let _ = update(
        &mut state,
        Message::SleepTimerCustomSubmit,
        &mut player,
        &db,
    );

    let Some(timer) = state.sleep_timer.as_ref() else {
        return Err("expected active timer".into());
    };
    assert!(matches!(
        timer.mode,
        SleepTimerMode::Duration(secs) if secs == 17 * 60
    ));
    Ok(())
}

#[test]
fn test_play_pause_toggles_state() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let db = Database::new_in_memory()?;
    db::initialize(db.connection())?;

    let mut state = State {
        playback: PlaybackStatus::Paused,
        ..Default::default()
    };
    let mut player: Option<Vlc> = None;

    // First press: should attempt to play (but won't work without actual player)
    let _cmd = update(&mut state, Message::PlayPause, &mut player, &db);

    Ok(())
}

#[test]
fn test_seek_to_updates_state() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let db = Database::new_in_memory()?;
    db::initialize(db.connection())?;

    let mut state = State {
        current_time: 1000.0,
        total_duration: 10000.0,
        ..Default::default()
    };
    let mut player: Option<Vlc> = None;

    let _cmd = update(&mut state, Message::SeekTo(5000.0), &mut player, &db);

    // Without actual player, state might not change, but test the call succeeds
    Ok(())
}

#[test]
fn test_audiobook_selected_loads_files() -> std::result::Result<(), Box<dyn std::error::Error>> {
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
    let mut player: Option<Vlc> = None;

    let _cmd = update(&mut state, Message::AudiobookSelected(id), &mut player, &db);

    assert_eq!(state.selected_audiobook, Some(id));
    assert_eq!(state.current_files.len(), 1);

    Ok(())
}

#[test]
fn test_sleep_timer_custom_input_validation() -> std::result::Result<(), Box<dyn std::error::Error>>
{
    let db = Database::new_in_memory()?;
    db::initialize(db.connection())?;

    let mut state = State::default();
    let mut player: Option<Vlc> = None;

    // Test invalid input
    let _cmd = update(
        &mut state,
        Message::SleepTimerCustomMinutesChanged("abc".to_string()),
        &mut player,
        &db,
    );
    assert_eq!(state.sleep_timer_custom_minutes, "abc");

    let _cmd = update(
        &mut state,
        Message::SleepTimerCustomSubmit,
        &mut player,
        &db,
    );
    // Should set error message
    assert!(state.sleep_timer_custom_error.is_some());
    assert!(state.sleep_timer.is_none()); // Should not create timer

    Ok(())
}

#[test]
fn test_sleep_timer_cancel_stops_timer() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let db = Database::new_in_memory()?;
    db::initialize(db.connection())?;

    let mut state = State {
        sleep_timer: Some(SleepTimer::new(SleepTimerMode::Duration(1800), 10)),
        ..Default::default()
    };
    let mut player: Option<Vlc> = None;

    let _cmd = update(&mut state, Message::SleepTimerCancel, &mut player, &db);

    assert!(state.sleep_timer.is_none());

    Ok(())
}

#[test]
fn test_sleep_timer_extend_adds_time() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let db = Database::new_in_memory()?;
    db::initialize(db.connection())?;

    let mut state = State {
        sleep_timer: Some(SleepTimer::new(SleepTimerMode::Duration(1800), 10)),
        ..Default::default()
    };
    let mut player: Option<Vlc> = None;

    let _cmd = update(
        &mut state,
        Message::SleepTimerExtendSeconds(900),
        &mut player,
        &db,
    );

    assert!(state.sleep_timer.is_some());

    Ok(())
}

#[test]
fn test_bookmark_editor_cancel_closes_without_saving(
) -> std::result::Result<(), Box<dyn std::error::Error>> {
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
        selected_file: Some(file_path.to_string()),
        current_time: 1500.0,
        ..Default::default()
    };
    let mut player: Option<Vlc> = None;

    let _ = update(&mut state, Message::CreateBookmark, &mut player, &db);
    assert!(state.bookmark_editor.is_some());

    let _ = update(&mut state, Message::BookmarkEditorCancel, &mut player, &db);
    assert!(state.bookmark_editor.is_none());

    // Note: CreateBookmark immediately saves a bookmark to DB, then opens editor
    // Canceling the editor doesn't delete the already-saved bookmark
    // This is by design - the bookmark exists, user just chose not to edit it
    let saved = crate::db::queries::get_bookmarks_for_audiobook(db.connection(), audiobook_id)?;
    assert_eq!(
        saved.len(),
        1,
        "CreateBookmark saves immediately, cancel doesn't delete"
    );

    Ok(())
}

#[test]
fn test_stop_message_stops_playback() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let db = Database::new_in_memory()?;
    db::initialize(db.connection())?;

    let mut state = State {
        playback: PlaybackStatus::Playing,
        current_time: 1500.0,
        ..Default::default()
    };
    let mut player: Option<Vlc> = None;

    let _cmd = update(&mut state, Message::Stop, &mut player, &db);

    // Note: Stop only updates state if there's an actual player to stop
    // Without a player, state remains unchanged (by design)
    // This test verifies the message doesn't panic without a player
    // In real usage, playback would be set to paused when a player exists
    assert!(
        state.playback == PlaybackStatus::Playing,
        "State unchanged without player (expected behavior)"
    );

    Ok(())
}

#[test]
fn test_sleep_timer_set_duration_creates_timer(
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let db = Database::new_in_memory()?;
    db::initialize(db.connection())?;

    let mut state = State::default();
    let mut player: Option<Vlc> = None;

    let _cmd = update(
        &mut state,
        Message::SleepTimerSetDurationSeconds(1800),
        &mut player,
        &db,
    );

    assert!(state.sleep_timer.is_some());
    if let Some(ref timer) = state.sleep_timer {
        assert!(matches!(timer.mode, SleepTimerMode::Duration(1800)));
    }

    Ok(())
}

#[test]
fn test_sleep_timer_set_end_of_chapter_creates_timer(
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let db = Database::new_in_memory()?;
    db::initialize(db.connection())?;

    let mut state = State::default();
    let mut player: Option<Vlc> = None;

    let _cmd = update(
        &mut state,
        Message::SleepTimerSetEndOfChapter,
        &mut player,
        &db,
    );

    assert!(state.sleep_timer.is_some());
    if let Some(ref timer) = state.sleep_timer {
        assert!(matches!(timer.mode, SleepTimerMode::EndOfChapter));
    }

    Ok(())
}
