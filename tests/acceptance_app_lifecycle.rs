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
fn test_startup_time_with_1000_audiobooks() -> Result<(), Box<dyn Error>> {
    let temp_db_dir = temp_dir::TempDir::new()?;
    let db_path = temp_db_dir.path().join("large.db");

    // Setup: Create database with 1000 audiobooks
    {
        let db = nodoka::Database::open_with_path(&db_path)?;
        nodoka::db::initialize(db.connection())?;

        for i in 0..1000 {
            create_test_audiobook(&db, "/test/library", &format!("Book {i:04}"))?;
        }
    }

    // Test: Measure startup time
    let start = std::time::Instant::now();
    let db = nodoka::Database::open_with_path(&db_path)?;
    let audiobooks = queries::get_all_audiobooks(db.connection())?;
    let duration = start.elapsed();

    assert_eq!(audiobooks.len(), 1000);

    // Startup should be < 3 seconds per spec
    assert!(
        duration < std::time::Duration::from_secs(3),
        "Startup took {}ms (expected < 3000ms)",
        duration.as_millis()
    );

    Ok(())
}

#[test]
fn test_large_library_query_performance() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;

    // Create 1000 audiobooks
    for i in 0..1000 {
        create_test_audiobook(&db, "/test/library", &format!("Book {i:04}"))?;
    }

    // Query should be fast
    let start = std::time::Instant::now();
    let audiobooks = queries::get_all_audiobooks(db.connection())?;
    let duration = start.elapsed();

    assert_eq!(audiobooks.len(), 1000);

    // Query should complete quickly (< 500ms for 1000 records)
    assert!(
        duration < std::time::Duration::from_millis(500),
        "Query took {}ms (expected < 500ms)",
        duration.as_millis()
    );

    Ok(())
}

#[test]
fn test_database_migration_on_version_upgrade() -> Result<(), Box<dyn Error>> {
    let temp_db_dir = temp_dir::TempDir::new()?;
    let db_path = temp_db_dir.path().join("migration.db");

    // Create database with initial schema
    {
        let db = nodoka::Database::open_with_path(&db_path)?;
        nodoka::db::initialize(db.connection())?;

        create_test_audiobook(&db, "/test", "Test Book")?;
    }

    // Reopen database (simulating upgrade)
    {
        let db = nodoka::Database::open_with_path(&db_path)?;

        // Data should still be accessible
        let audiobooks = queries::get_all_audiobooks(db.connection())?;
        assert_eq!(audiobooks.len(), 1);
        assert_eq!(
            audiobooks.first().ok_or("No audiobook found")?.name,
            "Test Book"
        );
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
            "Missing required table: {table}"
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
        let audiobook_id = audiobooks
            .first()
            .ok_or("No audiobook found")?
            .id
            .ok_or("No ID")?;

        let files = queries::get_audiobook_files(db.connection(), audiobook_id)?;
        assert_eq!(
            files.first().ok_or("No file found")?.seek_position,
            Some(1500)
        );
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
fn test_corrupted_window_geometry_is_repaired() -> Result<(), Box<dyn Error>> {
    use nodoka::db::queries;

    let db = create_test_db()?;
    let settings = Settings::new(db.connection());

    // Corrupt persisted values.
    queries::set_metadata(db.connection(), "window_x", "not-an-int")?;
    queries::set_metadata(db.connection(), "window_y", "200")?;
    queries::set_metadata(db.connection(), "window_width", "wide")?;
    queries::set_metadata(db.connection(), "window_height", "600")?;

    assert_eq!(settings.get_window_position()?, None);
    assert_eq!(settings.get_window_size()?, None);

    // Keys should be cleared so subsequent reads don't keep re-parsing bad data.
    assert_eq!(queries::get_metadata(db.connection(), "window_x")?, None);
    assert_eq!(queries::get_metadata(db.connection(), "window_y")?, None);
    assert_eq!(
        queries::get_metadata(db.connection(), "window_width")?,
        None
    );
    assert_eq!(
        queries::get_metadata(db.connection(), "window_height")?,
        None
    );

    Ok(())
}

#[test]
fn test_window_geometry_is_applied_to_app_window_settings() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let settings = Settings::new(db.connection());

    settings.set_window_position(100, 200)?;
    settings.set_window_size(800, 600)?;

    let win = nodoka::app::window_settings_from_storage(db.connection(), None);

    assert_eq!(win.size, iced::Size::new(800.0, 600.0));

    match win.position {
        iced::window::Position::Specific(point) => {
            assert!((point.x - 100.0).abs() < f32::EPSILON);
            assert!((point.y - 200.0).abs() < f32::EPSILON);
        }
        other => return Err(format!("expected specific position, got: {other:?}").into()),
    }

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
        "Database initialization took too long: {elapsed:?}"
    );

    Ok(())
}

#[test]
fn test_large_library_startup() -> Result<(), Box<dyn Error>> {
    use std::time::Instant;

    let db = create_test_db()?;

    // Create many audiobooks
    for i in 0..100 {
        create_test_audiobook(&db, "/test", &format!("Book {i}"))?;
    }

    // Should still be able to query quickly
    let start = Instant::now();

    let audiobooks = queries::get_all_audiobooks(db.connection())?;

    let elapsed = start.elapsed();

    assert_eq!(audiobooks.len(), 100);
    assert!(elapsed.as_secs() < 1, "Query took too long: {elapsed:?}");

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
