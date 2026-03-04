//! Signal handling for graceful cleanup on interruption.
//!
//! This module sets up a Ctrl+C (SIGINT) handler that triggers
//! comprehensive cleanup of resources including:
//! - Stopping the Vite dev server
//! - Removing the lock file
//! - Cleaning up partial artifacts
//!
//! The handler uses the shared state module to coordinate cleanup
//! across all modules.
//!
//! # Usage
//!
//! Call [`setup_signal_handler()`] early in the main function, before
//! any resources are allocated that might need cleanup.
//!
//! # Signal Safety
//!
//! The signal handler performs minimal work:
//! 1. Sets an atomic flag via [`state::set_interrupted()`](super::state::set_interrupted)
//! 2. Calls [`state::emergency_cleanup()`](super::state::emergency_cleanup) for immediate cleanup
//!
//! Actual cleanup operations (process killing, file removal) are delegated
//! to the state module to keep signal handling simple.

use anyhow::Result;

/// Set up the Ctrl+C handler
///
/// This should be called early in the main function, before any
/// resources are allocated that might need cleanup.
///
/// # Errors
///
/// Returns an error if the signal handler cannot be registered (e.g.,
/// if another handler is already installed).
pub fn setup_signal_handler() -> Result<()> {
    ctrlc::set_handler(|| {
        eprintln!("\n^C");
        eprintln!("Interrupt received, cleaning up...");
        eprintln!("├── Stopping Vite dev server...");
        eprintln!("├── Removing lock file...");

        // Signal the shared state that we've been interrupted
        super::state::set_interrupted();

        // Perform immediate cleanup that can be done from signal handler
        super::state::emergency_cleanup();

        eprintln!("└── Cleanup complete");
        eprintln!("\nScreenshot generation cancelled.");
    })?;

    Ok(())
}

/// Check if an interrupt signal was received
///
/// This is a convenience wrapper around [`state::is_interrupted()`](super::state::is_interrupted).
#[allow(dead_code)]
pub fn was_interrupted() -> bool {
    super::state::is_interrupted()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_was_interrupted_api() {
        // This test verifies the API works without actually triggering interrupts.
        // The actual interrupt behavior is tested via integration tests.
        let _ = was_interrupted(); // Just verify the function is callable
    }
}
