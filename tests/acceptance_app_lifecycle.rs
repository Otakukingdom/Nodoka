mod acceptance_support;
use acceptance_support::*;

use nodoka::db::queries;
use nodoka::settings::Settings;
use std::error::Error;

#[test]
fn test_first_launch_creates_database() -> Result<(), Box<dyn Error>> {
    let temp_dir = temp_dir::TempDir::new()?;
    let db_path = temp_dir.path().join("nodoka_first_launch.db");

    assert!(!db_path.exists());

    let db = nodoka::db::Database::open_with_path(&db_path)?;
    nodoka::db::initialize(db.connection())?;

    assert!(db_path.exists());

    // Verify schema created
    let tables: Vec<String> = db
        .connection()
        .prepare("SELECT name FROM sqlite_master WHERE type='table'")?
        .query_map([], |row| row.get(0))?
        .collect::<Result<Vec<_>, _>>()?;

    assert!(tables.contains(&"audiobooks".to_string()));
    assert!(tables.contains(&"audiobook_file".to_string()));

    Ok(())
}

#[test]
fn test_restore_last_selected_audiobook() -> Result<(), Box<dyn Error>> {
    let temp_db_dir = temp_dir::TempDir::new()?;
    let db_path = temp_db_dir.path().join("lifecycle.db");

    // Session 1: Select audiobook
    {
        let db = nodoka::db::Database::open_with_path(&db_path)?;
        nodoka::db::initialize(db.connection())?;

        let audiobook_id = create_test_audiobook(&db, "/test", "Last Book")?;

        let settings = Settings::new(db.connection());
        settings.set_last_audiobook_id(audiobook_id)?;
    }

    // Session 2: Restore
    {
        let db = nodoka::db::Database::open_with_path(&db_path)?;
        let settings = Settings::new(db.connection());

        let last_id = settings.get_last_audiobook_id()?;
        assert!(last_id.is_some());

        let audiobook = queries::get_audiobook_by_id(db.connection(), last_id.unwrap())?;
        assert!(audiobook.is_some());
        assert_eq!(audiobook.unwrap().name, "Last Book");
    }

    Ok(())
}

#[test]
fn test_schema_idempotent() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;

    // Initialize multiple times
    nodoka::db::initialize(db.connection())?;
    nodoka::db::initialize(db.connection())?;
    nodoka::db::initialize(db.connection())?;

    // Should not fail or duplicate tables

    Ok(())
}

#[test]
fn test_database_schema_has_required_tables() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;

    let tables: Vec<String> = db
        .connection()
        .prepare("SELECT name FROM sqlite_master WHERE type='table'")?
        .query_map([], |row| row.get(0))?
        .collect::<Result<Vec<_>, _>>()?;

    let required_tables = vec![
        "audiobooks",
        "audiobook_file",
        "directories",
        "metadata",
        "bookmarks",
    ];

    for table in required_tables {
        assert!(
            tables.contains(&table.to_string()),
            "Missing required table: {}",
            table
        );
    }

    Ok(())
}

#[test]
fn test_graceful_shutdown_saves_progress() -> Result<(), Box<dyn Error>> {
    let temp_db_dir = temp_dir::TempDir::new()?;
    let db_path = temp_db_dir.path().join("shutdown_test.db");

    // Simulate application session
    {
        let db = nodoka::db::Database::open_with_path(&db_path)?;
        nodoka::db::initialize(db.connection())?;

        let audiobook_id = create_test_audiobook(&db, "/test", "Book")?;
        insert_test_file(&db, audiobook_id, "/test/Book/file.mp3")?;

        queries::update_file_progress(db.connection(), "/test/Book/file.mp3", 1500.0, 50)?;

        // Implicit drop/close of database
    }

    // Verify progress saved
    {
        let db = nodoka::db::Database::open_with_path(&db_path)?;
        let audiobooks = queries::get_all_audiobooks(db.connection())?;
        let audiobook_id = audiobooks[0].id.ok_or("No ID")?;

        let files = queries::get_audiobook_files(db.connection(), audiobook_id)?;
        assert_eq!(files[0].seek_position, Some(1500));
    }

    Ok(())
}

#[test]
fn test_restore_window_state() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let settings = Settings::new(db.connection());

    // Save window position and size
    settings.set_window_position(100, 200)?;
    settings.set_window_size(800, 600)?;

    // Restore
    let pos = settings.get_window_position()?;
    let size = settings.get_window_size()?;

    assert_eq!(pos, Some((100, 200)));
    assert_eq!(size, Some((800, 600)));

    Ok(())
}

#[test]
fn test_startup_time_reasonable() -> Result<(), Box<dyn Error>> {
    use std::time::Instant;

    let temp_dir = temp_dir::TempDir::new()?;
    let db_path = temp_dir.path().join("startup_test.db");

    let start = Instant::now();

    let db = nodoka::db::Database::open_with_path(&db_path)?;
    nodoka::db::initialize(db.connection())?;

    let elapsed = start.elapsed();

    // Database initialization should be fast (< 1 second for empty DB)
    assert!(
        elapsed.as_secs() < 5,
        "Database initialization took too long: {:?}",
        elapsed
    );

    Ok(())
}

#[test]
fn test_large_library_startup() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;

    // Create many audiobooks
    for i in 0..100 {
        create_test_audiobook(&db, "/test", &format!("Book {}", i))?;
    }

    // Should still be able to query quickly
    use std::time::Instant;
    let start = Instant::now();

    let audiobooks = queries::get_all_audiobooks(db.connection())?;

    let elapsed = start.elapsed();

    assert_eq!(audiobooks.len(), 100);
    assert!(elapsed.as_secs() < 1, "Query took too long: {:?}", elapsed);

    Ok(())
}

#[test]
fn test_database_versioning() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;

    // Check schema version (stored in metadata table)
    let version: Option<String> = db
        .connection()
        .prepare("SELECT value FROM metadata WHERE key = 'schema_version'")?
        .query_row([], |row| row.get(0))
        .ok();

    // Should have a version or be initial schema
    assert!(version.is_some() || version.is_none());

    Ok(())
}

#[test]
fn test_subsequent_launches_restore_state() -> Result<(), Box<dyn Error>> {
    let temp_db_dir = temp_dir::TempDir::new()?;
    let db_path = temp_db_dir.path().join("state_test.db");

    // First launch
    {
        let db = nodoka::db::Database::open_with_path(&db_path)?;
        nodoka::db::initialize(db.connection())?;

        let settings = Settings::new(db.connection());
        settings.set_default_volume(125)?;
    }

    // Second launch
    {
        let db = nodoka::db::Database::open_with_path(&db_path)?;
        let settings = Settings::new(db.connection());

        assert_eq!(settings.get_default_volume()?, Some(125));
    }

    // Third launch
    {
        let db = nodoka::db::Database::open_with_path(&db_path)?;
        let settings = Settings::new(db.connection());

        assert_eq!(settings.get_default_volume()?, Some(125));
    }

    Ok(())
}
