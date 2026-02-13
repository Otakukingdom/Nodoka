use nodoka::db::Database;
use nodoka::settings::Settings;
use std::error::Error;

fn create_test_db() -> Result<Database, Box<dyn Error>> {
    let db = Database::new_in_memory()?;
    nodoka::db::initialize(db.connection())?;
    Ok(db)
}

#[test]
fn test_settings_volume_persistence() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let settings = Settings::new(db.connection());

    settings.set_volume(75)?;
    assert_eq!(settings.get_volume()?, 75);

    Ok(())
}

#[test]
fn test_settings_speed_persistence() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let settings = Settings::new(db.connection());

    settings.set_speed(1.5)?;
    assert!((settings.get_speed()? - 1.5).abs() < f32::EPSILON);

    Ok(())
}

#[test]
fn test_settings_persistence_across_instances() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;

    {
        let settings = Settings::new(db.connection());
        settings.set_volume(80)?;
        settings.set_speed(1.25)?;
    }

    {
        let settings = Settings::new(db.connection());
        assert_eq!(settings.get_volume()?, 80);
        assert!((settings.get_speed()? - 1.25).abs() < f32::EPSILON);
    }

    Ok(())
}

#[test]
fn test_settings_current_audiobook() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let settings = Settings::new(db.connection());

    assert_eq!(settings.get_current_audiobook()?, None);

    settings.set_current_audiobook(42)?;
    assert_eq!(settings.get_current_audiobook()?, Some(42));

    Ok(())
}

#[test]
fn test_settings_current_file() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let settings = Settings::new(db.connection());

    assert_eq!(settings.get_current_file()?, None);

    settings.set_current_file("/path/to/file.mp3")?;
    assert_eq!(
        settings.get_current_file()?,
        Some("/path/to/file.mp3".to_string())
    );

    Ok(())
}

#[test]
fn test_settings_defaults() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let settings = Settings::new(db.connection());

    assert_eq!(settings.get_volume()?, 100);
    assert!((settings.get_speed()? - 1.0).abs() < f32::EPSILON);

    Ok(())
}
