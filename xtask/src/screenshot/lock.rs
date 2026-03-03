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
