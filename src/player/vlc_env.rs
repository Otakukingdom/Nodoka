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

type InstanceFactory = fn() -> Option<vlc::Instance>;

static VLC_INSTANCE_FACTORY: OnceLock<Mutex<InstanceFactory>> = OnceLock::new();

#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum VlcInitEvent {
    SetupCalled,
    BeforeInstanceNew,
}

#[cfg(test)]
type VlcInitObserver = fn(VlcInitEvent);

#[cfg(test)]
static VLC_INIT_OBSERVER: OnceLock<Mutex<Option<VlcInitObserver>>> = OnceLock::new();

fn default_instance_factory() -> Option<vlc::Instance> {
    vlc::Instance::new()
}

#[cfg(test)]
fn emit_init_event(event: VlcInitEvent) {
    let Some(lock) = VLC_INIT_OBSERVER.get() else {
        return;
    };

    let observer = lock
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);

    if let Some(callback) = *observer {
        callback(event);
    }
}

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
    #[cfg(test)]
    emit_init_event(VlcInitEvent::SetupCalled);

    let _guard = VLC_ENV_LOCK
        .get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);

    // Check if already configured.
    // If the value is stale (no directory entries exist), clear it and attempt auto-detection.
    // Use `var_os` and `split_paths` to preserve non-UTF8 values and handle multi-entry vars.
    if let Some(existing_value) = env::var_os("VLC_PLUGIN_PATH") {
        let any_valid_dir = env::split_paths(existing_value.as_os_str()).any(|p| p.is_dir());
        if any_valid_dir {
            tracing::info!(
                "Using existing VLC_PLUGIN_PATH: {}",
                existing_value.to_string_lossy()
            );
            return;
        }

        tracing::debug!(
            "VLC_PLUGIN_PATH is set to '{}' but has no valid directory entries; attempting auto-detection",
            existing_value.to_string_lossy()
        );
        env::remove_var("VLC_PLUGIN_PATH");
    }

    // Try to detect VLC plugin path
    match detect_vlc_plugin_path() {
        Some(plugin_path) => {
            // Verify the detected path exists
            if plugin_path.is_dir() {
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
            #[cfg(target_os = "linux")]
            {
                tracing::debug!(
                    "VLC plugin path auto-detection not required on Linux; relying on system defaults"
                );
            }

            #[cfg(not(target_os = "linux"))]
            {
                tracing::warn!(
                    "Could not auto-detect VLC plugin path. VLC initialization may fail. \n\
                     Troubleshooting:\n\
                     - macOS: Install VLC.app to /Applications or ~/Applications\n\
                     - Windows: Install VLC to C:\\Program Files\\VideoLAN\\VLC\n\
                     - Or set VLC_PLUGIN_PATH environment variable manually"
                );
            }
        }
    }

    #[cfg(target_os = "windows")]
    {
        setup_windows_vlc_library_path();
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
    tracing::debug!("Verifying VLC availability...");

    create_vlc_instance().map_or_else(
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

pub(super) fn create_vlc_instance() -> Option<vlc::Instance> {
    setup_vlc_environment();

    if let Some(plugin_path) = env::var_os("VLC_PLUGIN_PATH") {
        tracing::debug!("VLC_PLUGIN_PATH = {}", plugin_path.to_string_lossy());
    } else {
        tracing::debug!("VLC_PLUGIN_PATH not set, relying on system defaults");
    }

    #[cfg(test)]
    emit_init_event(VlcInitEvent::BeforeInstanceNew);

    let factory_lock = VLC_INSTANCE_FACTORY
        .get_or_init(|| Mutex::new(default_instance_factory as InstanceFactory));

    let factory = factory_lock
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);

    (*factory)()
}

/// Installs or clears a test observer for VLC initialization ordering.
#[cfg(test)]
fn __set_vlc_init_observer_for_tests(observer: Option<VlcInitObserver>) -> VlcTestHookGuard {
    let lock = VLC_INIT_OBSERVER.get_or_init(|| Mutex::new(None));
    let mut guard = lock
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);

    let previous = *guard;
    *guard = observer;

    drop(guard);

    VlcTestHookGuard {
        restore_observer: ObserverRestore::Restore(previous),
        restore_factory: None,
    }
}

/// Overrides the VLC instance factory for deterministic tests.
///
/// Passing a factory that always returns `None` allows tests to force the VLC
/// error path even on machines where VLC is installed.
#[cfg(test)]
fn __set_vlc_instance_factory_for_tests(factory: InstanceFactory) -> VlcTestHookGuard {
    let lock = VLC_INSTANCE_FACTORY
        .get_or_init(|| Mutex::new(default_instance_factory as InstanceFactory));
    let mut guard = lock
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);

    let previous = *guard;
    *guard = factory;

    drop(guard);

    VlcTestHookGuard {
        restore_observer: ObserverRestore::Unchanged,
        restore_factory: Some(previous),
    }
}

#[cfg(test)]
struct VlcTestHookGuard {
    restore_observer: ObserverRestore,
    restore_factory: Option<InstanceFactory>,
}

#[cfg(test)]
#[derive(Clone, Copy)]
enum ObserverRestore {
    Unchanged,
    Restore(Option<VlcInitObserver>),
}

#[cfg(test)]
impl Drop for VlcTestHookGuard {
    fn drop(&mut self) {
        match self.restore_observer {
            ObserverRestore::Unchanged => {}
            ObserverRestore::Restore(previous) => {
                let lock = VLC_INIT_OBSERVER.get_or_init(|| Mutex::new(None));
                let mut guard = lock
                    .lock()
                    .unwrap_or_else(std::sync::PoisonError::into_inner);
                *guard = previous;
            }
        }

        if let Some(previous) = self.restore_factory.take() {
            let lock = VLC_INSTANCE_FACTORY
                .get_or_init(|| Mutex::new(default_instance_factory as InstanceFactory));
            let mut guard = lock
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            *guard = previous;
        }
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

#[cfg(target_os = "windows")]
fn detect_windows_vlc_install_dir() -> Option<PathBuf> {
    let install_dirs = vec![
        r"C:\Program Files\VideoLAN\VLC",
        r"C:\Program Files (x86)\VideoLAN\VLC",
    ];

    for dir_str in install_dirs {
        let dir = Path::new(dir_str);
        if dir.exists() {
            return Some(dir.to_path_buf());
        }
    }

    None
}

#[cfg(target_os = "windows")]
fn setup_windows_vlc_library_path() {
    // If already configured with an existing directory, do not override.
    if let Ok(existing) = env::var("VLC_LIB_PATH") {
        let path = Path::new(&existing);
        if path.is_dir() {
            tracing::info!("Using existing VLC_LIB_PATH: {}", existing);
            return;
        }

        tracing::debug!(
            "VLC_LIB_PATH is set to '{}' but directory does not exist; attempting auto-detection",
            existing
        );
        env::remove_var("VLC_LIB_PATH");
    }

    let Some(install_dir) = detect_windows_vlc_install_dir() else {
        return;
    };

    apply_windows_vlc_paths(&install_dir);
}

#[cfg(target_os = "windows")]
fn apply_windows_vlc_paths(install_dir: &Path) {
    if !install_dir.is_dir() {
        return;
    }

    tracing::info!(
        "Auto-detected VLC library path (Windows): {}",
        install_dir.display()
    );
    env::set_var("VLC_LIB_PATH", install_dir);

    // Some Windows setups require the lib directory on PATH for libvlc discovery.
    // Prepend it if it's not already present.
    let current_path = env::var_os("PATH").unwrap_or_default();
    let mut entries: Vec<PathBuf> = env::split_paths(&current_path).collect();

    fn normalize_for_contains(path: &Path) -> String {
        let mut s = path.to_string_lossy().replace('/', "\\");
        while s.ends_with('\\') {
            s.pop();
        }
        s.to_ascii_lowercase()
    }

    let needle = normalize_for_contains(install_dir);
    let already_present = entries
        .iter()
        .any(|p| normalize_for_contains(p.as_path()) == needle);

    if !already_present {
        entries.insert(0, install_dir.to_path_buf());
        if let Ok(joined) = env::join_paths(entries) {
            env::set_var("PATH", joined);
        } else {
            tracing::warn!("Failed to update PATH with VLC install directory");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::{env_lock, EnvVarGuard};
    use std::sync::Mutex;
    use temp_dir::TempDir;

    static INIT_EVENTS: Mutex<Vec<VlcInitEvent>> = Mutex::new(Vec::new());

    fn record_init_event(event: VlcInitEvent) {
        let mut events = INIT_EVENTS
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        events.push(event);
    }

    const fn always_fail_instance_creation() -> Option<vlc::Instance> {
        None
    }

    #[test]
    fn test_setup_vlc_environment_idempotent() -> Result<(), Box<dyn std::error::Error>> {
        let _lock = env_lock();
        let _guard = EnvVarGuard::capture("VLC_PLUGIN_PATH");

        let temp_dir = TempDir::new()?;
        let temp_path = temp_dir.path().to_owned();

        // Set a valid, existing path
        env::set_var("VLC_PLUGIN_PATH", &temp_path);

        // Call setup - should not override
        setup_vlc_environment();

        // Verify it wasn't changed
        let path = env::var_os("VLC_PLUGIN_PATH");
        assert_eq!(path, Some(temp_path.into_os_string()));

        // Restored by EnvVarGuard
        Ok(())
    }

    #[test]
    fn test_setup_runs_before_first_instance_creation_regression() {
        let _lock = env_lock();
        let _guard = EnvVarGuard::capture("VLC_PLUGIN_PATH");

        env::remove_var("VLC_PLUGIN_PATH");

        {
            let mut events = INIT_EVENTS
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            events.clear();
        }

        let _observer_guard = __set_vlc_init_observer_for_tests(Some(record_init_event));
        let _factory_guard = __set_vlc_instance_factory_for_tests(always_fail_instance_creation);

        let result = crate::player::Vlc::new();
        assert!(
            matches!(result, Err(crate::error::Error::Vlc(_))),
            "Expected deterministic VLC error when instance creation is forced to fail"
        );

        let events = INIT_EVENTS
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .clone();

        assert_eq!(
            events,
            vec![VlcInitEvent::SetupCalled, VlcInitEvent::BeforeInstanceNew]
        );
    }

    #[test]
    fn test_setup_vlc_environment_keeps_multi_entry_plugin_path_if_any_dir_exists(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let _lock = env_lock();
        let _guard = EnvVarGuard::capture("VLC_PLUGIN_PATH");

        let valid_dir = TempDir::new()?;
        let missing_dir = valid_dir.path().join("missing");

        let value = env::join_paths([missing_dir, valid_dir.path().to_path_buf()])?;
        env::set_var("VLC_PLUGIN_PATH", &value);

        setup_vlc_environment();

        assert_eq!(env::var_os("VLC_PLUGIN_PATH"), Some(value));
        Ok(())
    }

    #[test]
    #[cfg(all(unix, not(target_os = "macos")))]
    fn test_setup_vlc_environment_clears_non_unicode_invalid_plugin_path_entry(
    ) -> Result<(), Box<dyn std::error::Error>> {
        use std::ffi::OsString;
        use std::os::unix::ffi::OsStringExt;

        let _lock = env_lock();
        let _guard = EnvVarGuard::capture("VLC_PLUGIN_PATH");

        let temp_dir = TempDir::new()?;
        let mut name = b"not_a_dir_".to_vec();
        name.push(0xFF);
        name.extend_from_slice(b"_file");

        let file_path = temp_dir.path().join(OsString::from_vec(name));
        std::fs::write(&file_path, "not a directory")?;

        let original = file_path.into_os_string();
        env::set_var("VLC_PLUGIN_PATH", &original);

        setup_vlc_environment();

        assert_ne!(
            env::var_os("VLC_PLUGIN_PATH"),
            Some(original),
            "setup must not keep a non-Unicode VLC_PLUGIN_PATH pointing to a file"
        );
        Ok(())
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
    fn test_setup_vlc_environment_does_not_keep_stale_plugin_path(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let _lock = env_lock();
        let _guard = EnvVarGuard::capture("VLC_PLUGIN_PATH");

        let temp_dir = TempDir::new()?;
        let missing_path = temp_dir.path().join("missing_vlc_plugins_dir");
        assert!(!missing_path.exists());

        env::set_var("VLC_PLUGIN_PATH", &missing_path);

        setup_vlc_environment();

        if let Ok(path) = env::var("VLC_PLUGIN_PATH") {
            assert!(
                Path::new(&path).exists(),
                "VLC_PLUGIN_PATH must not point to a non-existent directory after setup: {path}"
            );
        }

        Ok(())
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

    #[test]
    fn test_setup_vlc_environment_does_not_accept_file_as_plugin_path(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let _lock = env_lock();
        let _guard = EnvVarGuard::capture("VLC_PLUGIN_PATH");

        let temp_dir = TempDir::new()?;
        let file_path = temp_dir.path().join("not_a_dir");
        std::fs::write(&file_path, "not a directory")?;

        env::set_var("VLC_PLUGIN_PATH", &file_path);

        setup_vlc_environment();

        if let Ok(plugin_path) = env::var("VLC_PLUGIN_PATH") {
            assert!(
                Path::new(&plugin_path).is_dir(),
                "VLC_PLUGIN_PATH must not point to a file after setup: {plugin_path}"
            );
        }

        Ok(())
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn test_apply_windows_vlc_paths_sets_vlc_lib_path_and_path(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let _lock = env_lock();
        let _vlc_lib_guard = EnvVarGuard::capture("VLC_LIB_PATH");
        let _path_guard = EnvVarGuard::capture("PATH");

        let temp_dir = TempDir::new()?;
        let install_dir = temp_dir.path().join("VLC");
        std::fs::create_dir_all(&install_dir)?;

        env::remove_var("VLC_LIB_PATH");
        env::set_var("PATH", "C:\\Windows\\System32");

        apply_windows_vlc_paths(&install_dir);

        assert_eq!(
            env::var_os("VLC_LIB_PATH"),
            Some(install_dir.clone().into_os_string())
        );

        let path = env::var("PATH")?;
        let install_str = install_dir.to_string_lossy();
        assert!(
            path.starts_with(&install_str),
            "PATH must be prepended with VLC install dir"
        );

        Ok(())
    }
}
