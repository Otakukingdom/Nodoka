mod acceptance_support;
use acceptance_support::*;

use nodoka::cover_art::{select, Source};
use std::error::Error;
use std::fs;
use std::path::Path;
use temp_dir::TempDir;

fn write_test_jpeg(path: &Path) -> Result<(), Box<dyn Error>> {
    let img = image::RgbImage::from_pixel(2, 2, image::Rgb([255, 0, 0]));
    let dyn_img = image::DynamicImage::ImageRgb8(img);

    let mut bytes = Vec::new();
    let mut encoder = image::codecs::jpeg::JpegEncoder::new(&mut bytes);
    encoder.encode_image(&dyn_img)?;

    fs::write(path, bytes)?;
    Ok(())
}

#[test]
fn test_cover_priority_folder_jpg_over_cover_jpg() -> Result<(), Box<dyn Error>> {
    let temp = TempDir::new()?;
    let fixtures = TestFixtures::new();

    let audiobook = temp.path().join("Book");
    fs::create_dir_all(&audiobook)?;
    fs::copy(
        fixtures.audio_path("sample_mp3.mp3"),
        audiobook.join("audio.mp3"),
    )?;

    write_test_jpeg(&audiobook.join("folder.jpg"))?;
    write_test_jpeg(&audiobook.join("cover.jpg"))?;

    let selection = select(&audiobook, None)?.ok_or("No cover selected")?;
    let Source::File(path) = selection.source else {
        return Err("Expected file cover art".into());
    };
    assert_eq!(
        path.file_name()
            .and_then(|n| n.to_str())
            .ok_or("Bad name")?,
        "folder.jpg"
    );

    Ok(())
}

#[test]
fn test_cover_priority_embedded_over_folder_image() -> Result<(), Box<dyn Error>> {
    let temp = TempDir::new()?;
    let fixtures = TestFixtures::new();

    let audiobook = temp.path().join("Book");
    fs::create_dir_all(&audiobook)?;
    fs::copy(
        fixtures.audio_path("sample_mp3.mp3"),
        audiobook.join("audio.mp3"),
    )?;

    let folder_path = audiobook.join("folder.jpg");
    write_test_jpeg(&folder_path)?;
    let embedded = fs::read(&folder_path)?;

    let selection = select(&audiobook, Some(&embedded))?.ok_or("No cover selected")?;
    assert!(matches!(selection.source, Source::Embedded));

    Ok(())
}

#[test]
fn test_corrupted_folder_image_falls_back_to_cover_jpg() -> Result<(), Box<dyn Error>> {
    let temp = TempDir::new()?;
    let fixtures = TestFixtures::new();

    let audiobook = temp.path().join("Book");
    fs::create_dir_all(&audiobook)?;
    fs::copy(
        fixtures.audio_path("sample_mp3.mp3"),
        audiobook.join("audio.mp3"),
    )?;

    fs::write(audiobook.join("folder.jpg"), b"not a valid jpeg")?;
    write_test_jpeg(&audiobook.join("cover.jpg"))?;

    let selection = select(&audiobook, None)?.ok_or("No cover selected")?;
    let Source::File(path) = selection.source else {
        return Err("Expected file cover art".into());
    };
    assert_eq!(
        path.file_name()
            .and_then(|n| n.to_str())
            .ok_or("Bad name")?,
        "cover.jpg"
    );

    Ok(())
}

#[test]
fn test_cover_detection_is_case_insensitive() -> Result<(), Box<dyn Error>> {
    let temp = TempDir::new()?;
    let fixtures = TestFixtures::new();

    let audiobook = temp.path().join("Book");
    fs::create_dir_all(&audiobook)?;
    fs::copy(
        fixtures.audio_path("sample_mp3.mp3"),
        audiobook.join("audio.mp3"),
    )?;

    write_test_jpeg(&audiobook.join("COVER.JPG"))?;

    let selection = select(&audiobook, None)?.ok_or("No cover selected")?;
    let Source::File(path) = selection.source else {
        return Err("Expected file cover art".into());
    };
    assert_eq!(
        path.file_name()
            .and_then(|n| n.to_str())
            .ok_or("Bad name")?,
        "COVER.JPG"
    );

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

    let selection = select(&audiobook, None)?;
    assert!(selection.is_none());

    Ok(())
}
