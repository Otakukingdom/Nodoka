use chrono::Utc;
use nodoka::db::{queries, Database};
use nodoka::models::{Audiobook, AudiobookFile, Directory};
use std::error::Error;
use std::io::{Error as IoError, ErrorKind};

fn create_test_db() -> Result<Database, Box<dyn Error>> {
    // Create in-memory database for testing
    let db = Database::new_in_memory()?;
    nodoka::db::initialize_schema(db.connection())?;
    Ok(db)
}

fn missing(message: &'static str) -> IoError {
    IoError::new(ErrorKind::Other, message)
}

#[test]
fn test_directory_crud_operations() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let conn = db.connection();

    // Insert directory
    let dir = Directory {
        full_path: "/test/audiobooks".to_string(),
        created_at: Utc::now(),
        last_scanned: None,
    };
    queries::insert_directory(conn, &dir)?;

    // Get all directories
    let dirs = queries::get_all_directories(conn)?;
    assert_eq!(dirs.len(), 1);
    let first_dir = dirs.first().ok_or_else(|| missing("Expected directory"))?;
    assert_eq!(first_dir.full_path, "/test/audiobooks");

    // Update last scanned
    queries::update_directory_last_scanned(conn, "/test/audiobooks")?;

    let dirs = queries::get_all_directories(conn)?;
    let updated_dir = dirs.first().ok_or_else(|| missing("Expected directory"))?;
    assert!(updated_dir.last_scanned.is_some());

    // Delete directory
    queries::delete_directory(conn, "/test/audiobooks")?;
    let dirs = queries::get_all_directories(conn)?;
    assert_eq!(dirs.len(), 0);
    Ok(())
}

#[test]
fn test_audiobook_crud_operations() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let conn = db.connection();

    // Insert directory first
    let dir = Directory {
        full_path: "/test/audiobooks".to_string(),
        created_at: Utc::now(),
        last_scanned: None,
    };
    queries::insert_directory(conn, &dir)?;

    // Insert audiobook
    let audiobook = Audiobook {
        id: None,
        directory: "/test/audiobooks".to_string(),
        name: "Test Audiobook".to_string(),
        full_path: "/test/audiobooks/test".to_string(),
        completeness: 0,
        default_order: 0,
        selected_file: None,
        created_at: Utc::now(),
    };
    let id = queries::insert_audiobook(conn, &audiobook)?;
    assert!(id > 0);

    // Get audiobook by ID
    let retrieved =
        queries::get_audiobook_by_id(conn, id)?.ok_or_else(|| missing("Audiobook not found"))?;
    assert_eq!(retrieved.name, "Test Audiobook");
    assert_eq!(retrieved.completeness, 0);

    // Get audiobook by path
    let by_path = queries::get_audiobook_by_path(conn, "/test/audiobooks/test")?
        .ok_or_else(|| missing("Audiobook not found"))?;
    assert_eq!(by_path.name, "Test Audiobook");

    // Update completeness
    queries::update_audiobook_completeness(conn, id, 50)?;
    let updated =
        queries::get_audiobook_by_id(conn, id)?.ok_or_else(|| missing("Audiobook not found"))?;
    assert_eq!(updated.completeness, 50);

    // Update selected file
    queries::update_audiobook_selected_file(conn, id, Some("/test/file.mp3"))?;
    let updated =
        queries::get_audiobook_by_id(conn, id)?.ok_or_else(|| missing("Audiobook not found"))?;
    assert_eq!(updated.selected_file, Some("/test/file.mp3".to_string()));

    // Get all audiobooks
    let all = queries::get_all_audiobooks(conn)?;
    assert_eq!(all.len(), 1);

    // Get audiobooks by directory
    let by_dir = queries::get_audiobooks_by_directory(conn, "/test/audiobooks")?;
    assert_eq!(by_dir.len(), 1);

    // Delete audiobook
    queries::delete_audiobook(conn, id)?;
    let deleted = queries::get_audiobook_by_id(conn, id)?;
    assert!(deleted.is_none());
    Ok(())
}

#[test]
fn test_audiobook_file_crud_operations() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let conn = db.connection();

    // Insert directory and audiobook first
    let dir = Directory {
        full_path: "/test/audiobooks".to_string(),
        created_at: Utc::now(),
        last_scanned: None,
    };
    queries::insert_directory(conn, &dir)?;

    let audiobook = Audiobook {
        id: None,
        directory: "/test/audiobooks".to_string(),
        name: "Test Audiobook".to_string(),
        full_path: "/test/audiobooks/test".to_string(),
        completeness: 0,
        default_order: 0,
        selected_file: None,
        created_at: Utc::now(),
    };
    let audiobook_id = queries::insert_audiobook(conn, &audiobook)?;

    // Insert file
    let file = AudiobookFile {
        audiobook_id,
        name: "Chapter 1.mp3".to_string(),
        full_path: "/test/audiobooks/test/Chapter 1.mp3".to_string(),
        length_of_file: Some(300_000),
        seek_position: None,
        position: 0,
        completeness: 0,
        file_exists: true,
        created_at: Utc::now(),
    };
    queries::insert_audiobook_file(conn, &file)?;

    // Get files for audiobook
    let files = queries::get_audiobook_files(conn, audiobook_id)?;
    assert_eq!(files.len(), 1);
    let first_file = files.first().ok_or_else(|| missing("Expected file"))?;
    assert_eq!(first_file.name, "Chapter 1.mp3");
    assert_eq!(first_file.length_of_file, Some(300_000));

    // Get file by path
    let by_path = queries::get_audiobook_file_by_path(conn, "/test/audiobooks/test/Chapter 1.mp3")?
        .ok_or_else(|| missing("File not found"))?;
    assert_eq!(by_path.name, "Chapter 1.mp3");

    // Update progress
    queries::update_file_progress(conn, "/test/audiobooks/test/Chapter 1.mp3", 150_000.0, 50)?;
    let updated = queries::get_audiobook_file_by_path(conn, "/test/audiobooks/test/Chapter 1.mp3")?
        .ok_or_else(|| missing("File not found"))?;
    assert_eq!(updated.seek_position, Some(150_000));
    assert_eq!(updated.completeness, 50);

    // Update file length
    queries::update_file_length(conn, "/test/audiobooks/test/Chapter 1.mp3", 350_000)?;
    let updated = queries::get_audiobook_file_by_path(conn, "/test/audiobooks/test/Chapter 1.mp3")?
        .ok_or_else(|| missing("File not found"))?;
    assert_eq!(updated.length_of_file, Some(350_000));

    // Mark file as missing
    queries::mark_file_exists(conn, "/test/audiobooks/test/Chapter 1.mp3", false)?;
    let updated = queries::get_audiobook_file_by_path(conn, "/test/audiobooks/test/Chapter 1.mp3")?
        .ok_or_else(|| missing("File not found"))?;
    assert!(!updated.file_exists);

    // Mark file as existing
    queries::mark_file_exists(conn, "/test/audiobooks/test/Chapter 1.mp3", true)?;
    let updated = queries::get_audiobook_file_by_path(conn, "/test/audiobooks/test/Chapter 1.mp3")?
        .ok_or_else(|| missing("File not found"))?;
    assert!(updated.file_exists);
    Ok(())
}

#[test]
fn test_audiobook_progress_operations() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let conn = db.connection();

    // Setup
    let dir = Directory {
        full_path: "/test/audiobooks".to_string(),
        created_at: Utc::now(),
        last_scanned: None,
    };
    queries::insert_directory(conn, &dir)?;

    let audiobook = Audiobook {
        id: None,
        directory: "/test/audiobooks".to_string(),
        name: "Test Audiobook".to_string(),
        full_path: "/test/audiobooks/test".to_string(),
        completeness: 0,
        default_order: 0,
        selected_file: None,
        created_at: Utc::now(),
    };
    let audiobook_id = queries::insert_audiobook(conn, &audiobook)?;

    // Add multiple files
    for i in 0..3 {
        let file = AudiobookFile {
            audiobook_id,
            name: format!("Chapter {}.mp3", i + 1),
            full_path: format!("/test/audiobooks/test/Chapter {}.mp3", i + 1),
            length_of_file: Some(300_000),
            seek_position: None,
            position: i,
            completeness: 0,
            file_exists: true,
            created_at: Utc::now(),
        };
        queries::insert_audiobook_file(conn, &file)?;
    }

    // Mark audiobook complete
    queries::mark_audiobook_complete(conn, audiobook_id)?;

    let audiobook = queries::get_audiobook_by_id(conn, audiobook_id)?
        .ok_or_else(|| missing("Audiobook not found"))?;
    assert_eq!(audiobook.completeness, 100);

    let files = queries::get_audiobook_files(conn, audiobook_id)?;
    assert!(files.iter().all(|f| f.completeness == 100));

    // Reset progress
    queries::reset_audiobook_progress(conn, audiobook_id)?;

    let audiobook = queries::get_audiobook_by_id(conn, audiobook_id)?
        .ok_or_else(|| missing("Audiobook not found"))?;
    assert_eq!(audiobook.completeness, 0);
    assert_eq!(audiobook.selected_file, None);

    let files = queries::get_audiobook_files(conn, audiobook_id)?;
    assert!(files.iter().all(|f| f.completeness == 0));
    assert!(files.iter().all(|f| f.seek_position.is_none()));
    Ok(())
}

#[test]
fn test_metadata_operations() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let conn = db.connection();

    // Set metadata
    queries::set_metadata(conn, "volume", "75")?;
    queries::set_metadata(conn, "speed", "1.5")?;

    // Get metadata
    let volume =
        queries::get_metadata(conn, "volume")?.ok_or_else(|| missing("Volume not found"))?;
    assert_eq!(volume, "75");

    let speed = queries::get_metadata(conn, "speed")?.ok_or_else(|| missing("Speed not found"))?;
    assert_eq!(speed, "1.5");

    // Update metadata
    queries::set_metadata(conn, "volume", "100")?;
    let volume =
        queries::get_metadata(conn, "volume")?.ok_or_else(|| missing("Volume not found"))?;
    assert_eq!(volume, "100");

    // Non-existent key
    let result = queries::get_metadata(conn, "nonexistent")?;
    assert!(result.is_none());
    Ok(())
}

#[test]
fn test_count_operations() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let conn = db.connection();

    // Initially zero
    let count = queries::count_audiobooks(conn)?;
    assert_eq!(count, 0);

    // Add directory and audiobooks
    let dir = Directory {
        full_path: "/test/audiobooks".to_string(),
        created_at: Utc::now(),
        last_scanned: None,
    };
    queries::insert_directory(conn, &dir)?;

    for i in 0..5 {
        let audiobook = Audiobook {
            id: None,
            directory: "/test/audiobooks".to_string(),
            name: format!("Audiobook {}", i + 1),
            full_path: format!("/test/audiobooks/book{}", i + 1),
            completeness: 0,
            default_order: i,
            selected_file: None,
            created_at: Utc::now(),
        };
        queries::insert_audiobook(conn, &audiobook)?;
    }

    let count = queries::count_audiobooks(conn)?;
    assert_eq!(count, 5);

    // Add files to first audiobook
    let audiobooks = queries::get_all_audiobooks(conn)?;
    let first = audiobooks
        .first()
        .ok_or_else(|| missing("Expected audiobook"))?;
    let first_id = first.id.ok_or_else(|| missing("Audiobook has no ID"))?;

    for i in 0..3 {
        let file = AudiobookFile {
            audiobook_id: first_id,
            name: format!("Chapter {}.mp3", i + 1),
            full_path: format!("/test/audiobooks/book1/Chapter {}.mp3", i + 1),
            length_of_file: Some(300_000),
            seek_position: None,
            position: i,
            completeness: 0,
            file_exists: true,
            created_at: Utc::now(),
        };
        queries::insert_audiobook_file(conn, &file)?;
    }

    let file_count = queries::count_audiobook_files(conn, first_id)?;
    assert_eq!(file_count, 3);
    Ok(())
}

#[test]
fn test_cascade_delete_directory() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let conn = db.connection();

    // Setup directory with audiobooks and files
    let dir = Directory {
        full_path: "/test/audiobooks".to_string(),
        created_at: Utc::now(),
        last_scanned: None,
    };
    queries::insert_directory(conn, &dir)?;

    let audiobook = Audiobook {
        id: None,
        directory: "/test/audiobooks".to_string(),
        name: "Test Audiobook".to_string(),
        full_path: "/test/audiobooks/test".to_string(),
        completeness: 0,
        default_order: 0,
        selected_file: None,
        created_at: Utc::now(),
    };
    let audiobook_id = queries::insert_audiobook(conn, &audiobook)?;

    let file = AudiobookFile {
        audiobook_id,
        name: "Chapter 1.mp3".to_string(),
        full_path: "/test/audiobooks/test/Chapter 1.mp3".to_string(),
        length_of_file: Some(300_000),
        seek_position: None,
        position: 0,
        completeness: 0,
        file_exists: true,
        created_at: Utc::now(),
    };
    queries::insert_audiobook_file(conn, &file)?;

    // Delete directory should cascade
    queries::delete_directory(conn, "/test/audiobooks")?;

    // Verify everything is deleted
    let dirs = queries::get_all_directories(conn)?;
    assert_eq!(dirs.len(), 0);

    let audiobooks = queries::get_all_audiobooks(conn)?;
    assert_eq!(audiobooks.len(), 0);

    let files = queries::get_audiobook_files(conn, audiobook_id)?;
    assert_eq!(files.len(), 0);
    Ok(())
}
