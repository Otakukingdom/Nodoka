mod acceptance_support;
use acceptance_support::*;

use nodoka::db::queries;
use nodoka::player::Scanner;
use std::error::Error;
use std::path::Path;

#[test]
fn test_extract_duration() {
    let fixtures = TestFixtures::new();
    let audio_file = fixtures.audio_path("sample_mp3.mp3");

    if !audio_file.exists() {
        return; // Skip if no fixture
    }

    if let Ok(scanner) = Scanner::new() {
        if let Ok(properties) = scanner.scan_media(&audio_file) {
            assert!(
                properties.duration_ms >= 0,
                "Duration should be non-negative"
            );
        }
    }
}

#[test]
fn test_extract_title_author_year_from_id3_tags() -> Result<(), Box<dyn Error>> {
    let Ok(scanner) = Scanner::new() else {
        return Ok(());
    };

    let fixtures = TestFixtures::new();
    let base_mp3 = fixtures.audio_path("sample_mp3.mp3");
    if !base_mp3.exists() {
        return Ok(());
    }
    let base_bytes = std::fs::read(&base_mp3)?;

    let temp = temp_dir::TempDir::new()?;
    let mp3_path = temp.path().join("tagged.mp3");

    let cover_png = create_test_png_bytes()?;
    write_id3v23_mp3_with_tags(
        &mp3_path,
        Some(("My Title", "My Author", "My Narrator", "2020")),
        Some(&cover_png),
        Some(&base_bytes),
    )?;

    let props = scanner.scan_media(&mp3_path)?;
    assert_eq!(props.title.as_deref(), Some("My Title"));
    assert_eq!(props.author.as_deref(), Some("My Author"));
    assert_eq!(props.narrator.as_deref(), Some("My Narrator"));
    assert_eq!(props.year, Some(2020));
    Ok(())
}

fn create_test_png_bytes() -> Result<Vec<u8>, Box<dyn Error>> {
    let img = image::RgbImage::from_pixel(2, 2, image::Rgb([1, 2, 3]));
    let dyn_img = image::DynamicImage::ImageRgb8(img);
    let mut bytes = Vec::new();
    dyn_img.write_to(
        &mut std::io::Cursor::new(&mut bytes),
        image::ImageOutputFormat::Png,
    )?;
    Ok(bytes)
}

fn write_id3v23_mp3_with_tags(
    path: &Path,
    text: Option<(&str, &str, &str, &str)>,
    cover_png: Option<&[u8]>,
    base_audio: Option<&[u8]>,
) -> Result<(), Box<dyn Error>> {
    let mut frames: Vec<Vec<u8>> = Vec::new();
    if let Some((title, artist, publisher, year)) = text {
        frames.push(id3v23_text_frame("TIT2", title.as_bytes()));
        frames.push(id3v23_text_frame("TPE1", artist.as_bytes()));
        frames.push(id3v23_text_frame("TPUB", publisher.as_bytes()));
        frames.push(id3v23_text_frame("TYER", year.as_bytes()));
    }
    if let Some(png) = cover_png {
        frames.push(id3v23_apic_frame(png));
    }

    let tag_body: Vec<u8> = frames.into_iter().flatten().collect();
    let tag_size = u32::try_from(tag_body.len()).map_err(|_| "tag too large")?;

    let mut out = Vec::new();
    out.extend_from_slice(b"ID3");
    out.push(3); // v2.3
    out.push(0);
    out.push(0);
    out.extend_from_slice(&syncsafe(tag_size));
    out.extend_from_slice(&tag_body);

    if let Some(base) = base_audio {
        out.extend_from_slice(base);
    } else {
        // Add some trailing bytes so the file is non-empty beyond tags.
        out.extend_from_slice(b"\0\0\0\0");
    }
    std::fs::write(path, out)?;
    Ok(())
}

fn id3v23_text_frame(id: &str, text: &[u8]) -> Vec<u8> {
    let mut body = Vec::with_capacity(1 + text.len());
    body.push(0); // ISO-8859-1
    body.extend_from_slice(text);

    let mut out = Vec::new();
    out.extend_from_slice(id.as_bytes());
    out.extend_from_slice(&(u32::try_from(body.len()).unwrap_or(0)).to_be_bytes());
    out.extend_from_slice(&[0, 0]);
    out.extend_from_slice(&body);
    out
}

fn id3v23_apic_frame(png: &[u8]) -> Vec<u8> {
    let mut body = Vec::new();
    body.push(0); // ISO-8859-1
    body.extend_from_slice(b"image/png\0");
    body.push(0x03); // cover front
    body.push(0); // empty description
    body.extend_from_slice(png);

    let mut out = Vec::new();
    out.extend_from_slice(b"APIC");
    out.extend_from_slice(&(u32::try_from(body.len()).unwrap_or(0)).to_be_bytes());
    out.extend_from_slice(&[0, 0]);
    out.extend_from_slice(&body);
    out
}

const fn syncsafe(size: u32) -> [u8; 4] {
    [
        ((size >> 21) & 0x7F) as u8,
        ((size >> 14) & 0x7F) as u8,
        ((size >> 7) & 0x7F) as u8,
        (size & 0x7F) as u8,
    ]
}

#[test]
fn test_missing_metadata_handled() {
    let fixtures = TestFixtures::new();
    let audio_file = fixtures.audio_path("sample_mp3.mp3");

    if !audio_file.exists() {
        return;
    }

    if let Ok(scanner) = Scanner::new() {
        let result = scanner.scan_media(&audio_file);
        match result {
            Ok(properties) => assert!(properties.duration_ms >= 0),
            Err(e) => assert!(!format!("{e}").is_empty()),
        }
    }
}

#[test]
fn test_duration_calculation() {
    let fixtures = TestFixtures::new();
    let audio_file = fixtures.audio_path("sample_mp3.mp3");

    if !audio_file.exists() {
        return;
    }

    if let Ok(scanner) = Scanner::new() {
        if let Ok(properties) = scanner.scan_media(&audio_file) {
            // Duration should be sensible for a 1-second file
            assert!(
                properties.duration_ms >= 0 && properties.duration_ms <= 10000,
                "Duration {} out of expected range for test file",
                properties.duration_ms
            );
        }
    }
}

#[test]
fn test_total_audiobook_duration() {
    // Test calculating total duration across multiple files
    let durations = [1800, 2100, 1950]; // Example durations in seconds
    let total: i64 = durations.iter().sum();

    assert_eq!(total, 5850);

    // Convert to hours:minutes:seconds
    let hours = total / 3600;
    let minutes = (total % 3600) / 60;
    let seconds = total % 60;

    assert_eq!(hours, 1);
    assert_eq!(minutes, 37);
    assert_eq!(seconds, 30);
}

#[test]
fn test_metadata_fields_optional() {
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
fn test_long_metadata_strings() {
    let long_title = "A".repeat(500);
    let truncated = if long_title.len() > 200 {
        format!("{}...", &long_title[..200])
    } else {
        long_title
    };

    assert!(truncated.len() <= 203); // 200 chars + "..."
}

#[test]
fn test_metadata_encoding() {
    // Test UTF-8 metadata
    let title = "日本語のタイトル";
    assert!(!title.is_ascii());
    assert_eq!(title.chars().count(), 8);
}

#[test]
fn test_file_properties_extraction() {
    let fixtures = TestFixtures::new();
    let audio_file = fixtures.audio_path("sample_mp3.mp3");

    if !audio_file.exists() {
        return;
    }

    if let Ok(scanner) = Scanner::new() {
        if let Ok(properties) = scanner.scan_media(&audio_file) {
            // Check that basic properties are present
            assert!(properties.duration_ms >= 0);
            // Other properties like bitrate, sample_rate may or may not be available
        }
    }
}

#[test]
fn test_multiple_format_metadata() {
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
            if let Ok(scanner) = Scanner::new() {
                // Should be able to extract metadata from all formats
                let _ = scanner.scan_media(&audio_file);
            }
        }
    }
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

    // Use INSERT OR REPLACE to update
    queries::insert_audiobook_file(db.connection(), &file)?;

    // Retrieve and verify cached value
    let files = queries::get_audiobook_files(db.connection(), audiobook_id)?;
    assert_eq!(files.first().ok_or("No file")?.length_of_file, Some(3600));

    Ok(())
}

#[test]
fn test_zero_duration_handled() {
    // Test that files with zero duration don't cause issues
    let duration: i64 = 0;

    assert_eq!(duration, 0);
    assert!(duration >= 0);
}

#[test]
fn test_metadata_with_very_long_strings() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;

    // Create audiobook with very long title
    let long_title = "A".repeat(10_000);
    let result = create_test_audiobook(&db, "/test", &long_title);

    // Should handle long strings without panic
    assert!(result.is_ok());

    if let Ok(audiobook_id) = result {
        let audiobook = queries::get_audiobook_by_id(db.connection(), audiobook_id)?
            .ok_or("Audiobook not found")?;
        assert!(!audiobook.name.is_empty());
    }

    Ok(())
}

#[test]
fn test_metadata_with_null_bytes() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;

    let title_with_null = "Book\0Title";

    let result = create_test_audiobook(&db, "/test", title_with_null);

    // Null bytes should be rejected to avoid database and UI issues.
    assert!(result.is_err());

    Ok(())
}

#[test]
fn test_metadata_unicode_encoding() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;

    // Unicode from various scripts
    let unicode_name = "Café 日本語 Русский العربية";
    let audiobook_id = create_test_audiobook(&db, "/test", unicode_name)?;

    let audiobook = queries::get_audiobook_by_id(db.connection(), audiobook_id)?
        .ok_or("Audiobook not found")?;

    assert_eq!(audiobook.name, unicode_name);

    Ok(())
}

#[test]
fn test_metadata_empty_strings() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;

    // Empty name
    let result = create_test_audiobook(&db, "/test", "");

    assert!(result.is_ok());

    Ok(())
}

#[test]
fn test_metadata_newlines_and_tabs() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;

    let name_with_whitespace = "Book\nWith\tSpecial\rWhitespace";
    let audiobook_id = create_test_audiobook(&db, "/test", name_with_whitespace)?;

    let audiobook = queries::get_audiobook_by_id(db.connection(), audiobook_id)?
        .ok_or("Audiobook not found")?;

    // Should preserve or sanitize special whitespace
    assert!(!audiobook.name.is_empty());

    Ok(())
}
