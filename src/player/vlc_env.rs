//! VLC environment configuration.
//!
//! This module handles platform-specific VLC environment setup,
//! ensuring the VLC library can locate its plugins directory.
//!
//! ## Platform Support
//!
//! - **macOS**: Detects VLC.app installation and sets plugin path
//! - **Linux**: Relies on system-wide VLC installation
//! - **Windows**: Detects VLC installation in Program Files
//!
//! ## Usage
//!
//! Call [`setup_vlc_environment()`] before creating any VLC instances:
//!
//! ```no_run
//! # use nodoka::player::setup_vlc_environment;
//! setup_vlc_environment();
//! ```

use std::env;
use std::path::{Path, PathBuf};

/// Sets up VLC environment variables for proper plugin loading.
///
/// This function detects the VLC installation on the system and configures
/// the `VLC_PLUGIN_PATH` environment variable if not already set.
///
/// ## Platform Behavior
///
/// - **macOS**: Checks for VLC.app in /Applications
/// - **Linux**: Assumes system-wide installation (no action needed)
/// - **Windows**: Checks common VLC installation paths
///
/// ## Notes
///
/// This function is idempotent - it only sets the environment variable
/// if it's not already configured.
pub fn setup_vlc_environment() {
    // Only set if not already configured
    if env::var("VLC_PLUGIN_PATH").is_ok() {
        tracing::debug!("VLC_PLUGIN_PATH already set");
        return;
    }

    if let Some(plugin_path) = detect_vlc_plugin_path() {
        tracing::info!("Setting VLC_PLUGIN_PATH to: {}", plugin_path.display());
        env::set_var("VLC_PLUGIN_PATH", plugin_path);
    } else {
        tracing::warn!(
            "Could not detect VLC plugin path. VLC initialization may fail. \
             Please install VLC or set VLC_PLUGIN_PATH manually."
        );
    }
}

/// Detects the VLC plugin directory path based on the operating system.
///
/// Returns `None` if VLC installation cannot be detected.
fn detect_vlc_plugin_path() -> Option<PathBuf> {
    #[cfg(target_os = "macos")]
    {
        detect_macos_vlc_plugin_path()
    }

    #[cfg(target_os = "windows")]
    {
        detect_windows_vlc_plugin_path()
    }

    #[cfg(target_os = "linux")]
    {
        // Linux typically has system-wide VLC installation
        // that doesn't require explicit plugin path
        None
    }
}

#[cfg(target_os = "macos")]
fn detect_macos_vlc_plugin_path() -> Option<PathBuf> {
    // Check standard VLC.app location
    let standard_path = Path::new("/Applications/VLC.app/Contents/MacOS/plugins");
    if standard_path.exists() {
        return Some(standard_path.to_path_buf());
    }

    // Check user Applications folder
    if let Ok(home) = env::var("HOME") {
        let user_path = PathBuf::from(home)
            .join("Applications")
            .join("VLC.app")
            .join("Contents")
            .join("MacOS")
            .join("plugins");
        if user_path.exists() {
            return Some(user_path);
        }
    }

    None
}

#[cfg(target_os = "windows")]
fn detect_windows_vlc_plugin_path() -> Option<PathBuf> {
    // Common VLC installation paths on Windows
    let paths = vec![
        r"C:\Program Files\VideoLAN\VLC\plugins\",
        r"C:\Program Files (x86)\VideoLAN\VLC\plugins\",
    ];

    for path_str in paths {
        let path = Path::new(path_str);
        if path.exists() {
            return Some(path.to_path_buf());
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_setup_vlc_environment_idempotent() {
        // Set a test value
        env::set_var("VLC_PLUGIN_PATH", "/test/path");

        // Call setup - should not override
        setup_vlc_environment();

        // Verify it wasn't changed
        let path = env::var("VLC_PLUGIN_PATH");
        assert!(path.is_ok(), "VLC_PLUGIN_PATH should be set");
        assert_eq!(path.as_ref().map(String::as_str), Ok("/test/path"));

        // Cleanup
        env::remove_var("VLC_PLUGIN_PATH");
    }

    #[test]
    fn test_detect_vlc_plugin_path_on_macos() {
        #[cfg(target_os = "macos")]
        {
            let path = detect_vlc_plugin_path();
            // Should either find VLC or return None
            if let Some(p) = path {
                assert!(p.to_string_lossy().contains("plugins"));
            }
        }
    }

    #[test]
    fn test_setup_without_vlc_installed() {
        // Remove any existing VLC_PLUGIN_PATH
        env::remove_var("VLC_PLUGIN_PATH");

        // Call setup (may not find VLC)
        setup_vlc_environment();

        // Should not panic even if VLC is not installed
        // The function should log a warning but continue
    }

    #[test]
    fn test_environment_setup_thread_safety() {
        use std::thread;

        // Remove existing path
        env::remove_var("VLC_PLUGIN_PATH");

        // Setup from multiple threads simultaneously
        let handles: Vec<_> = (0..5)
            .map(|_| {
                thread::spawn(|| {
                    setup_vlc_environment();
                })
            })
            .collect();

        // Wait for all threads
        for handle in handles {
            assert!(handle.join().is_ok(), "Thread should complete successfully");
        }

        // Should complete without panics
    }
}
