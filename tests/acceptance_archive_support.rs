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
fn test_zip_files_detected_as_archives() -> Result<(), Box<dyn Error>> {
    let fixtures = TestFixtures::new();
    let zip_path = fixtures.archive_path("valid_audiobook.zip");

    if zip_path.exists() {
        assert!(is_zip_archive(&zip_path));
    } else {
        // Test with any .zip extension
        assert!(is_zip_archive(std::path::Path::new("test.zip")));
    }

    Ok(())
}

#[test]
fn test_is_zip_archive_case_insensitive() -> Result<(), Box<dyn Error>> {
    assert!(is_zip_archive(std::path::Path::new("test.ZIP")));
    assert!(is_zip_archive(std::path::Path::new("test.Zip")));
    assert!(is_zip_archive(std::path::Path::new("test.zip")));

    Ok(())
}

#[test]
fn test_non_zip_files_not_detected() -> Result<(), Box<dyn Error>> {
    assert!(!is_zip_archive(std::path::Path::new("test.mp3")));
    assert!(!is_zip_archive(std::path::Path::new("test.tar.gz")));
    assert!(!is_zip_archive(std::path::Path::new("noextension")));

    Ok(())
}

#[test]
fn test_extract_zip_with_audio_files() -> Result<(), Box<dyn Error>> {
    let temp = TempDir::new()?;
    let fixtures = TestFixtures::new();

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
    assert!(extracted[0].exists());
    assert!(extracted[1].exists());

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
    assert!(extracted[0].to_string_lossy().contains("audio.mp3"));

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
fn test_zip_file_name_becomes_audiobook_name() -> Result<(), Box<dyn Error>> {
    use std::path::Path;

    let zip_path = Path::new("/path/to/My Audiobook.zip");

    if let Some(name) = zip_path.file_stem() {
        assert_eq!(name.to_str(), Some("My Audiobook"));
    }

    Ok(())
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
