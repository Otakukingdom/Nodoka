use chrono::Utc;
use nodoka::db::queries;
use nodoka::db::Database;
use nodoka::models::{Audiobook, AudiobookFile, Directory};
use nodoka::proxy::{AudiobookHandle, Cache};
use std::error::Error;
use std::rc::Rc;

fn create_test_db_with_data() -> Result<(Rc<Database>, i64), Box<dyn Error>> {
    let database = Database::new_in_memory()?;
    nodoka::db::initialize(database.connection())?;

    let dir = Directory {
        full_path: "/test/audiobooks".to_string(),
        created_at: Utc::now(),
        last_scanned: None,
    };
    queries::insert_directory(database.connection(), &dir)?;

    let audiobook = Audiobook::new(
        "/test/audiobooks".to_string(),
        "Integration Test Audiobook".to_string(),
        "/test/audiobooks/Integration Test Audiobook".to_string(),
        0,
    );
    let id = queries::insert_audiobook(database.connection(), &audiobook)?;

    Ok((Rc::new(database), id))
}

#[test]
fn test_proxy_manager_initialization() -> Result<(), Box<dyn Error>> {
    let (db, _) = create_test_db_with_data()?;
    let _manager = Cache::new(db);
    Ok(())
}

#[test]
fn test_proxy_manager_get_audiobook() -> Result<(), Box<dyn Error>> {
    let (db, id) = create_test_db_with_data()?;
    let manager = Cache::new(Rc::clone(&db));

    let proxy = manager.get_audiobook(id)?;
    assert_eq!(proxy.id(), id);

    let data = proxy.get_data();
    assert_eq!(data.name, "Integration Test Audiobook");

    Ok(())
}

#[test]
fn test_proxy_manager_caching_behavior() -> Result<(), Box<dyn Error>> {
    let (db, id) = create_test_db_with_data()?;
    let manager = Cache::new(Rc::clone(&db));

    let proxy1 = manager.get_audiobook(id)?;
    let proxy2 = manager.get_audiobook(id)?;

    assert_eq!(proxy1.id(), proxy2.id());

    Ok(())
}

#[test]
fn test_proxy_manager_cache_clear() -> Result<(), Box<dyn Error>> {
    let (db, id) = create_test_db_with_data()?;
    let manager = Cache::new(Rc::clone(&db));

    let _proxy1 = manager.get_audiobook(id)?;

    manager.clear_cache();

    let proxy2 = manager.get_audiobook(id)?;
    assert_eq!(proxy2.id(), id);

    Ok(())
}

#[test]
fn test_audiobook_proxy_with_files() -> Result<(), Box<dyn Error>> {
    let (db, id) = create_test_db_with_data()?;

    let mut file1 = AudiobookFile::new(
        id,
        "file1.mp3".to_string(),
        "/test/audiobooks/file1.mp3".to_string(),
        0,
    );
    file1.completeness = 50;
    queries::insert_audiobook_file(db.connection(), &file1)?;

    let mut file2 = AudiobookFile::new(
        id,
        "file2.mp3".to_string(),
        "/test/audiobooks/file2.mp3".to_string(),
        1,
    );
    file2.completeness = 100;
    queries::insert_audiobook_file(db.connection(), &file2)?;

    let proxy = AudiobookHandle::new(id, Rc::clone(&db))?;
    let all_files = proxy.get_files()?;

    assert_eq!(all_files.len(), 2);
    let first_file = all_files.first().ok_or("Expected first file")?;
    let second_file = all_files.get(1).ok_or("Expected second file")?;
    assert_eq!(first_file.completeness(), 50);
    assert_eq!(second_file.completeness(), 100);

    Ok(())
}

#[test]
fn test_audiobook_proxy_completeness_calculation() -> Result<(), Box<dyn Error>> {
    let (db, id) = create_test_db_with_data()?;

    let mut file1 = AudiobookFile::new(
        id,
        "file1.mp3".to_string(),
        "/test/audiobooks/file1.mp3".to_string(),
        0,
    );
    file1.completeness = 40;
    queries::insert_audiobook_file(db.connection(), &file1)?;

    let mut file2 = AudiobookFile::new(
        id,
        "file2.mp3".to_string(),
        "/test/audiobooks/file2.mp3".to_string(),
        1,
    );
    file2.completeness = 60;
    queries::insert_audiobook_file(db.connection(), &file2)?;

    let proxy = AudiobookHandle::new(id, Rc::clone(&db))?;
    proxy.update_completeness()?;

    let data = proxy.get_data();
    assert_eq!(data.completeness, 50);

    Ok(())
}

#[test]
fn test_proxy_refresh_after_database_changes() -> Result<(), Box<dyn Error>> {
    let (db, id) = create_test_db_with_data()?;
    let manager = Cache::new(Rc::clone(&db));

    let proxy = manager.get_audiobook(id)?;
    let data_before = proxy.get_data();

    queries::update_audiobook_completeness(db.connection(), id, 75)?;

    manager.clear_cache();

    let proxy_refreshed = manager.get_audiobook(id)?;
    let data_after = proxy_refreshed.get_data();

    assert_ne!(data_before.completeness, data_after.completeness);
    assert_eq!(data_after.completeness, 75);

    Ok(())
}
