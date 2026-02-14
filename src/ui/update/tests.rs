use super::{handle_file_selected, update, MediaControl};
use crate::db::{self, Database};
use crate::error::{Error, Result};
use crate::models::{Audiobook, AudiobookFile, SleepTimer, SleepTimerMode};
use crate::player::Vlc;
use crate::ui::{Message, State};
use std::path::Path;

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

    assert!((state.speed - 1.3).abs() < 0.0001);

    let saved = crate::db::queries::get_metadata(db.connection(), "speed")?
        .ok_or("missing speed metadata")?;
    assert_eq!(saved, "1.3");
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
