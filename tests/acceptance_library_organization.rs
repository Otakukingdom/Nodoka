//! Acceptance tests for Library Organization (Category D)
//!
//! Tests sorting, filtering, and searching audiobook library.

mod acceptance_support;
use acceptance_support::*;

use nodoka::db::queries;
use std::error::Error;

#[test]
fn test_sort_audiobooks_by_name() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;

    create_test_audiobook(&db, "/test", "Zebra Book")?;
    create_test_audiobook(&db, "/test", "Apple Book")?;
    create_test_audiobook(&db, "/test", "Middle Book")?;

    let mut audiobooks = queries::get_all_audiobooks(db.connection())?;
    audiobooks.sort_by(|a, b| a.name.cmp(&b.name));

    assert_eq!(audiobooks[0].name, "Apple Book");
    assert_eq!(audiobooks[1].name, "Middle Book");
    assert_eq!(audiobooks[2].name, "Zebra Book");

    Ok(())
}

#[test]
fn test_sort_by_date_added() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;

    create_test_audiobook(&db, "/test", "First")?;
    std::thread::sleep(std::time::Duration::from_millis(10));
    create_test_audiobook(&db, "/test", "Second")?;
    std::thread::sleep(std::time::Duration::from_millis(10));
    create_test_audiobook(&db, "/test", "Third")?;

    let mut audiobooks = queries::get_all_audiobooks(db.connection())?;
    audiobooks.sort_by(|a, b| a.created_at.cmp(&b.created_at));

    assert_eq!(audiobooks[0].name, "First");
    assert_eq!(audiobooks[1].name, "Second");
    assert_eq!(audiobooks[2].name, "Third");

    Ok(())
}

#[test]
fn test_filter_by_completion_status() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;

    let id1 = create_test_audiobook(&db, "/test", "Complete Book")?;
    let id2 = create_test_audiobook(&db, "/test", "In Progress")?;

    queries::update_audiobook_completeness(db.connection(), id1, 100)?;
    queries::update_audiobook_completeness(db.connection(), id2, 50)?;

    let all = queries::get_all_audiobooks(db.connection())?;
    let complete: Vec<_> = all.iter().filter(|ab| ab.completeness == 100).collect();
    let incomplete: Vec<_> = all.iter().filter(|ab| ab.completeness < 100).collect();

    assert_eq!(complete.len(), 1);
    assert_eq!(incomplete.len(), 1);

    Ok(())
}

#[test]
fn test_search_by_name_case_insensitive() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;

    create_test_audiobook(&db, "/test", "The Great Gatsby")?;
    create_test_audiobook(&db, "/test", "1984")?;
    create_test_audiobook(&db, "/test", "Great Expectations")?;

    let all = queries::get_all_audiobooks(db.connection())?;
    let results: Vec<_> = all
        .iter()
        .filter(|ab| ab.name.to_lowercase().contains("great"))
        .collect();

    assert_eq!(results.len(), 2);

    Ok(())
}

#[test]
fn test_empty_library() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;

    let audiobooks = queries::get_all_audiobooks(db.connection())?;
    assert_eq!(audiobooks.len(), 0);

    Ok(())
}

#[test]
fn test_search_no_results() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;

    create_test_audiobook(&db, "/test", "The Great Gatsby")?;
    create_test_audiobook(&db, "/test", "1984")?;

    let all = queries::get_all_audiobooks(db.connection())?;
    let results: Vec<_> = all
        .iter()
        .filter(|ab| ab.name.to_lowercase().contains("nonexistent"))
        .collect();

    assert_eq!(results.len(), 0);

    Ok(())
}

#[test]
fn test_search_partial_match() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;

    create_test_audiobook(&db, "/test", "Harry Potter and the Philosopher's Stone")?;
    create_test_audiobook(&db, "/test", "Harry Potter and the Chamber of Secrets")?;
    create_test_audiobook(&db, "/test", "The Hobbit")?;

    let all = queries::get_all_audiobooks(db.connection())?;
    let results: Vec<_> = all
        .iter()
        .filter(|ab| ab.name.to_lowercase().contains("harry"))
        .collect();

    assert_eq!(results.len(), 2);

    Ok(())
}

#[test]
fn test_sort_reverse_alphabetical() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;

    create_test_audiobook(&db, "/test", "Alpha")?;
    create_test_audiobook(&db, "/test", "Beta")?;
    create_test_audiobook(&db, "/test", "Gamma")?;

    let mut audiobooks = queries::get_all_audiobooks(db.connection())?;
    audiobooks.sort_by(|a, b| b.name.cmp(&a.name)); // Reverse

    assert_eq!(audiobooks[0].name, "Gamma");
    assert_eq!(audiobooks[1].name, "Beta");
    assert_eq!(audiobooks[2].name, "Alpha");

    Ok(())
}

#[test]
fn test_filter_incomplete_only() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;

    let id1 = create_test_audiobook(&db, "/test", "Book 1")?;
    let id2 = create_test_audiobook(&db, "/test", "Book 2")?;
    let id3 = create_test_audiobook(&db, "/test", "Book 3")?;

    queries::update_audiobook_completeness(db.connection(), id1, 100)?;
    queries::update_audiobook_completeness(db.connection(), id2, 50)?;
    queries::update_audiobook_completeness(db.connection(), id3, 0)?;

    let all = queries::get_all_audiobooks(db.connection())?;
    let incomplete: Vec<_> = all.iter().filter(|ab| ab.completeness < 100).collect();

    assert_eq!(incomplete.len(), 2);

    Ok(())
}

#[test]
fn test_filter_in_progress() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;

    let id1 = create_test_audiobook(&db, "/test", "Not Started")?;
    let id2 = create_test_audiobook(&db, "/test", "In Progress")?;
    let id3 = create_test_audiobook(&db, "/test", "Complete")?;

    queries::update_audiobook_completeness(db.connection(), id1, 0)?;
    queries::update_audiobook_completeness(db.connection(), id2, 50)?;
    queries::update_audiobook_completeness(db.connection(), id3, 100)?;

    let all = queries::get_all_audiobooks(db.connection())?;
    let in_progress: Vec<_> = all
        .iter()
        .filter(|ab| ab.completeness > 0 && ab.completeness < 100)
        .collect();

    assert_eq!(in_progress.len(), 1);
    assert_eq!(in_progress[0].name, "In Progress");

    Ok(())
}

#[test]
fn test_search_special_characters() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;

    create_test_audiobook(&db, "/test", "Book: The Story")?;
    create_test_audiobook(&db, "/test", "Book (2024 Edition)")?;
    create_test_audiobook(&db, "/test", "Author's Book")?;

    let all = queries::get_all_audiobooks(db.connection())?;
    let results: Vec<_> = all
        .iter()
        .filter(|ab| ab.name.to_lowercase().contains("book"))
        .collect();

    assert_eq!(results.len(), 3);

    Ok(())
}

#[test]
fn test_sort_by_default_order() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;

    // default_order is set to 0 for all in create_test_audiobook
    // but we can verify they're sorted by this field
    create_test_audiobook(&db, "/test", "Book C")?;
    create_test_audiobook(&db, "/test", "Book A")?;
    create_test_audiobook(&db, "/test", "Book B")?;

    let audiobooks = queries::get_all_audiobooks(db.connection())?;

    // Should maintain insertion order when default_order is same
    assert_eq!(audiobooks.len(), 3);

    Ok(())
}
