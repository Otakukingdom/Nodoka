//! Signal handling for graceful cleanup on interruption.

use anyhow::Result;

/// Set up the Ctrl+C handler
pub fn setup_signal_handler() -> Result<()> {
    ctrlc::set_handler(|| {
        eprintln!("\n^C");
        eprintln!("Interrupt received, cleaning up...");

        // Signal the server module to stop waiting
        super::server::set_interrupted();
    })?;

    Ok(())
}

/// Check if an interrupt signal was received
#[allow(dead_code)]
pub fn was_interrupted() -> bool {
    super::server::is_interrupted()
}
