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

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_create_directory_structure() {
        let temp = TempDir::new().unwrap();
        let output_dir = temp.path().join("screenshots");

        create_directory_structure(&output_dir).unwrap();

        assert!(output_dir.exists());
        for category in CATEGORIES {
            assert!(
                output_dir.join(category).exists(),
                "Category directory '{}' should exist",
                category
            );
        }
    }

    #[test]
    fn test_create_existing_directory() {
        let temp = TempDir::new().unwrap();
        let output_dir = temp.path().join("screenshots");

        // Create twice - should not fail
        create_directory_structure(&output_dir).unwrap();
        create_directory_structure(&output_dir).unwrap();

        assert!(output_dir.exists());
    }

    #[test]
    fn test_categories_not_empty() {
        // Ensure we have categories defined
        assert!(!CATEGORIES.is_empty(), "CATEGORIES should not be empty");
    }

    #[test]
    fn test_nested_category_directories() {
        let temp = TempDir::new().unwrap();
        let output_dir = temp.path().join("screenshots");

        create_directory_structure(&output_dir).unwrap();

        // Check nested directories like modals/settings
        assert!(output_dir.join("modals/settings").exists());
        assert!(output_dir.join("modals/keyboard-help").exists());
    }
}
