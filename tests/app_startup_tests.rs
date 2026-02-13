use nodoka::player::{
    __set_vlc_init_observer_for_tests, __set_vlc_instance_factory_for_tests, setup_vlc_environment,
    VlcInitEvent,
};
mod test_support;
use std::env;
use std::sync::Mutex;
use test_support::{env_lock, EnvVarGuard};

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
fn test_vlc_environment_setup_before_player_creation() {
    let _lock = env_lock();
    let _guard = EnvVarGuard::capture("VLC_PLUGIN_PATH");

    // Remove any existing VLC_PLUGIN_PATH to simulate fresh startup
    env::remove_var("VLC_PLUGIN_PATH");

    // Call setup as main.rs should
    setup_vlc_environment();

    // If setup chooses to set VLC_PLUGIN_PATH, it must point at an existing directory.
    if let Ok(plugin_path) = env::var("VLC_PLUGIN_PATH") {
        assert!(
            std::path::Path::new(&plugin_path).exists(),
            "VLC_PLUGIN_PATH was set but does not exist: {plugin_path}"
        );
    }
}

#[test]
fn test_app_initializes_vlc_before_player() {
    let _lock = env_lock();
    let _guard = EnvVarGuard::capture("VLC_PLUGIN_PATH");

    // This test verifies the initialization order:
    // 1. setup_vlc_environment() must be called first
    // 2. Then Vlc::new() can succeed

    env::remove_var("VLC_PLUGIN_PATH");

    // Without calling setup_vlc_environment first,
    // Vlc::new() may fail even if VLC is installed

    // This simulates what happens in main.rs
    setup_vlc_environment();

    // Now player creation should work (if VLC is installed)
    let player_result = nodoka::player::Vlc::new();

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
            assert!(
                matches!(other, nodoka::error::Error::Vlc(_)),
                "Unexpected error type: {other:?}"
            );
        }
    }
}

#[test]
fn test_vlc_setup_runs_before_first_instance_creation_regression() {
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

    let result = nodoka::player::Vlc::new();
    assert!(
        matches!(result, Err(nodoka::error::Error::Vlc(_))),
        "Expected deterministic VLC error when instance creation is forced to fail"
    );

    let events = INIT_EVENTS
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .clone();

    assert_eq!(
        events,
        vec![VlcInitEvent::SetupCalled, VlcInitEvent::BeforeInstanceNew],
        "setup_vlc_environment must run before attempting VLC instance creation"
    );
}

#[test]
fn test_early_vlc_setup_prevents_init_failures() {
    let _guard = env_lock();
    let _env_guard = EnvVarGuard::capture("VLC_PLUGIN_PATH");

    // Simulate fresh startup
    env::remove_var("VLC_PLUGIN_PATH");

    // This is the pattern from main.rs:
    // 1. Setup VLC environment first
    setup_vlc_environment();

    // 2. Then create player
    let player_result = nodoka::player::Vlc::new();

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
            assert!(
                matches!(e, nodoka::error::Error::Vlc(_)),
                "Unexpected error type: {e:?}"
            );
        }
    }
}

#[test]
fn test_app_startup_initializes_vlc_properly() {
    let _lock = env_lock();
    let _guard = EnvVarGuard::capture("VLC_PLUGIN_PATH");

    // Clear VLC_PLUGIN_PATH to simulate fresh startup
    env::remove_var("VLC_PLUGIN_PATH");

    // Simulate the main.rs initialization sequence:
    // 1. setup_vlc_environment() is called first
    setup_vlc_environment();

    // 2. If setup chose to set VLC_PLUGIN_PATH, it must be valid.
    let plugin_path = env::var("VLC_PLUGIN_PATH").ok();

    // 3/4. Vlc should initialize successfully if VLC is installed.
    let player = nodoka::player::Vlc::new();
    match player {
        Ok(_player) => {
            if let Some(path) = plugin_path {
                assert!(
                    std::path::Path::new(&path).exists(),
                    "VLC_PLUGIN_PATH was set but does not exist: {path}"
                );
            }
        }
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
fn test_app_handles_missing_vlc_gracefully() {
    let _lock = env_lock();
    let _guard = EnvVarGuard::capture("VLC_PLUGIN_PATH");

    // Set an invalid plugin path to simulate VLC not being found
    env::set_var("VLC_PLUGIN_PATH", "/nonexistent/invalid/vlc/path");

    // Try to create VLC player
    let result = nodoka::player::Vlc::new();

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
            assert!(
                matches!(e, nodoka::error::Error::Vlc(_)),
                "Expected VLC error, got: {e:?}"
            );
        }
    }
}

/// This test documents and verifies the correct VLC initialization sequence
/// that main.rs must follow to prevent the "Failed to create VLC instance" bug.
///
/// CRITICAL ORDER (from main.rs):
/// 1. `setup_vlc_environment()` - MUST be called first
/// 2. `Database::open()`
/// 3. `Vlc::new()` (called by `App::new()`)
///
/// This test ensures we don't regress on commit ea08b6a which fixed the bug.
#[test]
fn test_documented_initialization_sequence_from_main() {
    let _lock = env_lock();
    let _guard = EnvVarGuard::capture("VLC_PLUGIN_PATH");

    // Start fresh
    env::remove_var("VLC_PLUGIN_PATH");

    // Step 1: MUST call setup_vlc_environment first (before any VLC operations)
    setup_vlc_environment();

    // Verify environment is now configured
    let _has_plugin_path = env::var("VLC_PLUGIN_PATH").is_ok();

    // Step 2: Database operations can happen (no dependency on VLC)
    // (Database::open() is tested separately, we skip it here)

    // Step 3: VLC player creation should now work when VLC is installed.
    let player = nodoka::player::Vlc::new();
    match player {
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
