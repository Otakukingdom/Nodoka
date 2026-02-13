use nodoka::tasks::calculate_checksum;
use std::fs;
use std::io::Write;
use std::{error::Error, path::PathBuf};
use temp_dir::TempDir;

#[tokio::test]
async fn test_checksum_calculation() -> Result<(), Box<dyn Error>> {
    // Create temporary file
    let temp_dir = TempDir::new()?;
    let file_path = temp_dir.path().join("test.txt");

    let mut file = fs::File::create(&file_path)?;
    file.write_all(b"Hello, World!")?;
    drop(file);

    // Calculate checksum
    let checksum = calculate_checksum(&file_path).await?;

    // Verify checksum (SHA-256 of "Hello, World!")
    assert_eq!(
        checksum,
        "dffd6021bb2bd5b0af676290809ec3a53191dd81c7f70a4b28688a362182986f"
    );
    Ok(())
}

#[tokio::test]
async fn test_checksum_nonexistent_file() {
    let result = calculate_checksum(&PathBuf::from("/nonexistent/file.txt")).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_checksum_empty_file() -> Result<(), Box<dyn Error>> {
    let temp_dir = TempDir::new()?;
    let file_path = temp_dir.path().join("empty.txt");

    fs::File::create(&file_path)?;

    let checksum = calculate_checksum(&file_path).await?;

    // SHA-256 of empty string
    assert_eq!(
        checksum,
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    );
    Ok(())
}

#[tokio::test]
async fn test_checksum_large_file() -> Result<(), Box<dyn Error>> {
    let temp_dir = TempDir::new()?;
    let file_path = temp_dir.path().join("large.txt");

    let mut file = fs::File::create(&file_path)?;
    // Write 1MB of data
    let data = vec![b'A'; 1024 * 1024];
    file.write_all(&data)?;
    drop(file);

    let checksum = calculate_checksum(&file_path).await?;

    // Verify it completes without error and returns a valid SHA-256 hash
    assert_eq!(checksum.len(), 64);
    assert!(checksum.chars().all(|c| c.is_ascii_hexdigit()));
    Ok(())
}
