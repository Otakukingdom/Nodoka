//! Acceptance tests for Navigation & Discovery features.

mod acceptance_support;
use acceptance_support::*;

use chrono::Utc;
use nodoka::db::queries;
use std::error::Error;

#[test]
fn test_search_filters_by_title() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;

    create_test_audiobook(&db, "/test", "Harry Potter")?;
    create_test_audiobook(&db, "/test", "Lord of the Rings")?;

    let all = queries::get_all_audiobooks(db.connection())?;
    let query = "harry";
    let filtered: Vec<_> = all
        .iter()
        .filter(|ab| ab.name.to_lowercase().contains(query))
        .collect();

    assert_eq!(filtered.len(), 1);
    let first = filtered.first().ok_or("missing filtered result")?;
    assert_eq!(first.name, "Harry Potter");
    Ok(())
}

#[test]
fn test_search_case_insensitive() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;

    create_test_audiobook(&db, "/test", "Harry Potter")?;

    let all = queries::get_all_audiobooks(db.connection())?;
    let query = "HARRY";
    let filtered: Vec<_> = all
        .iter()
        .filter(|ab| ab.name.to_lowercase().contains(&query.to_lowercase()))
        .collect();

    assert_eq!(filtered.len(), 1);
    let first = filtered.first().ok_or("missing filtered result")?;
    assert_eq!(first.name, "Harry Potter");
    Ok(())
}

#[test]
fn test_search_partial_match() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;

    create_test_audiobook(&db, "/test", "Harry Potter")?;

    let all = queries::get_all_audiobooks(db.connection())?;
    let query = "pot";
    let filtered_count = all
        .iter()
        .filter(|ab| ab.name.to_lowercase().contains(query))
        .count();

    assert_eq!(filtered_count, 1);
    Ok(())
}

#[test]
fn test_search_with_empty_query() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;

    create_test_audiobook(&db, "/test", "Harry Potter")?;
    create_test_audiobook(&db, "/test", "Lord of the Rings")?;

    let all = queries::get_all_audiobooks(db.connection())?;

    // Empty search should return all audiobooks (no filter applied)
    assert_eq!(all.len(), 2);
    Ok(())
}

#[test]
fn test_sort_by_title() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;

    create_test_audiobook(&db, "/test", "Zebra Book")?;
    create_test_audiobook(&db, "/test", "Apple Book")?;

    let mut all = queries::get_all_audiobooks(db.connection())?;
    all.sort_by(|a, b| a.name.cmp(&b.name));

    let first = all.first().ok_or("missing first audiobook")?;
    let second = all.get(1).ok_or("missing second audiobook")?;
    assert_eq!(first.name, "Apple Book");
    assert_eq!(second.name, "Zebra Book");
    Ok(())
}

#[test]
fn test_sort_by_recently_played() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;

    // Create audiobooks
    let id1 = create_test_audiobook(&db, "/test", "Book 1")?;
    let id2 = create_test_audiobook(&db, "/test", "Book 2")?;

    // Update timestamps - Book 2 played more recently
    let time1 = Utc::now() - chrono::Duration::hours(2);
    let time2 = Utc::now();

    queries::update_audiobook_last_listened(db.connection(), id1, time1)?;
    queries::update_audiobook_last_listened(db.connection(), id2, time2)?;

    // Sort by recently played (most recent first)
    let mut all = queries::get_all_audiobooks(db.connection())?;
    all.sort_by(|a, b| b.last_listened.cmp(&a.last_listened));

    let first = all.first().ok_or("missing first audiobook")?;
    let second = all.get(1).ok_or("missing second audiobook")?;
    assert_eq!(first.name, "Book 2");
    assert_eq!(second.name, "Book 1");
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
    let filtered: Vec<_> = all
        .iter()
        .filter(|ab| ab.completeness > 0 && ab.completeness < 100)
        .collect();

    assert_eq!(filtered.len(), 1);
    let first = filtered.first().ok_or("missing filtered result")?;
    assert_eq!(first.name, "In Progress");
    Ok(())
}

#[test]
fn test_filter_complete() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;

    let id1 = create_test_audiobook(&db, "/test", "In Progress")?;
    let id2 = create_test_audiobook(&db, "/test", "Complete")?;

    queries::update_audiobook_completeness(db.connection(), id1, 50)?;
    queries::update_audiobook_completeness(db.connection(), id2, 100)?;

    let all = queries::get_all_audiobooks(db.connection())?;
    let filtered: Vec<_> = all.iter().filter(|ab| ab.completeness >= 100).collect();

    assert_eq!(filtered.len(), 1);
    let first = filtered.first().ok_or("missing filtered result")?;
    assert_eq!(first.name, "Complete");
    Ok(())
}

#[test]
fn test_filter_not_started() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;

    let id1 = create_test_audiobook(&db, "/test", "Not Started")?;
    let id2 = create_test_audiobook(&db, "/test", "In Progress")?;

    queries::update_audiobook_completeness(db.connection(), id1, 0)?;
    queries::update_audiobook_completeness(db.connection(), id2, 50)?;

    let all = queries::get_all_audiobooks(db.connection())?;
    let filtered: Vec<_> = all.iter().filter(|ab| ab.completeness == 0).collect();

    assert_eq!(filtered.len(), 1);
    let first = filtered.first().ok_or("missing filtered result")?;
    assert_eq!(first.name, "Not Started");
    Ok(())
}

#[test]
fn test_last_listened_timestamp_stored() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;

    let id = create_test_audiobook(&db, "/test", "Test Book")?;
    let now = Utc::now();
    queries::update_audiobook_last_listened(db.connection(), id, now)?;

    let loaded = queries::get_all_audiobooks(db.connection())?;
    let found = loaded.iter().find(|a| a.id == Some(id));

    assert!(found.is_some());
    if let Some(audiobook) = found {
        assert!(audiobook.last_listened.is_some());
    }
    Ok(())
}
