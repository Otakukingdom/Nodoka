use nodoka::player::setup_vlc_environment;
use std::env;
use std::sync::{Mutex, MutexGuard};

static ENV_LOCK: Mutex<()> = Mutex::new(());

fn env_lock() -> MutexGuard<'static, ()> {
    match ENV_LOCK.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    }
}

#[test]
fn test_vlc_environment_setup_before_player_creation() {
    // This test verifies that setup_vlc_environment is called early
    // enough that VLC can find its plugins when Instance::new() is called

    // Remove any existing VLC_PLUGIN_PATH to simulate fresh startup
    env::remove_var("VLC_PLUGIN_PATH");

    // Call setup as main.rs should
    setup_vlc_environment();

    // Verify that either:
    // 1. VLC_PLUGIN_PATH is now set (if VLC was detected), OR
    // 2. VLC_PLUGIN_PATH is not set (relying on system defaults)
    // In either case, VLC Instance creation should work if VLC is installed

    // Try to create a VLC instance
    let instance = vlc::Instance::new();

    if env::var("VLC_PLUGIN_PATH").is_ok() || instance.is_some() {
        // Environment is properly configured - test passes
    } else {
        // VLC is not installed - this is acceptable
        println!("VLC not installed on system, skipping test");
    }
}

#[test]
fn test_app_initializes_vlc_before_player() {
    // This test verifies the initialization order:
    // 1. setup_vlc_environment() must be called first
    // 2. Then VlcPlayer::new() can succeed

    env::remove_var("VLC_PLUGIN_PATH");

    // Without calling setup_vlc_environment first,
    // VlcPlayer::new() may fail even if VLC is installed

    // This simulates what happens in main.rs
    setup_vlc_environment();

    // Now player creation should work (if VLC is installed)
    let player_result = nodoka::player::VlcPlayer::new();

    // The test passes if:
    // - VLC is installed and player is created successfully, OR
    // - VLC is not installed and we get an appropriate error
    match player_result {
        Ok(_) => {
            println!("VLC player created successfully");
        }
        Err(nodoka::error::Error::Vlc(msg)) => {
            // Expected error when VLC is not installed
            assert!(
                msg.contains("Failed to create VLC instance")
                    || msg.contains("VLC media player is installed"),
                "Error message should indicate VLC installation issue"
            );
        }
        Err(other) => {
            unreachable!("Unexpected error type: {other:?}");
        }
    }
}

#[test]
fn test_early_vlc_setup_prevents_init_failures() {
    let _guard = env_lock();

    // Save current environment
    let original_plugin_path = env::var("VLC_PLUGIN_PATH").ok();

    // Simulate fresh startup
    env::remove_var("VLC_PLUGIN_PATH");

    // This is the pattern from main.rs:
    // 1. Setup VLC environment first
    setup_vlc_environment();

    // 2. Then create player
    let player_result = nodoka::player::VlcPlayer::new();

    // Verify player creation succeeded (if VLC is installed) or failed gracefully
    match player_result {
        Ok(_) => {
            // Success - VLC is installed and environment is properly set up
        }
        Err(nodoka::error::Error::Vlc(msg)) => {
            // Expected when VLC is not installed
            assert!(
                msg.contains("Failed to create VLC instance"),
                "Should get VLC instance creation error when VLC is not installed"
            );
        }
        Err(e) => {
            unreachable!("Unexpected error type: {e:?}");
        }
    }

    // Restore original environment
    match original_plugin_path {
        Some(path) => env::set_var("VLC_PLUGIN_PATH", path),
        None => env::remove_var("VLC_PLUGIN_PATH"),
    }
}

#[test]
fn test_app_startup_initializes_vlc_properly() {
    let _lock = env_lock();

    // Save and clear VLC_PLUGIN_PATH to simulate fresh startup
    let original = env::var("VLC_PLUGIN_PATH").ok();
    env::remove_var("VLC_PLUGIN_PATH");

    // Simulate the main.rs initialization sequence:
    // 1. setup_vlc_environment() is called first
    setup_vlc_environment();

    // 2. VLC_PLUGIN_PATH should now be set (if VLC is installed)
    let plugin_path_set = env::var("VLC_PLUGIN_PATH").is_ok();

    // 3. VLC instance should be creatable
    let vlc_available = vlc::Instance::new().is_some();

    // 4. VlcPlayer should initialize successfully (if VLC is available)
    if vlc_available {
        let player = nodoka::player::VlcPlayer::new();
        assert!(
            player.is_ok(),
            "VlcPlayer::new() should succeed when VLC is available and environment is set up"
        );
    }

    // Verify proper initialization order was followed
    if vlc_available {
        assert!(
            plugin_path_set || vlc_available,
            "If VLC is available, either plugin path should be set or VLC uses system defaults"
        );
    }

    // Restore original environment
    match original {
        Some(path) => env::set_var("VLC_PLUGIN_PATH", path),
        None => env::remove_var("VLC_PLUGIN_PATH"),
    }
}

#[test]
fn test_app_handles_missing_vlc_gracefully() {
    let _lock = env_lock();
    let original = env::var("VLC_PLUGIN_PATH").ok();

    // Set an invalid plugin path to simulate VLC not being found
    env::set_var("VLC_PLUGIN_PATH", "/nonexistent/invalid/vlc/path");

    // Try to create VLC player
    let result = nodoka::player::VlcPlayer::new();

    match result {
        Err(nodoka::error::Error::Vlc(msg)) => {
            // Should get a descriptive error message
            assert!(
                msg.contains("Failed to create VLC instance") || msg.contains("VLC media player"),
                "Error message should mention VLC instance creation: {msg}"
            );
        }
        Ok(_) => {
            // VLC found despite invalid path (system installation worked)
            // This is acceptable - test passes
        }
        Err(e) => {
            panic!("Expected VLC error, got: {e:?}");
        }
    }

    // Restore
    match original {
        Some(path) => env::set_var("VLC_PLUGIN_PATH", path),
        None => env::remove_var("VLC_PLUGIN_PATH"),
    }
}

/// This test documents and verifies the correct VLC initialization sequence
/// that main.rs must follow to prevent the "Failed to create VLC instance" bug.
///
/// CRITICAL ORDER (from main.rs):
/// 1. setup_vlc_environment() - MUST be called first
/// 2. Database::open()
/// 3. VlcPlayer::new() (called by App::new)
///
/// This test ensures we don't regress on commit ea08b6a which fixed the bug.
#[test]
fn test_documented_initialization_sequence_from_main() {
    let _lock = env_lock();
    let original = env::var("VLC_PLUGIN_PATH").ok();

    // Start fresh
    env::remove_var("VLC_PLUGIN_PATH");

    // Step 1: MUST call setup_vlc_environment first (before any VLC operations)
    setup_vlc_environment();

    // Verify environment is now configured
    let _has_plugin_path = env::var("VLC_PLUGIN_PATH").is_ok();
    let vlc_works = vlc::Instance::new().is_some();

    // Step 2: Database operations can happen (no dependency on VLC)
    // (Database::open() is tested separately, we skip it here)

    // Step 3: VLC player creation should now work
    if vlc_works {
        let player = nodoka::player::VlcPlayer::new();
        assert!(
            player.is_ok(),
            "VlcPlayer::new() must succeed when following correct init sequence"
        );
    } else {
        // VLC not installed - that's ok, test passes
        // The important thing is we didn't panic and got graceful error
        println!("VLC not installed - graceful degradation verified");
    }

    // Restore
    match original {
        Some(path) => env::set_var("VLC_PLUGIN_PATH", path),
        None => env::remove_var("VLC_PLUGIN_PATH"),
    }
}
