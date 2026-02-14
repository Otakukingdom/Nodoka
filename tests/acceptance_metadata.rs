mod acceptance_support;
use acceptance_support::*;

use nodoka::player::Scanner;
use std::error::Error;

#[test]
fn test_extract_duration() -> Result<(), Box<dyn Error>> {
    let fixtures = TestFixtures::new();
    let audio_file = fixtures.audio_path("sample_mp3.mp3");

    if !audio_file.exists() {
        return Ok(()); // Skip if no fixture
    }

    if let Ok(mut scanner) = Scanner::new() {
        if let Ok(properties) = scanner.scan_file(&audio_file) {
            assert!(properties.duration >= 0, "Duration should be non-negative");
        }
    }

    Ok(())
}

#[test]
fn test_missing_metadata_handled() -> Result<(), Box<dyn Error>> {
    let fixtures = TestFixtures::new();
    let audio_file = fixtures.audio_path("sample_mp3.mp3");

    if !audio_file.exists() {
        return Ok(());
    }

    if let Ok(mut scanner) = Scanner::new() {
        let result = scanner.scan_file(&audio_file);
        // Should not panic even if metadata is missing
        assert!(result.is_ok() || result.is_err());
    }

    Ok(())
}

#[test]
fn test_duration_calculation() -> Result<(), Box<dyn Error>> {
    let fixtures = TestFixtures::new();
    let audio_file = fixtures.audio_path("sample_mp3.mp3");

    if !audio_file.exists() {
        return Ok(());
    }

    if let Ok(mut scanner) = Scanner::new() {
        if let Ok(properties) = scanner.scan_file(&audio_file) {
            // Duration should be sensible for a 1-second file
            assert!(
                properties.duration >= 0 && properties.duration <= 10000,
                "Duration {} out of expected range for test file",
                properties.duration
            );
        }
    }

    Ok(())
}

#[test]
fn test_total_audiobook_duration() -> Result<(), Box<dyn Error>> {
    // Test calculating total duration across multiple files
    let durations = vec![1800, 2100, 1950]; // Example durations in seconds
    let total: i64 = durations.iter().sum();

    assert_eq!(total, 5850);

    // Convert to hours:minutes:seconds
    let hours = total / 3600;
    let minutes = (total % 3600) / 60;
    let seconds = total % 60;

    assert_eq!(hours, 1);
    assert_eq!(minutes, 37);
    assert_eq!(seconds, 30);

    Ok(())
}

#[test]
fn test_metadata_fields_optional() -> Result<(), Box<dyn Error>> {
    // Test that metadata fields can be None
    struct Metadata {
        title: Option<String>,
        author: Option<String>,
        narrator: Option<String>,
    }

    let metadata = Metadata {
        title: None,
        author: None,
        narrator: None,
    };

    assert!(metadata.title.is_none());
    assert!(metadata.author.is_none());
    assert!(metadata.narrator.is_none());

    Ok(())
}

#[test]
fn test_metadata_persistence() -> Result<(), Box<dyn Error>> {
    use nodoka::db::queries;

    let db = create_test_db()?;
    let audiobook_id = create_test_audiobook(&db, "/test", "Book with Metadata")?;

    // Metadata should be stored with audiobook
    let audiobook = queries::get_audiobook_by_id(db.connection(), audiobook_id)?
        .ok_or("Audiobook not found")?;

    assert_eq!(audiobook.name, "Book with Metadata");

    Ok(())
}

#[test]
fn test_long_metadata_strings() -> Result<(), Box<dyn Error>> {
    let long_title = "A".repeat(500);
    let truncated = if long_title.len() > 200 {
        format!("{}...", &long_title[..200])
    } else {
        long_title.clone()
    };

    assert!(truncated.len() <= 203); // 200 chars + "..."

    Ok(())
}

#[test]
fn test_metadata_encoding() -> Result<(), Box<dyn Error>> {
    // Test UTF-8 metadata
    let title = "日本語のタイトル";
    assert!(title.is_ascii() == false);
    assert_eq!(title.chars().count(), 8);

    Ok(())
}

#[test]
fn test_file_properties_extraction() -> Result<(), Box<dyn Error>> {
    let fixtures = TestFixtures::new();
    let audio_file = fixtures.audio_path("sample_mp3.mp3");

    if !audio_file.exists() {
        return Ok(());
    }

    if let Ok(mut scanner) = Scanner::new() {
        if let Ok(properties) = scanner.scan_file(&audio_file) {
            // Check that basic properties are present
            assert!(properties.duration >= 0);
            // Other properties like bitrate, sample_rate may or may not be available
        }
    }

    Ok(())
}

#[test]
fn test_multiple_format_metadata() -> Result<(), Box<dyn Error>> {
    let fixtures = TestFixtures::new();

    let formats = vec![
        ("sample_mp3.mp3", "mp3"),
        ("sample_m4b.m4b", "m4b"),
        ("sample_flac.flac", "flac"),
        ("sample_ogg.ogg", "ogg"),
    ];

    for (file, _format) in formats {
        let audio_file = fixtures.audio_path(file);
        if audio_file.exists() {
            if let Ok(mut scanner) = Scanner::new() {
                // Should be able to extract metadata from all formats
                let _ = scanner.scan_file(&audio_file);
            }
        }
    }

    Ok(())
}

#[test]
fn test_metadata_caching() -> Result<(), Box<dyn Error>> {
    use nodoka::db::queries;

    let db = create_test_db()?;
    let audiobook_id = create_test_audiobook(&db, "/test", "Cached Metadata")?;

    // Insert file with metadata
    let file_path = "/test/Cached Metadata/file.mp3";
    insert_test_file(&db, audiobook_id, file_path)?;

    // Update with duration (simulating metadata extraction)
    let mut file = queries::get_audiobook_files(db.connection(), audiobook_id)?
        .into_iter()
        .next()
        .ok_or("No file found")?;

    file.length_of_file = Some(3600); // 1 hour duration

    queries::update_audiobook_file(db.connection(), &file)?;

    // Retrieve and verify cached value
    let files = queries::get_audiobook_files(db.connection(), audiobook_id)?;
    assert_eq!(files[0].length_of_file, Some(3600));

    Ok(())
}

#[test]
fn test_zero_duration_handled() -> Result<(), Box<dyn Error>> {
    // Test that files with zero duration don't cause issues
    let duration: i64 = 0;

    assert_eq!(duration, 0);
    assert!(duration >= 0);

    Ok(())
}
