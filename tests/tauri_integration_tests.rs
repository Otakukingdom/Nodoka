//! Integration tests for Tauri commands
//!
//! Tests the Tauri IPC command handlers to ensure they work correctly
//! with the database and player components.

// Allow unwrap and indexing in test code
#![allow(clippy::unwrap_used)]
#![allow(clippy::indexing_slicing)]

use chrono::Utc;
use nodoka::db::{self, queries, Database};
use nodoka::models::{Audiobook, AudiobookFile, Bookmark, Directory};
use nodoka::tauri::AppState;
use std::error::Error;
use temp_dir::TempDir;

fn create_test_db() -> Result<Database, Box<dyn Error>> {
    let db = Database::new_in_memory()?;
    db::initialize(db.connection())?;
    Ok(db)
}

fn create_test_audiobook(db: &Database, name: &str) -> Result<i64, Box<dyn Error>> {
    let conn = db.connection();
    let audiobook = Audiobook {
        id: None,
        directory: "/test".to_string(),
        name: name.to_string(),
        author: None,
        full_path: format!("/test/{}", name),
        completeness: 0,
        default_order: 0,
        selected_file: None,
        total_duration_ms: None,
        remaining_duration_ms: None,
        created_at: Utc::now(),
        last_listened: None,
    };
    Ok(queries::insert_audiobook(conn, &audiobook)?)
}

fn create_test_file(db: &Database, audiobook_id: i64, name: &str, position: i32) -> Result<(), Box<dyn Error>> {
    let conn = db.connection();
    let file = AudiobookFile {
        audiobook_id,
        name: name.to_string(),
        full_path: format!("/test/{}", name),
        length_of_file: Some(1800000),
        seek_position: Some(0),
        checksum: Some("test_checksum".to_string()),
        position,
        completeness: 0,
        file_exists: true,
        created_at: Utc::now(),
    };
    queries::insert_audiobook_file(conn, &file)?;
    Ok(())
}

// ============================================================================
// AppState Tests
// ============================================================================

#[test]
fn test_app_state_creation() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let state = AppState::new(db, None);

    // Verify we can lock the database
    let _locked_db = state.db.lock();

    // Verify player is None
    let player = state.player.lock();
    assert!(player.is_none());

    Ok(())
}

#[test]
fn test_app_state_with_player_initialization() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let state = AppState::new(db, None);

    // Player should be None since we didn't initialize VLC
    let player_guard = state.player.lock();
    assert!(player_guard.is_none());

    Ok(())
}

// ============================================================================
// Database Command Tests
// ============================================================================

#[test]
fn test_get_all_audiobooks_empty() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let state = AppState::new(db, None);

    let locked_db = state.db.lock();
    let audiobooks = queries::get_all_audiobooks(locked_db.connection())?;

    assert!(audiobooks.is_empty());
    Ok(())
}

#[test]
fn test_get_all_audiobooks_with_data() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    create_test_audiobook(&db, "Test Book 1")?;
    create_test_audiobook(&db, "Test Book 2")?;

    let state = AppState::new(db, None);
    let locked_db = state.db.lock();
    let audiobooks = queries::get_all_audiobooks(locked_db.connection())?;

    assert_eq!(audiobooks.len(), 2);
    Ok(())
}

#[test]
fn test_get_audiobook_by_id() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let id = create_test_audiobook(&db, "Test Book")?;

    let state = AppState::new(db, None);
    let locked_db = state.db.lock();
    let audiobook = queries::get_audiobook_by_id(locked_db.connection(), id)?;

    assert!(audiobook.is_some());
    assert_eq!(audiobook.unwrap().name, "Test Book");
    Ok(())
}

#[test]
fn test_get_audiobook_by_id_not_found() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;

    let state = AppState::new(db, None);
    let locked_db = state.db.lock();
    let audiobook = queries::get_audiobook_by_id(locked_db.connection(), 999)?;

    assert!(audiobook.is_none());
    Ok(())
}

#[test]
fn test_get_audiobook_files() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let audiobook_id = create_test_audiobook(&db, "Test Book")?;
    create_test_file(&db, audiobook_id, "Chapter 1", 0)?;
    create_test_file(&db, audiobook_id, "Chapter 2", 1)?;

    let state = AppState::new(db, None);
    let locked_db = state.db.lock();
    let files = queries::get_audiobook_files(locked_db.connection(), audiobook_id)?;

    assert_eq!(files.len(), 2);
    assert_eq!(files[0].name, "Chapter 1");
    assert_eq!(files[1].name, "Chapter 2");
    Ok(())
}

// ============================================================================
// Bookmark Command Tests
// ============================================================================

#[test]
fn test_bookmark_crud_operations() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let audiobook_id = create_test_audiobook(&db, "Test Book")?;
    create_test_file(&db, audiobook_id, "Chapter 1", 0)?;

    let state = AppState::new(db, None);

    // Create bookmark
    let bookmark = Bookmark {
        id: None,
        audiobook_id,
        file_path: "/test/Chapter 1".to_string(),
        position_ms: 5000,
        label: "Test Bookmark".to_string(),
        note: Some("Test note".to_string()),
        created_at: Utc::now(),
    };

    {
        let locked_db = state.db.lock();
        let bookmark_id = queries::insert_bookmark(locked_db.connection(), &bookmark)?;
        assert!(bookmark_id > 0);
    }

    // Read bookmarks
    {
        let locked_db = state.db.lock();
        let bookmarks = queries::get_bookmarks_for_audiobook(locked_db.connection(), audiobook_id)?;
        assert_eq!(bookmarks.len(), 1);
        assert_eq!(bookmarks[0].label, "Test Bookmark");
        assert_eq!(bookmarks[0].position_ms, 5000);
    }

    // Update bookmark
    {
        let locked_db = state.db.lock();
        let mut bookmarks = queries::get_bookmarks_for_audiobook(locked_db.connection(), audiobook_id)?;
        let mut updated = bookmarks.remove(0);
        updated.label = "Updated Bookmark".to_string();
        queries::update_bookmark(locked_db.connection(), &updated)?;
    }

    // Verify update
    {
        let locked_db = state.db.lock();
        let bookmarks = queries::get_bookmarks_for_audiobook(locked_db.connection(), audiobook_id)?;
        assert_eq!(bookmarks[0].label, "Updated Bookmark");
    }

    // Delete bookmark
    {
        let locked_db = state.db.lock();
        let bookmarks = queries::get_bookmarks_for_audiobook(locked_db.connection(), audiobook_id)?;
        queries::delete_bookmark(locked_db.connection(), bookmarks[0].id.unwrap())?;
    }

    // Verify deletion
    {
        let locked_db = state.db.lock();
        let bookmarks = queries::get_bookmarks_for_audiobook(locked_db.connection(), audiobook_id)?;
        assert!(bookmarks.is_empty());
    }

    Ok(())
}

// ============================================================================
// Directory Command Tests
// ============================================================================

#[test]
fn test_directory_operations() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let state = AppState::new(db, None);

    // Create directory
    let (_temp, dir_path) = create_temp_directory()?;
    {
        let locked_db = state.db.lock();
        let directory = Directory {
            full_path: dir_path.clone(),
            created_at: Utc::now(),
            last_scanned: None,
        };
        queries::insert_directory(locked_db.connection(), &directory)?;
    }

    // Read directories
    {
        let locked_db = state.db.lock();
        let directories = queries::get_all_directories(locked_db.connection())?;
        assert_eq!(directories.len(), 1);
        assert_eq!(directories[0].full_path, dir_path);
    }

    // Delete directory
    {
        let locked_db = state.db.lock();
        queries::delete_directory(locked_db.connection(), &dir_path)?;
    }

    // Verify deletion
    {
        let locked_db = state.db.lock();
        let directories = queries::get_all_directories(locked_db.connection())?;
        assert!(directories.is_empty());
    }

    Ok(())
}

fn create_temp_directory() -> Result<(TempDir, String), Box<dyn Error>> {
    let temp = TempDir::new()?;
    let dir_path = temp.path().to_string_lossy().to_string();
    Ok((temp, dir_path))
}

// ============================================================================
// Settings Command Tests
// ============================================================================

#[test]
fn test_settings_storage() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let state = AppState::new(db, None);

    // Set and get metadata values
    {
        let locked_db = state.db.lock();
        queries::set_metadata(locked_db.connection(), "current_audiobook_id", "42")?;
    }

    {
        let locked_db = state.db.lock();
        let value = queries::get_metadata(locked_db.connection(), "current_audiobook_id")?;
        assert_eq!(value, Some("42".to_string()));
    }

    // Update metadata
    {
        let locked_db = state.db.lock();
        queries::set_metadata(locked_db.connection(), "current_audiobook_id", "99")?;
    }

    {
        let locked_db = state.db.lock();
        let value = queries::get_metadata(locked_db.connection(), "current_audiobook_id")?;
        assert_eq!(value, Some("99".to_string()));
    }

    Ok(())
}

// ============================================================================
// Concurrent Access Tests
// ============================================================================

#[test]
fn test_concurrent_db_access() -> Result<(), Box<dyn Error>> {
    use std::sync::Arc;
    use std::thread;

    let db = create_test_db()?;
    let state = Arc::new(AppState::new(db, None));

    let mut handles = vec![];

    // Spawn multiple threads that access the database
    for i in 0..5 {
        let state_clone = Arc::clone(&state);
        let handle = thread::spawn(move || {
            let locked_db = state_clone.db.lock();
            let audiobook = Audiobook {
                id: None,
                directory: "/test".to_string(),
                name: format!("Concurrent Book {}", i),
                author: None,
                full_path: format!("/test/concurrent_{}", i),
                completeness: 0,
                default_order: 0,
                selected_file: None,
                total_duration_ms: None,
                remaining_duration_ms: None,
                created_at: Utc::now(),
                last_listened: None,
            };
            queries::insert_audiobook(locked_db.connection(), &audiobook)
        });
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap()?;
    }

    // Verify all audiobooks were created
    let locked_db = state.db.lock();
    let audiobooks = queries::get_all_audiobooks(locked_db.connection())?;
    assert_eq!(audiobooks.len(), 5);

    Ok(())
}

// ============================================================================
// Data Integrity Tests
// ============================================================================

#[test]
fn test_audiobook_progress_update() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let audiobook_id = create_test_audiobook(&db, "Progress Test Book")?;
    let state = AppState::new(db, None);

    // Update completeness
    {
        let locked_db = state.db.lock();
        queries::update_audiobook_completeness(locked_db.connection(), audiobook_id, 75)?;
    }

    // Verify update
    {
        let locked_db = state.db.lock();
        let audiobook = queries::get_audiobook_by_id(locked_db.connection(), audiobook_id)?;
        assert_eq!(audiobook.unwrap().completeness, 75);
    }

    Ok(())
}

#[test]
fn test_file_seek_position_update() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let audiobook_id = create_test_audiobook(&db, "Seek Test Book")?;
    create_test_file(&db, audiobook_id, "Chapter 1", 0)?;
    let state = AppState::new(db, None);

    // Update seek position using update_file_progress
    {
        let locked_db = state.db.lock();
        queries::update_file_progress(locked_db.connection(), "/test/Chapter 1", 12345.0, 50)?;
    }

    // Verify update
    {
        let locked_db = state.db.lock();
        let files = queries::get_audiobook_files(locked_db.connection(), audiobook_id)?;
        assert_eq!(files[0].seek_position, Some(12345));
        assert_eq!(files[0].completeness, 50);
    }

    Ok(())
}
