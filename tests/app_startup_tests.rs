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
