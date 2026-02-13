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
use std::sync::{Mutex, OnceLock};

static VLC_ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

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
/// ## Idempotency
///
/// This function is safe to call multiple times. It only sets the environment
/// variable if it's not already configured. Subsequent calls will detect the
/// existing configuration and return early.
///
/// ## Usage
///
/// In application code, call this once at startup before creating any VLC instances.
/// When using the player module as a library, player constructors will call this
/// automatically, but calling it explicitly at startup is recommended for reliability.
pub fn setup_vlc_environment() {
    let _guard = VLC_ENV_LOCK
        .get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);

    // Check if already configured
    if let Ok(existing_path) = env::var("VLC_PLUGIN_PATH") {
        tracing::debug!("VLC_PLUGIN_PATH already set to: {}", existing_path);

        // Validate that the path exists
        let path = Path::new(&existing_path);
        if path.exists() {
            tracing::info!("Using existing VLC_PLUGIN_PATH: {}", existing_path);
        } else {
            tracing::warn!(
                "VLC_PLUGIN_PATH is set to '{}' but directory does not exist. \
                 VLC initialization may fail.",
                existing_path
            );
        }
        return;
    }

    // Try to detect VLC plugin path
    match detect_vlc_plugin_path() {
        Some(plugin_path) => {
            // Verify the detected path exists
            if plugin_path.exists() {
                tracing::info!("Auto-detected VLC plugin path: {}", plugin_path.display());
                env::set_var("VLC_PLUGIN_PATH", &plugin_path);
            } else {
                tracing::error!(
                    "Detected VLC plugin path '{}' does not exist. \
                     VLC initialization will likely fail.",
                    plugin_path.display()
                );
            }
        }
        None => {
            tracing::warn!(
                "Could not auto-detect VLC plugin path. VLC initialization may fail. \n\
                 Troubleshooting:\n\
                 - macOS: Install VLC.app to /Applications or ~/Applications\n\
                 - Linux: Install vlc and libvlc-dev via package manager\n\
                 - Windows: Install VLC to C:\\Program Files\\VideoLAN\\VLC\n\
                 - Or set VLC_PLUGIN_PATH environment variable manually"
            );
        }
    }
}

/// Tests if VLC can be initialized with current environment settings.
///
/// Returns `true` if a VLC instance can be successfully created.
///
/// # Examples
///
/// ```no_run
/// # use nodoka::player::verify_vlc_available;
/// if verify_vlc_available() {
///     println!("VLC is available");
/// } else {
///     println!("VLC is not available");
/// }
/// ```
pub fn verify_vlc_available() -> bool {
    use vlc::Instance;

    tracing::debug!("Verifying VLC availability...");

    Instance::new().map_or_else(
        || {
            tracing::error!("Failed to create VLC instance");
            false
        },
        |_instance| {
            tracing::info!("VLC instance created successfully");
            true
        },
    )
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
    use crate::test_support::{env_lock, EnvVarGuard};

    #[test]
    fn test_setup_vlc_environment_idempotent() {
        let _lock = env_lock();
        let _guard = EnvVarGuard::capture("VLC_PLUGIN_PATH");

        // Set a test value
        env::set_var("VLC_PLUGIN_PATH", "/test/path");

        // Call setup - should not override
        setup_vlc_environment();

        // Verify it wasn't changed
        let path = env::var("VLC_PLUGIN_PATH");
        assert!(path.is_ok(), "VLC_PLUGIN_PATH should be set");
        assert_eq!(path.as_ref().map(String::as_str), Ok("/test/path"));

        // Restored by EnvVarGuard
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
        let _lock = env_lock();
        let _guard = EnvVarGuard::capture("VLC_PLUGIN_PATH");

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

        let _lock = env_lock();
        let _guard = EnvVarGuard::capture("VLC_PLUGIN_PATH");

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

        if let Ok(path) = env::var("VLC_PLUGIN_PATH") {
            assert!(
                Path::new(&path).exists(),
                "VLC_PLUGIN_PATH was set but path does not exist: {path}"
            );
        }

        // Restored by EnvVarGuard
    }
}
