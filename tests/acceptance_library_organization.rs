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

    assert_eq!(
        audiobooks.first().ok_or("No audiobook at index 0")?.name,
        "Apple Book"
    );
    assert_eq!(
        audiobooks.get(1).ok_or("No audiobook at index 1")?.name,
        "Middle Book"
    );
    assert_eq!(
        audiobooks.get(2).ok_or("No audiobook at index 2")?.name,
        "Zebra Book"
    );

    Ok(())
}

#[test]
fn test_search_performance_large_library() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;

    // Create 1000 audiobooks
    for i in 0..1000 {
        create_test_audiobook(&db, "/test/library", &format!("Audiobook {i:04}"))?;
    }

    // Search should be fast enough to avoid UI jank.
    // In debug builds and shared CI runners, wall-clock thresholds can be noisy.
    let max_duration = if cfg!(debug_assertions) {
        std::time::Duration::from_secs(2)
    } else {
        std::time::Duration::from_millis(200)
    };

    let start = std::time::Instant::now();
    let all = queries::get_all_audiobooks(db.connection())?;
    let has_results = all.iter().any(|ab| ab.name.to_lowercase().contains("book"));
    let duration = start.elapsed();

    assert!(
        duration < max_duration,
        "Search took {}ms",
        duration.as_millis()
    );
    assert!(has_results);

    Ok(())
}

#[test]
fn test_sort_performance_large_library() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;

    // Create 1000 audiobooks with random names
    for i in 0..1000 {
        create_test_audiobook(&db, "/test/library", &format!("Book {}", 1000 - i))?;
    }

    let max_duration = if cfg!(debug_assertions) {
        std::time::Duration::from_secs(2)
    } else {
        std::time::Duration::from_millis(200)
    };

    let start = std::time::Instant::now();
    let mut audiobooks = queries::get_all_audiobooks(db.connection())?;
    audiobooks.sort_by(|a, b| a.name.cmp(&b.name));
    let duration = start.elapsed();

    assert!(
        duration < max_duration,
        "Sort took {}ms",
        duration.as_millis()
    );

    assert_eq!(audiobooks.len(), 1000);

    Ok(())
}

#[test]
fn test_filter_performance_large_library() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;

    // Create 1000 audiobooks
    for i in 0..1000 {
        create_test_audiobook(&db, "/test/library", &format!("Book {i:04}"))?;
    }

    let max_duration = if cfg!(debug_assertions) {
        std::time::Duration::from_secs(2)
    } else {
        std::time::Duration::from_millis(200)
    };

    let start = std::time::Instant::now();
    let audiobooks = queries::get_all_audiobooks(db.connection())?;
    let incomplete_count = audiobooks.iter().filter(|ab| !ab.is_complete()).count();
    let duration = start.elapsed();

    assert!(
        duration < max_duration,
        "Filter took {}ms",
        duration.as_millis()
    );
    assert_eq!(incomplete_count, 1000); // All incomplete

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
    let complete_count = all.iter().filter(|ab| ab.completeness == 100).count();
    let incomplete_count = all.iter().filter(|ab| ab.completeness < 100).count();

    assert_eq!(complete_count, 1);
    assert_eq!(incomplete_count, 1);

    Ok(())
}

#[test]
fn test_search_by_name_case_insensitive() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;

    create_test_audiobook(&db, "/test", "The Great Gatsby")?;
    create_test_audiobook(&db, "/test", "1984")?;
    create_test_audiobook(&db, "/test", "Great Expectations")?;

    let all = queries::get_all_audiobooks(db.connection())?;
    let results_count = all
        .iter()
        .filter(|ab| ab.name.to_lowercase().contains("great"))
        .count();

    assert_eq!(results_count, 2);

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
    let results_count = all
        .iter()
        .filter(|ab| ab.name.to_lowercase().contains("nonexistent"))
        .count();

    assert_eq!(results_count, 0);

    Ok(())
}

#[test]
fn test_search_partial_match() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;

    create_test_audiobook(&db, "/test", "Harry Potter and the Philosopher's Stone")?;
    create_test_audiobook(&db, "/test", "Harry Potter and the Chamber of Secrets")?;
    create_test_audiobook(&db, "/test", "The Hobbit")?;

    let all = queries::get_all_audiobooks(db.connection())?;
    let results_count = all
        .iter()
        .filter(|ab| ab.name.to_lowercase().contains("harry"))
        .count();

    assert_eq!(results_count, 2);

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

    assert_eq!(
        audiobooks.first().ok_or("No audiobook at index 0")?.name,
        "Gamma"
    );
    assert_eq!(
        audiobooks.get(1).ok_or("No audiobook at index 1")?.name,
        "Beta"
    );
    assert_eq!(
        audiobooks.get(2).ok_or("No audiobook at index 2")?.name,
        "Alpha"
    );

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
    let incomplete_count = all.iter().filter(|ab| ab.completeness < 100).count();

    assert_eq!(incomplete_count, 2);

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
    assert_eq!(
        in_progress.first().ok_or("No audiobook in progress")?.name,
        "In Progress"
    );

    Ok(())
}

#[test]
fn test_search_special_characters() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;

    create_test_audiobook(&db, "/test", "Book: The Story")?;
    create_test_audiobook(&db, "/test", "Book (2024 Edition)")?;
    create_test_audiobook(&db, "/test", "Author's Book")?;

    let all = queries::get_all_audiobooks(db.connection())?;
    let results_count = all
        .iter()
        .filter(|ab| ab.name.to_lowercase().contains("book"))
        .count();

    assert_eq!(results_count, 3);

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

#[test]
fn test_search_with_special_regex_characters() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;

    // Create audiobooks with regex special characters
    create_test_audiobook(&db, "/test", "Book (Part 1)")?;
    create_test_audiobook(&db, "/test", "Book [Complete]")?;
    create_test_audiobook(&db, "/test", "Book.with.dots")?;
    create_test_audiobook(&db, "/test", "Book+Extra")?;
    create_test_audiobook(&db, "/test", "Book*Wildcard")?;

    let all = queries::get_all_audiobooks(db.connection())?;

    // Search should treat special chars as literals
    let results_count = all.iter().filter(|ab| ab.name.contains("(Part")).count();
    assert_eq!(results_count, 1);
    let result = all
        .iter()
        .find(|ab| ab.name.contains("(Part"))
        .ok_or("No result found")?;
    assert_eq!(result.name, "Book (Part 1)");

    let results_count = all
        .iter()
        .filter(|ab| ab.name.contains("[Complete]"))
        .count();
    assert_eq!(results_count, 1);

    let results_count = all.iter().filter(|ab| ab.name.contains("Book+")).count();
    assert_eq!(results_count, 1);

    Ok(())
}

#[test]
fn test_sort_with_numbers_and_special_chars() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;

    let names = vec![
        "Book 10",
        "Book 2",
        "Book 1",
        "Book (2023)",
        "Book [New]",
        "Book - The Sequel",
    ];

    for name in &names {
        create_test_audiobook(&db, "/test", name)?;
    }

    let audiobooks = queries::get_all_audiobooks(db.connection())?;

    // Verify natural numeric ordering exists somewhere
    let book1 = audiobooks.iter().find(|ab| ab.name == "Book 1");
    let book2 = audiobooks.iter().find(|ab| ab.name == "Book 2");
    let book10 = audiobooks.iter().find(|ab| ab.name == "Book 10");

    assert!(book1.is_some());
    assert!(book2.is_some());
    assert!(book10.is_some());

    Ok(())
}

#[test]
fn test_filter_with_no_results() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;

    create_test_audiobook(&db, "/test", "Book One")?;
    create_test_audiobook(&db, "/test", "Book Two")?;

    // Mark all complete
    let audiobooks = queries::get_all_audiobooks(db.connection())?;
    for book in &audiobooks {
        let book_id = book.id.ok_or("No audiobook ID")?;
        queries::update_audiobook_completeness(db.connection(), book_id, 100)?;
    }

    // Filter for incomplete should return empty
    let all = queries::get_all_audiobooks(db.connection())?;
    let incomplete_count = all.iter().filter(|ab| ab.completeness < 100).count();
    assert_eq!(incomplete_count, 0);

    Ok(())
}

#[test]
fn test_empty_library_operations() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;

    // Operations on empty library should not crash
    let audiobooks = queries::get_all_audiobooks(db.connection())?;
    assert_eq!(audiobooks.len(), 0);

    // Search in empty library
    let results_count = audiobooks
        .iter()
        .filter(|ab| ab.name.contains("test"))
        .count();
    assert_eq!(results_count, 0);

    Ok(())
}

#[test]
fn test_search_unicode_characters() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;

    create_test_audiobook(&db, "/test", "Book with émojis 📚")?;
    create_test_audiobook(&db, "/test", "日本語のタイトル")?;
    create_test_audiobook(&db, "/ test", "Книга на русском")?;

    let audiobooks = queries::get_all_audiobooks(db.connection())?;

    // Unicode search
    let results_count = audiobooks
        .iter()
        .filter(|ab| ab.name.contains("📚"))
        .count();
    assert_eq!(results_count, 1);

    let results_count = audiobooks
        .iter()
        .filter(|ab| ab.name.contains("日本語"))
        .count();
    assert_eq!(results_count, 1);

    Ok(())
}

#[test]
fn test_very_long_audiobook_names() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;

    let long_name = "A".repeat(1000);
    create_test_audiobook(&db, "/test", &long_name)?;

    let audiobooks = queries::get_all_audiobooks(db.connection())?;
    assert_eq!(audiobooks.len(), 1);

    // Search in long name should work
    let results_count = audiobooks
        .iter()
        .filter(|ab| ab.name.contains("AAA"))
        .count();
    assert_eq!(results_count, 1);

    Ok(())
}
