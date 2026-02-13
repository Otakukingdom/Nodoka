use sha2::{Digest, Sha256};
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

/// Calculates SHA-256 checksum of a file
///
/// # Errors
///
/// Returns an error if the file cannot be read
pub async fn calculate_checksum(path: &Path) -> Result<String, std::io::Error> {
    let mut file = File::open(path).await?;
    let mut hasher = Sha256::new();
    let mut buffer = vec![0u8; 8192];

    loop {
        let n = file.read(&mut buffer).await?;
        if n == 0 {
            break;
        }
        if let Some(slice) = buffer.get(..n) {
            hasher.update(slice);
        }
    }

    let result = hasher.finalize();
    Ok(format!("{result:x}"))
}
