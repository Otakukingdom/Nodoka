//! Screenshot output directory structure and organization

use anyhow::{Context, Result};
use std::path::Path;

/// Screenshot category directories
const CATEGORIES: &[&str] = &[
    "app-shell",
    "library-panel",
    "details-panel",
    "player-controls",
    "modals/settings",
    "modals/keyboard-help",
    "modals/sleep-timer",
    "modals/bookmark-editor",
    "modals/confirmation",
    "notifications",
    "responsive",
    "accessibility",
    "special-states", // Drag-drop overlay, scan progress, focus mode, reduced motion
    "edge-cases",
];

/// Create the screenshot output directory structure
pub fn create_directory_structure(output_dir: &Path) -> Result<()> {
    // Create root output directory
    std::fs::create_dir_all(output_dir)
        .with_context(|| format!("Failed to create output directory: {}", output_dir.display()))?;

    // Create category subdirectories
    for category in CATEGORIES {
        let dir = output_dir.join(category);
        std::fs::create_dir_all(&dir)
            .with_context(|| format!("Failed to create category directory: {}", dir.display()))?;
    }

    Ok(())
}
