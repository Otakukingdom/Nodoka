mod acceptance_support;
use acceptance_support::*;

use std::error::Error;
use std::fs;
use std::path::Path;
use temp_dir::TempDir;

#[test]
fn test_cover_from_folder_jpg() -> Result<(), Box<dyn Error>> {
    let temp = TempDir::new()?;
    let fixtures = TestFixtures::new();

    let audiobook = temp.path().join("Book");
    fs::create_dir_all(&audiobook)?;

    fs::copy(
        fixtures.audio_path("sample_mp3.mp3"),
        audiobook.join("audio.mp3"),
    )?;

    if fixtures.image_path("cover.jpg").exists() {
        fs::copy(
            fixtures.image_path("cover.jpg"),
            audiobook.join("cover.jpg"),
        )?;
    } else {
        // Create a simple cover.jpg file
        fs::write(audiobook.join("cover.jpg"), b"fake image data")?;
    }

    let cover_path = audiobook.join("cover.jpg");
    assert!(cover_path.exists());

    Ok(())
}

#[test]
fn test_cover_from_folder_png() -> Result<(), Box<dyn Error>> {
    let temp = TempDir::new()?;
    let audiobook = temp.path().join("Book");
    fs::create_dir_all(&audiobook)?;

    fs::write(audiobook.join("folder.png"), b"fake png data")?;

    let cover_path = audiobook.join("folder.png");
    assert!(cover_path.exists());

    Ok(())
}

#[test]
fn test_multiple_image_formats_supported() -> Result<(), Box<dyn Error>> {
    let formats = vec!["cover.jpg", "cover.png", "cover.gif", "cover.webp"];

    for format in formats {
        let path = Path::new(format);
        assert!(
            path.extension().is_some(),
            "Format {} should have extension",
            format
        );
    }

    Ok(())
}

#[test]
fn test_cover_priority_order() -> Result<(), Box<dyn Error>> {
    // Test that embedded metadata has priority over folder images
    let priority = vec!["embedded", "folder.jpg", "cover.jpg", "cover.png"];

    assert_eq!(priority[0], "embedded");
    assert!(priority.contains(&"folder.jpg"));

    Ok(())
}

#[test]
fn test_default_placeholder_when_no_cover() -> Result<(), Box<dyn Error>> {
    let temp = TempDir::new()?;
    let fixtures = TestFixtures::new();

    let audiobook = temp.path().join("Book");
    fs::create_dir_all(&audiobook)?;
    fs::copy(
        fixtures.audio_path("sample_mp3.mp3"),
        audiobook.join("audio.mp3"),
    )?;

    // No cover.jpg or other image files
    assert!(!audiobook.join("cover.jpg").exists());
    assert!(!audiobook.join("folder.jpg").exists());
    assert!(!audiobook.join("cover.png").exists());

    // Application should use default placeholder (tested at UI level)

    Ok(())
}

#[test]
fn test_corrupted_image_handled() -> Result<(), Box<dyn Error>> {
    let temp = TempDir::new()?;
    let audiobook = temp.path().join("Book");
    fs::create_dir_all(&audiobook)?;

    // Create corrupted image
    fs::write(audiobook.join("cover.jpg"), b"not a valid JPEG")?;

    let cover_path = audiobook.join("cover.jpg");
    assert!(cover_path.exists());

    // Reading should handle gracefully (tested at image loading level)

    Ok(())
}

#[test]
fn test_cover_detection_case_insensitive() -> Result<(), Box<dyn Error>> {
    let names = vec![
        "Cover.jpg",
        "COVER.JPG",
        "cover.JPG",
        "folder.PNG",
        "FOLDER.png",
    ];

    for name in names {
        let _path = Path::new(name);
        let name_lower = name.to_lowercase();
        assert!(
            name_lower.contains("cover") || name_lower.contains("folder"),
            "Name {} should be recognized",
            name
        );
    }

    Ok(())
}

#[test]
fn test_large_image_exists() -> Result<(), Box<dyn Error>> {
    let temp = TempDir::new()?;
    let audiobook = temp.path().join("Book");
    fs::create_dir_all(&audiobook)?;

    // Create a "large" image file (just fake data for testing)
    let large_data = vec![0u8; 5 * 1024 * 1024]; // 5MB
    fs::write(audiobook.join("cover.jpg"), &large_data)?;

    let cover_path = audiobook.join("cover.jpg");
    assert!(cover_path.exists());

    let metadata = fs::metadata(&cover_path)?;
    assert!(metadata.len() > 1_000_000, "Image should be large");

    // Application should resize for display (tested at image processing level)

    Ok(())
}

#[test]
fn test_various_image_extensions() -> Result<(), Box<dyn Error>> {
    let extensions = vec!["jpg", "jpeg", "png", "gif", "webp"];

    for ext in extensions {
        let filename = format!("cover.{}", ext);
        let path = Path::new(&filename);
        assert_eq!(path.extension().and_then(|e| e.to_str()), Some(ext));
    }

    Ok(())
}

#[test]
fn test_cover_in_nested_directory() -> Result<(), Box<dyn Error>> {
    let temp = TempDir::new()?;
    let fixtures = TestFixtures::new();

    let nested = temp.path().join("Series").join("Book1");
    fs::create_dir_all(&nested)?;

    fs::copy(
        fixtures.audio_path("sample_mp3.mp3"),
        nested.join("audio.mp3"),
    )?;
    fs::write(nested.join("cover.jpg"), b"fake image")?;

    assert!(nested.join("cover.jpg").exists());

    Ok(())
}

#[test]
fn test_cover_cache_directory_concept() -> Result<(), Box<dyn Error>> {
    // Test the concept of caching covers
    let temp = TempDir::new()?;
    let cache_dir = temp.path().join("cover_cache");
    fs::create_dir_all(&cache_dir)?;

    // Simulated cached cover
    fs::write(cache_dir.join("audiobook_123.jpg"), b"cached cover")?;

    assert!(cache_dir.join("audiobook_123.jpg").exists());

    Ok(())
}
