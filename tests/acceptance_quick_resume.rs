//! Acceptance tests for Quick Resume (`PlayLastListened`) feature.
//!
//! Tests the primary use case: users opening the app and immediately continuing
//! where they left off.

#![allow(clippy::indexing_slicing)] // Test assertions use indexing for clarity
#![allow(clippy::unwrap_used)] // Test assertions use unwrap for clarity

mod acceptance_support;
use acceptance_support::*;

use chrono::Utc;
use nodoka::db::queries;
use std::error::Error;

#[test]
fn test_play_last_listened_finds_most_recent() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;

    // Create three audiobooks with different play times
    let id1 = create_test_audiobook(&db, "/test", "Old Book")?;
    let id2 = create_test_audiobook(&db, "/test", "Recent Book")?;
    let id3 = create_test_audiobook(&db, "/test", "Current Book")?;

    let day_ago = Utc::now() - chrono::Duration::days(1);
    let hour_ago = Utc::now() - chrono::Duration::hours(1);
    let now = Utc::now();

    queries::update_audiobook_last_listened(db.connection(), id1, day_ago)?;
    queries::update_audiobook_last_listened(db.connection(), id2, hour_ago)?;
    queries::update_audiobook_last_listened(db.connection(), id3, now)?;

    // Load all audiobooks
    let audiobooks = queries::get_all_audiobooks(db.connection())?;

    // Find most recently played (simulating PlayLastListened logic)
    let last = audiobooks
        .iter()
        .filter(|ab| ab.last_listened.is_some())
        .max_by_key(|ab| ab.last_listened);

    assert!(last.is_some(), "Should find a most recent audiobook");
    assert_eq!(last.unwrap().name, "Current Book");

    Ok(())
}

#[test]
fn test_play_last_listened_ignores_never_played() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;

    let _id1 = create_test_audiobook(&db, "/test", "Never Played")?;
    let id2 = create_test_audiobook(&db, "/test", "Played Once")?;

    let now = Utc::now();
    queries::update_audiobook_last_listened(db.connection(), id2, now)?;

    let audiobooks = queries::get_all_audiobooks(db.connection())?;
    let last = audiobooks
        .iter()
        .filter(|ab| ab.last_listened.is_some())
        .max_by_key(|ab| ab.last_listened);

    assert!(last.is_some());
    assert_eq!(last.unwrap().name, "Played Once");

    Ok(())
}

#[test]
fn test_play_last_listened_handles_empty_library() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;

    let audiobooks = queries::get_all_audiobooks(db.connection())?;
    assert!(audiobooks.is_empty(), "Library should be empty");

    let last = audiobooks
        .iter()
        .filter(|ab| ab.last_listened.is_some())
        .max_by_key(|ab| ab.last_listened);

    assert!(last.is_none(), "Should handle empty library gracefully");

    Ok(())
}

#[test]
fn test_play_last_listened_handles_no_played_audiobooks() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;

    // Create audiobooks but don't set last_listened
    let _id1 = create_test_audiobook(&db, "/test", "Book 1")?;
    let _id2 = create_test_audiobook(&db, "/test", "Book 2")?;

    let audiobooks = queries::get_all_audiobooks(db.connection())?;
    assert_eq!(audiobooks.len(), 2, "Should have 2 audiobooks");

    let last = audiobooks
        .iter()
        .filter(|ab| ab.last_listened.is_some())
        .max_by_key(|ab| ab.last_listened);

    assert!(
        last.is_none(),
        "Should return None when no audiobooks have been played"
    );

    Ok(())
}

#[test]
fn test_continue_listening_card_shows_last_audiobook() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;

    let id1 = create_test_audiobook(&db, "/test", "Book 1")?;
    let id2 = create_test_audiobook(&db, "/test", "Book 2")?;

    let older = Utc::now() - chrono::Duration::hours(2);
    let newer = Utc::now();

    queries::update_audiobook_last_listened(db.connection(), id1, older)?;
    queries::update_audiobook_last_listened(db.connection(), id2, newer)?;

    let audiobooks = queries::get_all_audiobooks(db.connection())?;
    let last = audiobooks
        .iter()
        .filter(|ab| ab.last_listened.is_some())
        .max_by_key(|ab| ab.last_listened);

    assert!(last.is_some());
    assert_eq!(
        last.unwrap().name,
        "Book 2",
        "Continue listening card should display most recent audiobook"
    );

    Ok(())
}

#[test]
fn test_play_last_listened_with_multiple_plays_same_day() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;

    let id1 = create_test_audiobook(&db, "/test", "Morning Book")?;
    let id2 = create_test_audiobook(&db, "/test", "Afternoon Book")?;
    let id3 = create_test_audiobook(&db, "/test", "Evening Book")?;

    let morning = Utc::now() - chrono::Duration::hours(8);
    let afternoon = Utc::now() - chrono::Duration::hours(4);
    let evening = Utc::now() - chrono::Duration::hours(1);

    queries::update_audiobook_last_listened(db.connection(), id1, morning)?;
    queries::update_audiobook_last_listened(db.connection(), id2, afternoon)?;
    queries::update_audiobook_last_listened(db.connection(), id3, evening)?;

    let audiobooks = queries::get_all_audiobooks(db.connection())?;
    let last = audiobooks
        .iter()
        .filter(|ab| ab.last_listened.is_some())
        .max_by_key(|ab| ab.last_listened);

    assert!(last.is_some());
    assert_eq!(
        last.unwrap().name,
        "Evening Book",
        "Should select most recent even when multiple plays on same day"
    );

    Ok(())
}

#[test]
fn test_play_last_listened_with_progress() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;

    let id = create_test_audiobook(&db, "/test", "In Progress Book")?;

    // Set progress and last listened
    queries::update_audiobook_completeness(db.connection(), id, 50)?;
    let now = Utc::now();
    queries::update_audiobook_last_listened(db.connection(), id, now)?;

    let audiobooks = queries::get_all_audiobooks(db.connection())?;
    let last = audiobooks
        .iter()
        .filter(|ab| ab.last_listened.is_some())
        .max_by_key(|ab| ab.last_listened);

    assert!(last.is_some());
    let audiobook = last.unwrap();
    assert_eq!(audiobook.name, "In Progress Book");
    assert_eq!(
        audiobook.completeness, 50,
        "Should preserve progress for quick resume"
    );

    Ok(())
}

#[test]
fn test_play_last_listened_sorts_correctly() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;

    // Create 5 audiobooks with timestamps in random order
    let id1 = create_test_audiobook(&db, "/test", "Book A")?;
    let id2 = create_test_audiobook(&db, "/test", "Book B")?;
    let id3 = create_test_audiobook(&db, "/test", "Book C")?;
    let id4 = create_test_audiobook(&db, "/test", "Book D")?;
    let id5 = create_test_audiobook(&db, "/test", "Book E")?;

    let t1 = Utc::now() - chrono::Duration::days(5);
    let t2 = Utc::now() - chrono::Duration::days(3);
    let t3 = Utc::now() - chrono::Duration::days(1);
    let t4 = Utc::now() - chrono::Duration::hours(6);
    let t5 = Utc::now();

    queries::update_audiobook_last_listened(db.connection(), id3, t1)?; // Book C: oldest
    queries::update_audiobook_last_listened(db.connection(), id1, t4)?; // Book A: recent
    queries::update_audiobook_last_listened(db.connection(), id5, t2)?; // Book E: old
    queries::update_audiobook_last_listened(db.connection(), id2, t5)?; // Book B: newest
    queries::update_audiobook_last_listened(db.connection(), id4, t3)?; // Book D: recent

    let audiobooks = queries::get_all_audiobooks(db.connection())?;

    // Sort by last listened descending (most recent first)
    let mut sorted: Vec<_> = audiobooks
        .iter()
        .filter(|ab| ab.last_listened.is_some())
        .collect();
    sorted.sort_by(|a, b| b.last_listened.cmp(&a.last_listened));

    assert_eq!(sorted.len(), 5);
    assert_eq!(sorted[0].name, "Book B"); // Most recent
    assert_eq!(sorted[1].name, "Book A");
    assert_eq!(sorted[2].name, "Book D");
    assert_eq!(sorted[3].name, "Book E");
    assert_eq!(sorted[4].name, "Book C"); // Oldest

    Ok(())
}

#[test]
fn test_play_last_listened_timestamp_precision() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;

    let id1 = create_test_audiobook(&db, "/test", "Book 1")?;
    let id2 = create_test_audiobook(&db, "/test", "Book 2")?;

    // Set timestamps very close together (1 second apart)
    let t1 = Utc::now();
    std::thread::sleep(std::time::Duration::from_millis(1100));
    let t2 = Utc::now();

    queries::update_audiobook_last_listened(db.connection(), id1, t1)?;
    queries::update_audiobook_last_listened(db.connection(), id2, t2)?;

    let audiobooks = queries::get_all_audiobooks(db.connection())?;
    let last = audiobooks
        .iter()
        .filter(|ab| ab.last_listened.is_some())
        .max_by_key(|ab| ab.last_listened);

    assert!(last.is_some());
    assert_eq!(
        last.unwrap().name,
        "Book 2",
        "Should correctly identify most recent even with close timestamps"
    );

    Ok(())
}
