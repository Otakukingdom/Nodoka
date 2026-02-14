mod acceptance_support;
use acceptance_support::*;

use nodoka::cover_art::{select, Source};
use nodoka::cover_cache;
use nodoka::player::Scanner;
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

fn create_test_png_bytes() -> Result<Vec<u8>, Box<dyn Error>> {
    let img = image::RgbImage::from_pixel(2, 2, image::Rgb([9, 8, 7]));
    let dyn_img = image::DynamicImage::ImageRgb8(img);
    let mut bytes = Vec::new();
    dyn_img.write_to(
        &mut std::io::Cursor::new(&mut bytes),
        image::ImageOutputFormat::Png,
    )?;
    Ok(bytes)
}

fn write_id3v23_mp3_with_apic(
    path: &Path,
    cover_png: &[u8],
    base_audio: &[u8],
) -> Result<(), Box<dyn Error>> {
    let apic = {
        let mut body = Vec::new();
        body.push(0);
        body.extend_from_slice(b"image/png\0");
        body.push(0x03);
        body.push(0);
        body.extend_from_slice(cover_png);

        let mut out = Vec::new();
        out.extend_from_slice(b"APIC");
        out.extend_from_slice(&(u32::try_from(body.len()).unwrap_or(0)).to_be_bytes());
        out.extend_from_slice(&[0, 0]);
        out.extend_from_slice(&body);
        out
    };

    let tag_size = u32::try_from(apic.len()).map_err(|_| "tag too large")?;
    let mut out = Vec::new();
    out.extend_from_slice(b"ID3");
    out.push(3);
    out.push(0);
    out.push(0);
    out.extend_from_slice(&[
        ((tag_size >> 21) & 0x7F) as u8,
        ((tag_size >> 14) & 0x7F) as u8,
        ((tag_size >> 7) & 0x7F) as u8,
        (tag_size & 0x7F) as u8,
    ]);
    out.extend_from_slice(&apic);
    out.extend_from_slice(base_audio);
    fs::write(path, out)?;
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
fn test_cover_cache_extracts_embedded_id3_apic_end_to_end() -> Result<(), Box<dyn Error>> {
    // Skip if VLC scanner is unavailable on this machine.
    if Scanner::new().is_err() {
        return Ok(());
    }

    let temp = TempDir::new()?;
    let fixtures = TestFixtures::new();
    let base_mp3 = fixtures.audio_path("sample_mp3.mp3");
    if !base_mp3.exists() {
        return Ok(());
    }
    let base_bytes = fs::read(&base_mp3)?;

    let audiobook = temp.path().join("Book");
    fs::create_dir_all(&audiobook)?;

    let cover_png = create_test_png_bytes()?;
    let mp3_path = audiobook.join("audio.mp3");
    write_id3v23_mp3_with_apic(&mp3_path, &cover_png, &base_bytes)?;

    let thumb = cover_cache::ensure_cover_thumbnail(99, &audiobook)?;
    assert!(thumb.is_some(), "expected embedded cover thumbnail");
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
