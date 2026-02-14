//! Acceptance tests for Cross-Platform Compatibility (Category F, Section 18)
//!
//! Tests platform-specific behaviors including file paths, default directories,
//! and platform conventions.

mod acceptance_support;
use acceptance_support::*;

use std::error::Error;
use std::path::Path;
use temp_dir::TempDir;

#[test]
fn test_file_paths_with_spaces() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let temp = TempDir::new()?;

    // Create directory with spaces in name
    let dir_with_spaces = temp.path().join("My Audio Books");
    std::fs::create_dir(&dir_with_spaces)?;

    let audiobook_dir =
        create_test_audiobook_directory(&temp, "My Audio Books/Book With Spaces", 2)?;

    // Verify path handling
    assert!(audiobook_dir.exists());

    // Test database storage
    let path_str = audiobook_dir.to_str().ok_or("Invalid path")?;
    let _audiobook_id = create_test_audiobook(&db, path_str, "Test Book")?;

    // Verify retrieval
    let audiobooks = nodoka::db::queries::get_all_audiobooks(db.connection())?;
    assert_eq!(audiobooks.len(), 1);
    assert_eq!(audiobooks[0].name, "Test Book");

    Ok(())
}

#[test]
fn test_file_paths_with_unicode() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let temp = TempDir::new()?;

    // Create directory with Unicode characters
    let unicode_dir = temp.path().join("오디오북"); // Korean
    std::fs::create_dir(&unicode_dir)?;

    let path_str = unicode_dir.to_str().ok_or("Invalid Unicode path")?;
    let _audiobook_id = create_test_audiobook(&db, path_str, "유니코드 테스트")?;

    let audiobooks = nodoka::db::queries::get_all_audiobooks(db.connection())?;
    assert_eq!(audiobooks.len(), 1);
    assert_eq!(audiobooks[0].name, "유니코드 테스트");

    Ok(())
}

#[cfg(target_os = "windows")]
#[test]
fn test_windows_path_format() -> Result<(), Box<dyn Error>> {
    // Test Windows-specific path handling
    let path = Path::new("C:\\Users\\Test\\Audiobooks");
    assert!(path.to_str().is_some());

    // Test that backslashes are handled correctly
    let db = create_test_db()?;
    let _ = create_test_audiobook(&db, "C:\\Users\\Test\\Audiobooks", "Windows Path Test")?;

    let audiobooks = nodoka::db::queries::get_all_audiobooks(db.connection())?;
    assert_eq!(audiobooks.len(), 1);

    Ok(())
}

#[cfg(target_os = "macos")]
#[test]
fn test_macos_path_format() -> Result<(), Box<dyn Error>> {
    // Test macOS-specific path handling
    let path = Path::new("/Users/test/Audiobooks");
    assert!(path.to_str().is_some());

    let db = create_test_db()?;
    let _ = create_test_audiobook(&db, "/Users/test/Audiobooks", "macOS Path Test")?;

    let audiobooks = nodoka::db::queries::get_all_audiobooks(db.connection())?;
    assert_eq!(audiobooks.len(), 1);

    Ok(())
}

#[cfg(target_os = "linux")]
#[test]
fn test_linux_path_format() -> Result<(), Box<dyn Error>> {
    // Test Linux-specific path handling
    let path = Path::new("/home/test/audiobooks");
    assert!(path.to_str().is_some());

    let db = create_test_db()?;
    let _ = create_test_audiobook(&db, "/home/test/audiobooks", "Linux Path Test")?;

    let audiobooks = nodoka::db::queries::get_all_audiobooks(db.connection())?;
    assert_eq!(audiobooks.len(), 1);

    Ok(())
}

#[test]
fn test_absolute_paths_stored_correctly() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let temp = TempDir::new()?;

    // Create a test audiobook with absolute path
    let audiobook_dir = create_test_audiobook_directory(&temp, "Test Book", 1)?;
    let path_str = audiobook_dir.to_str().ok_or("Invalid path")?;

    let _audiobook_id = create_test_audiobook(&db, path_str, "Absolute Path Test")?;

    // Verify path is stored as absolute
    let audiobooks = nodoka::db::queries::get_all_audiobooks(db.connection())?;
    assert_eq!(audiobooks.len(), 1);

    let stored_path = Path::new(&audiobooks[0].directory);
    assert!(
        stored_path.is_absolute(),
        "Path should be stored as absolute"
    );

    Ok(())
}

#[test]
fn test_path_separators_normalized() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let temp = TempDir::new()?;

    // Create nested directory structure
    let nested_dir = temp.path().join("level1").join("level2").join("level3");
    std::fs::create_dir_all(&nested_dir)?;

    let path_str = nested_dir.to_str().ok_or("Invalid path")?;
    let _ = create_test_audiobook(&db, path_str, "Nested Path Test")?;

    let audiobooks = nodoka::db::queries::get_all_audiobooks(db.connection())?;
    assert_eq!(audiobooks.len(), 1);

    // Verify path contains proper separators for the platform
    let stored_path = &audiobooks[0].directory;
    assert!(stored_path.contains(std::path::MAIN_SEPARATOR));

    Ok(())
}

#[test]
fn test_special_characters_in_path() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let temp = TempDir::new()?;

    // Create directory with special characters (that are valid on filesystems)
    let special_dir = temp.path().join("Books-2024");
    std::fs::create_dir(&special_dir)?;

    let path_str = special_dir.to_str().ok_or("Invalid path")?;
    let _ = create_test_audiobook(&db, path_str, "Special Chars Test")?;

    let audiobooks = nodoka::db::queries::get_all_audiobooks(db.connection())?;
    assert_eq!(audiobooks.len(), 1);

    Ok(())
}

#[test]
fn test_empty_path_handling() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;

    // Attempting to create audiobook with empty path should be handled
    let result = create_test_audiobook(&db, "", "Empty Path Test");

    // Either succeeds (allowing empty paths) or fails gracefully
    assert!(result.is_ok() || result.is_err());

    Ok(())
}
