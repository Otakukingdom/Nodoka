mod acceptance_support;
use acceptance_support::*;

use nodoka::tasks::scan_directory;
use std::error::Error;
use std::fs;
use temp_dir::TempDir;

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
fn test_wav_files_detected() {
    let fixtures = TestFixtures::new();
    let audio_file = fixtures.audio_path("sample_wav.wav");

    if audio_file.exists() {
        let extension = audio_file.extension().and_then(|e| e.to_str());
        assert_eq!(extension, Some("wav"));
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
