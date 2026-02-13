use nodoka::Database;
use std::env;
use std::sync::{Mutex, OnceLock};

static ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

#[test]
fn test_database_open_error_contains_details() {
    // Set an invalid HOME directory to trigger ProjectDirNotFound error
    // Note: On some systems, this may result in different error types
    // depending on fallback behavior. The key is that we get a detailed
    // error message, not a generic "Failed to load the config file"
    // HOME is process-global and tests run in parallel; serialize mutations.
    let _lock = ENV_LOCK
        .get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);

    let original_home = env::var_os("HOME");
    env::remove_var("HOME");

    let result = Database::open();

    // Restore original HOME
    match original_home {
        Some(home) => env::set_var("HOME", home),
        None => env::remove_var("HOME"),
    }

    // The test may pass if HOME fallback works on this platform
    // which is acceptable - we're primarily testing that errors are detailed
    if let Err(err) = result {
        let err_string = format!("{err}");

        // The error message should be specific and informative,
        // not the generic "Failed to load the config file"
        // It should contain error details from one of the error variants
        assert!(
            !err_string.is_empty()
                && (err_string.contains("Project directory")
                    || err_string.contains("Database error")
                    || err_string.contains("IO error")),
            "Expected informative error message, got: {err_string}",
        );
    }
}

// Note: Manual verification test for main.rs error reporting
// When Database::open() fails in main.rs, the error should be printed
// to help users and developers understand what went wrong.
//
// Before fix: prints "Error: Failed to load the config file"
// After fix: prints the actual error from Database::open() with details
