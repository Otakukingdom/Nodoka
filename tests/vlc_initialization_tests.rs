use nodoka::player::VlcPlayer;
use std::env;

#[test]
fn test_vlc_initialization_with_automatic_setup() {
    // Clear any existing VLC plugin path to test automatic detection
    env::remove_var("VLC_PLUGIN_PATH");

    // This should succeed with automatic environment setup
    let result = VlcPlayer::new();

    // On macOS with VLC installed, automatic setup should work
    #[cfg(target_os = "macos")]
    {
        if std::path::Path::new("/Applications/VLC.app/Contents/MacOS/plugins").exists() {
            assert!(
                result.is_ok(),
                "VLC initialization should succeed with automatic plugin path detection"
            );
        }
    }

    // On other platforms or if VLC is not installed, we just verify it doesn't panic
    #[cfg(not(target_os = "macos"))]
    {
        let _ = result;
    }
}

#[test]
fn test_vlc_initialization_with_plugin_path() {
    // Set plugin path to VLC.app location on macOS
    if cfg!(target_os = "macos") {
        env::set_var(
            "VLC_PLUGIN_PATH",
            "/Applications/VLC.app/Contents/MacOS/plugins",
        );
    }

    // This should succeed with proper environment setup
    let result = VlcPlayer::new();

    assert!(
        result.is_ok(),
        "VLC initialization should succeed with VLC_PLUGIN_PATH set"
    );
}
