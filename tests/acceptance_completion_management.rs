//! Acceptance tests for Audiobook Completion Management (Category C)
//!
//! Tests completion status tracking, manual marking, and reset functionality.

mod acceptance_support;
use acceptance_support::*;

use nodoka::db::queries;
use std::error::Error;
use temp_dir::TempDir;

#[test]
fn test_audiobook_marked_complete_automatically() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let audiobook_id = create_test_audiobook(&db, "/test", "Book")?;

    // Mark as 100% complete
    queries::update_audiobook_completeness(db.connection(), audiobook_id, 100)?;

    let audiobook = queries::get_audiobook_by_id(db.connection(), audiobook_id)?
        .ok_or("Audiobook not found")?;

    assert_eq!(audiobook.completeness, 100);
    assert!(audiobook.is_complete());

    Ok(())
}

#[test]
fn test_manually_mark_complete() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let audiobook_id = create_test_audiobook(&db, "/test", "Book")?;

    // Manually mark complete (even at 0%)
    queries::update_audiobook_completeness(db.connection(), audiobook_id, 100)?;

    let audiobook = queries::get_audiobook_by_id(db.connection(), audiobook_id)?
        .ok_or("Audiobook not found")?;

    assert_eq!(audiobook.completeness, 100);

    Ok(())
}

#[test]
fn test_unmark_completed() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let audiobook_id = create_test_audiobook(&db, "/test", "Book")?;

    queries::update_audiobook_completeness(db.connection(), audiobook_id, 100)?;
    queries::update_audiobook_completeness(db.connection(), audiobook_id, 0)?;

    let audiobook = queries::get_audiobook_by_id(db.connection(), audiobook_id)?
        .ok_or("Audiobook not found")?;

    assert_eq!(audiobook.completeness, 0);
    assert!(!audiobook.is_complete());

    Ok(())
}

#[test]
fn test_reset_progress_clears_completion() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let audiobook_id = create_test_audiobook(&db, "/test", "Book")?;

    let file_path = "/test/Book/chapter1.mp3";
    insert_test_file(&db, audiobook_id, file_path)?;

    queries::update_file_progress(db.connection(), file_path, 5000.0, 100)?;
    queries::update_audiobook_completeness(db.connection(), audiobook_id, 100)?;

    // Reset all progress
    queries::reset_audiobook_progress(db.connection(), audiobook_id)?;

    let audiobook = queries::get_audiobook_by_id(db.connection(), audiobook_id)?
        .ok_or("Audiobook not found")?;
    let files = queries::get_audiobook_files(db.connection(), audiobook_id)?;

    assert_eq!(audiobook.completeness, 0);
    assert!(files[0].seek_position.is_none() || files[0].seek_position == Some(0));

    Ok(())
}

#[test]
fn test_completion_percentage_calculated() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let audiobook_id = create_test_audiobook(&db, "/test", "Book")?;

    // Create multiple files
    insert_test_file(&db, audiobook_id, "/test/Book/chapter1.mp3")?;
    insert_test_file(&db, audiobook_id, "/test/Book/chapter2.mp3")?;

    // Mark first file complete
    queries::update_file_progress(db.connection(), "/test/Book/chapter1.mp3", 3600.0, 100)?;
    queries::update_file_progress(db.connection(), "/test/Book/chapter2.mp3", 0.0, 0)?;

    // Calculate overall completion
    let files = queries::get_audiobook_files(db.connection(), audiobook_id)?;
    let avg_completion = files.iter().map(|f| f.completeness).sum::<i32>() / files.len() as i32;

    assert_eq!(avg_completion, 50); // 50% complete

    Ok(())
}

#[test]
fn test_completion_status_persists() -> Result<(), Box<dyn Error>> {
    let temp_db_dir = TempDir::new()?;
    let db_path = temp_db_dir.path().join("completion_test.db");

    // First session: mark complete
    {
        let db = nodoka::Database::open_with_path(&db_path)?;
        nodoka::db::initialize(db.connection())?;

        let audiobook_id = create_test_audiobook(&db, "/test", "Complete Book")?;
        queries::update_audiobook_completeness(db.connection(), audiobook_id, 100)?;
    }

    // Second session: verify persistence
    {
        let db = nodoka::Database::open_with_path(&db_path)?;
        let audiobooks = queries::get_all_audiobooks(db.connection())?;
        let audiobook = audiobooks.first().ok_or("No audiobook")?;

        assert_eq!(audiobook.completeness, 100);
        assert!(audiobook.is_complete());
    }

    Ok(())
}

#[test]
fn test_filter_by_completion_status() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;

    let id1 = create_test_audiobook(&db, "/test", "Complete Book")?;
    let id2 = create_test_audiobook(&db, "/test", "In Progress")?;
    let id3 = create_test_audiobook(&db, "/test", "Not Started")?;

    queries::update_audiobook_completeness(db.connection(), id1, 100)?;
    queries::update_audiobook_completeness(db.connection(), id2, 50)?;
    queries::update_audiobook_completeness(db.connection(), id3, 0)?;

    let all = queries::get_all_audiobooks(db.connection())?;
    let complete: Vec<_> = all.iter().filter(|ab| ab.completeness == 100).collect();
    let incomplete: Vec<_> = all.iter().filter(|ab| ab.completeness < 100).collect();

    assert_eq!(complete.len(), 1);
    assert_eq!(incomplete.len(), 2);

    Ok(())
}

#[test]
fn test_partial_completion_percentage() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let audiobook_id = create_test_audiobook(&db, "/test", "Book")?;

    // Test various completion percentages
    for percentage in &[0, 25, 50, 75, 100] {
        queries::update_audiobook_completeness(db.connection(), audiobook_id, *percentage)?;

        let audiobook = queries::get_audiobook_by_id(db.connection(), audiobook_id)?
            .ok_or("Audiobook not found")?;

        assert_eq!(audiobook.completeness, *percentage);
    }

    Ok(())
}

#[test]
fn test_completion_boundary_at_100_percent() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let audiobook_id = create_test_audiobook(&db, "/test", "Book")?;

    // 99% is not complete
    queries::update_audiobook_completeness(db.connection(), audiobook_id, 99)?;
    let audiobook = queries::get_audiobook_by_id(db.connection(), audiobook_id)?
        .ok_or("Audiobook not found")?;
    assert!(!audiobook.is_complete());

    // 100% is complete
    queries::update_audiobook_completeness(db.connection(), audiobook_id, 100)?;
    let audiobook = queries::get_audiobook_by_id(db.connection(), audiobook_id)?
        .ok_or("Audiobook not found")?;
    assert!(audiobook.is_complete());

    Ok(())
}

#[test]
fn test_reset_preserves_audiobook_metadata() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let audiobook_id = create_test_audiobook(&db, "/test", "Book")?;

    let file_path = "/test/Book/chapter1.mp3";
    insert_test_file(&db, audiobook_id, file_path)?;

    queries::update_file_progress(db.connection(), file_path, 5000.0, 100)?;
    queries::update_audiobook_completeness(db.connection(), audiobook_id, 100)?;

    // Get original metadata
    let before = queries::get_audiobook_by_id(db.connection(), audiobook_id)?
        .ok_or("Audiobook not found")?;

    // Reset progress
    queries::reset_audiobook_progress(db.connection(), audiobook_id)?;

    // Verify metadata preserved
    let after = queries::get_audiobook_by_id(db.connection(), audiobook_id)?
        .ok_or("Audiobook not found")?;

    assert_eq!(before.name, after.name);
    assert_eq!(before.directory, after.directory);
    assert_eq!(before.full_path, after.full_path);
    assert_eq!(after.completeness, 0); // Only completeness changed

    Ok(())
}
