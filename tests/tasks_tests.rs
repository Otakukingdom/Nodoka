use nodoka::tasks::calculate_checksum;
use std::fs;
use std::io::Write;
use temp_dir::TempDir;

#[tokio::test]
async fn test_checksum_calculation() {
    // Create temporary file
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let file_path = temp_dir.path().join("test.txt");
    
    let mut file = fs::File::create(&file_path).expect("Failed to create file");
    file.write_all(b"Hello, World!").expect("Failed to write to file");
    drop(file);

    // Calculate checksum
    let checksum = calculate_checksum(&file_path)
        .await
        .expect("Failed to calculate checksum");

    // Verify checksum (SHA-256 of "Hello, World!")
    assert_eq!(
        checksum,
        "dffd6021bb2bd5b0af676290809ec3a53191dd81c7f70a4b28688a362182986f"
    );
}

#[tokio::test]
async fn test_checksum_nonexistent_file() {
    let result = calculate_checksum(&std::path::PathBuf::from("/nonexistent/file.txt")).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_checksum_empty_file() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let file_path = temp_dir.path().join("empty.txt");
    
    fs::File::create(&file_path).expect("Failed to create file");

    let checksum = calculate_checksum(&file_path)
        .await
        .expect("Failed to calculate checksum");

    // SHA-256 of empty string
    assert_eq!(
        checksum,
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    );
}

#[tokio::test]
async fn test_checksum_large_file() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let file_path = temp_dir.path().join("large.txt");
    
    let mut file = fs::File::create(&file_path).expect("Failed to create file");
    // Write 1MB of data
    let data = vec![b'A'; 1024 * 1024];
    file.write_all(&data).expect("Failed to write to file");
    drop(file);

    let checksum = calculate_checksum(&file_path)
        .await
        .expect("Failed to calculate checksum");

    // Verify it completes without error and returns a valid SHA-256 hash
    assert_eq!(checksum.len(), 64);
    assert!(checksum.chars().all(|c| c.is_ascii_hexdigit()));
}
