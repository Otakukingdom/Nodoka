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
async fn test_scan_root_with_audio_files_is_detected_as_audiobook() -> Result<(), Box<dyn Error>> {
    let temp = TempDir::new()?;
    let fixtures = TestFixtures::new();

    // Place an audio file directly in the scan root.
    fs::copy(
        fixtures.audio_path("sample_mp3.mp3"),
        temp.path().join("chapter01.mp3"),
    )?;

    let discovered = scan_directory(temp.path().to_path_buf()).await?;

    assert_eq!(discovered.len(), 1);
    let book = discovered.first().ok_or("No audiobook discovered")?;
    assert_eq!(book.path, temp.path().to_path_buf());
    assert_eq!(book.files.len(), 1);

    let expected_name = temp
        .path()
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or("Invalid scan root name")?;
    assert_eq!(book.name, expected_name);

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
async fn test_scan_directory_skips_unreadable_entries() -> Result<(), Box<dyn Error>> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        let temp = TempDir::new()?;
        let fixtures = TestFixtures::new();

        let book_dir = temp.path().join("Book");
        fs::create_dir_all(&book_dir)?;
        fs::copy(
            fixtures.audio_path("sample_mp3.mp3"),
            book_dir.join("chapter1.mp3"),
        )?;

        let unreadable = temp.path().join("unreadable");
        fs::create_dir_all(&unreadable)?;
        fs::set_permissions(&unreadable, fs::Permissions::from_mode(0o000))?;

        let discovered = scan_directory(temp.path().to_path_buf()).await?;
        assert!(
            discovered.iter().any(|b| b.path == book_dir),
            "expected scan to succeed and include readable audiobook"
        );

        // Restore permissions so TempDir cleanup can remove it.
        let _ = fs::set_permissions(&unreadable, fs::Permissions::from_mode(0o700));
    }

    Ok(())
}

#[tokio::test]
async fn test_symbolic_links_handling() -> Result<(), Box<dyn Error>> {
    #[cfg(unix)]
    {
        let scan_root = TempDir::new()?;
        let external = TempDir::new()?;
        let fixtures = TestFixtures::new();

        // Create an audiobook *outside* the scan root, reachable only via symlink.
        let external_books = external.path().join("external_audiobooks");
        fs::create_dir_all(&external_books)?;
        let book = external_books.join("TestBook");
        fs::create_dir_all(&book)?;
        fs::copy(
            fixtures.audio_path("sample_mp3.mp3"),
            book.join("chapter1.mp3"),
        )?;

        // Create symbolic link inside scan root to the external directory.
        let link_dir = scan_root.path().join("linked_audiobooks");
        #[cfg(unix)]
        std::os::unix::fs::symlink(&external_books, &link_dir)?;

        // Scanning should handle symlinks without infinite loops
        let discovered = scan_directory(scan_root.path().to_path_buf()).await?;

        assert!(
            discovered.iter().any(|b| b.name == "TestBook"),
            "expected symlinked audiobook to be discovered"
        );
    }

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
        .name
        .contains("Chapter 1.mp3"));
    assert!(first_book
        .files
        .get(1)
        .ok_or("No file at index 1")?
        .name
        .contains("Chapter 2.mp3"));
    assert!(first_book
        .files
        .get(2)
        .ok_or("No file at index 2")?
        .name
        .contains("Chapter 10.mp3"));
    assert!(first_book
        .files
        .get(3)
        .ok_or("No file at index 3")?
        .name
        .contains("Chapter 20.mp3"));

    Ok(())
}
