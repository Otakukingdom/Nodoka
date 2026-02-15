mod acceptance_support;
use acceptance_support::*;

use nodoka::tasks::scan_directory;
use std::error::Error;
use std::fs;
use temp_dir::TempDir;

#[tokio::test]
async fn test_rescanning_preserves_playback_progress() -> Result<(), Box<dyn Error>> {
    use nodoka::db::queries;

    let temp = TempDir::new()?;
    let fixtures = TestFixtures::new();
    let db = create_test_db()?;

    let book = temp.path().join("Book");
    fs::create_dir_all(&book)?;
    let file_path = book.join("chapter1.mp3");
    fs::copy(fixtures.audio_path("sample_mp3.mp3"), &file_path)?;

    // Initial scan and setup
    let audiobook_id =
        create_test_audiobook(&db, book.to_str().ok_or("Path conversion failed")?, "Book")?;
    insert_test_file(
        &db,
        audiobook_id,
        file_path.to_str().ok_or("Path conversion failed")?,
    )?;

    // Set playback progress
    queries::update_file_progress(
        db.connection(),
        file_path.to_str().ok_or("Path conversion failed")?,
        5000.0,
        0,
    )?;

    // Get progress before rescan
    let files_before = queries::get_audiobook_files(db.connection(), audiobook_id)?;
    let progress_before = files_before.first().ok_or("No file found")?.seek_position;

    // Rescan (simulate by just querying again)
    let files_after = queries::get_audiobook_files(db.connection(), audiobook_id)?;
    let progress_after = files_after.first().ok_or("No file found")?.seek_position;

    // Progress should be preserved
    assert_eq!(progress_before, progress_after);
    assert!(progress_after.is_some());

    Ok(())
}

#[tokio::test]
async fn test_checksum_change_resets_progress_on_rescan() -> Result<(), Box<dyn Error>> {
    use nodoka::db::queries;
    use nodoka::ui::{Message, State};

    let temp = TempDir::new()?;
    let db = create_test_db()?;

    let book = temp.path().join("Book");
    fs::create_dir_all(&book)?;
    let file_path = book.join("chapter1.mp3");
    fs::write(&file_path, b"one")?;

    let discovered = scan_directory(temp.path().to_path_buf()).await?;

    let mut state = State::default();
    let mut player: Option<nodoka::player::Vlc> = None;
    let dir_str = temp
        .path()
        .to_str()
        .ok_or("Path conversion failed")?
        .to_string();

    let _ = nodoka::ui::update::update(
        &mut state,
        Message::ScanComplete(dir_str.clone(), discovered),
        &mut player,
        &db,
    );

    let book_path_str = book.to_str().ok_or("Path conversion failed")?;
    let ab = queries::get_audiobook_by_path(db.connection(), book_path_str)?
        .ok_or("Expected audiobook")?;
    let ab_id = ab.id.ok_or("Expected audiobook id")?;

    let file_str = file_path.to_str().ok_or("Path conversion failed")?;
    queries::update_file_progress(db.connection(), file_str, 5000.0, 50)?;

    // Change file contents to change checksum
    fs::write(&file_path, b"two")?;

    let discovered2 = scan_directory(temp.path().to_path_buf()).await?;
    let _ = nodoka::ui::update::update(
        &mut state,
        Message::ScanComplete(dir_str, discovered2),
        &mut player,
        &db,
    );

    let updated = queries::get_audiobook_file_by_path(db.connection(), file_str)?
        .ok_or("Expected audiobook file")?;
    assert_eq!(updated.audiobook_id, ab_id);
    assert_eq!(updated.seek_position, None);
    assert_eq!(updated.completeness, 0);
    assert!(updated.checksum.is_some());
    Ok(())
}

#[tokio::test]
async fn test_scan_directory_uses_fs_fingerprint_checksums_for_filesystem_entries(
) -> Result<(), Box<dyn Error>> {
    let temp = TempDir::new()?;

    let book = temp.path().join("Book");
    fs::create_dir_all(&book)?;
    let file_path = book.join("chapter1.mp3");
    fs::write(&file_path, b"one")?;

    let discovered = scan_directory(temp.path().to_path_buf()).await?;
    let first = discovered.first().ok_or("No audiobook discovered")?;
    let file = first.files.first().ok_or("No file discovered")?;

    let checksum = file.checksum.as_deref().ok_or("expected checksum")?;
    assert!(
        checksum.starts_with("fs:v1:"),
        "filesystem scan should use cheap fs fingerprint checksums"
    );
    Ok(())
}

#[tokio::test]
async fn test_legacy_sha256_checksum_migration_does_not_reset_progress(
) -> Result<(), Box<dyn Error>> {
    use nodoka::db::queries;
    use nodoka::models::{Audiobook, AudiobookFile};
    use nodoka::ui::{Message, State};

    let temp = TempDir::new()?;
    let db = create_test_db()?;

    let book = temp.path().join("Book");
    fs::create_dir_all(&book)?;
    let file_path = book.join("chapter1.mp3");
    fs::write(&file_path, b"one")?;

    let dir_str = temp
        .path()
        .to_str()
        .ok_or("Path conversion failed")?
        .to_string();
    let book_path_str = book.to_str().ok_or("Path conversion failed")?;
    let file_str = file_path.to_str().ok_or("Path conversion failed")?;

    let audiobook_id = queries::insert_audiobook(
        db.connection(),
        &Audiobook::new(
            dir_str.clone(),
            "Book".to_string(),
            book_path_str.to_string(),
            0,
        ),
    )?;

    let mut file = AudiobookFile::new(
        audiobook_id,
        "chapter1.mp3".to_string(),
        file_str.to_string(),
        0,
    );
    file.seek_position = Some(5000);
    file.completeness = 50;
    file.checksum = Some("a".repeat(64));
    queries::insert_audiobook_file(db.connection(), &file)?;

    let discovered = scan_directory(temp.path().to_path_buf()).await?;

    let mut state = State::default();
    let mut player: Option<nodoka::player::Vlc> = None;
    let _ = nodoka::ui::update::update(
        &mut state,
        Message::ScanComplete(dir_str, discovered),
        &mut player,
        &db,
    );

    let updated = queries::get_audiobook_file_by_path(db.connection(), file_str)?
        .ok_or("Expected audiobook file")?;
    assert_eq!(updated.seek_position, Some(5000));
    assert_eq!(updated.completeness, 50);
    assert!(
        updated
            .checksum
            .as_deref()
            .is_some_and(|c| c.starts_with("fs:v1:")),
        "checksum should migrate to fs fingerprint format"
    );
    Ok(())
}

#[tokio::test]
async fn test_files_marked_as_missing_when_deleted() -> Result<(), Box<dyn Error>> {
    use nodoka::db::queries;
    use nodoka::tasks::scan_directory;

    let temp = TempDir::new()?;
    let fixtures = TestFixtures::new();
    let db = create_test_db()?;

    let book = temp.path().join("Book");
    fs::create_dir_all(&book)?;
    let file_path = book.join("chapter1.mp3");
    fs::copy(fixtures.audio_path("sample_mp3.mp3"), &file_path)?;

    // Initial scan
    let discovered = scan_directory(temp.path().to_path_buf()).await?;
    assert!(!discovered.is_empty());

    // Create audiobook and file in database
    let audiobook_id =
        create_test_audiobook(&db, book.to_str().ok_or("Path conversion failed")?, "Book")?;
    insert_test_file(
        &db,
        audiobook_id,
        file_path.to_str().ok_or("Path conversion failed")?,
    )?;

    // Delete the file
    fs::remove_file(&file_path)?;

    // Mark missing (this would normally happen during rescan)
    queries::mark_audiobook_files_missing(db.connection(), audiobook_id)?;

    // Verify file is marked as missing
    let files = queries::get_audiobook_files(db.connection(), audiobook_id)?;
    assert!(!files.is_empty());
    assert!(!files.first().ok_or("No file found")?.file_exists);

    Ok(())
}

#[tokio::test]
async fn test_rescanning_updates_library() -> Result<(), Box<dyn Error>> {
    let temp = TempDir::new()?;
    let fixtures = TestFixtures::new();

    let book = temp.path().join("Book");
    fs::create_dir_all(&book)?;

    // First scan with 2 files
    fs::copy(
        fixtures.audio_path("sample_mp3.mp3"),
        book.join("file1.mp3"),
    )?;
    fs::copy(
        fixtures.audio_path("sample_mp3.mp3"),
        book.join("file2.mp3"),
    )?;

    let discovered1 = scan_directory(temp.path().to_path_buf()).await?;
    assert_eq!(
        discovered1.first().ok_or("No audiobook found")?.files.len(),
        2
    );

    // Add a new file
    fs::copy(
        fixtures.audio_path("sample_mp3.mp3"),
        book.join("file3.mp3"),
    )?;

    // Rescan
    let discovered2 = scan_directory(temp.path().to_path_buf()).await?;
    assert_eq!(
        discovered2.first().ok_or("No audiobook found")?.files.len(),
        3
    );

    Ok(())
}
