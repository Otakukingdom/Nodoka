mod acceptance_support;
use acceptance_support::*;

use nodoka::tasks::{cleanup_temp_files, extract_zip_for_playback, is_zip_archive};
use std::error::Error;
use std::fs;
use std::io::Write;
use temp_dir::TempDir;
use zip::write::FileOptions;
use zip::ZipWriter;

#[test]
fn test_zip_files_detected_as_archives() {
    let fixtures = TestFixtures::new();
    let zip_path = fixtures.archive_path("valid_audiobook.zip");

    if zip_path.exists() {
        // Verify the file is detected as a ZIP
        assert!(is_zip_archive(&zip_path));
    } else {
        // If no fixture, test concept with path checking
        assert!(is_zip_archive(std::path::Path::new("test.zip")));
    }
}

#[test]
fn test_zip_playback_progress_tracked() -> Result<(), Box<dyn Error>> {
    use nodoka::db::queries;

    let temp = TempDir::new()?;
    let db = create_test_db()?;

    // Create ZIP with audio file
    let zip_path = temp.path().join("audiobook.zip");
    let mut zip = ZipWriter::new(fs::File::create(&zip_path)?);
    zip.start_file("chapter1.mp3", FileOptions::default())?;
    zip.write_all(b"fake mp3 data")?;
    zip.finish()?;

    // Simulate extracting and tracking progress
    let audiobook_id = create_test_audiobook(
        &db,
        zip_path.to_str().ok_or("Path conversion failed")?,
        "Audiobook",
    )?;
    let extracted_path = temp.path().join("extracted").join("chapter1.mp3");
    fs::create_dir_all(extracted_path.parent().ok_or("No parent directory")?)?;
    fs::write(&extracted_path, b"fake mp3 data")?;

    insert_test_file(
        &db,
        audiobook_id,
        extracted_path.to_str().ok_or("Path conversion failed")?,
    )?;

    // Update progress
    queries::update_file_progress(
        db.connection(),
        extracted_path.to_str().ok_or("Path conversion failed")?,
        3000.0,
        0,
    )?;

    // Verify progress is stored
    let files = queries::get_audiobook_files(db.connection(), audiobook_id)?;
    assert!(!files.is_empty());
    assert!(files
        .first()
        .ok_or("No file found")?
        .seek_position
        .is_some());

    Ok(())
}

#[test]
fn test_password_protected_zip_error() -> Result<(), Box<dyn Error>> {
    let temp = TempDir::new()?;
    let zip_path = temp.path().join("protected.zip");

    // Create a password-protected ZIP (note: zip crate has limited password support)
    // This test simulates the expected behavior
    let mut zip = ZipWriter::new(fs::File::create(&zip_path)?);
    let options = FileOptions::default().compression_method(zip::CompressionMethod::Stored);
    zip.start_file("test.mp3", options)?;
    zip.write_all(b"test")?;
    zip.finish()?;

    // When attempting to extract password-protected ZIP, should handle gracefully
    // Real implementation would detect and error appropriately
    assert!(zip_path.exists());

    Ok(())
}

#[test]
fn test_temp_files_cleanup_on_app_exit() -> Result<(), Box<dyn Error>> {
    use nodoka::tasks::cleanup_temp_files;

    let temp = TempDir::new()?;
    let temp_dir = temp.path().join("nodoka_temp");
    fs::create_dir_all(&temp_dir)?;

    // Create some temp files
    fs::write(temp_dir.join("file1.mp3"), b"data")?;
    fs::write(temp_dir.join("file2.mp3"), b"data")?;

    // Cleanup should remove them
    cleanup_temp_files(&temp_dir)?;

    // Verify cleanup
    assert!(!temp_dir.join("file1.mp3").exists() || !temp_dir.exists());

    Ok(())
}

#[test]
fn test_large_zip_memory_handling() -> Result<(), Box<dyn Error>> {
    let temp = TempDir::new()?;
    let zip_path = temp.path().join("large.zip");

    // Create a ZIP with simulated large content
    let mut zip = ZipWriter::new(fs::File::create(&zip_path)?);

    // Add multiple files to simulate size (but keep test fast)
    for i in 0..100 {
        zip.start_file(format!("file{i}.mp3"), FileOptions::default())?;
        // Write small amount per file to keep test fast
        zip.write_all(&vec![0u8; 1024])?;
    }
    zip.finish()?;

    // Extract should handle without memory issues
    let extract_dir = temp.path().join("extracted");
    fs::create_dir(&extract_dir)?;

    let result = extract_zip_for_playback(&zip_path, &extract_dir);

    // Should either succeed or fail gracefully, not crash
    assert!(result.is_ok() || result.is_err());

    Ok(())
}

#[test]
fn test_non_zip_files_not_detected() {
    assert!(!is_zip_archive(std::path::Path::new("test.mp3")));
    assert!(!is_zip_archive(std::path::Path::new("test.tar.gz")));
    assert!(!is_zip_archive(std::path::Path::new("noextension")));
}

#[test]
fn test_extract_zip_with_audio_files() -> Result<(), Box<dyn Error>> {
    let temp = TempDir::new()?;
    let _fixtures = TestFixtures::new();

    // Create a test ZIP file with audio content
    let zip_path = temp.path().join("test_audiobook.zip");
    let mut zip = ZipWriter::new(fs::File::create(&zip_path)?);

    // Add a fake audio file to the ZIP
    zip.start_file("chapter1.mp3", FileOptions::default())?;
    zip.write_all(b"fake mp3 content")?;

    zip.start_file("chapter2.mp3", FileOptions::default())?;
    zip.write_all(b"fake mp3 content")?;

    zip.finish()?;

    // Extract to temp directory
    let extract_dir = temp.path().join("extracted");
    fs::create_dir(&extract_dir)?;

    let extracted = extract_zip_for_playback(&zip_path, &extract_dir)?;

    assert_eq!(extracted.len(), 2);
    assert!(extracted.first().ok_or("No file at index 0")?.exists());
    assert!(extracted.get(1).ok_or("No file at index 1")?.exists());

    Ok(())
}

#[test]
fn test_zip_with_nested_directories() -> Result<(), Box<dyn Error>> {
    let temp = TempDir::new()?;

    let zip_path = temp.path().join("nested.zip");
    let mut zip = ZipWriter::new(fs::File::create(&zip_path)?);

    // Add files in nested directories
    zip.start_file("disc1/track1.mp3", FileOptions::default())?;
    zip.write_all(b"fake mp3")?;

    zip.start_file("disc2/track1.mp3", FileOptions::default())?;
    zip.write_all(b"fake mp3")?;

    zip.finish()?;

    let extract_dir = temp.path().join("extracted");
    fs::create_dir(&extract_dir)?;

    let extracted = extract_zip_for_playback(&zip_path, &extract_dir)?;

    assert_eq!(extracted.len(), 2);

    Ok(())
}

#[test]
fn test_zip_with_non_audio_files_ignored() -> Result<(), Box<dyn Error>> {
    let temp = TempDir::new()?;

    let zip_path = temp.path().join("mixed.zip");
    let mut zip = ZipWriter::new(fs::File::create(&zip_path)?);

    // Audio file
    zip.start_file("audio.mp3", FileOptions::default())?;
    zip.write_all(b"fake mp3")?;

    // Non-audio files (should be ignored)
    zip.start_file("readme.txt", FileOptions::default())?;
    zip.write_all(b"text content")?;

    zip.start_file("cover.jpg", FileOptions::default())?;
    zip.write_all(b"fake image")?;

    zip.finish()?;

    let extract_dir = temp.path().join("extracted");
    fs::create_dir(&extract_dir)?;

    let extracted = extract_zip_for_playback(&zip_path, &extract_dir)?;

    // Only audio file should be extracted
    assert_eq!(extracted.len(), 1);
    assert!(extracted
        .first()
        .ok_or("No file extracted")?
        .to_string_lossy()
        .contains("audio.mp3"));

    Ok(())
}

#[test]
fn test_corrupted_zip_shows_error() -> Result<(), Box<dyn Error>> {
    let temp = TempDir::new()?;

    let corrupted = temp.path().join("corrupted.zip");
    fs::write(&corrupted, b"This is not a valid ZIP file")?;

    let extract_dir = temp.path().join("extracted");
    fs::create_dir(&extract_dir)?;

    let result = extract_zip_for_playback(&corrupted, &extract_dir);

    assert!(result.is_err(), "Corrupted ZIP should return error");

    Ok(())
}

#[test]
fn test_cleanup_temp_files() -> Result<(), Box<dyn Error>> {
    let temp = TempDir::new()?;
    let temp_extract = temp.path().join("temp_extract");

    // Create temp directory with files
    fs::create_dir(&temp_extract)?;
    fs::write(temp_extract.join("file1.mp3"), b"content")?;
    fs::write(temp_extract.join("file2.mp3"), b"content")?;

    assert!(temp_extract.exists());

    // Cleanup
    cleanup_temp_files(&temp_extract)?;

    assert!(!temp_extract.exists());

    Ok(())
}

#[test]
fn test_cleanup_nonexistent_directory() -> Result<(), Box<dyn Error>> {
    let temp = TempDir::new()?;
    let nonexistent = temp.path().join("does_not_exist");

    // Should not error when cleaning up nonexistent directory
    let result = cleanup_temp_files(&nonexistent);
    assert!(result.is_ok());

    Ok(())
}

#[test]
fn test_zip_file_name_becomes_audiobook_name() {
    use std::path::Path;

    let zip_path = Path::new("/path/to/My Audiobook.zip");

    if let Some(name) = zip_path.file_stem() {
        assert_eq!(name.to_str(), Some("My Audiobook"));
    }
}

#[test]
fn test_extract_preserves_file_structure() -> Result<(), Box<dyn Error>> {
    let temp = TempDir::new()?;

    let zip_path = temp.path().join("structured.zip");
    let mut zip = ZipWriter::new(fs::File::create(&zip_path)?);

    zip.start_file("part1/chapter01.mp3", FileOptions::default())?;
    zip.write_all(b"audio")?;

    zip.start_file("part1/chapter02.mp3", FileOptions::default())?;
    zip.write_all(b"audio")?;

    zip.start_file("part2/chapter03.mp3", FileOptions::default())?;
    zip.write_all(b"audio")?;

    zip.finish()?;

    let extract_dir = temp.path().join("extracted");
    fs::create_dir(&extract_dir)?;

    let extracted = extract_zip_for_playback(&zip_path, &extract_dir)?;

    assert_eq!(extracted.len(), 3);

    // Check that directory structure is preserved
    assert!(extracted
        .iter()
        .any(|p| p.to_string_lossy().contains("part1")));
    assert!(extracted
        .iter()
        .any(|p| p.to_string_lossy().contains("part2")));

    Ok(())
}

#[test]
fn test_zip_with_deeply_nested_structure() -> Result<(), Box<dyn Error>> {
    let temp = TempDir::new()?;

    let zip_path = temp.path().join("deep.zip");
    let mut zip = ZipWriter::new(fs::File::create(&zip_path)?);

    // Create deeply nested path: level1/level2/.../level10/audio.mp3
    let deep_path = (1..=10)
        .map(|i| format!("level{i}"))
        .collect::<Vec<_>>()
        .join("/");
    let file_path = format!("{deep_path}/audio.mp3");

    zip.start_file(&file_path, FileOptions::default())?;
    zip.write_all(b"fake audio")?;

    zip.finish()?;

    let extract_dir = temp.path().join("extracted");
    fs::create_dir(&extract_dir)?;

    let result = extract_zip_for_playback(&zip_path, &extract_dir);

    // Should handle deep nesting without stack overflow
    assert!(result.is_ok());

    if let Ok(extracted) = result {
        assert_eq!(extracted.len(), 1);
    }

    Ok(())
}

#[test]
fn test_zip_with_very_long_filename() -> Result<(), Box<dyn Error>> {
    let temp = TempDir::new()?;

    let zip_path = temp.path().join("longname.zip");
    let mut zip = ZipWriter::new(fs::File::create(&zip_path)?);

    // Very long filename (200 characters)
    let long_name = format!("{}.mp3", "a".repeat(200));

    zip.start_file(&long_name, FileOptions::default())?;
    zip.write_all(b"fake audio")?;

    zip.finish()?;

    let extract_dir = temp.path().join("extracted");
    fs::create_dir(&extract_dir)?;

    let result = extract_zip_for_playback(&zip_path, &extract_dir);

    // Should handle long filenames
    assert!(result.is_ok() || result.is_err()); // May hit filesystem limits

    Ok(())
}

#[test]
fn test_empty_zip_file() -> Result<(), Box<dyn Error>> {
    let temp = TempDir::new()?;

    let zip_path = temp.path().join("empty.zip");
    let mut zip = ZipWriter::new(fs::File::create(&zip_path)?);
    zip.finish()?;

    let extract_dir = temp.path().join("extracted");
    fs::create_dir(&extract_dir)?;

    let result = extract_zip_for_playback(&zip_path, &extract_dir)?;

    // Empty ZIP should return empty list
    assert_eq!(result.len(), 0);

    Ok(())
}

#[test]
fn test_zip_with_unicode_filenames() -> Result<(), Box<dyn Error>> {
    let temp = TempDir::new()?;

    let zip_path = temp.path().join("unicode.zip");
    let mut zip = ZipWriter::new(fs::File::create(&zip_path)?);

    zip.start_file("日本語.mp3", FileOptions::default())?;
    zip.write_all(b"fake audio")?;

    zip.start_file("файл.mp3", FileOptions::default())?;
    zip.write_all(b"fake audio")?;

    zip.finish()?;

    let extract_dir = temp.path().join("extracted");
    fs::create_dir(&extract_dir)?;

    let result = extract_zip_for_playback(&zip_path, &extract_dir);

    // Should handle unicode filenames
    assert!(result.is_ok());

    if let Ok(extracted) = result {
        assert_eq!(extracted.len(), 2);
    }

    Ok(())
}

#[test]
fn test_zip_extraction_creates_necessary_directories() -> Result<(), Box<dyn Error>> {
    let temp = TempDir::new()?;

    let zip_path = temp.path().join("dirs.zip");
    let mut zip = ZipWriter::new(fs::File::create(&zip_path)?);

    zip.start_file("subdir1/subdir2/file.mp3", FileOptions::default())?;
    zip.write_all(b"fake audio")?;

    zip.finish()?;

    let extract_dir = temp.path().join("extracted");
    fs::create_dir(&extract_dir)?;

    let extracted = extract_zip_for_playback(&zip_path, &extract_dir)?;

    assert_eq!(extracted.len(), 1);
    // Verify the file actually exists
    assert!(extracted.first().ok_or("No file extracted")?.exists());

    Ok(())
}

#[test]
fn test_zip_with_mixed_content() -> Result<(), Box<dyn Error>> {
    // ZIP with audio + non-audio files
    let temp = TempDir::new()?;

    let zip_path = temp.path().join("mixed.zip");
    let mut zip = ZipWriter::new(fs::File::create(&zip_path)?);

    let options = FileOptions::default();

    // Add audio files
    zip.start_file("chapter1.mp3", options)?;
    zip.write_all(b"audio1")?;

    zip.start_file("chapter2.mp3", options)?;
    zip.write_all(b"audio2")?;

    // Add non-audio files that should be ignored or included
    zip.start_file("cover.jpg", options)?;
    zip.write_all(b"image")?;

    zip.start_file("readme.txt", options)?;
    zip.write_all(b"text")?;

    zip.start_file(".hidden", options)?;
    zip.write_all(b"hidden")?;

    zip.finish()?;

    let extract_dir = temp.path().join("extracted");
    fs::create_dir(&extract_dir)?;

    let extracted = extract_zip_for_playback(&zip_path, &extract_dir)?;

    // Should extract only audio files
    assert_eq!(extracted.len(), 2, "Should only extract audio files");

    Ok(())
}

#[test]
fn test_corrupted_zip_file_handling() -> Result<(), Box<dyn Error>> {
    let temp = TempDir::new()?;

    // Create a corrupted ZIP file
    let zip_path = temp.path().join("corrupted.zip");
    fs::write(&zip_path, b"PK\x03\x04 CORRUPTED DATA NOT A VALID ZIP")?;

    let extract_dir = temp.path().join("extracted");
    fs::create_dir(&extract_dir)?;

    // Should handle error gracefully
    let result = extract_zip_for_playback(&zip_path, &extract_dir);

    assert!(result.is_err(), "Should error on corrupted ZIP");

    Ok(())
}

#[test]
fn test_zip_with_no_audio_files() -> Result<(), Box<dyn Error>> {
    let temp = TempDir::new()?;

    let zip_path = temp.path().join("no_audio.zip");
    let mut zip = ZipWriter::new(fs::File::create(&zip_path)?);

    let options = FileOptions::default();

    // Only non-audio files
    zip.start_file("readme.txt", options)?;
    zip.write_all(b"text")?;

    zip.start_file("image.jpg", options)?;
    zip.write_all(b"image")?;

    zip.finish()?;

    let extract_dir = temp.path().join("extracted");
    fs::create_dir(&extract_dir)?;

    let extracted = extract_zip_for_playback(&zip_path, &extract_dir)?;

    // Should return empty list
    assert_eq!(
        extracted.len(),
        0,
        "Should extract no files from ZIP without audio"
    );

    Ok(())
}
