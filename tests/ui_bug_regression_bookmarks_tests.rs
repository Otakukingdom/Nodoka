//! Regression tests for UI bugs discovered during systematic testing.
//!
//! Tests in this file focus on bookmarks and bookmark editor behavior.

use nodoka::models::{Audiobook, AudiobookFile, Bookmark};
use nodoka::ui::{Message, State};

mod acceptance_support;

/// Bug #0003: Bookmark editor handles empty labels correctly
///
/// Scenario: User tries to save a bookmark with an empty label.
///
/// Expected: Label defaults to "Bookmark" rather than saving empty string.
#[test]
fn test_bug_0003_bookmark_editor_empty_label_handling(
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let db = acceptance_support::create_test_db()?;
    let audiobook_id = acceptance_support::create_test_audiobook(&db, "/test", "Book")?;

    let file_path = "/test/Book/ch1.mp3";
    acceptance_support::insert_test_file(&db, audiobook_id, file_path)?;

    let mut state = State {
        selected_audiobook: Some(audiobook_id),
        selected_file: Some(file_path.to_string()),
        current_time: 1234.0,
        ..Default::default()
    };
    let mut player = None;

    let _ = nodoka::ui::update::update(&mut state, Message::CreateBookmark, &mut player, &db);
    assert!(state.bookmark_editor.is_some());

    let _ = nodoka::ui::update::update(
        &mut state,
        Message::BookmarkEditorLabelChanged("   ".to_string()),
        &mut player,
        &db,
    );
    let _ = nodoka::ui::update::update(&mut state, Message::BookmarkEditorSave, &mut player, &db);

    let saved = nodoka::db::queries::get_bookmarks_for_audiobook(db.connection(), audiobook_id)?;
    let first = saved.first().ok_or("expected saved bookmark")?;
    assert_eq!(first.label, "Bookmark");
    Ok(())
}

/// Bug #0013: Bookmark list shows correct file status
///
/// Scenario: Bookmark references a file that no longer exists.
///
/// Expected: Bookmark is marked with warning indicator.
#[test]
fn test_bug_0013_bookmark_missing_file_indication_does_not_panic() {
    let bookmark = Bookmark {
        id: Some(1),
        audiobook_id: 1,
        file_path: "/test/missing.mp3".to_string(),
        position_ms: 1000,
        label: "Test Bookmark".to_string(),
        note: None,
        created_at: chrono::Utc::now(),
    };

    let file = AudiobookFile {
        audiobook_id: 1,
        name: "missing.mp3".to_string(),
        full_path: "/test/missing.mp3".to_string(),
        length_of_file: Some(3_600_000),
        seek_position: None,
        checksum: None,
        position: 0,
        completeness: 0,
        file_exists: false,
        created_at: chrono::Utc::now(),
    };

    let bookmarks = vec![bookmark];
    let files = vec![file];

    let element = nodoka::ui::components::bookmarks::view(&bookmarks, &files);
    drop(element);
}

/// Bug #0026: Bookmark editor cleared early when switching audiobooks
///
/// Scenario: User has bookmark editor open, then switches to a different audiobook.
///
/// Expected: Bookmark editor is closed immediately when switching audiobooks.
#[test]
fn test_bug_0026_bookmark_editor_closed_on_audiobook_switch(
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let db = acceptance_support::create_test_db()?;
    let old_id = acceptance_support::create_test_audiobook(&db, "/test", "Old")?;
    let new_id = acceptance_support::create_test_audiobook(&db, "/test", "New")?;
    acceptance_support::insert_test_file(&db, old_id, "/test/Old/ch1.mp3")?;
    acceptance_support::insert_test_file(&db, new_id, "/test/New/ch1.mp3")?;

    let mut old_ab = Audiobook::new(
        "/test".to_string(),
        "Old".to_string(),
        "/test/Old".to_string(),
        0,
    );
    old_ab.id = Some(old_id);

    let mut new_ab = Audiobook::new(
        "/test".to_string(),
        "New".to_string(),
        "/test/New".to_string(),
        0,
    );
    new_ab.id = Some(new_id);

    let mut state = State {
        audiobooks: vec![old_ab, new_ab],
        selected_audiobook: Some(old_id),
        bookmark_editor: Some(nodoka::ui::BookmarkEditor {
            id: Some(1),
            audiobook_id: old_id,
            file_path: "/test/Old/ch1.mp3".to_string(),
            position_ms: 1000,
            label: "Old Bookmark".to_string(),
            note: String::new(),
        }),
        ..Default::default()
    };

    let mut player = None;
    let _ = nodoka::ui::update::update(
        &mut state,
        Message::AudiobookSelected(new_id),
        &mut player,
        &db,
    );

    assert!(state.bookmark_editor.is_none());
    Ok(())
}

/// Bug #0035: Bookmark editor with extremely long label or note
///
/// Scenario: User creates a bookmark with a very long label (1000+ characters)
/// or note text, which could cause UI layout issues.
///
/// Expected: Bookmark editor accepts long text, UI renders without panic.
#[test]
fn test_bug_0035_bookmark_with_extremely_long_text() {
    let long_label = "L".repeat(2000); // 2000 character label
    let long_note = "N".repeat(5000); // 5000 character note

    let editor = nodoka::ui::BookmarkEditor {
        id: None,
        audiobook_id: 1,
        file_path: "/test/file.mp3".to_string(),
        position_ms: 1000,
        label: long_label,
        note: long_note,
    };

    let element = nodoka::ui::components::bookmarks::editor(&editor);
    drop(element);
}

/// Bug #0041: Bookmark position exceeds file duration
///
/// Scenario: Bookmark has `position_ms` greater than file length (data corruption).
///
/// Expected: UI displays bookmark but prevents seeking beyond file length.
/// Bug #0049: Bookmark creation at position 0:00
///
/// Scenario: User creates bookmark at the very start of file (0ms).
///
/// Expected: Bookmark saves successfully with position 0.
#[test]
fn test_bug_0049_bookmark_at_zero_position() -> std::result::Result<(), Box<dyn std::error::Error>>
{
    let db = acceptance_support::create_test_db()?;
    let audiobook_id = acceptance_support::create_test_audiobook(&db, "/test", "Book")?;
    let file_path = "/test/Book/ch1.mp3";
    acceptance_support::insert_test_file(&db, audiobook_id, file_path)?;

    let mut state = State {
        selected_audiobook: Some(audiobook_id),
        selected_file: Some(file_path.to_string()),
        current_time: 0.0,
        ..Default::default()
    };
    let mut player = None;

    let _ = nodoka::ui::update::update(&mut state, Message::CreateBookmark, &mut player, &db);
    let saved = nodoka::db::queries::get_bookmarks_for_audiobook(db.connection(), audiobook_id)?;
    let first = saved.first().ok_or("expected bookmark")?;
    assert_eq!(first.position_ms, 0);
    assert!(!first.label.is_empty());
    Ok(())
}

/// Bug #0054: Text input field focus transitions
///
/// Scenario: User tabs between bookmark label and note fields.
///
/// Expected: Focus moves correctly, no input loss.
#[test]
fn test_bug_0054_text_input_focus_transitions_do_not_lose_text(
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let db = acceptance_support::create_test_db()?;
    let audiobook_id = acceptance_support::create_test_audiobook(&db, "/test", "Book")?;
    let file_path = "/test/Book/ch1.mp3";
    acceptance_support::insert_test_file(&db, audiobook_id, file_path)?;

    let mut state = State {
        selected_audiobook: Some(audiobook_id),
        selected_file: Some(file_path.to_string()),
        current_time: 1000.0,
        ..Default::default()
    };
    let mut player = None;

    let _ = nodoka::ui::update::update(&mut state, Message::CreateBookmark, &mut player, &db);
    assert!(state.bookmark_editor.is_some());

    let _ = nodoka::ui::update::update(
        &mut state,
        Message::BookmarkEditorLabelChanged("Updated Label".to_string()),
        &mut player,
        &db,
    );
    let _ = nodoka::ui::update::update(
        &mut state,
        Message::BookmarkEditorNoteChanged("Updated Note".to_string()),
        &mut player,
        &db,
    );

    let editor = state.bookmark_editor.as_ref().ok_or("expected editor")?;
    assert_eq!(editor.label, "Updated Label");
    assert_eq!(editor.note, "Updated Note");
    Ok(())
}

/// Bug #0060: Concurrent bookmark save and jump operations
///
/// Scenario: User saves bookmark and immediately clicks to jump to another.
///
/// Expected: Both operations complete successfully without conflict.
#[test]
fn test_bug_0060_concurrent_bookmark_operations(
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let db = acceptance_support::create_test_db()?;
    let audiobook_id = acceptance_support::create_test_audiobook(&db, "/test", "Book")?;
    let file_path = "/test/Book/ch1.mp3";
    acceptance_support::insert_test_file(&db, audiobook_id, file_path)?;

    let mut state = State {
        selected_audiobook: Some(audiobook_id),
        selected_file: Some(file_path.to_string()),
        current_time: 1000.0,
        ..Default::default()
    };
    let mut player = None;

    let _ = nodoka::ui::update::update(&mut state, Message::CreateBookmark, &mut player, &db);
    state.current_time = 5000.0;
    let _ = nodoka::ui::update::update(&mut state, Message::CreateBookmark, &mut player, &db);

    let saved = nodoka::db::queries::get_bookmarks_for_audiobook(db.connection(), audiobook_id)?;
    assert!(saved.len() >= 2);
    let positions: Vec<i64> = saved.iter().map(|b| b.position_ms).collect();
    assert!(positions.contains(&1000));
    assert!(positions.contains(&5000));
    Ok(())
}
