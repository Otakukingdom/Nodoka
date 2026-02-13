use chrono::Utc;
use nodoka::db::queries;
use nodoka::db::Database;
use nodoka::models::{Audiobook, AudiobookFile, Directory};
use nodoka::settings::Settings;
use std::error::Error;
use std::fs;
use temp_dir::TempDir;

fn create_test_db() -> Result<Database, Box<dyn Error>> {
    let db = Database::new_in_memory()?;
    nodoka::db::initialize(db.connection())?;
    Ok(db)
}

#[test]
fn test_add_directory_workflow() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let temp = TempDir::new()?;
    let dir_path = temp.path().to_str().ok_or("Invalid path")?;

    let dir = Directory {
        full_path: dir_path.to_string(),
        created_at: Utc::now(),
        last_scanned: None,
    };

    queries::insert_directory(db.connection(), &dir)?;

    let audiobook = Audiobook::new(
        dir_path.to_string(),
        "Test Audiobook".to_string(),
        format!("{dir_path}/Test Audiobook"),
        0,
    );
    let audiobook_id = queries::insert_audiobook(db.connection(), &audiobook)?;

    let audiobooks = queries::get_all_audiobooks(db.connection())?;
    assert_eq!(audiobooks.len(), 1);
    let first_audiobook = audiobooks.first().ok_or("Expected audiobook")?;
    assert_eq!(first_audiobook.name, "Test Audiobook");

    let mut file = AudiobookFile::new(
        audiobook_id,
        "chapter1.mp3".to_string(),
        format!("{dir_path}/Test Audiobook/chapter1.mp3"),
        0,
    );
    file.length_of_file = Some(3600);
    queries::insert_audiobook_file(db.connection(), &file)?;

    let files = queries::get_audiobook_files(db.connection(), audiobook_id)?;
    assert_eq!(files.len(), 1);

    Ok(())
}

#[test]
fn test_playback_progress_workflow() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let temp = TempDir::new()?;
    let dir_path = temp.path().to_str().ok_or("Invalid path")?;

    let dir = Directory {
        full_path: dir_path.to_string(),
        created_at: Utc::now(),
        last_scanned: None,
    };
    queries::insert_directory(db.connection(), &dir)?;

    let audiobook = Audiobook::new(
        dir_path.to_string(),
        "Progress Test".to_string(),
        format!("{dir_path}/Progress Test"),
        0,
    );
    let audiobook_id = queries::insert_audiobook(db.connection(), &audiobook)?;

    let mut file = AudiobookFile::new(
        audiobook_id,
        "chapter1.mp3".to_string(),
        format!("{dir_path}/Progress Test/chapter1.mp3"),
        0,
    );
    file.length_of_file = Some(3600);
    file.seek_position = Some(0);
    queries::insert_audiobook_file(db.connection(), &file)?;

    queries::update_file_progress(
        db.connection(),
        &format!("{dir_path}/Progress Test/chapter1.mp3"),
        1800.0,
        50,
    )?;

    let all_files = queries::get_audiobook_files(db.connection(), audiobook_id)?;
    let first_file = all_files.first().ok_or("Expected file")?;
    assert_eq!(first_file.seek_position, Some(1800));

    let settings = Settings::new(db.connection());
    settings.set_current_audiobook(audiobook_id)?;

    let current = settings.get_current_audiobook()?;
    assert_eq!(current, Some(audiobook_id));

    Ok(())
}

#[test]
fn test_directory_removal_workflow() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let temp = TempDir::new()?;
    let dir_path = temp.path().to_str().ok_or("Invalid path")?;

    let dir = Directory {
        full_path: dir_path.to_string(),
        created_at: Utc::now(),
        last_scanned: None,
    };
    queries::insert_directory(db.connection(), &dir)?;

    let audiobook = Audiobook::new(
        dir_path.to_string(),
        "Removal Test".to_string(),
        format!("{dir_path}/Removal Test"),
        0,
    );
    let audiobook_id = queries::insert_audiobook(db.connection(), &audiobook)?;

    let audiobooks_before = queries::get_all_audiobooks(db.connection())?;
    assert_eq!(audiobooks_before.len(), 1);

    queries::delete_directory(db.connection(), dir_path)?;

    let dirs_after = queries::get_all_directories(db.connection())?;
    assert_eq!(dirs_after.len(), 0);

    let audiobooks_after = queries::get_all_audiobooks(db.connection())?;
    assert_eq!(
        audiobooks_after
            .iter()
            .filter(|a| a.id == Some(audiobook_id))
            .count(),
        0
    );

    Ok(())
}

#[test]
fn test_settings_persistence_workflow() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;

    {
        let settings = Settings::new(db.connection());
        settings.set_volume(85)?;
        settings.set_speed(1.5)?;
    }

    {
        let settings = Settings::new(db.connection());
        assert_eq!(settings.get_volume()?, 85);
        assert!((settings.get_speed()? - 1.5).abs() < f32::EPSILON);
    }

    Ok(())
}

#[test]
fn test_multi_file_audiobook_workflow() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let temp = TempDir::new()?;
    let dir_path = temp.path().to_str().ok_or("Invalid path")?;

    let audiobook_dir = temp.path().join("Multi File Audiobook");
    fs::create_dir(&audiobook_dir)?;

    let dir = Directory {
        full_path: dir_path.to_string(),
        created_at: Utc::now(),
        last_scanned: None,
    };
    queries::insert_directory(db.connection(), &dir)?;

    let audiobook = Audiobook::new(
        dir_path.to_string(),
        "Multi File Audiobook".to_string(),
        audiobook_dir.to_str().ok_or("Invalid path")?.to_string(),
        0,
    );
    let audiobook_id = queries::insert_audiobook(db.connection(), &audiobook)?;

    for i in 1..=3 {
        let mut file = AudiobookFile::new(
            audiobook_id,
            format!("chapter{i}.mp3"),
            audiobook_dir
                .join(format!("chapter{i}.mp3"))
                .to_str()
                .ok_or("Invalid path")?
                .to_string(),
            i - 1,
        );
        file.length_of_file = Some(3600);
        queries::insert_audiobook_file(db.connection(), &file)?;
    }

    let files = queries::get_audiobook_files(db.connection(), audiobook_id)?;
    assert_eq!(files.len(), 3);

    Ok(())
}
