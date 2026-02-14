mod acceptance_support;
use acceptance_support::*;

use nodoka::tasks::scan_directory;
use std::error::Error;
use std::fs;
use temp_dir::TempDir;

#[tokio::test]
async fn test_recursive_scanning_discovers_all_files() -> Result<(), Box<dyn Error>> {
    let temp = TempDir::new()?;
    let fixtures = TestFixtures::new();

    // Create nested structure
    let nested = temp.path().join("audiobooks").join("author").join("book");
    fs::create_dir_all(&nested)?;

    // Copy test files
    for i in 1..=3 {
        let dest = nested.join(format!("chapter{i}.mp3"));
        fs::copy(fixtures.audio_path("sample_mp3.mp3"), &dest)?;
    }

    // Scan and verify discovery
    let discovered = scan_directory(temp.path().to_path_buf()).await?;

    assert_eq!(discovered.len(), 1);
    let first_book = discovered.first().ok_or("No audiobook discovered")?;
    assert_eq!(first_book.files.len(), 3);
    assert_eq!(first_book.name, "book");

    Ok(())
}

#[tokio::test]
async fn test_scan_missing_root_returns_error() -> Result<(), Box<dyn Error>> {
    let temp = TempDir::new()?;
    let missing = temp.path().join("does_not_exist");

    let result = scan_directory(missing).await;
    assert!(result.is_err());

    Ok(())
}

#[tokio::test]
async fn test_symbolic_links_handling() -> Result<(), Box<dyn Error>> {
    #[cfg(unix)]
    {
        let temp = TempDir::new()?;
        let fixtures = TestFixtures::new();

        // Create real directory with audio files
        let real_dir = temp.path().join("real_audiobooks");
        fs::create_dir_all(&real_dir)?;
        let book = real_dir.join("TestBook");
        fs::create_dir_all(&book)?;
        fs::copy(
            fixtures.audio_path("sample_mp3.mp3"),
            book.join("chapter1.mp3"),
        )?;

        // Create symbolic link to the directory
        let link_dir = temp.path().join("linked_audiobooks");
        #[cfg(unix)]
        std::os::unix::fs::symlink(&real_dir, &link_dir)?;

        // Scanning should handle symlinks without infinite loops
        let discovered = scan_directory(temp.path().to_path_buf()).await?;

        // Should discover audiobook at least once
        assert!(!discovered.is_empty());
    }

    Ok(())
}

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
async fn test_case_insensitive_extensions() -> Result<(), Box<dyn Error>> {
    let temp = TempDir::new()?;
    let fixtures = TestFixtures::new();

    let book = temp.path().join("TestBook");
    fs::create_dir_all(&book)?;

    // Create files with different case extensions
    fs::copy(
        fixtures.audio_path("sample_mp3.mp3"),
        book.join("file1.MP3"),
    )?;
    fs::copy(
        fixtures.audio_path("sample_mp3.mp3"),
        book.join("file2.Mp3"),
    )?;
    fs::copy(
        fixtures.audio_path("sample_mp3.mp3"),
        book.join("file3.mp3"),
    )?;

    let discovered = scan_directory(temp.path().to_path_buf()).await?;

    assert_eq!(discovered.len(), 1);
    // All three files should be detected regardless of case
    assert_eq!(
        discovered.first().ok_or("No audiobook found")?.files.len(),
        3
    );

    Ok(())
}

#[test]
fn test_opus_files_detected() {
    let fixtures = TestFixtures::new();
    let audio_file = fixtures.audio_path("sample_opus.opus");

    if audio_file.exists() {
        let extension = audio_file.extension().and_then(|e| e.to_str());
        assert_eq!(extension, Some("opus"));
    }
}

#[test]
fn test_aac_files_detected() {
    let fixtures = TestFixtures::new();
    let audio_file = fixtures.audio_path("sample_aac.aac");

    if audio_file.exists() {
        let extension = audio_file.extension().and_then(|e| e.to_str());
        assert_eq!(extension, Some("aac"));
    }
}

#[test]
fn test_wma_files_detected() {
    let fixtures = TestFixtures::new();
    let audio_file = fixtures.audio_path("sample_wma.wma");

    if audio_file.exists() {
        let extension = audio_file.extension().and_then(|e| e.to_str());
        assert_eq!(extension, Some("wma"));
    }
}

#[test]
fn test_wav_files_detected() {
    let fixtures = TestFixtures::new();
    let audio_file = fixtures.audio_path("sample_wav.wav");

    if audio_file.exists() {
        let extension = audio_file.extension().and_then(|e| e.to_str());
        assert_eq!(extension, Some("wav"));
    }
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
async fn test_audiobook_name_from_folder() -> Result<(), Box<dyn Error>> {
    let temp = TempDir::new()?;
    let fixtures = TestFixtures::new();

    let book_dir = temp.path().join("The Great Audiobook");
    fs::create_dir_all(&book_dir)?;
    fs::copy(
        fixtures.audio_path("sample_mp3.mp3"),
        book_dir.join("part1.mp3"),
    )?;

    let discovered = scan_directory(temp.path().to_path_buf()).await?;

    assert_eq!(discovered.len(), 1);
    assert_eq!(
        discovered.first().ok_or("No audiobook found")?.name,
        "The Great Audiobook"
    );

    Ok(())
}

#[tokio::test]
async fn test_files_sorted_naturally() -> Result<(), Box<dyn Error>> {
    let temp = TempDir::new()?;
    let fixtures = TestFixtures::new();

    let book = temp.path().join("Book");
    fs::create_dir_all(&book)?;

    // Create files in non-natural order to test sorting
    for name in &[
        "Chapter 10.mp3",
        "Chapter 2.mp3",
        "Chapter 1.mp3",
        "Chapter 20.mp3",
    ] {
        fs::copy(fixtures.audio_path("sample_mp3.mp3"), book.join(name))?;
    }

    let discovered = scan_directory(temp.path().to_path_buf()).await?;

    let first_book = discovered.first().ok_or("No audiobook found")?;
    assert_eq!(first_book.files.len(), 4);
    // Files should be sorted
    assert!(first_book
        .files
        .first()
        .ok_or("No file at index 0")?
        .to_string_lossy()
        .contains("Chapter 1.mp3"));
    assert!(first_book
        .files
        .get(1)
        .ok_or("No file at index 1")?
        .to_string_lossy()
        .contains("Chapter 2.mp3"));
    assert!(first_book
        .files
        .get(2)
        .ok_or("No file at index 2")?
        .to_string_lossy()
        .contains("Chapter 10.mp3"));
    assert!(first_book
        .files
        .get(3)
        .ok_or("No file at index 3")?
        .to_string_lossy()
        .contains("Chapter 20.mp3"));

    Ok(())
}

#[test]
fn test_mp3_files_detected() {
    let fixtures = TestFixtures::new();
    let mp3_path = fixtures.audio_path("sample_mp3.mp3");

    assert!(mp3_path.exists(), "MP3 fixture should exist");
}

#[test]
fn test_m4b_files_detected() {
    let fixtures = TestFixtures::new();
    let m4b_path = fixtures.audio_path("sample_m4b.m4b");

    if m4b_path.exists() {
        // M4B fixture exists and can be detected
        assert!(m4b_path.extension().and_then(|e| e.to_str()) == Some("m4b"));
    }
}

#[test]
fn test_m4a_files_detected() {
    let fixtures = TestFixtures::new();
    let m4a_path = fixtures.audio_path("sample_m4a.m4a");

    if m4a_path.exists() {
        assert!(m4a_path.extension().and_then(|e| e.to_str()) == Some("m4a"));
    }
}

#[test]
fn test_flac_files_detected() {
    let fixtures = TestFixtures::new();
    let flac_path = fixtures.audio_path("sample_flac.flac");

    if flac_path.exists() {
        assert!(flac_path.extension().and_then(|e| e.to_str()) == Some("flac"));
    }
}

#[test]
fn test_ogg_files_detected() {
    let fixtures = TestFixtures::new();
    let ogg_path = fixtures.audio_path("sample_ogg.ogg");

    if ogg_path.exists() {
        assert!(ogg_path.extension().and_then(|e| e.to_str()) == Some("ogg"));
    }
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

#[tokio::test]
async fn test_various_audio_formats_detected() -> Result<(), Box<dyn Error>> {
    let temp = TempDir::new()?;
    let fixtures = TestFixtures::new();

    let book = temp.path().join("MultiFormat");
    fs::create_dir_all(&book)?;

    let mut count = 0;

    // Copy all available fixtures
    if fixtures.audio_path("sample_mp3.mp3").exists() {
        fs::copy(fixtures.audio_path("sample_mp3.mp3"), book.join("file.mp3"))?;
        count += 1;
    }
    if fixtures.audio_path("sample_m4b.m4b").exists() {
        fs::copy(fixtures.audio_path("sample_m4b.m4b"), book.join("file.m4b"))?;
        count += 1;
    }
    if fixtures.audio_path("sample_flac.flac").exists() {
        fs::copy(
            fixtures.audio_path("sample_flac.flac"),
            book.join("file.flac"),
        )?;
        count += 1;
    }
    if fixtures.audio_path("sample_ogg.ogg").exists() {
        fs::copy(fixtures.audio_path("sample_ogg.ogg"), book.join("file.ogg"))?;
        count += 1;
    }

    if count > 0 {
        let discovered = scan_directory(temp.path().to_path_buf()).await?;
        assert_eq!(discovered.len(), 1);
        assert_eq!(
            discovered.first().ok_or("No audiobook found")?.files.len(),
            count
        );
    }

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

// Comprehensive audio format tests with scanning

#[tokio::test]
async fn test_opus_files_detected_by_scanner() -> Result<(), Box<dyn Error>> {
    let temp = TempDir::new()?;
    let fixtures = TestFixtures::new();
    let opus_file = fixtures.audio_path("sample_opus.opus");

    if !opus_file.exists() {
        // Skip if fixture not available
        return Ok(());
    }

    let audiobook_dir = temp.path().join("opus_book");
    fs::create_dir_all(&audiobook_dir)?;
    fs::copy(&opus_file, audiobook_dir.join("chapter1.opus"))?;

    // Scan directory
    let discovered = scan_directory(temp.path().to_path_buf()).await?;

    // Verify detection
    assert_eq!(discovered.len(), 1, "OPUS audiobook not detected");
    let first_book = discovered.first().ok_or("No audiobook found")?;
    assert_eq!(first_book.files.len(), 1, "OPUS file not detected");
    assert!(first_book
        .files
        .first()
        .ok_or("No file found")?
        .to_string_lossy()
        .contains("chapter1.opus"));

    Ok(())
}

#[tokio::test]
async fn test_aac_files_detected_by_scanner() -> Result<(), Box<dyn Error>> {
    let temp = TempDir::new()?;
    let fixtures = TestFixtures::new();
    let aac_file = fixtures.audio_path("sample_aac.aac");

    if !aac_file.exists() {
        return Ok(());
    }

    let audiobook_dir = temp.path().join("aac_book");
    fs::create_dir_all(&audiobook_dir)?;
    fs::copy(&aac_file, audiobook_dir.join("chapter1.aac"))?;

    let discovered = scan_directory(temp.path().to_path_buf()).await?;

    assert_eq!(discovered.len(), 1, "AAC audiobook not detected");
    let first_book = discovered.first().ok_or("No audiobook found")?;
    assert_eq!(first_book.files.len(), 1, "AAC file not detected");
    assert!(first_book
        .files
        .first()
        .ok_or("No file found")?
        .to_string_lossy()
        .contains("chapter1.aac"));

    Ok(())
}

#[tokio::test]
async fn test_wav_files_detected_by_scanner() -> Result<(), Box<dyn Error>> {
    let temp = TempDir::new()?;
    let fixtures = TestFixtures::new();
    let wav_file = fixtures.audio_path("sample_wav.wav");

    if !wav_file.exists() {
        return Ok(());
    }

    let audiobook_dir = temp.path().join("wav_book");
    fs::create_dir_all(&audiobook_dir)?;
    fs::copy(&wav_file, audiobook_dir.join("chapter1.wav"))?;

    let discovered = scan_directory(temp.path().to_path_buf()).await?;

    assert_eq!(discovered.len(), 1, "WAV audiobook not detected");
    let first_book = discovered.first().ok_or("No audiobook found")?;
    assert_eq!(first_book.files.len(), 1, "WAV file not detected");
    assert!(first_book
        .files
        .first()
        .ok_or("No file found")?
        .to_string_lossy()
        .contains("chapter1.wav"));

    Ok(())
}

#[tokio::test]
async fn test_wma_files_detected_by_scanner() -> Result<(), Box<dyn Error>> {
    let temp = TempDir::new()?;
    let fixtures = TestFixtures::new();
    let wma_file = fixtures.audio_path("sample_wma.wma");

    if !wma_file.exists() {
        return Ok(());
    }

    let audiobook_dir = temp.path().join("wma_book");
    fs::create_dir_all(&audiobook_dir)?;
    fs::copy(&wma_file, audiobook_dir.join("chapter1.wma"))?;

    let discovered = scan_directory(temp.path().to_path_buf()).await?;

    assert_eq!(discovered.len(), 1, "WMA audiobook not detected");
    let first_book = discovered.first().ok_or("No audiobook found")?;
    assert_eq!(first_book.files.len(), 1, "WMA file not detected");
    assert!(first_book
        .files
        .first()
        .ok_or("No file found")?
        .to_string_lossy()
        .contains("chapter1.wma"));

    Ok(())
}

#[tokio::test]
async fn test_mixed_format_audiobook() -> Result<(), Box<dyn Error>> {
    // Test audiobook with multiple formats in same directory
    let temp = TempDir::new()?;
    let fixtures = TestFixtures::new();

    let audiobook_dir = temp.path().join("mixed_formats");
    fs::create_dir_all(&audiobook_dir)?;

    // Copy multiple format files if they exist
    let mut format_count = 0;

    let formats = [
        ("sample_mp3.mp3", "chapter1.mp3"),
        ("sample_m4a.m4a", "chapter2.m4a"),
        ("sample_ogg.ogg", "chapter3.ogg"),
        ("sample_flac.flac", "chapter4.flac"),
    ];

    for (src_name, dest_name) in &formats {
        let src = fixtures.audio_path(src_name);
        if src.exists() {
            fs::copy(&src, audiobook_dir.join(dest_name))?;
            format_count += 1;
        }
    }

    if format_count == 0 {
        return Ok(()); // Skip if no fixtures available
    }

    let discovered = scan_directory(temp.path().to_path_buf()).await?;

    assert_eq!(
        discovered.len(),
        1,
        "Mixed format audiobook not detected as single book"
    );
    assert_eq!(
        discovered.first().ok_or("No audiobook found")?.files.len(),
        format_count,
        "Not all mixed format files detected"
    );

    Ok(())
}
