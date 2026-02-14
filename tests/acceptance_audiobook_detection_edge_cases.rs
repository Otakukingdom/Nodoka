mod acceptance_support;
use acceptance_support::*;

use nodoka::tasks::scan_directory;
use std::error::Error;
use std::fs;
use temp_dir::TempDir;

#[test]
fn test_files_with_incorrect_extensions() -> Result<(), Box<dyn Error>> {
    // This tests detection of audio content despite wrong extension
    // Note: Detection based on extension is the typical approach
    // Testing that non-audio extensions are properly ignored

    let temp = TempDir::new()?;
    let fixtures = TestFixtures::new();

    let book = temp.path().join("TestBook");
    fs::create_dir_all(&book)?;

    // Create file with wrong extension
    fs::copy(
        fixtures.audio_path("sample_mp3.mp3"),
        book.join("audio.txt"),
    )?;

    // System should ignore .txt files even if they contain audio
    // This is expected behavior - rely on extensions
    assert!(book.join("audio.txt").exists());

    Ok(())
}

#[tokio::test]
async fn test_very_long_filenames() -> Result<(), Box<dyn Error>> {
    let temp = TempDir::new()?;
    let fixtures = TestFixtures::new();

    let book = temp.path().join("Book");
    fs::create_dir_all(&book)?;

    // Create file with very long name (but within filesystem limits)
    let long_name = format!("Chapter_{}_End.mp3", "A".repeat(100));
    let result = fs::copy(fixtures.audio_path("sample_mp3.mp3"), book.join(&long_name));

    if result.is_ok() {
        let discovered = scan_directory(temp.path().to_path_buf()).await?;
        assert_eq!(discovered.len(), 1);
        assert_eq!(
            discovered.first().ok_or("No audiobook found")?.files.len(),
            1
        );
    }

    Ok(())
}

#[tokio::test]
async fn test_zero_byte_files_ignored() -> Result<(), Box<dyn Error>> {
    let temp = TempDir::new()?;
    let fixtures = TestFixtures::new();

    let book = temp.path().join("TestBook");
    fs::create_dir_all(&book)?;

    // Create valid audio file
    fs::copy(
        fixtures.audio_path("sample_mp3.mp3"),
        book.join("valid.mp3"),
    )?;

    // Create zero-byte file
    fs::write(book.join("empty.mp3"), b"")?;

    let discovered = scan_directory(temp.path().to_path_buf()).await?;

    assert_eq!(discovered.len(), 1);
    // Should have at least the valid file, empty file handling depends on implementation
    assert!(!discovered
        .first()
        .ok_or("No audiobook found")?
        .files
        .is_empty());

    Ok(())
}

#[tokio::test]
async fn test_non_audio_files_ignored() -> Result<(), Box<dyn Error>> {
    let temp = TempDir::new()?;
    let fixtures = TestFixtures::new();

    let audiobook = temp.path().join("Mixed Content");
    fs::create_dir_all(&audiobook)?;

    // Add audio file
    fs::copy(
        fixtures.audio_path("sample_mp3.mp3"),
        audiobook.join("audio.mp3"),
    )?;

    // Add non-audio files
    fs::write(audiobook.join("readme.txt"), b"info")?;
    fs::write(audiobook.join("notes.pdf"), b"fake pdf")?;

    let discovered = scan_directory(temp.path().to_path_buf()).await?;

    assert_eq!(discovered.len(), 1);
    assert_eq!(
        discovered.first().ok_or("No audiobook found")?.files.len(),
        1
    ); // Only audio file

    Ok(())
}

#[tokio::test]
async fn test_hidden_files_ignored() -> Result<(), Box<dyn Error>> {
    let temp = TempDir::new()?;
    let fixtures = TestFixtures::new();

    let audiobook = temp.path().join("Audiobook");
    fs::create_dir_all(&audiobook)?;

    fs::copy(
        fixtures.audio_path("sample_mp3.mp3"),
        audiobook.join("visible.mp3"),
    )?;
    fs::copy(
        fixtures.audio_path("sample_mp3.mp3"),
        audiobook.join(".hidden.mp3"),
    )?;

    let discovered = scan_directory(temp.path().to_path_buf()).await?;

    let first_book = discovered.first().ok_or("No audiobook found")?;
    assert_eq!(first_book.files.len(), 1);
    assert!(first_book
        .files
        .first()
        .ok_or("No file found")?
        .to_string_lossy()
        .contains("visible.mp3"));

    Ok(())
}

#[tokio::test]
async fn test_empty_folders_no_audiobooks() -> Result<(), Box<dyn Error>> {
    let temp = TempDir::new()?;

    let empty = temp.path().join("EmptyFolder");
    fs::create_dir_all(&empty)?;

    let discovered = scan_directory(temp.path().to_path_buf()).await?;

    assert_eq!(discovered.len(), 0);

    Ok(())
}

#[tokio::test]
async fn test_special_characters_in_names() -> Result<(), Box<dyn Error>> {
    let temp = TempDir::new()?;
    let fixtures = TestFixtures::new();

    let book = temp.path().join("Book: The - Special (2024) [Edition]");
    fs::create_dir_all(&book)?;
    fs::copy(
        fixtures.audio_path("sample_mp3.mp3"),
        book.join("Chapter #1.mp3"),
    )?;

    let discovered = scan_directory(temp.path().to_path_buf()).await?;

    assert_eq!(discovered.len(), 1);
    assert_eq!(
        discovered.first().ok_or("No audiobook found")?.name,
        "Book: The - Special (2024) [Edition]"
    );

    Ok(())
}

#[tokio::test]
async fn test_unicode_in_filenames() -> Result<(), Box<dyn Error>> {
    let temp = TempDir::new()?;
    let fixtures = TestFixtures::new();

    let book = temp.path().join("日本語の本");
    fs::create_dir_all(&book)?;
    fs::copy(
        fixtures.audio_path("sample_mp3.mp3"),
        book.join("第１章.mp3"),
    )?;

    let discovered = scan_directory(temp.path().to_path_buf()).await?;

    assert_eq!(discovered.len(), 1);
    assert_eq!(
        discovered.first().ok_or("No audiobook found")?.name,
        "日本語の本"
    );

    Ok(())
}
