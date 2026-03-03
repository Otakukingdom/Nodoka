//! Dependency management for screenshot generation.
//!
//! Handles npm dependencies and Playwright browser installation
//! with freshness detection to avoid unnecessary reinstalls.

use anyhow::{bail, Context, Result};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Instant;

/// File storing the hash of package-lock.json from last install
const LOCK_HASH_FILE: &str = ".screenshot-lock-hash";

/// Check if npm install is needed and run it if necessary
pub fn ensure_npm_dependencies(ui_dir: &Path) -> Result<()> {
    println!("├── Checking node_modules freshness...");

    if needs_npm_install(ui_dir)? {
        println!("│   └── [Stale] Running npm install...");
        let start = Instant::now();
        run_npm_install(ui_dir)?;
        println!(
            "│       └── [OK] Dependencies installed ({:.1}s)",
            start.elapsed().as_secs_f64()
        );

        // Store hash after successful install
        store_lock_hash(ui_dir)?;
    } else {
        println!("│   └── [OK] Dependencies up to date");
    }

    Ok(())
}

/// Check if Playwright Chromium is installed and install if needed
pub fn ensure_playwright_browser(ui_dir: &Path) -> Result<()> {
    println!("├── Checking Playwright browsers...");

    if !is_chromium_installed()? {
        println!("│   └── [Missing] Installing Chromium...");
        let start = Instant::now();
        run_playwright_install(ui_dir)?;
        println!(
            "│       └── [OK] Chromium installed ({:.1}s)",
            start.elapsed().as_secs_f64()
        );
    } else {
        println!("│   └── [OK] Chromium already installed");
    }

    Ok(())
}

fn needs_npm_install(ui_dir: &Path) -> Result<bool> {
    let node_modules = ui_dir.join("node_modules");
    let lock_file = ui_dir.join("package-lock.json");
    let hash_file = node_modules.join(LOCK_HASH_FILE);

    // node_modules doesn't exist
    if !node_modules.exists() {
        return Ok(true);
    }

    // package-lock.json doesn't exist (unusual but handle it)
    if !lock_file.exists() {
        return Ok(true);
    }

    // Hash file doesn't exist - need to check against actual
    let current_hash = hash_file_contents(&lock_file)?;
    if !hash_file.exists() {
        return Ok(true);
    }

    // Compare hashes
    let stored_hash = fs::read_to_string(&hash_file).unwrap_or_default();
    Ok(stored_hash != current_hash)
}

fn hash_file_contents(path: &Path) -> Result<String> {
    let contents = fs::read(path).context("Failed to read file for hashing")?;
    let mut hasher = Sha256::new();
    hasher.update(&contents);
    Ok(format!("{:x}", hasher.finalize()))
}

fn store_lock_hash(ui_dir: &Path) -> Result<()> {
    let lock_file = ui_dir.join("package-lock.json");
    let hash_file = ui_dir.join("node_modules").join(LOCK_HASH_FILE);
    let hash = hash_file_contents(&lock_file)?;
    fs::write(&hash_file, &hash).context("Failed to write lock hash file")?;
    Ok(())
}

fn run_npm_install(ui_dir: &Path) -> Result<()> {
    let status = Command::new("npm")
        .arg("install")
        .current_dir(ui_dir)
        .status()
        .context("Failed to run npm install")?;

    if !status.success() {
        bail!(
            "npm install failed with status: {}.\n\
             Check network connection and try running 'npm install' manually.",
            status
        );
    }

    Ok(())
}

fn is_chromium_installed() -> Result<bool> {
    let cache_dir = get_playwright_cache_dir()?;
    let chromium_dir = cache_dir.join("chromium");

    // Check if chromium directory exists and has content
    if !chromium_dir.exists() {
        return Ok(false);
    }

    // Check for actual browser executable
    #[cfg(target_os = "macos")]
    let executable = chromium_dir
        .join("chrome-mac")
        .join("Chromium.app");
    #[cfg(target_os = "linux")]
    let executable = chromium_dir.join("chrome-linux").join("chrome");
    #[cfg(target_os = "windows")]
    let executable = chromium_dir.join("chrome-win").join("chrome.exe");

    Ok(executable.exists())
}

#[cfg(target_os = "macos")]
fn get_playwright_cache_dir() -> Result<PathBuf> {
    dirs::home_dir()
        .map(|h| h.join("Library").join("Caches").join("ms-playwright"))
        .context("Could not find home directory")
}

#[cfg(target_os = "linux")]
fn get_playwright_cache_dir() -> Result<PathBuf> {
    dirs::home_dir()
        .map(|h| h.join(".cache").join("ms-playwright"))
        .context("Could not find home directory")
}

#[cfg(target_os = "windows")]
fn get_playwright_cache_dir() -> Result<PathBuf> {
    std::env::var("LOCALAPPDATA")
        .map(PathBuf::from)
        .map(|p| p.join("ms-playwright"))
        .context("Could not find LOCALAPPDATA environment variable")
}

fn run_playwright_install(ui_dir: &Path) -> Result<()> {
    let status = Command::new("npx")
        .args(["playwright", "install", "chromium"])
        .current_dir(ui_dir)
        .status()
        .context("Failed to run playwright install")?;

    if !status.success() {
        bail!(
            "Playwright browser install failed with status: {}.\n\
             Check network connection.",
            status
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_hash_file_contents() {
        let mut file = NamedTempFile::new().unwrap();
        write!(file, "test content").unwrap();

        let hash1 = hash_file_contents(file.path()).unwrap();
        let hash2 = hash_file_contents(file.path()).unwrap();

        // Same content should produce same hash
        assert_eq!(hash1, hash2);
        assert!(!hash1.is_empty());
    }

    #[test]
    fn test_hash_different_contents() {
        let mut file1 = NamedTempFile::new().unwrap();
        let mut file2 = NamedTempFile::new().unwrap();
        write!(file1, "content 1").unwrap();
        write!(file2, "content 2").unwrap();

        let hash1 = hash_file_contents(file1.path()).unwrap();
        let hash2 = hash_file_contents(file2.path()).unwrap();

        // Different content should produce different hash
        assert_ne!(hash1, hash2);
    }
}
