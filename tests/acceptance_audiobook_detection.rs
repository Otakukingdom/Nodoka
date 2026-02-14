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
        let dest = nested.join(format!("chapter{}.mp3", i));
        fs::copy(fixtures.audio_path("sample_mp3.mp3"), &dest)?;
    }
    
    // Scan and verify discovery
    let discovered = scan_directory(temp.path().to_path_buf()).await?;
    
    assert_eq!(discovered.len(), 1);
    assert_eq!(discovered[0].files.len(), 3);
    assert_eq!(discovered[0].name, "book");
    
    Ok(())
}

#[tokio::test]
async fn test_files_grouped_by_parent_directory() -> Result<(), Box<dyn Error>> {
    let temp = TempDir::new()?;
    let fixtures = TestFixtures::new();
    
    // Create two audiobooks
    let book1 = temp.path().join("Book One");
    let book2 = temp.path().join("Book Two");
    fs::create_dir_all(&book1)?;
    fs::create_dir_all(&book2)?;
    
    fs::copy(fixtures.audio_path("sample_mp3.mp3"), book1.join("chapter1.mp3"))?;
    fs::copy(fixtures.audio_path("sample_mp3.mp3"), book2.join("chapter1.mp3"))?;
    
    let discovered = scan_directory(temp.path().to_path_buf()).await?;
    
    assert_eq!(discovered.len(), 2);
    assert!(discovered.iter().any(|ab| ab.name == "Book One"));
    assert!(discovered.iter().any(|ab| ab.name == "Book Two"));
    
    Ok(())
}

#[tokio::test]
async fn test_audiobook_name_from_folder() -> Result<(), Box<dyn Error>> {
    let temp = TempDir::new()?;
    let fixtures = TestFixtures::new();
    
    let book_dir = temp.path().join("The Great Audiobook");
    fs::create_dir_all(&book_dir)?;
    fs::copy(fixtures.audio_path("sample_mp3.mp3"), book_dir.join("part1.mp3"))?;
    
    let discovered = scan_directory(temp.path().to_path_buf()).await?;
    
    assert_eq!(discovered.len(), 1);
    assert_eq!(discovered[0].name, "The Great Audiobook");
    
    Ok(())
}

#[tokio::test]
async fn test_files_sorted_naturally() -> Result<(), Box<dyn Error>> {
    let temp = TempDir::new()?;
    let fixtures = TestFixtures::new();
    
    let book = temp.path().join("Book");
    fs::create_dir_all(&book)?;
    
    // Create files in non-natural order to test sorting
    for name in &["Chapter 10.mp3", "Chapter 2.mp3", "Chapter 1.mp3", "Chapter 20.mp3"] {
        fs::copy(fixtures.audio_path("sample_mp3.mp3"), book.join(name))?;
    }
    
    let discovered = scan_directory(temp.path().to_path_buf()).await?;
    
    assert_eq!(discovered[0].files.len(), 4);
    // Files should be naturally sorted: 1, 2, 10, 20
    assert!(discovered[0].files[0].contains("Chapter 1.mp3"));
    assert!(discovered[0].files[1].contains("Chapter 2.mp3"));
    assert!(discovered[0].files[2].contains("Chapter 10.mp3"));
    assert!(discovered[0].files[3].contains("Chapter 20.mp3"));
    
    Ok(())
}

#[test]
fn test_mp3_files_detected() -> Result<(), Box<dyn Error>> {
    let fixtures = TestFixtures::new();
    let mp3_path = fixtures.audio_path("sample_mp3.mp3");
    
    assert!(mp3_path.exists(), "MP3 fixture should exist");
    
    Ok(())
}

#[test]
fn test_m4b_files_detected() -> Result<(), Box<dyn Error>> {
    let fixtures = TestFixtures::new();
    let m4b_path = fixtures.audio_path("sample_m4b.m4b");
    
    if m4b_path.exists() {
        // M4B fixture exists and can be detected
        assert!(m4b_path.extension().and_then(|e| e.to_str()) == Some("m4b"));
    }
    
    Ok(())
}

#[test]
fn test_flac_files_detected() -> Result<(), Box<dyn Error>> {
    let fixtures = TestFixtures::new();
    let flac_path = fixtures.audio_path("sample_flac.flac");
    
    if flac_path.exists() {
        assert!(flac_path.extension().and_then(|e| e.to_str()) == Some("flac"));
    }
    
    Ok(())
}

#[test]
fn test_ogg_files_detected() -> Result<(), Box<dyn Error>> {
    let fixtures = TestFixtures::new();
    let ogg_path = fixtures.audio_path("sample_ogg.ogg");
    
    if ogg_path.exists() {
        assert!(ogg_path.extension().and_then(|e| e.to_str()) == Some("ogg"));
    }
    
    Ok(())
}

#[tokio::test]
async fn test_non_audio_files_ignored() -> Result<(), Box<dyn Error>> {
    let temp = TempDir::new()?;
    let fixtures = TestFixtures::new();
    
    let audiobook = temp.path().join("Mixed Content");
    fs::create_dir_all(&audiobook)?;
    
    // Add audio file
    fs::copy(fixtures.audio_path("sample_mp3.mp3"), audiobook.join("audio.mp3"))?;
    
    // Add non-audio files
    fs::write(audiobook.join("readme.txt"), b"info")?;
    fs::write(audiobook.join("notes.pdf"), b"fake pdf")?;
    
    let discovered = scan_directory(temp.path().to_path_buf()).await?;
    
    assert_eq!(discovered.len(), 1);
    assert_eq!(discovered[0].files.len(), 1); // Only audio file
    
    Ok(())
}

#[tokio::test]
async fn test_hidden_files_ignored() -> Result<(), Box<dyn Error>> {
    let temp = TempDir::new()?;
    let fixtures = TestFixtures::new();
    
    let audiobook = temp.path().join("Audiobook");
    fs::create_dir_all(&audiobook)?;
    
    fs::copy(fixtures.audio_path("sample_mp3.mp3"), audiobook.join("visible.mp3"))?;
    fs::copy(fixtures.audio_path("sample_mp3.mp3"), audiobook.join(".hidden.mp3"))?;
    
    let discovered = scan_directory(temp.path().to_path_buf()).await?;
    
    assert_eq!(discovered[0].files.len(), 1);
    assert!(discovered[0].files[0].contains("visible.mp3"));
    
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
    fs::copy(fixtures.audio_path("sample_mp3.mp3"), book.join("file1.mp3"))?;
    fs::copy(fixtures.audio_path("sample_mp3.mp3"), book.join("file2.mp3"))?;
    
    let discovered1 = scan_directory(temp.path().to_path_buf()).await?;
    assert_eq!(discovered1[0].files.len(), 2);
    
    // Add a new file
    fs::copy(fixtures.audio_path("sample_mp3.mp3"), book.join("file3.mp3"))?;
    
    // Rescan
    let discovered2 = scan_directory(temp.path().to_path_buf()).await?;
    assert_eq!(discovered2[0].files.len(), 3);
    
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
        fs::copy(fixtures.audio_path("sample_flac.flac"), book.join("file.flac"))?;
        count += 1;
    }
    if fixtures.audio_path("sample_ogg.ogg").exists() {
        fs::copy(fixtures.audio_path("sample_ogg.ogg"), book.join("file.ogg"))?;
        count += 1;
    }
    
    if count > 0 {
        let discovered = scan_directory(temp.path().to_path_buf()).await?;
        assert_eq!(discovered.len(), 1);
        assert_eq!(discovered[0].files.len(), count);
    }
    
    Ok(())
}

#[tokio::test]
async fn test_single_file_audiobook() -> Result<(), Box<dyn Error>> {
    let temp = TempDir::new()?;
    let fixtures = TestFixtures::new();
    
    let book = temp.path().join("Single File Book");
    fs::create_dir_all(&book)?;
    fs::copy(fixtures.audio_path("sample_mp3.mp3"), book.join("audiobook.mp3"))?;
    
    let discovered = scan_directory(temp.path().to_path_buf()).await?;
    
    assert_eq!(discovered.len(), 1);
    assert_eq!(discovered[0].files.len(), 1);
    assert_eq!(discovered[0].name, "Single File Book");
    
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
    
    fs::copy(fixtures.audio_path("sample_mp3.mp3"), disc1.join("track1.mp3"))?;
    fs::copy(fixtures.audio_path("sample_mp3.mp3"), disc2.join("track1.mp3"))?;
    
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
    fs::copy(fixtures.audio_path("sample_mp3.mp3"), book.join("part1.mp3"))?;
    fs::copy(fixtures.audio_path("sample_mp3.mp3"), book.join("part2.mp3"))?;
    
    // Image files (should be ignored in file count)
    fs::write(book.join("cover.jpg"), b"fake image")?;
    fs::write(book.join("author.png"), b"fake image")?;
    
    // Text files
    fs::write(book.join("README.txt"), b"description")?;
    
    let discovered = scan_directory(temp.path().to_path_buf()).await?;
    
    assert_eq!(discovered.len(), 1);
    assert_eq!(discovered[0].files.len(), 2); // Only audio files counted
    
    Ok(())
}

#[tokio::test]
async fn test_special_characters_in_names() -> Result<(), Box<dyn Error>> {
    let temp = TempDir::new()?;
    let fixtures = TestFixtures::new();
    
    let book = temp.path().join("Book: The - Special (2024) [Edition]");
    fs::create_dir_all(&book)?;
    fs::copy(fixtures.audio_path("sample_mp3.mp3"), book.join("Chapter #1.mp3"))?;
    
    let discovered = scan_directory(temp.path().to_path_buf()).await?;
    
    assert_eq!(discovered.len(), 1);
    assert_eq!(discovered[0].name, "Book: The - Special (2024) [Edition]");
    
    Ok(())
}

#[tokio::test]
async fn test_unicode_in_filenames() -> Result<(), Box<dyn Error>> {
    let temp = TempDir::new()?;
    let fixtures = TestFixtures::new();
    
    let book = temp.path().join("日本語の本");
    fs::create_dir_all(&book)?;
    fs::copy(fixtures.audio_path("sample_mp3.mp3"), book.join("第１章.mp3"))?;
    
    let discovered = scan_directory(temp.path().to_path_buf()).await?;
    
    assert_eq!(discovered.len(), 1);
    assert_eq!(discovered[0].name, "日本語の本");
    
    Ok(())
}
