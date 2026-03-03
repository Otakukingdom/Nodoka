//! Pre-flight environment validation for screenshot generation.
//!
//! Validates the environment before any setup operations to fail fast
//! with actionable error messages.

use anyhow::{bail, Context, Result};
use std::path::Path;
use std::process::Command;

/// Minimum required Node.js major version
const MIN_NODE_VERSION: u32 = 18;

/// Minimum required disk space in bytes (500 MB)
const MIN_DISK_SPACE_BYTES: u64 = 500 * 1024 * 1024;

/// Run all pre-flight checks, returning on first failure
pub fn run_preflight_checks(project_root: &Path) -> Result<()> {
    check_working_directory(project_root)?;
    check_nodejs()?;
    check_npm()?;
    check_disk_space(project_root)?;

    Ok(())
}

fn check_working_directory(project_root: &Path) -> Result<()> {
    let ui_dir = project_root.join("ui");
    let package_json = ui_dir.join("package.json");

    if !ui_dir.exists() {
        bail!(
            "Must run from project root (ui/ directory not found).\n\
             Current directory: {}",
            project_root.display()
        );
    }

    if !package_json.exists() {
        bail!("ui/package.json not found. Ensure the project is properly set up.");
    }

    println!("├── Checking working directory...");
    println!("│   └── [OK] ui/ directory found");

    Ok(())
}

fn check_nodejs() -> Result<()> {
    println!("├── Checking Node.js...");

    let output = Command::new("node")
        .arg("--version")
        .output()
        .context("Failed to execute node --version")?;

    if !output.status.success() {
        bail!("Node.js not found. Install from https://nodejs.org");
    }

    let version_str = String::from_utf8_lossy(&output.stdout);
    let version = parse_node_version(&version_str)?;

    if version < MIN_NODE_VERSION {
        bail!(
            "Node.js {}+ required, found v{}.x.x\n\
             Upgrade at https://nodejs.org",
            MIN_NODE_VERSION,
            version
        );
    }

    println!("│   └── [OK] Node.js v{} found", version);

    Ok(())
}

fn parse_node_version(version_str: &str) -> Result<u32> {
    // Parse "v20.11.0" -> 20
    let trimmed = version_str.trim().trim_start_matches('v');
    let major: u32 = trimmed
        .split('.')
        .next()
        .context("Invalid version format")?
        .parse()
        .context("Failed to parse version number")?;
    Ok(major)
}

fn check_npm() -> Result<()> {
    println!("├── Checking npm...");

    let output = Command::new("npm")
        .arg("--version")
        .output()
        .context("Failed to execute npm --version")?;

    if !output.status.success() {
        bail!("npm not found. Reinstall Node.js from https://nodejs.org");
    }

    let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
    println!("│   └── [OK] npm {} found", version);

    Ok(())
}

fn check_disk_space(project_root: &Path) -> Result<()> {
    println!("├── Checking disk space...");

    let available = get_available_disk_space(project_root)?;
    let available_mb = available / (1024 * 1024);

    if available < MIN_DISK_SPACE_BYTES {
        bail!(
            "Insufficient disk space. Need at least 500 MB, have {} MB available.\n\
             Free up disk space and try again.",
            available_mb
        );
    }

    println!("│   └── [OK] {} MB available", available_mb);

    Ok(())
}

#[cfg(unix)]
fn get_available_disk_space(path: &Path) -> Result<u64> {
    use std::mem::MaybeUninit;

    let path_str = path.to_string_lossy();
    let path_cstr = std::ffi::CString::new(path_str.as_ref())?;

    unsafe {
        let mut statvfs: libc::statvfs = MaybeUninit::zeroed().assume_init();
        if libc::statvfs(path_cstr.as_ptr(), &mut statvfs) != 0 {
            bail!("Failed to get disk space information");
        }
        Ok(u64::from(statvfs.f_bavail) * statvfs.f_frsize)
    }
}

#[cfg(windows)]
fn get_available_disk_space(path: &Path) -> Result<u64> {
    // On Windows, use the root of the drive
    let root = path
        .ancestors()
        .last()
        .context("Failed to get drive root")?;

    let output = Command::new("wmic")
        .args([
            "logicaldisk",
            "where",
            &format!("DeviceID='{}'", root.display()),
            "get",
            "FreeSpace",
        ])
        .output()
        .context("Failed to run wmic")?;

    let output_str = String::from_utf8_lossy(&output.stdout);
    // Parse output: "FreeSpace\n123456789\n"
    for line in output_str.lines().skip(1) {
        if let Ok(bytes) = line.trim().parse::<u64>() {
            return Ok(bytes);
        }
    }

    bail!("Failed to parse disk space from wmic output")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_node_version() {
        assert_eq!(parse_node_version("v20.11.0").unwrap(), 20);
        assert_eq!(parse_node_version("v18.0.0").unwrap(), 18);
        assert_eq!(parse_node_version("v22.1.2").unwrap(), 22);
        assert_eq!(parse_node_version("20.11.0").unwrap(), 20);
    }

    #[test]
    fn test_parse_node_version_invalid() {
        assert!(parse_node_version("").is_err());
        assert!(parse_node_version("invalid").is_err());
    }
}
