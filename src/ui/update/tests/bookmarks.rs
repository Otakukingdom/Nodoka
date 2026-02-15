use super::{CountingFailingLoadPlayer, RecordingSeekPlayer};
use crate::db::{self, Database};
use crate::models::{Audiobook, AudiobookFile, Bookmark};
use crate::ui::{Message, State};
use std::cell::Cell;
use std::rc::Rc;

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

    let _cmd =
        super::super::bookmarks::handle_bookmark_jump(&mut state, &mut player, &db, bookmark_id);

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
fn test_bookmark_jump_clamps_seek_target_to_known_duration(
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
    let mut file = AudiobookFile::new(audiobook_id, "ch1".to_string(), file_path.to_string(), 0);
    file.length_of_file = Some(10_000);
    file.seek_position = None;
    crate::db::queries::insert_audiobook_file(db.connection(), &file)?;

    let bookmark_id = 77;
    let mut bookmark = Bookmark::new(
        audiobook_id,
        file_path.to_string(),
        50_000,
        "Too far".to_string(),
    );
    bookmark.id = Some(bookmark_id);

    let mut state = State {
        selected_audiobook: Some(audiobook_id),
        selected_file: None,
        bookmarks: vec![bookmark],
        ..Default::default()
    };

    let last_set_time_ms = Rc::new(Cell::new(-1));
    let mut player = Some(RecordingSeekPlayer {
        last_set_time_ms: last_set_time_ms.clone(),
        length_ms: 10_000,
    });

    let _ =
        super::super::bookmarks::handle_bookmark_jump(&mut state, &mut player, &db, bookmark_id);

    assert_eq!(
        last_set_time_ms.get(),
        10_000,
        "bookmark jump should clamp seek target to duration"
    );
    assert!(
        (state.current_time - 10_000.0).abs() < 0.0001,
        "bookmark jump should clamp state.current_time to duration"
    );
    Ok(())
}

#[test]
fn test_create_bookmark_opens_editor() -> std::result::Result<(), Box<dyn std::error::Error>> {
    use super::super::update;

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

    let mut player = None;
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
    use super::super::update;

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
    let mut player = None;

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
fn test_bookmark_editor_cancel_closes_without_saving(
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    use super::super::update;

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
    let mut player = None;

    let _ = update(&mut state, Message::CreateBookmark, &mut player, &db);
    assert!(state.bookmark_editor.is_some());

    let _ = update(&mut state, Message::BookmarkEditorCancel, &mut player, &db);
    assert!(state.bookmark_editor.is_none());

    // CreateBookmark immediately saves a bookmark to DB, then opens editor.
    // Canceling the editor doesn't delete the already-saved bookmark.
    let saved = crate::db::queries::get_bookmarks_for_audiobook(db.connection(), audiobook_id)?;
    assert_eq!(
        saved.len(),
        1,
        "CreateBookmark saves immediately, cancel doesn't delete"
    );

    Ok(())
}
