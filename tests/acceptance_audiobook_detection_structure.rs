mod acceptance_support;
use acceptance_support::*;

use nodoka::tasks::scan_directory;
use std::error::Error;
use std::fs;
use temp_dir::TempDir;

#[tokio::test]
async fn test_multi_disc_audiobooks_structure() -> Result<(), Box<dyn Error>> {
    let temp = TempDir::new()?;
    let fixtures = TestFixtures::new();

    // Create multi-disc structure
    let book_root = temp.path().join("The Great Audiobook");
    fs::create_dir_all(&book_root)?;

    let disc1 = book_root.join("Disc 01");
    let disc2 = book_root.join("Disc 02");
    fs::create_dir_all(&disc1)?;
    fs::create_dir_all(&disc2)?;

    fs::copy(
        fixtures.audio_path("sample_mp3.mp3"),
        disc1.join("track01.mp3"),
    )?;
    fs::copy(
        fixtures.audio_path("sample_mp3.mp3"),
        disc1.join("track02.mp3"),
    )?;
    fs::copy(
        fixtures.audio_path("sample_mp3.mp3"),
        disc2.join("track01.mp3"),
    )?;

    let discovered = scan_directory(temp.path().to_path_buf()).await?;

    // Multi-disc structure: each disc is detected as separate audiobook
    // or combined as one, depending on implementation
    assert!(!discovered.is_empty());

    Ok(())
}

#[tokio::test]
async fn test_multi_disc_audiobooks() -> Result<(), Box<dyn Error>> {
    let temp = TempDir::new()?;
    let fixtures = TestFixtures::new();

    // Create multi-disc structure: Book/Disc 1/, Book/Disc 2/
    let disc1 = temp.path().join("Audiobook").join("Disc 1");
    let disc2 = temp.path().join("Audiobook").join("Disc 2");
    fs::create_dir_all(&disc1)?;
    fs::create_dir_all(&disc2)?;

    fs::copy(
        fixtures.audio_path("sample_mp3.mp3"),
        disc1.join("Track 01.mp3"),
    )?;
    fs::copy(
        fixtures.audio_path("sample_mp3.mp3"),
        disc2.join("Track 01.mp3"),
    )?;

    let discovered = scan_directory(temp.path().to_path_buf()).await?;

    // Should discover 2 audiobooks (one per disc folder)
    assert!(
        discovered.len() >= 2,
        "Should discover multiple disc folders as separate audiobooks"
    );

    Ok(())
}

#[tokio::test]
async fn test_single_file_audiobook() -> Result<(), Box<dyn Error>> {
    let temp = TempDir::new()?;
    let fixtures = TestFixtures::new();

    let book = temp.path().join("Single File Book");
    fs::create_dir_all(&book)?;
    fs::copy(
        fixtures.audio_path("sample_mp3.mp3"),
        book.join("audiobook.mp3"),
    )?;

    let discovered = scan_directory(temp.path().to_path_buf()).await?;

    assert_eq!(discovered.len(), 1);
    let first_book = discovered.first().ok_or("No audiobook found")?;
    assert_eq!(first_book.files.len(), 1);
    assert_eq!(first_book.name, "Single File Book");

    Ok(())
}

#[tokio::test]
async fn test_nested_subdirectories() -> Result<(), Box<dyn Error>> {
    let temp = TempDir::new()?;
    let fixtures = TestFixtures::new();

    // Create deep nesting: library/series/book/disc/files
    let disc1 = temp.path().join("Series").join("Book1").join("Disc1");
    let disc2 = temp.path().join("Series").join("Book1").join("Disc2");
    fs::create_dir_all(&disc1)?;
    fs::create_dir_all(&disc2)?;

    fs::copy(
        fixtures.audio_path("sample_mp3.mp3"),
        disc1.join("track1.mp3"),
    )?;
    fs::copy(
        fixtures.audio_path("sample_mp3.mp3"),
        disc2.join("track1.mp3"),
    )?;

    let discovered = scan_directory(temp.path().to_path_buf()).await?;

    // Should find 2 audiobooks (one per disc)
    assert!(discovered.len() >= 2);

    Ok(())
}

#[tokio::test]
async fn test_mixed_content_folders() -> Result<(), Box<dyn Error>> {
    let temp = TempDir::new()?;
    let fixtures = TestFixtures::new();

    let book = temp.path().join("Book with Images");
    fs::create_dir_all(&book)?;

    // Audio files
    fs::copy(
        fixtures.audio_path("sample_mp3.mp3"),
        book.join("part1.mp3"),
    )?;
    fs::copy(
        fixtures.audio_path("sample_mp3.mp3"),
        book.join("part2.mp3"),
    )?;

    // Image files (should be ignored in file count)
    fs::write(book.join("cover.jpg"), b"fake image")?;
    fs::write(book.join("author.png"), b"fake image")?;

    // Text files
    fs::write(book.join("README.txt"), b"description")?;

    let discovered = scan_directory(temp.path().to_path_buf()).await?;

    assert_eq!(discovered.len(), 1);
    assert_eq!(
        discovered.first().ok_or("No audiobook found")?.files.len(),
        2
    ); // Only audio files counted

    Ok(())
}
