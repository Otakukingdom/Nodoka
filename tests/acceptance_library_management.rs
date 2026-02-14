//! Acceptance tests for Library Source Management (Category A)
//!
//! Tests directory addition, removal, persistence, and edge cases.

mod acceptance_support;
use acceptance_support::*;

use chrono::Utc;
use nodoka::db::queries;
use nodoka::models::Directory;
use std::error::Error;
use temp_dir::TempDir;

#[test]
fn test_add_directory_via_database() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let temp = TempDir::new()?;
    let dir_path = temp.path().to_str().ok_or("Invalid path")?;

    let dir = Directory {
        full_path: dir_path.to_string(),
        created_at: Utc::now(),
        last_scanned: None,
    };

    queries::insert_directory(db.connection(), &dir)?;

    let directories = queries::get_all_directories(db.connection())?;
    assert_eq!(directories.len(), 1);
    assert_eq!(
        directories.first().ok_or("No directory")?.full_path,
        dir_path
    );

    Ok(())
}

#[test]
fn test_remove_directory_removes_audiobooks() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let temp = TempDir::new()?;
    let dir_path = temp.path().to_str().ok_or("Invalid path")?;

    // Add directory and audiobook
    let dir = Directory {
        full_path: dir_path.to_string(),
        created_at: Utc::now(),
        last_scanned: None,
    };
    queries::insert_directory(db.connection(), &dir)?;

    let _audiobook_id = create_test_audiobook(&db, dir_path, "Test Book")?;

    // Verify audiobook exists
    let audiobooks_before = queries::get_all_audiobooks(db.connection())?;
    assert_eq!(audiobooks_before.len(), 1);

    // Remove directory
    queries::delete_directory(db.connection(), dir_path)?;

    // Verify audiobook removed
    let audiobooks_after = queries::get_all_audiobooks(db.connection())?;
    assert_eq!(audiobooks_after.len(), 0);

    Ok(())
}

#[test]
fn test_directories_persist_across_restarts() -> Result<(), Box<dyn Error>> {
    let temp_db = TempDir::new()?;
    let db_path = temp_db.path().join("test.db");
    let temp_dir = TempDir::new()?;
    let dir_path = temp_dir.path().to_str().ok_or("Invalid path")?.to_string();

    // First session: add directory
    {
        let db = nodoka::Database::open_with_path(&db_path)?;
        nodoka::db::initialize(db.connection())?;

        let dir = Directory {
            full_path: dir_path.clone(),
            created_at: Utc::now(),
            last_scanned: None,
        };
        queries::insert_directory(db.connection(), &dir)?;
    }

    // Second session: verify persistence
    {
        let db = nodoka::Database::open_with_path(&db_path)?;
        let directories = queries::get_all_directories(db.connection())?;
        assert_eq!(directories.len(), 1);
        assert_eq!(
            directories.first().ok_or("No directory")?.full_path,
            dir_path
        );
    }

    Ok(())
}

#[test]
fn test_duplicate_directory_rejected() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let temp = TempDir::new()?;
    let dir_path = temp.path().to_str().ok_or("Invalid path")?;

    let dir = Directory {
        full_path: dir_path.to_string(),
        created_at: Utc::now(),
        last_scanned: None,
    };

    // First insert succeeds
    queries::insert_directory(db.connection(), &dir)?;

    // Second insert with same path - should replace due to INSERT OR REPLACE
    queries::insert_directory(db.connection(), &dir)?;

    // Should still have only one entry
    let directories = queries::get_all_directories(db.connection())?;
    assert_eq!(directories.len(), 1);

    Ok(())
}

#[test]
fn test_empty_directories_handled() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let temp = TempDir::new()?;
    let empty_dir = temp.path().join("empty");
    std::fs::create_dir(&empty_dir)?;
    let dir_path = empty_dir.to_str().ok_or("Invalid path")?;

    let dir = Directory {
        full_path: dir_path.to_string(),
        created_at: Utc::now(),
        last_scanned: None,
    };

    // Should be able to add empty directory
    queries::insert_directory(db.connection(), &dir)?;

    let directories = queries::get_all_directories(db.connection())?;
    assert_eq!(directories.len(), 1);

    Ok(())
}

#[test]
fn test_directory_with_special_characters() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let temp = TempDir::new()?;
    let special_dir = temp.path().join("My Audiobooks (2024)");
    std::fs::create_dir(&special_dir)?;
    let dir_path = special_dir.to_str().ok_or("Invalid path")?;

    let dir = Directory {
        full_path: dir_path.to_string(),
        created_at: Utc::now(),
        last_scanned: None,
    };

    queries::insert_directory(db.connection(), &dir)?;

    let directories = queries::get_all_directories(db.connection())?;
    assert_eq!(directories.len(), 1);
    assert!(directories
        .first()
        .ok_or("No directory")?
        .full_path
        .contains("(2024)"));

    Ok(())
}

#[test]
fn test_last_scanned_timestamp_updates() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let temp = TempDir::new()?;
    let dir_path = temp.path().to_str().ok_or("Invalid path")?;

    let dir = Directory {
        full_path: dir_path.to_string(),
        created_at: Utc::now(),
        last_scanned: None,
    };

    queries::insert_directory(db.connection(), &dir)?;

    // Verify initially None
    let dirs = queries::get_all_directories(db.connection())?;
    assert!(dirs.first().ok_or("No directory")?.last_scanned.is_none());

    // Update last scanned
    queries::update_directory_last_scanned(db.connection(), dir_path)?;

    // Verify updated
    let dirs = queries::get_all_directories(db.connection())?;
    assert!(dirs.first().ok_or("No directory")?.last_scanned.is_some());

    Ok(())
}

#[test]
fn test_nonexistent_directory_can_be_added() -> Result<(), Box<dyn Error>> {
    // Spec: Adding a directory that doesn't exist should show an error.
    let db = create_test_db()?;

    let dir = Directory {
        full_path: "/nonexistent/path".to_string(),
        created_at: Utc::now(),
        last_scanned: None,
    };

    let result = queries::insert_directory(db.connection(), &dir);
    assert!(result.is_err());

    let directories = queries::get_all_directories(db.connection())?;
    assert_eq!(directories.len(), 0);

    Ok(())
}

#[test]
fn test_directory_removal_is_persisted() -> Result<(), Box<dyn Error>> {
    let temp_db = TempDir::new()?;
    let db_path = temp_db.path().join("test.db");
    let temp_dir = TempDir::new()?;
    let dir_path = temp_dir.path().to_str().ok_or("Invalid path")?.to_string();

    // First session: add and remove directory
    {
        let db = nodoka::Database::open_with_path(&db_path)?;
        nodoka::db::initialize(db.connection())?;

        let dir = Directory {
            full_path: dir_path.clone(),
            created_at: Utc::now(),
            last_scanned: None,
        };
        queries::insert_directory(db.connection(), &dir)?;
        queries::delete_directory(db.connection(), &dir_path)?;
    }

    // Second session: verify removal persisted
    {
        let db = nodoka::Database::open_with_path(&db_path)?;
        let directories = queries::get_all_directories(db.connection())?;
        assert_eq!(directories.len(), 0);
    }

    Ok(())
}
