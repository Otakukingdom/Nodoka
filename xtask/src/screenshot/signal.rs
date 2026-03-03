//! Signal handling for graceful cleanup on interruption.

use anyhow::Result;

/// Set up the Ctrl+C handler
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
#[allow(dead_code)]
pub fn was_interrupted() -> bool {
    super::state::is_interrupted()
}
