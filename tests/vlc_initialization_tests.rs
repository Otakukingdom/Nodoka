use nodoka::player::{setup_vlc_environment, verify_vlc_available, Vlc};
use std::env;
use std::ffi::OsString;
use std::sync::{Mutex, MutexGuard};
use temp_dir::TempDir;

static ENV_MUTEX: Mutex<()> = Mutex::new(());

fn env_lock() -> MutexGuard<'static, ()> {
    match ENV_MUTEX.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    }
}

struct EnvVarGuard {
    key: &'static str,
    previous: Option<OsString>,
}

impl EnvVarGuard {
    fn capture(key: &'static str) -> Self {
        let previous = env::var_os(key);
        Self { key, previous }
    }
}

impl Drop for EnvVarGuard {
    fn drop(&mut self) {
        match &self.previous {
            Some(value) => env::set_var(self.key, value),
            None => env::remove_var(self.key),
        }
    }
}

#[test]
fn test_setup_vlc_environment_respects_existing_plugin_path(
) -> Result<(), Box<dyn std::error::Error>> {
    let _lock = env_lock();
    let _guard = EnvVarGuard::capture("VLC_PLUGIN_PATH");

    let temp_dir = TempDir::new()?;
    env::set_var("VLC_PLUGIN_PATH", temp_dir.path());
    setup_vlc_environment();

    assert_eq!(
        env::var_os("VLC_PLUGIN_PATH"),
        Some(temp_dir.path().to_owned().into_os_string()),
        "setup_vlc_environment must not override an explicit VLC_PLUGIN_PATH"
    );

    Ok(())
}

#[test]
fn test_setup_vlc_environment_sets_plugin_path_when_detectable_on_macos() {
    let _lock = env_lock();
    let _guard = EnvVarGuard::capture("VLC_PLUGIN_PATH");

    env::remove_var("VLC_PLUGIN_PATH");
    setup_vlc_environment();

    #[cfg(target_os = "macos")]
    {
        let standard = "/Applications/VLC.app/Contents/MacOS/plugins";
        let standard_path_exists = std::path::Path::new(standard).exists();

        let user_path = env::var_os("HOME").map(|home| {
            std::path::Path::new(&home)
                .join("Applications")
                .join("VLC.app")
                .join("Contents")
                .join("MacOS")
                .join("plugins")
        });
        let user_path_exists = user_path.as_ref().is_some_and(|p| p.exists());

        let expected = if standard_path_exists {
            Some(OsString::from(standard))
        } else if user_path_exists {
            user_path.map(std::path::PathBuf::into_os_string)
        } else {
            None
        };

        assert_eq!(
            env::var_os("VLC_PLUGIN_PATH"),
            expected,
            "setup_vlc_environment should set VLC_PLUGIN_PATH iff a standard VLC.app plugins directory is detectable"
        );
    }
}

#[test]
fn test_vlc_player_new_smoke_when_vlc_available() {
    let _lock = env_lock();
    let _guard = EnvVarGuard::capture("VLC_PLUGIN_PATH");

    env::remove_var("VLC_PLUGIN_PATH");
    setup_vlc_environment();

    let result = Vlc::new();
    match result {
        Ok(_player) => {}
        Err(nodoka::error::Error::Vlc(msg)) => assert!(
            msg.contains("Failed to create VLC instance"),
            "Unexpected VLC error creating player: {msg}"
        ),
        Err(other) => assert!(
            matches!(other, nodoka::error::Error::Vlc(_)),
            "Unexpected error creating Vlc player: {other:?}"
        ),
    }
}

#[test]
fn test_vlc_player_fails_gracefully_when_vlc_unavailable() {
    let _lock = env_lock();
    let _guard = EnvVarGuard::capture("VLC_PLUGIN_PATH");

    // Remove VLC_PLUGIN_PATH to simulate missing VLC
    env::remove_var("VLC_PLUGIN_PATH");

    // Don't call setup_vlc_environment to simulate a broken environment
    let result = Vlc::new();

    match result {
        Err(nodoka::error::Error::Vlc(msg)) => {
            assert!(msg.contains("Failed to create VLC instance"));
        }
        Ok(_) => {
            // VLC was found anyway, skip test
        }
        Err(other) => {
            assert!(
                matches!(other, nodoka::error::Error::Vlc(_)),
                "Expected VLC error, got: {other:?}"
            );
        }
    }
}

#[test]
fn test_vlc_initialization_with_invalid_plugin_path() {
    let _lock = env_lock();
    let _guard = EnvVarGuard::capture("VLC_PLUGIN_PATH");

    // Set an invalid plugin path
    env::set_var("VLC_PLUGIN_PATH", "/nonexistent/invalid/path");

    let result = Vlc::new();

    // Should either succeed (if VLC can find plugins elsewhere) or fail gracefully
    if let Err(e) = result {
        assert!(matches!(e, nodoka::error::Error::Vlc(_)));
    }
}

#[test]
fn test_verify_vlc_available() {
    let _lock = env_lock();
    let _guard = EnvVarGuard::capture("VLC_PLUGIN_PATH");

    // Setup environment first
    setup_vlc_environment();

    // Check if VLC is available
    let is_available = verify_vlc_available();

    // The test passes regardless of whether VLC is installed
    // We're just testing that the function doesn't panic
    if is_available {
        println!("VLC is available on this system");
    } else {
        println!("VLC is not available on this system");
    }
}

#[test]
fn test_vlc_initialization_order_prevents_failures() {
    let _lock = env_lock();
    let _guard = EnvVarGuard::capture("VLC_PLUGIN_PATH");

    // Clear environment to simulate fresh start
    env::remove_var("VLC_PLUGIN_PATH");

    // Pattern from main.rs: setup_vlc_environment BEFORE creating player
    setup_vlc_environment();

    // Now Vlc creation should work (if VLC is installed)
    let player_result = Vlc::new();

    // Check that we either:
    // 1. Successfully created player (VLC is installed), OR
    // 2. Got a clear VLC error (VLC not installed)
    match player_result {
        Ok(_player) => {
            // Success! VLC is installed and environment was set up correctly
        }
        Err(nodoka::error::Error::Vlc(msg)) => {
            // VLC not installed - verify we get a helpful error message
            assert!(
                msg.contains("Failed to create VLC instance"),
                "Should get clear error when VLC unavailable: {msg}"
            );
        }
        Err(other) => {
            assert!(
                matches!(other, nodoka::error::Error::Vlc(_)),
                "Unexpected error type: {other:?}"
            );
        }
    }
}
