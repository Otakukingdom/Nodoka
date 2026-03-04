//! Lock file management for preventing concurrent screenshot runs.
//!
//! Uses RAII pattern to ensure lock is released even on panic or interrupt.

use anyhow::{bail, Context, Result};
use std::fs;
use std::io::{Read, Write};
use std::path::PathBuf;

/// Lock file for preventing concurrent runs
pub struct LockFile {
    path: PathBuf,
}

impl LockFile {
    /// Attempt to acquire the lock file
    ///
    /// Fails if another valid lock exists (process still running).
    /// Removes stale locks automatically (process no longer running).
    pub fn acquire(ui_dir: &std::path::Path) -> Result<Self> {
        let path = ui_dir.join(".screenshot.lock");

        println!("├── Checking for existing lock...");

        if path.exists() {
            match Self::check_existing_lock(&path) {
                Ok(()) => {
                    // Stale lock, already removed
                    println!("│   └── [Stale] Removed old lock file");
                }
                Err(e) => {
                    // Active lock exists
                    bail!("{}", e);
                }
            }
        }

        // Create new lock with our PID
        let pid = std::process::id();
        let mut file = fs::File::create(&path)
            .with_context(|| format!("Failed to create lock file at {}", path.display()))?;
        write!(file, "{}", pid)?;

        // Register lock path for emergency cleanup
        super::state::register_lock_path(path.clone());

        println!("│   └── [OK] Lock acquired (PID {})", pid);

        Ok(Self { path })
    }

    fn check_existing_lock(path: &std::path::Path) -> Result<()> {
        let mut file =
            fs::File::open(path).context("Failed to open existing lock file")?;

        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .context("Failed to read lock file")?;

        let pid: u32 = contents
            .trim()
            .parse()
            .context("Invalid PID in lock file")?;

        if is_process_running(pid)? {
            bail!(
                "Screenshot generation already running (PID {}). \
                 Wait for it to complete or remove {} if stale.",
                pid,
                path.display()
            );
        }

        // Stale lock - remove it
        fs::remove_file(path).context("Failed to remove stale lock file")?;

        Ok(())
    }

    /// Explicitly release the lock
    #[allow(dead_code)]
    pub fn release(self) {
        drop(self);
    }
}

impl Drop for LockFile {
    fn drop(&mut self) {
        // Best effort cleanup - don't panic in destructor
        let _ = fs::remove_file(&self.path);
    }
}

#[cfg(unix)]
fn is_process_running(pid: u32) -> Result<bool> {
    // SAFETY: kill with signal 0 just checks if process exists
    unsafe {
        let result = libc::kill(pid as i32, 0);
        Ok(result == 0)
    }
}

#[cfg(windows)]
fn is_process_running(pid: u32) -> Result<bool> {
    let output = Command::new("tasklist")
        .args(["/FI", &format!("PID eq {}", pid)])
        .output()
        .context("Failed to run tasklist")?;

    let output_str = String::from_utf8_lossy(&output.stdout);
    Ok(output_str.contains(&pid.to_string()))
}

#[cfg(windows)]
use std::process::Command;

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_lock_file_creates_on_acquire() {
        let temp = TempDir::new().unwrap();
        let ui_dir = temp.path();
        let lock_path = ui_dir.join(".screenshot.lock");

        assert!(!lock_path.exists());

        let lock = LockFile::acquire(ui_dir).unwrap();

        assert!(lock_path.exists());
        let contents = std::fs::read_to_string(&lock_path).unwrap();
        assert!(!contents.is_empty());

        drop(lock);
        assert!(!lock_path.exists());
    }

    #[test]
    fn test_stale_lock_removed() {
        let temp = TempDir::new().unwrap();
        let ui_dir = temp.path();
        let lock_path = ui_dir.join(".screenshot.lock");

        // Create stale lock with non-existent PID (use a very high PID that's unlikely to exist)
        std::fs::write(&lock_path, "99999999").unwrap();
        assert!(lock_path.exists());

        let lock = LockFile::acquire(ui_dir).unwrap();

        // Should have replaced with our PID
        let contents = std::fs::read_to_string(&lock_path).unwrap();
        assert_ne!(contents, "99999999");

        drop(lock);
        assert!(!lock_path.exists());
    }

    #[test]
    fn test_lock_raii_cleanup() {
        let temp = TempDir::new().unwrap();
        let ui_dir = temp.path();
        let lock_path = ui_dir.join(".screenshot.lock");

        {
            let _lock = LockFile::acquire(ui_dir).unwrap();
            assert!(lock_path.exists());
        }

        // Lock should be released when dropped
        assert!(!lock_path.exists());
    }

    #[test]
    fn test_lock_release_explicit() {
        let temp = TempDir::new().unwrap();
        let ui_dir = temp.path();
        let lock_path = ui_dir.join(".screenshot.lock");

        let lock = LockFile::acquire(ui_dir).unwrap();
        assert!(lock_path.exists());

        lock.release();

        assert!(!lock_path.exists());
    }

    #[test]
    fn test_lock_contents_is_valid_pid() {
        let temp = TempDir::new().unwrap();
        let ui_dir = temp.path();
        let lock_path = ui_dir.join(".screenshot.lock");

        let _lock = LockFile::acquire(ui_dir).unwrap();

        let contents = std::fs::read_to_string(&lock_path).unwrap();
        let parsed_pid: u32 = contents.trim().parse().expect("Lock should contain valid PID");
        assert!(parsed_pid > 0);
    }
}
