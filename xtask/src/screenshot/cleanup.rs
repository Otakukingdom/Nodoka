//! Cleanup operations for screenshot generation.
//!
//! Handles stale processes, port conflicts, and artifact cleanup
//! to ensure a clean environment for each run.

use anyhow::{Context, Result};
use std::fs;
use std::path::Path;
use std::process::Command;

#[cfg(windows)]
use anyhow::bail;

/// Vite dev server port
const DEV_SERVER_PORT: u16 = 5173;

/// Kill any process using port 5173
pub fn kill_stale_dev_server() -> Result<()> {
    println!("├── Checking port {}...", DEV_SERVER_PORT);

    if let Some(pid) = find_process_on_port(DEV_SERVER_PORT)? {
        println!("│   └── [In use by PID {}] Killing process...", pid);
        kill_process_tree(pid)?;
        println!("│       └── [OK] Process terminated");
    } else {
        println!("│   └── [OK] Port is free");
    }

    Ok(())
}

/// Kill orphaned Playwright/Chromium processes
pub fn kill_orphaned_playwright_processes() -> Result<()> {
    println!("├── Checking for orphaned Playwright processes...");

    let pids = find_playwright_processes()?;
    if !pids.is_empty() {
        println!(
            "│   └── [Found {}] Killing orphaned processes...",
            pids.len()
        );
        for pid in pids {
            let _ = kill_process(pid);
        }
        println!("│       └── [OK] Orphaned processes killed");
    } else {
        println!("│   └── [OK] No orphaned processes found");
    }

    Ok(())
}

/// Remove previous run artifacts
pub fn cleanup_previous_artifacts(project_root: &Path, ui_dir: &Path) -> Result<()> {
    println!("├── Removing previous artifacts...");

    let artifacts = [
        project_root.join("screenshots"),
        ui_dir.join("test-results-screenshots"),
        ui_dir.join("playwright-report-screenshots"),
        ui_dir.join("e2e-screenshots"),
    ];

    for artifact in &artifacts {
        if artifact.exists() {
            fs::remove_dir_all(artifact)
                .with_context(|| format!("Failed to remove {}", artifact.display()))?;
        }
    }

    // Remove partial screenshot files (*.png.tmp)
    remove_partial_files(&project_root.join("screenshots"))?;

    println!("│   └── [OK] Artifacts cleaned");
    Ok(())
}

fn remove_partial_files(dir: &Path) -> Result<()> {
    if !dir.exists() {
        return Ok(());
    }

    for entry in fs::read_dir(dir).context("Failed to read directory")? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            remove_partial_files(&path)?;
        } else if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
            if name.ends_with(".tmp") {
                let _ = fs::remove_file(&path);
            }
        }
    }

    Ok(())
}

// === Platform-specific implementations ===

#[cfg(unix)]
fn find_process_on_port(port: u16) -> Result<Option<u32>> {
    // Try lsof first (macOS)
    let output = Command::new("lsof")
        .args(["-i", &format!(":{}", port), "-t"])
        .output();

    if let Ok(output) = output {
        if output.status.success() && !output.stdout.is_empty() {
            let pid_str = String::from_utf8_lossy(&output.stdout);
            if let Some(first_line) = pid_str.trim().lines().next() {
                if let Ok(pid) = first_line.parse::<u32>() {
                    return Ok(Some(pid));
                }
            }
        }
    }

    // Try ss (Linux)
    let output = Command::new("ss")
        .args(["-tlnp", &format!("sport = :{}", port)])
        .output();

    if let Ok(output) = output {
        if output.status.success() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            // Parse PID from ss output: "pid=12345"
            for line in output_str.lines() {
                if let Some(start) = line.find("pid=") {
                    let pid_str = &line[start + 4..];
                    if let Some(end) = pid_str.find(',') {
                        if let Ok(pid) = pid_str[..end].parse::<u32>() {
                            return Ok(Some(pid));
                        }
                    }
                }
            }
        }
    }

    Ok(None)
}

#[cfg(windows)]
fn find_process_on_port(port: u16) -> Result<Option<u32>> {
    let output = Command::new("netstat")
        .args(["-ano"])
        .output()
        .context("Failed to run netstat")?;

    let output_str = String::from_utf8_lossy(&output.stdout);
    let port_str = format!(":{}", port);

    for line in output_str.lines() {
        if line.contains(&port_str) && line.contains("LISTENING") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if let Some(pid_str) = parts.last() {
                if let Ok(pid) = pid_str.parse::<u32>() {
                    return Ok(Some(pid));
                }
            }
        }
    }

    Ok(None)
}

#[cfg(unix)]
fn kill_process_tree(pid: u32) -> Result<()> {
    // Kill child processes first
    let _ = Command::new("pkill")
        .args(["-P", &pid.to_string()])
        .output();

    // Give them a moment
    std::thread::sleep(std::time::Duration::from_millis(100));

    // Kill the parent
    kill_process(pid)
}

#[cfg(windows)]
fn kill_process_tree(pid: u32) -> Result<()> {
    let output = Command::new("taskkill")
        .args(["/PID", &pid.to_string(), "/T", "/F"])
        .output();

    match output {
        Ok(o) if o.status.success() => Ok(()),
        _ => bail!("Failed to kill process tree for PID {}", pid),
    }
}

#[cfg(unix)]
fn kill_process(pid: u32) -> Result<()> {
    // SAFETY: libc::kill is a standard POSIX call
    unsafe {
        let result = libc::kill(pid as i32, libc::SIGTERM);
        if result != 0 {
            // Try SIGKILL if SIGTERM fails
            let _ = libc::kill(pid as i32, libc::SIGKILL);
        }
    }
    Ok(())
}

#[cfg(windows)]
fn kill_process(pid: u32) -> Result<()> {
    let output = Command::new("taskkill")
        .args(["/PID", &pid.to_string(), "/F"])
        .output();

    match output {
        Ok(o) if o.status.success() => Ok(()),
        _ => bail!("Failed to kill process PID {}", pid),
    }
}

#[cfg(unix)]
fn find_playwright_processes() -> Result<Vec<u32>> {
    let output = Command::new("pgrep")
        .args(["-f", "playwright|chromium"])
        .output();

    let mut pids = Vec::new();
    if let Ok(output) = output {
        if output.status.success() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            for line in output_str.lines() {
                if let Ok(pid) = line.trim().parse::<u32>() {
                    pids.push(pid);
                }
            }
        }
    }

    Ok(pids)
}

#[cfg(windows)]
fn find_playwright_processes() -> Result<Vec<u32>> {
    // Windows implementation would use tasklist with filtering
    // For simplicity, return empty - Windows handles process cleanup differently
    Ok(Vec::new())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_cleanup_previous_artifacts_nonexistent() {
        let temp = TempDir::new().unwrap();
        let ui_dir = temp.path().join("ui");

        // Should not fail even if artifacts don't exist
        cleanup_previous_artifacts(temp.path(), &ui_dir).unwrap();
    }

    #[test]
    fn test_cleanup_previous_artifacts_removes_screenshots() {
        let temp = TempDir::new().unwrap();
        let screenshots = temp.path().join("screenshots");
        fs::create_dir_all(&screenshots).unwrap();
        fs::write(screenshots.join("test.png"), "data").unwrap();

        let ui_dir = temp.path().join("ui");
        cleanup_previous_artifacts(temp.path(), &ui_dir).unwrap();

        assert!(!screenshots.exists());
    }

    #[test]
    fn test_cleanup_previous_artifacts_removes_test_results() {
        let temp = TempDir::new().unwrap();
        let ui_dir = temp.path().join("ui");
        let test_results = ui_dir.join("test-results-screenshots");
        fs::create_dir_all(&test_results).unwrap();
        fs::write(test_results.join("result.txt"), "data").unwrap();

        cleanup_previous_artifacts(temp.path(), &ui_dir).unwrap();

        assert!(!test_results.exists());
    }

    #[test]
    fn test_cleanup_previous_artifacts_removes_playwright_report() {
        let temp = TempDir::new().unwrap();
        let ui_dir = temp.path().join("ui");
        let report = ui_dir.join("playwright-report-screenshots");
        fs::create_dir_all(&report).unwrap();

        cleanup_previous_artifacts(temp.path(), &ui_dir).unwrap();

        assert!(!report.exists());
    }

    #[test]
    fn test_remove_partial_files() {
        let temp = TempDir::new().unwrap();
        let dir = temp.path().join("screenshots");
        fs::create_dir_all(&dir).unwrap();

        fs::write(dir.join("good.png"), "data").unwrap();
        fs::write(dir.join("partial.png.tmp"), "partial").unwrap();

        remove_partial_files(&dir).unwrap();

        assert!(dir.join("good.png").exists());
        assert!(!dir.join("partial.png.tmp").exists());
    }

    #[test]
    fn test_remove_partial_files_nested() {
        let temp = TempDir::new().unwrap();
        let nested = temp.path().join("screenshots").join("category");
        fs::create_dir_all(&nested).unwrap();

        fs::write(nested.join("partial.png.tmp"), "partial").unwrap();
        fs::write(nested.join("complete.png"), "data").unwrap();

        remove_partial_files(temp.path().join("screenshots").as_path()).unwrap();

        assert!(!nested.join("partial.png.tmp").exists());
        assert!(nested.join("complete.png").exists());
    }

    #[test]
    fn test_remove_partial_files_nonexistent_dir() {
        let temp = TempDir::new().unwrap();
        let nonexistent = temp.path().join("nonexistent");

        // Should not fail on nonexistent directory
        remove_partial_files(&nonexistent).unwrap();
    }

    #[test]
    fn test_remove_partial_files_multiple_tmp() {
        let temp = TempDir::new().unwrap();
        let dir = temp.path().join("screenshots");
        fs::create_dir_all(&dir).unwrap();

        fs::write(dir.join("file1.png.tmp"), "partial").unwrap();
        fs::write(dir.join("file2.png.tmp"), "partial").unwrap();
        fs::write(dir.join("file3.txt.tmp"), "partial").unwrap();
        fs::write(dir.join("good.png"), "data").unwrap();

        remove_partial_files(&dir).unwrap();

        assert!(!dir.join("file1.png.tmp").exists());
        assert!(!dir.join("file2.png.tmp").exists());
        assert!(!dir.join("file3.txt.tmp").exists());
        assert!(dir.join("good.png").exists());
    }
}
