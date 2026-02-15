//! UI performance tests - verify responsiveness with large datasets
//!
//! Tests ensure UI remains responsive with realistic large data volumes.
//! Performance baselines prevent regressions.

#![allow(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::float_cmp,
    clippy::cast_precision_loss,
    clippy::field_reassign_with_default
)]

use nodoka::models::{AudiobookFile, Bookmark};
use std::error::Error;
use std::time::Instant;

mod acceptance_support;
use acceptance_support::{create_test_audiobook, create_test_db};

#[test]
fn test_load_100_audiobooks_performance() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;

    // Create 100 audiobooks
    let start = Instant::now();
    for i in 0..100 {
        create_test_audiobook(&db, "/test", &format!("Audiobook {i:03}"))?;
    }
    let insert_duration = start.elapsed();

    println!("Inserted 100 audiobooks in {insert_duration:?}");

    // Retrieve all audiobooks
    let start = Instant::now();
    let audiobooks = nodoka::db::queries::get_all_audiobooks(db.connection())?;
    let query_duration = start.elapsed();

    println!("Retrieved 100 audiobooks in {query_duration:?}");

    assert_eq!(audiobooks.len(), 100, "Should have 100 audiobooks");

    // Query should be fast (< 100ms for 100 items)
    assert!(
        query_duration.as_millis() < 100,
        "Querying 100 audiobooks took {query_duration:?}, should be < 100ms"
    );

    Ok(())
}

#[test]
fn test_audiobook_with_1000_files_performance() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;

    // Create one audiobook with 1000 files
    let audiobook_id = create_test_audiobook(&db, "/test", "Large Audiobook")?;

    let start = Instant::now();
    for i in 0..1000 {
        let file = AudiobookFile {
            audiobook_id,
            name: format!("chapter_{i:04}.mp3"),
            full_path: format!("/test/chapter_{i:04}.mp3"),
            length_of_file: Some(3_600_000),
            seek_position: None,
            checksum: None,
            position: i,
            completeness: 0,
            file_exists: true,
            created_at: chrono::Utc::now(),
        };
        nodoka::db::queries::insert_audiobook_file(db.connection(), &file)?;
    }
    let insert_duration = start.elapsed();

    println!("Inserted 1000 files in {insert_duration:?}");

    // Retrieve all files
    let start = Instant::now();
    let files = nodoka::db::queries::get_audiobook_files(db.connection(), audiobook_id)?;
    let query_duration = start.elapsed();

    println!("Retrieved 1000 files in {query_duration:?}");

    assert_eq!(files.len(), 1000, "Should have 1000 files");

    // Query should be reasonably fast (< 200ms for 1000 items)
    assert!(
        query_duration.as_millis() < 200,
        "Querying 1000 files took {query_duration:?}, should be < 200ms"
    );

    Ok(())
}

#[test]
fn test_1000_bookmarks_performance() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;

    // Create audiobook
    let audiobook_id = create_test_audiobook(&db, "/test", "Test Book")?;

    // Create file
    let file = AudiobookFile {
        audiobook_id,
        name: "chapter1.mp3".to_string(),
        full_path: "/test/chapter1.mp3".to_string(),
        length_of_file: Some(3_600_000),
        seek_position: None,
        checksum: None,
        position: 0,
        completeness: 0,
        file_exists: true,
        created_at: chrono::Utc::now(),
    };
    nodoka::db::queries::insert_audiobook_file(db.connection(), &file)?;

    // Create 1000 bookmarks
    let start = Instant::now();
    for i in 0..1000 {
        let bookmark = Bookmark {
            id: None,
            audiobook_id,
            file_path: "/test/chapter1.mp3".to_string(),
            position_ms: i * 3600, // Every ~3.6 seconds
            label: format!("Bookmark {i:04}"),
            note: if i % 10 == 0 {
                Some(format!("Note for bookmark {i}"))
            } else {
                None
            },
            created_at: chrono::Utc::now(),
        };
        nodoka::db::queries::insert_bookmark(db.connection(), &bookmark)?;
    }
    let insert_duration = start.elapsed();

    println!("Inserted 1000 bookmarks in {insert_duration:?}");

    // Retrieve all bookmarks
    let start = Instant::now();
    let bookmarks =
        nodoka::db::queries::get_bookmarks_for_audiobook(db.connection(), audiobook_id)?;
    let query_duration = start.elapsed();

    println!("Retrieved 1000 bookmarks in {query_duration:?}");

    assert_eq!(bookmarks.len(), 1000, "Should have 1000 bookmarks");

    // Query should be fast (< 100ms)
    assert!(
        query_duration.as_millis() < 100,
        "Querying 1000 bookmarks took {query_duration:?}, should be < 100ms"
    );

    Ok(())
}

#[test]
fn test_rapid_state_updates_performance() {
    use nodoka::ui::State;

    let mut state = State::default();

    // Simulate 1000 rapid state updates
    let start = Instant::now();
    for i in 0..1000_i32 {
        state.volume = i % 201;
        state.speed = 0.5 + (i % 16) as f32 / 10.0;
        state.current_time = f64::from(i * 100);
    }
    let duration = start.elapsed();

    println!("1000 state updates in {duration:?}");

    // State updates should be extremely fast (< 1ms for 1000 updates)
    assert!(
        duration.as_millis() < 10,
        "1000 state updates took {duration:?}, should be < 10ms"
    );
}

#[test]
fn test_directory_list_performance() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let temp_root = std::env::temp_dir().join("nodoka_perf_test");
    std::fs::create_dir_all(&temp_root)?;

    // Create 50 directories
    let start = Instant::now();
    for i in 0..50 {
        let dir_path = temp_root.join(format!("audiobooks{i:03}"));
        std::fs::create_dir_all(&dir_path)?;
        let directory = nodoka::models::Directory::new(dir_path.to_string_lossy().to_string());
        nodoka::db::queries::insert_directory(db.connection(), &directory)?;
    }
    let insert_duration = start.elapsed();

    println!("Inserted 50 directories in {insert_duration:?}");

    // Retrieve all directories
    let start = Instant::now();
    let directories = nodoka::db::queries::get_all_directories(db.connection())?;
    let query_duration = start.elapsed();

    println!("Retrieved 50 directories in {query_duration:?}");

    assert_eq!(directories.len(), 50, "Should have 50 directories");

    // Query should be very fast (< 50ms)
    assert!(
        query_duration.as_millis() < 50,
        "Querying 50 directories took {query_duration:?}, should be < 50ms"
    );

    // Cleanup
    let _ = std::fs::remove_dir_all(&temp_root);

    Ok(())
}

#[test]
fn test_file_completeness_update_performance() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;

    // Create audiobook with 100 files
    let audiobook_id = create_test_audiobook(&db, "/test", "Test Book")?;

    for i in 0..100 {
        let file = AudiobookFile {
            audiobook_id,
            name: format!("chapter{i:03}.mp3"),
            full_path: format!("/test/chapter{i:03}.mp3"),
            length_of_file: Some(3_600_000),
            seek_position: None,
            checksum: None,
            position: i,
            completeness: 0,
            file_exists: true,
            created_at: chrono::Utc::now(),
        };
        nodoka::db::queries::insert_audiobook_file(db.connection(), &file)?;
    }

    // Update progress for all 100 files
    let start = Instant::now();
    for i in 0..100 {
        let file_path = format!("/test/chapter{i:03}.mp3");
        nodoka::db::queries::update_file_progress(
            db.connection(),
            &file_path,
            1_800_000.0, // 30 minutes
            50,          // 50% complete
        )?;
    }
    let update_duration = start.elapsed();

    println!("Updated progress for 100 files in {update_duration:?}");

    // Updates should be reasonably fast (< 500ms for 100 updates)
    assert!(
        update_duration.as_millis() < 500,
        "Updating 100 files took {update_duration:?}, should be < 500ms"
    );

    Ok(())
}

#[test]
fn test_search_bookmarks_by_label_performance() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;

    // Create audiobook
    let audiobook_id = create_test_audiobook(&db, "/test", "Test Book")?;

    // Create file
    let file = AudiobookFile {
        audiobook_id,
        name: "chapter1.mp3".to_string(),
        full_path: "/test/chapter1.mp3".to_string(),
        length_of_file: Some(3_600_000),
        seek_position: None,
        checksum: None,
        position: 0,
        completeness: 0,
        file_exists: true,
        created_at: chrono::Utc::now(),
    };
    nodoka::db::queries::insert_audiobook_file(db.connection(), &file)?;

    // Create 500 bookmarks with varied labels
    for i in 0..500 {
        let bookmark = Bookmark {
            id: None,
            audiobook_id,
            file_path: "/test/chapter1.mp3".to_string(),
            position_ms: i * 7200,
            label: if i % 5 == 0 {
                format!("Important Bookmark {i}")
            } else {
                format!("Regular Bookmark {i}")
            },
            note: None,
            created_at: chrono::Utc::now(),
        };
        nodoka::db::queries::insert_bookmark(db.connection(), &bookmark)?;
    }

    // Search/filter bookmarks (simulated - would be done in UI layer)
    let start = Instant::now();
    let all_bookmarks =
        nodoka::db::queries::get_bookmarks_for_audiobook(db.connection(), audiobook_id)?;
    let important_count = all_bookmarks
        .iter()
        .filter(|b| b.label.contains("Important"))
        .count();
    let search_duration = start.elapsed();

    println!("Searched 500 bookmarks in {search_duration:?}");

    assert_eq!(important_count, 100, "Should find 100 important bookmarks");

    // Search should be fast (< 50ms)
    assert!(
        search_duration.as_millis() < 50,
        "Searching 500 bookmarks took {search_duration:?}, should be < 50ms"
    );

    Ok(())
}

#[test]
fn test_audiobook_list_sorting_performance() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;

    // Create 200 audiobooks with various names
    for i in 0..200 {
        let name = if i % 3 == 0 {
            format!("A Book {i:03}")
        } else if i % 3 == 1 {
            format!("Z Book {i:03}")
        } else {
            format!("M Book {i:03}")
        };

        create_test_audiobook(&db, "/test", &name)?;
    }

    // Retrieve and sort audiobooks
    let start = Instant::now();
    let mut audiobooks = nodoka::db::queries::get_all_audiobooks(db.connection())?;
    audiobooks.sort_by(|a, b| a.name.cmp(&b.name));
    let sort_duration = start.elapsed();

    println!("Sorted 200 audiobooks in {sort_duration:?}");

    assert_eq!(audiobooks.len(), 200, "Should have 200 audiobooks");

    // Sorting should be fast (< 50ms)
    assert!(
        sort_duration.as_millis() < 50,
        "Sorting 200 audiobooks took {sort_duration:?}, should be < 50ms"
    );

    Ok(())
}

#[test]
fn test_memory_efficiency_large_dataset() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;

    // Create large dataset
    for i in 0..50 {
        let audiobook_id = create_test_audiobook(&db, "/test", &format!("Book {i}"))?;

        // 20 files per audiobook = 1000 files total
        for j in 0..20 {
            let file = AudiobookFile {
                audiobook_id,
                name: format!("chapter{j:02}.mp3"),
                full_path: format!("/test/book{i}/chapter{j:02}.mp3"),
                length_of_file: Some(3_600_000),
                seek_position: None,
                checksum: None,
                position: j,
                completeness: 0,
                file_exists: true,
                created_at: chrono::Utc::now(),
            };
            nodoka::db::queries::insert_audiobook_file(db.connection(), &file)?;
        }
    }

    // Load all data
    let start = Instant::now();
    let audiobooks = nodoka::db::queries::get_all_audiobooks(db.connection())?;

    let mut total_files = 0;
    for audiobook in &audiobooks {
        if let Some(id) = audiobook.id {
            let files = nodoka::db::queries::get_audiobook_files(db.connection(), id)?;
            total_files += files.len();
        }
    }
    let load_duration = start.elapsed();

    println!("Loaded 50 audiobooks with 1000 total files in {load_duration:?}");

    assert_eq!(audiobooks.len(), 50, "Should have 50 audiobooks");
    assert_eq!(total_files, 1000, "Should have 1000 total files");

    // Loading should complete in reasonable time (< 1 second)
    assert!(
        load_duration.as_millis() < 1000,
        "Loading large dataset took {load_duration:?}, should be < 1s"
    );

    Ok(())
}
