//! Acceptance tests for Progress Tracking and Persistence (Category B)
//!
//! Tests playback position saving, restoration across restarts, and crash recovery.

mod acceptance_support;
use acceptance_support::*;

use nodoka::db::queries;
use std::error::Error;
use temp_dir::TempDir;

#[test]
fn test_progress_saved_on_file_update() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let audiobook_id = create_test_audiobook(&db, "/test", "Book")?;

    let file_path = "/test/Book/chapter1.mp3";
    insert_test_file(&db, audiobook_id, file_path)?;

    // Simulate playback progress
    queries::update_file_progress(db.connection(), file_path, 1500.0, 42)?;

    let files = queries::get_audiobook_files(db.connection(), audiobook_id)?;
    assert_eq!(files[0].seek_position, Some(1500));
    assert_eq!(files[0].completeness, 42);

    Ok(())
}

#[test]
fn test_progress_persists_across_restarts() -> Result<(), Box<dyn Error>> {
    let temp_db_dir = TempDir::new()?;
    let db_path = temp_db_dir.path().join("progress_test.db");

    // First session: save progress
    {
        let db = nodoka::Database::open_with_path(&db_path)?;
        nodoka::db::initialize(db.connection())?;

        let audiobook_id = create_test_audiobook(&db, "/test", "Book")?;
        let file_path = "/test/Book/chapter1.mp3";
        insert_test_file(&db, audiobook_id, file_path)?;

        queries::update_file_progress(db.connection(), file_path, 2000.0, 50)?;
    }

    // Second session: restore progress
    {
        let db = nodoka::Database::open_with_path(&db_path)?;

        let audiobooks = queries::get_all_audiobooks(db.connection())?;
        let audiobook_id = audiobooks[0].id.ok_or("No ID")?;

        let files = queries::get_audiobook_files(db.connection(), audiobook_id)?;
        assert_eq!(files[0].seek_position, Some(2000));
        assert_eq!(files[0].completeness, 50);
    }

    Ok(())
}

#[test]
fn test_independent_progress_per_file() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let audiobook_id = create_test_audiobook(&db, "/test", "Book")?;

    let file1 = "/test/Book/chapter1.mp3";
    let file2 = "/test/Book/chapter2.mp3";

    insert_test_file(&db, audiobook_id, file1)?;
    insert_test_file(&db, audiobook_id, file2)?;

    queries::update_file_progress(db.connection(), file1, 1000.0, 100)?;
    queries::update_file_progress(db.connection(), file2, 500.0, 25)?;

    let files = queries::get_audiobook_files(db.connection(), audiobook_id)?;

    let f1 = files
        .iter()
        .find(|f| f.full_path == file1)
        .ok_or("File 1 not found")?;
    let f2 = files
        .iter()
        .find(|f| f.full_path == file2)
        .ok_or("File 2 not found")?;

    assert_eq!(f1.completeness, 100);
    assert_eq!(f2.completeness, 25);

    Ok(())
}

#[test]
fn test_reset_progress_clears_all() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let audiobook_id = create_test_audiobook(&db, "/test", "Book")?;

    let file_path = "/test/Book/chapter1.mp3";
    insert_test_file(&db, audiobook_id, file_path)?;

    // Set some progress
    queries::update_file_progress(db.connection(), file_path, 5000.0, 100)?;
    queries::update_audiobook_completeness(db.connection(), audiobook_id, 100)?;

    // Reset all progress
    queries::reset_audiobook_progress(db.connection(), audiobook_id)?;

    let audiobook = queries::get_audiobook_by_id(db.connection(), audiobook_id)?
        .ok_or("Audiobook not found")?;
    let files = queries::get_audiobook_files(db.connection(), audiobook_id)?;

    assert_eq!(audiobook.completeness, 0);
    // After reset, seek_position should be None or 0
    assert!(files[0].seek_position.is_none() || files[0].seek_position == Some(0));
    assert_eq!(files[0].completeness, 0);

    Ok(())
}

#[test]
fn test_selected_file_tracking() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let audiobook_id = create_test_audiobook(&db, "/test", "Book")?;

    // Set selected file
    queries::update_audiobook_selected_file(
        db.connection(),
        audiobook_id,
        Some("/test/Book/chapter2.mp3"),
    )?;

    let audiobook = queries::get_audiobook_by_id(db.connection(), audiobook_id)?
        .ok_or("Audiobook not found")?;

    assert_eq!(
        audiobook.selected_file,
        Some("/test/Book/chapter2.mp3".to_string())
    );

    Ok(())
}

#[test]
fn test_selected_file_persists_across_restarts() -> Result<(), Box<dyn Error>> {
    let temp_db_dir = TempDir::new()?;
    let db_path = temp_db_dir.path().join("selected_file_test.db");

    // First session: set selected file
    {
        let db = nodoka::Database::open_with_path(&db_path)?;
        nodoka::db::initialize(db.connection())?;

        let audiobook_id = create_test_audiobook(&db, "/test", "Book")?;
        queries::update_audiobook_selected_file(
            db.connection(),
            audiobook_id,
            Some("/test/Book/chapter3.mp3"),
        )?;
    }

    // Second session: restore selected file
    {
        let db = nodoka::Database::open_with_path(&db_path)?;
        let audiobooks = queries::get_all_audiobooks(db.connection())?;
        let audiobook = audiobooks.first().ok_or("No audiobook")?;

        assert_eq!(
            audiobook.selected_file,
            Some("/test/Book/chapter3.mp3".to_string())
        );
    }

    Ok(())
}

#[test]
fn test_completeness_percentage_tracked() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let audiobook_id = create_test_audiobook(&db, "/test", "Book")?;

    // Update completeness
    queries::update_audiobook_completeness(db.connection(), audiobook_id, 75)?;

    let audiobook = queries::get_audiobook_by_id(db.connection(), audiobook_id)?
        .ok_or("Audiobook not found")?;

    assert_eq!(audiobook.completeness, 75);

    Ok(())
}

#[test]
fn test_multiple_audiobooks_independent_progress() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;

    let book1_id = create_test_audiobook(&db, "/test", "Book 1")?;
    let book2_id = create_test_audiobook(&db, "/test", "Book 2")?;

    queries::update_audiobook_completeness(db.connection(), book1_id, 100)?;
    queries::update_audiobook_completeness(db.connection(), book2_id, 25)?;

    let book1 =
        queries::get_audiobook_by_id(db.connection(), book1_id)?.ok_or("Book 1 not found")?;
    let book2 =
        queries::get_audiobook_by_id(db.connection(), book2_id)?.ok_or("Book 2 not found")?;

    assert_eq!(book1.completeness, 100);
    assert_eq!(book2.completeness, 25);

    Ok(())
}

#[test]
fn test_progress_with_multiple_files() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let audiobook_id = create_test_audiobook(&db, "/test", "Multi-File Book")?;

    // Insert multiple files
    for i in 1..=5 {
        let file_path = format!("/test/Multi-File Book/chapter{i}.mp3");
        insert_test_file(&db, audiobook_id, &file_path)?;
    }

    // Set different progress for each file
    queries::update_file_progress(
        db.connection(),
        "/test/Multi-File Book/chapter1.mp3",
        3600.0,
        100,
    )?;
    queries::update_file_progress(
        db.connection(),
        "/test/Multi-File Book/chapter2.mp3",
        1800.0,
        50,
    )?;
    queries::update_file_progress(
        db.connection(),
        "/test/Multi-File Book/chapter3.mp3",
        0.0,
        0,
    )?;

    let files = queries::get_audiobook_files(db.connection(), audiobook_id)?;

    assert_eq!(files[0].completeness, 100);
    assert_eq!(files[1].completeness, 50);
    assert_eq!(files[2].completeness, 0);

    Ok(())
}

#[test]
fn test_file_position_precision() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let audiobook_id = create_test_audiobook(&db, "/test", "Book")?;

    let file_path = "/test/Book/chapter1.mp3";
    insert_test_file(&db, audiobook_id, file_path)?;

    // Test precise position
    queries::update_file_progress(db.connection(), file_path, 12345.678, 33)?;

    let files = queries::get_audiobook_files(db.connection(), audiobook_id)?;

    // Position stored as milliseconds
    assert!(files[0].seek_position.is_some());
    let position = files[0].seek_position.ok_or("No position")?;

    // Should be close to 12345 (allowing for conversion precision)
    assert!((position - 12345).abs() < 10);

    Ok(())
}

#[test]
fn test_periodic_auto_save_simulation() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let audiobook_id = create_test_audiobook(&db, "/test", "Book")?;

    let file_path = "/test/Book/chapter1.mp3";
    insert_test_file(&db, audiobook_id, file_path)?;

    // Simulate periodic auto-save during playback
    // Position updates should occur every ~5 seconds during playback

    // Initial position
    queries::update_file_progress(db.connection(), file_path, 0.0, 0)?;

    // After 5 seconds of playback
    queries::update_file_progress(db.connection(), file_path, 5000.0, 5)?;
    let files = queries::get_audiobook_files(db.connection(), audiobook_id)?;
    assert_eq!(files[0].seek_position, Some(5000));

    // After 10 seconds
    queries::update_file_progress(db.connection(), file_path, 10000.0, 10)?;
    let files = queries::get_audiobook_files(db.connection(), audiobook_id)?;
    assert_eq!(files[0].seek_position, Some(10000));

    // After 15 seconds
    queries::update_file_progress(db.connection(), file_path, 15000.0, 15)?;
    let files = queries::get_audiobook_files(db.connection(), audiobook_id)?;
    assert_eq!(files[0].seek_position, Some(15000));

    // Progress should be persisted and recoverable even without explicit pause/stop
    Ok(())
}

#[test]
fn test_crash_recovery_via_periodic_save() -> Result<(), Box<dyn Error>> {
    let temp_db_dir = TempDir::new()?;
    let db_path = temp_db_dir.path().join("crash_recovery_test.db");

    // Simulate application session with periodic saves
    {
        let db = nodoka::Database::open_with_path(&db_path)?;
        nodoka::db::initialize(db.connection())?;

        let audiobook_id = create_test_audiobook(&db, "/test", "Book")?;
        let file_path = "/test/Book/chapter1.mp3";
        insert_test_file(&db, audiobook_id, file_path)?;

        // Simulate periodic saves during playback (every 5 seconds)
        queries::update_file_progress(db.connection(), file_path, 5000.0, 5)?;
        queries::update_file_progress(db.connection(), file_path, 10000.0, 10)?;
        queries::update_file_progress(db.connection(), file_path, 15000.0, 15)?;

        // Simulate crash - no explicit close/cleanup
        // Database should have persisted the last auto-save
    }

    // Recover after "crash" - open database again
    {
        let db = nodoka::Database::open_with_path(&db_path)?;

        let audiobooks = queries::get_all_audiobooks(db.connection())?;
        let audiobook_id = audiobooks[0].id.ok_or("No ID")?;

        let files = queries::get_audiobook_files(db.connection(), audiobook_id)?;

        // Should have recovered the last auto-saved position (15 seconds)
        assert_eq!(files[0].seek_position, Some(15000));
        assert_eq!(files[0].completeness, 15);
    }

    Ok(())
}
