use nodoka::db::queries;
use nodoka::db::Database;
use nodoka::models::Directory;
use std::error::Error;
use std::fs;
use temp_dir::TempDir;

fn create_test_db() -> Result<Database, Box<dyn Error>> {
    let db = Database::new_in_memory()?;
    nodoka::db::initialize(db.connection())?;
    Ok(db)
}

#[test]
fn test_directory_insertion_and_retrieval() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let temp = TempDir::new()?;
    let dir_path = temp.path().to_str().ok_or("Invalid path")?;

    let dir = Directory {
        full_path: dir_path.to_string(),
        created_at: chrono::Utc::now(),
        last_scanned: None,
    };

    queries::insert_directory(db.connection(), &dir)?;

    let directories = queries::get_all_directories(db.connection())?;
    assert_eq!(directories.len(), 1);
    let first_dir = directories.first().ok_or("Expected directory")?;
    assert_eq!(first_dir.full_path, dir_path);

    Ok(())
}

#[test]
fn test_nested_directory_handling() -> Result<(), Box<dyn Error>> {
    let temp = TempDir::new()?;
    let nested_path = temp.path().join("audiobooks").join("author");
    fs::create_dir_all(&nested_path)?;

    assert!(nested_path.exists());
    assert!(nested_path.is_dir());

    Ok(())
}

#[test]
fn test_directory_update_last_scanned() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let temp = TempDir::new()?;
    let dir_path = temp.path().to_str().ok_or("Invalid path")?;

    let dir = Directory {
        full_path: dir_path.to_string(),
        created_at: chrono::Utc::now(),
        last_scanned: None,
    };

    queries::insert_directory(db.connection(), &dir)?;

    let dirs_before = queries::get_all_directories(db.connection())?;
    let first_dir_before = dirs_before.first().ok_or("Expected directory")?;
    assert!(first_dir_before.last_scanned.is_none());

    queries::update_directory_last_scanned(db.connection(), dir_path)?;

    let dirs_after = queries::get_all_directories(db.connection())?;
    let first_dir_after = dirs_after.first().ok_or("Expected directory")?;
    assert!(first_dir_after.last_scanned.is_some());

    Ok(())
}

#[test]
fn test_empty_directory_handling() -> Result<(), Box<dyn Error>> {
    let temp = TempDir::new()?;
    let empty_dir = temp.path().join("empty");
    fs::create_dir(&empty_dir)?;

    assert!(empty_dir.exists());
    assert!(empty_dir.is_dir());

    assert!(fs::read_dir(&empty_dir)?.next().is_none());

    Ok(())
}

#[test]
fn test_directory_with_files() -> Result<(), Box<dyn Error>> {
    let temp = TempDir::new()?;
    let file_path = temp.path().join("test.txt");
    fs::write(&file_path, b"test content")?;

    assert!(file_path.exists());
    assert!(file_path.is_file());

    Ok(())
}

#[test]
fn test_multiple_directory_scanning() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let temp1 = TempDir::new()?;
    let temp2 = TempDir::new()?;

    let dir1_path = temp1.path().to_str().ok_or("Invalid path")?;
    let dir2_path = temp2.path().to_str().ok_or("Invalid path")?;

    let dir1 = Directory {
        full_path: dir1_path.to_string(),
        created_at: chrono::Utc::now(),
        last_scanned: None,
    };

    let dir2 = Directory {
        full_path: dir2_path.to_string(),
        created_at: chrono::Utc::now(),
        last_scanned: None,
    };

    queries::insert_directory(db.connection(), &dir1)?;
    queries::insert_directory(db.connection(), &dir2)?;

    let directories = queries::get_all_directories(db.connection())?;
    assert_eq!(directories.len(), 2);

    Ok(())
}

#[test]
fn test_directory_deletion() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let temp = TempDir::new()?;
    let dir_path = temp.path().to_str().ok_or("Invalid path")?;

    let dir = Directory {
        full_path: dir_path.to_string(),
        created_at: chrono::Utc::now(),
        last_scanned: None,
    };

    queries::insert_directory(db.connection(), &dir)?;

    let dirs_before = queries::get_all_directories(db.connection())?;
    assert_eq!(dirs_before.len(), 1);

    queries::delete_directory(db.connection(), dir_path)?;

    let dirs_after = queries::get_all_directories(db.connection())?;
    assert_eq!(dirs_after.len(), 0);

    Ok(())
}
