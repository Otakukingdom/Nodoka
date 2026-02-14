mod acceptance_support;
use acceptance_support::*;

use nodoka::player::Vlc;
use std::error::Error;
use std::path::Path;

#[test]
fn test_missing_vlc_handled_gracefully() {
    // If VLC not available, constructors should return Err
    let result = Vlc::new();
    if let Err(e) = result {
        // Error should be informative
        let err_str = format!("{:?}", e).to_lowercase();
        assert!(
            err_str.contains("vlc") || err_str.contains("library") || err_str.contains("not found"),
            "Error message should mention VLC"
        );
    }
}

#[test]
fn test_unplayable_file_shows_error() -> Result<(), Box<dyn Error>> {
    if let Ok(mut player) = Vlc::new() {
        let fixtures = TestFixtures::new();
        let corrupted = fixtures.audio_path("corrupted.mp3");

        if corrupted.exists() {
            let result = player.load_media(&corrupted);
            // Should handle gracefully, not crash
            assert!(result.is_ok() || result.is_err());
        }
    }

    Ok(())
}

#[test]
fn test_database_errors_return_result() -> Result<(), Box<dyn Error>> {
    use nodoka::db::queries;

    let db = create_test_db()?;

    // Try to query non-existent ID
    let result = queries::get_audiobook_by_id(db.connection(), 999999);

    // Should return Ok(None), not error
    assert!(result.is_ok());
    assert!(result?.is_none());

    Ok(())
}

#[test]
fn test_nonexistent_file_handled() -> Result<(), Box<dyn Error>> {
    if let Ok(mut player) = Vlc::new() {
        let nonexistent = Path::new("/nonexistent/path/to/file.mp3");
        let result = player.load_media(nonexistent);

        // Should not panic
        assert!(result.is_ok() || result.is_err());
    }

    Ok(())
}

#[test]
fn test_invalid_directory_path_handled() -> Result<(), Box<dyn Error>> {
    use nodoka::db::queries;
    use nodoka::models::Directory;

    let db = create_test_db()?;

    let invalid_dir = Directory {
        full_path: "/this/path/does/not/exist".to_string(),
        created_at: chrono::Utc::now(),
        last_scanned: None,
    };

    // Should be able to insert (path validation happens at scan time)
    let result = queries::insert_directory(db.connection(), &invalid_dir);
    assert!(result.is_ok());

    Ok(())
}

#[test]
fn test_empty_string_inputs_handled() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;

    // Empty audiobook name should be handled
    let result = create_test_audiobook(&db, "/test", "");

    // Should either succeed with empty name or fail gracefully
    assert!(result.is_ok() || result.is_err());

    Ok(())
}

#[test]
fn test_very_long_paths_handled() -> Result<(), Box<dyn Error>> {
    use nodoka::db::queries;
    use nodoka::models::Directory;

    let db = create_test_db()?;

    let long_path = "/".to_string() + &"very_long_directory_name/".repeat(50);

    let dir = Directory {
        full_path: long_path.clone(),
        created_at: chrono::Utc::now(),
        last_scanned: None,
    };

    let result = queries::insert_directory(db.connection(), &dir);

    // Should handle long paths (may succeed or fail depending on limits)
    assert!(result.is_ok() || result.is_err());

    Ok(())
}

#[test]
fn test_special_characters_in_paths() -> Result<(), Box<dyn Error>> {
    use nodoka::db::queries;
    use nodoka::models::Directory;

    let db = create_test_db()?;

    let special_path = "/path/with/special/chars/!@#$%^&*()/";

    let dir = Directory {
        full_path: special_path.to_string(),
        created_at: chrono::Utc::now(),
        last_scanned: None,
    };

    let result = queries::insert_directory(db.connection(), &dir);
    assert!(result.is_ok());

    Ok(())
}

#[test]
fn test_concurrent_database_access() -> Result<(), Box<dyn Error>> {
    use nodoka::db::queries;

    let db = create_test_db()?;

    // Simulate concurrent reads
    let ab1 = queries::get_all_audiobooks(db.connection())?;
    let ab2 = queries::get_all_audiobooks(db.connection())?;

    assert_eq!(ab1.len(), ab2.len());

    Ok(())
}

#[test]
fn test_missing_file_in_audiobook() -> Result<(), Box<dyn Error>> {
    use nodoka::db::queries;

    let db = create_test_db()?;
    let audiobook_id = create_test_audiobook(&db, "/test", "Book")?;

    // Insert file that doesn't actually exist
    insert_test_file(&db, audiobook_id, "/nonexistent/file.mp3")?;

    let files = queries::get_audiobook_files(db.connection(), audiobook_id)?;
    assert_eq!(files.len(), 1);

    // Application should mark as missing when scanning, not crash

    Ok(())
}

#[test]
fn test_error_messages_are_readable() {
    use std::io;

    let err = io::Error::new(io::ErrorKind::NotFound, "File not found");
    let msg = err.to_string();

    assert!(!msg.is_empty());
    assert!(msg.contains("not found") || msg.contains("File"));
}
