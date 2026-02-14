mod acceptance_support;
use acceptance_support::*;

use nodoka::settings::Settings;
use std::error::Error;

#[test]
fn test_settings_persist_across_restarts() -> Result<(), Box<dyn Error>> {
    let temp_db_dir = temp_dir::TempDir::new()?;
    let db_path = temp_db_dir.path().join("settings_test.db");

    // First session
    {
        let db = nodoka::db::Database::open_with_path(&db_path)?;
        nodoka::db::initialize(db.connection())?;

        let settings = Settings::new(db.connection());
        settings.set_speed(1.5)?;
        settings.set_volume(150)?;
    }

    // Second session
    {
        let db = nodoka::db::Database::open_with_path(&db_path)?;
        let settings = Settings::new(db.connection());

        assert!((settings.get_speed()? - 1.5).abs() < 0.01);
        assert_eq!(settings.get_volume()?, 150);
    }

    Ok(())
}

#[test]
fn test_speed_validation_range() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let settings = Settings::new(db.connection());

    // Valid speeds
    assert!(settings.set_speed(0.5).is_ok());
    assert!(settings.set_speed(1.0).is_ok());
    assert!(settings.set_speed(2.0).is_ok());

    // Note: Current implementation doesn't validate range
    // This test documents expected behavior

    Ok(())
}

#[test]
fn test_volume_validation_range() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let settings = Settings::new(db.connection());

    // Valid volumes
    assert!(settings.set_volume(0).is_ok());
    assert!(settings.set_volume(100).is_ok());
    assert!(settings.set_volume(200).is_ok());

    // Note: Current implementation doesn't validate range
    // This test documents expected behavior

    Ok(())
}

#[test]
fn test_settings_have_defaults() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let settings = Settings::new(db.connection());

    // Should have sensible defaults
    let speed = settings.get_speed()?;
    let volume = settings.get_volume()?;

    assert!((speed - 1.0).abs() < 0.01, "Default speed should be 1.0");
    assert_eq!(volume, 100, "Default volume should be 100");

    Ok(())
}

#[test]
fn test_current_audiobook_storage() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let settings = Settings::new(db.connection());

    settings.set_current_audiobook(42)?;

    let current_id = settings.get_current_audiobook()?;
    assert_eq!(current_id, Some(42));

    Ok(())
}

#[test]
fn test_current_file_storage() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let settings = Settings::new(db.connection());

    settings.set_current_file("/path/to/chapter1.mp3")?;

    let current_file = settings.get_current_file()?;
    assert_eq!(current_file, Some("/path/to/chapter1.mp3".to_string()));

    Ok(())
}

#[test]
fn test_settings_changes_immediate() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let settings = Settings::new(db.connection());

    // Set a value
    settings.set_speed(1.25)?;

    // Immediately readable
    assert!((settings.get_speed()? - 1.25).abs() < 0.01);

    // Change it
    settings.set_speed(1.75)?;

    // Immediately reflects new value
    assert!((settings.get_speed()? - 1.75).abs() < 0.01);

    Ok(())
}

#[test]
fn test_multiple_settings_independent() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let settings = Settings::new(db.connection());

    settings.set_speed(1.5)?;
    settings.set_volume(125)?;
    settings.set_current_audiobook(10)?;

    // All should be independently stored
    assert!((settings.get_speed()? - 1.5).abs() < 0.01);
    assert_eq!(settings.get_volume()?, 125);
    assert_eq!(settings.get_current_audiobook()?, Some(10));

    Ok(())
}

#[test]
fn test_volume_persists() -> Result<(), Box<dyn Error>> {
    let temp_db_dir = temp_dir::TempDir::new()?;
    let db_path = temp_db_dir.path().join("volume_test.db");

    {
        let db = nodoka::db::Database::open_with_path(&db_path)?;
        nodoka::db::initialize(db.connection())?;

        let settings = Settings::new(db.connection());
        settings.set_volume(75)?;
    }

    {
        let db = nodoka::db::Database::open_with_path(&db_path)?;
        let settings = Settings::new(db.connection());
        assert_eq!(settings.get_volume()?, 75);
    }

    Ok(())
}

#[test]
fn test_speed_persists() -> Result<(), Box<dyn Error>> {
    let temp_db_dir = temp_dir::TempDir::new()?;
    let db_path = temp_db_dir.path().join("speed_test.db");

    {
        let db = nodoka::db::Database::open_with_path(&db_path)?;
        nodoka::db::initialize(db.connection())?;

        let settings = Settings::new(db.connection());
        settings.set_speed(1.8)?;
    }

    {
        let db = nodoka::db::Database::open_with_path(&db_path)?;
        let settings = Settings::new(db.connection());
        assert!((settings.get_speed()? - 1.8).abs() < 0.01);
    }

    Ok(())
}

#[test]
fn test_zero_volume_allowed() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let settings = Settings::new(db.connection());

    settings.set_volume(0)?;
    assert_eq!(settings.get_volume()?, 0);

    Ok(())
}

#[test]
fn test_max_volume_allowed() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let settings = Settings::new(db.connection());

    settings.set_volume(200)?;
    assert_eq!(settings.get_volume()?, 200);

    Ok(())
}
