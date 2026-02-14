//! Acceptance tests for Bookmarks (Category C)
//!
//! Tests bookmark creation, editing, deletion, and persistence.

mod acceptance_support;
use acceptance_support::*;

use nodoka::db::queries;
use nodoka::models::Bookmark;
use std::error::Error;
use temp_dir::TempDir;

#[test]
fn test_create_bookmark_at_position() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let audiobook_id = create_test_audiobook(&db, "/test", "Book")?;

    let bookmark = Bookmark::new(
        audiobook_id,
        "/test/Book/chapter1.mp3".to_string(),
        1500,
        "Important point".to_string(),
    );

    let bookmark_id = queries::insert_bookmark(db.connection(), &bookmark)?;
    assert!(bookmark_id > 0);

    Ok(())
}

#[test]
fn test_bookmark_with_label_and_note() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let audiobook_id = create_test_audiobook(&db, "/test", "Book")?;

    let bookmark = Bookmark::new(
        audiobook_id,
        "/test/Book/chapter1.mp3".to_string(),
        2000,
        "Chapter 1 Summary".to_string(),
    )
    .with_note(Some(
        "Important historical context discussed here".to_string(),
    ));

    let id = queries::insert_bookmark(db.connection(), &bookmark)?;

    let bookmarks = queries::get_bookmarks_for_audiobook(db.connection(), audiobook_id)?;
    let saved = bookmarks
        .iter()
        .find(|b| b.id == Some(id))
        .ok_or("Bookmark not found")?;

    assert_eq!(saved.label, "Chapter 1 Summary");
    assert_eq!(
        saved.note,
        Some("Important historical context discussed here".to_string())
    );

    Ok(())
}

#[test]
fn test_bookmarks_listed_chronologically() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let audiobook_id = create_test_audiobook(&db, "/test", "Book")?;

    // Create bookmarks at different positions
    let b1 = Bookmark::new(
        audiobook_id,
        "/test/Book/chapter1.mp3".to_string(),
        500,
        "First".to_string(),
    );
    let b2 = Bookmark::new(
        audiobook_id,
        "/test/Book/chapter1.mp3".to_string(),
        1500,
        "Second".to_string(),
    );
    let b3 = Bookmark::new(
        audiobook_id,
        "/test/Book/chapter2.mp3".to_string(),
        300,
        "Third".to_string(),
    );

    queries::insert_bookmark(db.connection(), &b1)?;
    std::thread::sleep(std::time::Duration::from_millis(10));
    queries::insert_bookmark(db.connection(), &b2)?;
    std::thread::sleep(std::time::Duration::from_millis(10));
    queries::insert_bookmark(db.connection(), &b3)?;

    let bookmarks = queries::get_bookmarks_for_audiobook(db.connection(), audiobook_id)?;

    assert_eq!(bookmarks.len(), 3);
    assert_eq!(bookmarks[0].label, "First");
    assert_eq!(bookmarks[1].label, "Second");
    assert_eq!(bookmarks[2].label, "Third");

    Ok(())
}

#[test]
fn test_delete_bookmark() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let audiobook_id = create_test_audiobook(&db, "/test", "Book")?;

    let bookmark = Bookmark::new(
        audiobook_id,
        "/test/Book/chapter1.mp3".to_string(),
        1000,
        "Test".to_string(),
    );
    let id = queries::insert_bookmark(db.connection(), &bookmark)?;

    queries::delete_bookmark(db.connection(), id)?;

    let bookmarks = queries::get_bookmarks_for_audiobook(db.connection(), audiobook_id)?;
    assert_eq!(bookmarks.len(), 0);

    Ok(())
}

#[test]
fn test_edit_bookmark_label_and_note() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let audiobook_id = create_test_audiobook(&db, "/test", "Book")?;

    let bookmark = Bookmark::new(
        audiobook_id,
        "/test/Book/chapter1.mp3".to_string(),
        1000,
        "Original".to_string(),
    );
    let id = queries::insert_bookmark(db.connection(), &bookmark)?;

    let mut updated = queries::get_bookmarks_for_audiobook(db.connection(), audiobook_id)?
        .into_iter()
        .find(|b| b.id == Some(id))
        .ok_or("Not found")?;

    updated.label = "Updated Label".to_string();
    updated.note = Some("New note".to_string());

    queries::update_bookmark(db.connection(), &updated)?;

    let bookmarks = queries::get_bookmarks_for_audiobook(db.connection(), audiobook_id)?;
    assert_eq!(bookmarks[0].label, "Updated Label");
    assert_eq!(bookmarks[0].note, Some("New note".to_string()));

    Ok(())
}

#[test]
fn test_bookmarks_persist_across_restarts() -> Result<(), Box<dyn Error>> {
    let temp_db_dir = TempDir::new()?;
    let db_path = temp_db_dir.path().join("bookmarks_test.db");

    // First session: create bookmark
    {
        let db = nodoka::Database::open_with_path(&db_path)?;
        nodoka::db::initialize(db.connection())?;

        let audiobook_id = create_test_audiobook(&db, "/test", "Book")?;
        let bookmark = Bookmark::new(
            audiobook_id,
            "/test/Book/chapter1.mp3".to_string(),
            3000,
            "Persistent Bookmark".to_string(),
        );
        queries::insert_bookmark(db.connection(), &bookmark)?;
    }

    // Second session: verify bookmark persisted
    {
        let db = nodoka::Database::open_with_path(&db_path)?;
        let audiobooks = queries::get_all_audiobooks(db.connection())?;
        let audiobook_id = audiobooks[0].id.ok_or("No ID")?;

        let bookmarks = queries::get_bookmarks_for_audiobook(db.connection(), audiobook_id)?;
        assert_eq!(bookmarks.len(), 1);
        assert_eq!(bookmarks[0].label, "Persistent Bookmark");
        assert_eq!(bookmarks[0].position_ms, 3000);
    }

    Ok(())
}

#[test]
fn test_multiple_bookmarks_same_audiobook() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let audiobook_id = create_test_audiobook(&db, "/test", "Book")?;

    // Create multiple bookmarks
    for i in 0..5 {
        let bookmark = Bookmark::new(
            audiobook_id,
            format!("/test/Book/chapter{}.mp3", i % 2 + 1),
            (i * 1000) as i64,
            format!("Bookmark {}", i + 1),
        );
        queries::insert_bookmark(db.connection(), &bookmark)?;
    }

    let bookmarks = queries::get_bookmarks_for_audiobook(db.connection(), audiobook_id)?;
    assert_eq!(bookmarks.len(), 5);

    Ok(())
}

#[test]
fn test_bookmarks_per_audiobook_isolated() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;

    let book1_id = create_test_audiobook(&db, "/test", "Book 1")?;
    let book2_id = create_test_audiobook(&db, "/test", "Book 2")?;

    // Add bookmarks to each
    let b1 = Bookmark::new(
        book1_id,
        "/test/Book 1/chapter1.mp3".to_string(),
        1000,
        "Book 1 Mark".to_string(),
    );
    let b2 = Bookmark::new(
        book2_id,
        "/test/Book 2/chapter1.mp3".to_string(),
        2000,
        "Book 2 Mark".to_string(),
    );

    queries::insert_bookmark(db.connection(), &b1)?;
    queries::insert_bookmark(db.connection(), &b2)?;

    // Verify isolation
    let book1_marks = queries::get_bookmarks_for_audiobook(db.connection(), book1_id)?;
    let book2_marks = queries::get_bookmarks_for_audiobook(db.connection(), book2_id)?;

    assert_eq!(book1_marks.len(), 1);
    assert_eq!(book2_marks.len(), 1);
    assert_eq!(book1_marks[0].label, "Book 1 Mark");
    assert_eq!(book2_marks[0].label, "Book 2 Mark");

    Ok(())
}

#[test]
fn test_bookmark_without_note() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let audiobook_id = create_test_audiobook(&db, "/test", "Book")?;

    let bookmark = Bookmark::new(
        audiobook_id,
        "/test/Book/chapter1.mp3".to_string(),
        1000,
        "Label Only".to_string(),
    );

    let id = queries::insert_bookmark(db.connection(), &bookmark)?;

    let bookmarks = queries::get_bookmarks_for_audiobook(db.connection(), audiobook_id)?;
    let saved = bookmarks
        .iter()
        .find(|b| b.id == Some(id))
        .ok_or("Not found")?;

    assert_eq!(saved.label, "Label Only");
    assert!(saved.note.is_none());

    Ok(())
}

#[test]
fn test_bookmark_position_stored_accurately() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let audiobook_id = create_test_audiobook(&db, "/test", "Book")?;

    let bookmark = Bookmark::new(
        audiobook_id,
        "/test/Book/chapter1.mp3".to_string(),
        123456,
        "Precise Position".to_string(),
    );

    let id = queries::insert_bookmark(db.connection(), &bookmark)?;

    let bookmarks = queries::get_bookmarks_for_audiobook(db.connection(), audiobook_id)?;
    let saved = bookmarks
        .iter()
        .find(|b| b.id == Some(id))
        .ok_or("Not found")?;

    assert_eq!(saved.position_ms, 123456);

    Ok(())
}

#[test]
fn test_bookmark_file_path_tracked() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let audiobook_id = create_test_audiobook(&db, "/test", "Book")?;

    let bookmark = Bookmark::new(
        audiobook_id,
        "/test/Book/chapter5.mp3".to_string(),
        1000,
        "Chapter 5 Mark".to_string(),
    );

    let id = queries::insert_bookmark(db.connection(), &bookmark)?;

    let bookmarks = queries::get_bookmarks_for_audiobook(db.connection(), audiobook_id)?;
    let saved = bookmarks
        .iter()
        .find(|b| b.id == Some(id))
        .ok_or("Not found")?;

    assert_eq!(saved.file_path, "/test/Book/chapter5.mp3");

    Ok(())
}
