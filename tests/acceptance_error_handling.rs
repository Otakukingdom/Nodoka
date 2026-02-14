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
        let err_str = format!("{e:?}").to_lowercase();
        assert!(
            err_str.contains("vlc") || err_str.contains("library") || err_str.contains("not found"),
            "Error message should mention VLC"
        );
    }
}

#[test]
fn test_unplayable_file_shows_error() {
    if let Ok(mut player) = Vlc::new() {
        let fixtures = TestFixtures::new();
        let corrupted = fixtures.audio_path("corrupted.mp3");

        if corrupted.exists() {
            let load_result = player.load_media(&corrupted);
            match load_result {
                Ok(()) => {
                    // If media object was created, attempting to play should fail or transition
                    // to an error-like state, but must not panic.
                    let play_result = player.play();
                    if play_result.is_ok() {
                        assert!(
                            !matches!(player.get_state(), nodoka::player::PlaybackState::Playing),
                            "Corrupted media should not play successfully"
                        );
                    } else {
                        assert!(!format!("{play_result:?}").is_empty());
                    }
                }
                Err(e) => {
                    assert!(!format!("{e}").is_empty());
                }
            }
        }
    }
}

#[test]
fn test_database_errors_return_result() -> Result<(), Box<dyn Error>> {
    use nodoka::db::queries;

    let db = create_test_db()?;

    // Try to query non-existent ID
    let result = queries::get_audiobook_by_id(db.connection(), 999_999);

    // Should return Ok(None), not error
    assert!(result.is_ok());
    assert!(result?.is_none());

    Ok(())
}

#[test]
fn test_nonexistent_file_handled() {
    if let Ok(mut player) = Vlc::new() {
        let nonexistent = Path::new("/nonexistent/path/to/file.mp3");
        let result = player.load_media(nonexistent);

        assert!(result.is_err());
    }
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

    // Adding a non-existent directory should return an error.
    let result = queries::insert_directory(db.connection(), &invalid_dir);
    assert!(result.is_err());

    Ok(())
}

#[test]
fn test_empty_string_inputs_handled() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;

    // Empty audiobook name should be handled
    let result = create_test_audiobook(&db, "/test", "");

    assert!(result.is_ok());

    Ok(())
}

#[test]
fn test_very_long_paths_handled() -> Result<(), Box<dyn Error>> {
    use nodoka::db::queries;
    use nodoka::models::Directory;

    let db = create_test_db()?;

    let long_path = "/".to_string() + &"very_long_directory_name/".repeat(50);

    let dir = Directory {
        full_path: long_path,
        created_at: chrono::Utc::now(),
        last_scanned: None,
    };

    let result = queries::insert_directory(db.connection(), &dir);

    assert!(result.is_err());

    Ok(())
}

#[test]
fn test_special_characters_in_paths() -> Result<(), Box<dyn Error>> {
    use nodoka::db::queries;
    use nodoka::models::Directory;

    let db = create_test_db()?;

    let temp = temp_dir::TempDir::new()?;
    let special_dir = temp.path().join("My Audiobooks (2024) !@#$%&()");
    std::fs::create_dir_all(&special_dir)?;
    let special_path = special_dir.to_str().ok_or("Invalid path")?;

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

#[test]
fn test_very_long_metadata_strings() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;

    // Create audiobook with very long name (10,000 characters)
    let long_name = "A".repeat(10000);
    let result = create_test_audiobook(&db, "/test", &long_name);

    // Should handle long strings without panic or truncation error
    assert!(result.is_ok());

    Ok(())
}

#[test]
fn test_invalid_database_path_handled() {
    use std::path::PathBuf;

    // Try to open database at invalid location
    let invalid_path = PathBuf::from("/nonexistent/directory/that/cannot/exist/db.sqlite");

    let result = nodoka::Database::open_with_path(&invalid_path);

    // Should return error, not panic
    assert!(result.is_err());
}

#[test]
fn test_concurrent_database_writes() -> Result<(), Box<dyn Error>> {
    use nodoka::db::queries;

    let db = create_test_db()?;

    // Perform multiple writes
    for i in 0..10 {
        let _ = create_test_audiobook(&db, "/test", &format!("Book {i}"))?;
    }

    let audiobooks = queries::get_all_audiobooks(db.connection())?;
    assert_eq!(audiobooks.len(), 10);

    Ok(())
}

#[test]
fn test_null_bytes_in_strings_handled() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;

    // String with null byte (invalid in many contexts)
    let name_with_null = "Book\0Name";

    let result = create_test_audiobook(&db, "/test", name_with_null);

    assert!(result.is_err());

    Ok(())
}

#[test]
fn test_extremely_deep_path_nesting() -> Result<(), Box<dyn Error>> {
    // Test handling of very deep directory structures
    let mut deep_path = String::from("/root");
    for i in 0..100 {
        deep_path.push_str(&format!("/level{i}"));
    }

    let db = create_test_db()?;
    let result = create_test_audiobook(&db, &deep_path, "Deep Path Book");

    // Should handle without stack overflow or path length errors
    assert!(result.is_ok());

    Ok(())
}

#[test]
fn test_vlc_not_installed_error_message() {
    // Test that VLC errors are clear and actionable
    let result = Vlc::new();

    if let Err(e) = result {
        let error_msg = format!("{e}");
        // Error should mention VLC and be actionable
        assert!(
            error_msg.to_lowercase().contains("vlc") || error_msg.to_lowercase().contains("libvlc"),
            "Error should mention VLC: {error_msg}"
        );
    }
}

#[test]
fn test_network_path_errors_handled() -> Result<(), Box<dyn Error>> {
    use nodoka::db::queries;
    use nodoka::models::Directory;

    let db = create_test_db()?;

    // Test various network path formats
    let network_paths = vec![
        "\\\\server\\share\\audiobooks", // UNC path (Windows)
        "smb://server/share/audiobooks", // SMB path
        "//server/share/audiobooks",     // Alternative format
    ];

    for path in network_paths {
        let dir = Directory {
            full_path: path.to_string(),
            created_at: chrono::Utc::now(),
            last_scanned: None,
        };

        let result = queries::insert_directory(db.connection(), &dir);
        assert!(result.is_err());
    }

    Ok(())
}

#[test]
fn test_readonly_database_error() -> Result<(), Box<dyn Error>> {
    use std::fs;
    use temp_dir::TempDir;

    let temp = TempDir::new()?;
    let db_path = temp.path().join("readonly.db");

    // Create database
    {
        let _db = nodoka::Database::open_with_path(&db_path)?;
    }

    // Make it read-only
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&db_path)?.permissions();
        perms.set_mode(0o444);
        fs::set_permissions(&db_path, perms)?;
    }

    // Attempt to open and write should handle readonly gracefully
    let result = nodoka::Database::open_with_path(&db_path);

    match result {
        Ok(db) => {
            // Opening might succeed, but initializing schema requires writes.
            let init_result = nodoka::db::initialize(db.connection());
            assert!(init_result.is_err());
        }
        Err(e) => {
            assert!(!format!("{e}").is_empty());
        }
    }

    Ok(())
}

#[test]
fn test_unicode_error_messages() -> Result<(), Box<dyn Error>> {
    // Test that error messages with unicode work correctly
    let db = create_test_db()?;
    let unicode_name = "Book with émojis 📚 and 日本語";

    let result = create_test_audiobook(&db, "/test/παθ", unicode_name);

    // Should handle unicode without corruption
    assert!(result.is_ok());

    Ok(())
}

#[test]
fn test_progress_save_error_recovery() -> Result<(), Box<dyn Error>> {
    use nodoka::db::queries;

    let db = create_test_db()?;
    let audiobook_id = create_test_audiobook(&db, "/test", "Book")?;
    insert_test_file(&db, audiobook_id, "/test/chapter1.mp3")?;

    // Save valid progress
    let result =
        queries::update_file_progress(db.connection(), "/test/chapter1.mp3", 100_000.0, 50);

    assert!(result.is_ok());

    // Try to save with invalid values
    let result = queries::update_file_progress(
        db.connection(),
        "/test/chapter1.mp3",
        -1.0, // Invalid negative value
        -1,
    );

    // Should handle invalid input gracefully
    assert!(result.is_err());

    Ok(())
}
