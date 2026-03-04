//! Integration tests for screenshot generation workflow.
//!
//! These tests verify the complete lifecycle including:
//! - Lock file management
//! - Directory structure creation
//! - README generation
//!
//! Note: Tests that require network or process spawning are marked
//! with `#[ignore]` and can be run with `cargo test -- --ignored`

#[cfg(test)]
mod tests {
    use std::fs;
    use tempfile::TempDir;

    // Import modules from parent
    use super::super::cleanup;
    use super::super::lock;
    use super::super::organizer;
    use super::super::readme;

    /// Test lock file lifecycle - acquire and release
    #[test]
    fn test_lock_lifecycle() {
        let temp = TempDir::new().unwrap();
        let lock_path = temp.path().join(".screenshot.lock");

        // Acquire lock
        {
            let _lock = lock::LockFile::acquire(temp.path()).unwrap();
            assert!(lock_path.exists(), "Lock file should exist after acquire");
        }

        // Lock should be released on drop
        assert!(
            !lock_path.exists(),
            "Lock file should be removed after drop"
        );
    }

    /// Test directory structure creation
    #[test]
    fn test_directory_structure() {
        let temp = TempDir::new().unwrap();
        organizer::create_directory_structure(temp.path()).unwrap();

        // Verify all expected directories exist
        assert!(temp.path().join("app-shell").exists());
        assert!(temp.path().join("library-panel").exists());
        assert!(temp.path().join("details-panel").exists());
        assert!(temp.path().join("player-controls").exists());
        assert!(temp.path().join("modals/settings").exists());
        assert!(temp.path().join("modals/keyboard-help").exists());
        assert!(temp.path().join("modals/sleep-timer").exists());
        assert!(temp.path().join("modals/bookmark-editor").exists());
        assert!(temp.path().join("modals/confirmation").exists());
        assert!(temp.path().join("notifications").exists());
        assert!(temp.path().join("responsive").exists());
        assert!(temp.path().join("accessibility").exists());
        assert!(temp.path().join("special-states").exists());
        assert!(temp.path().join("edge-cases").exists());
    }

    /// Test concurrent run prevention
    #[test]
    fn test_concurrent_run_prevention() {
        let temp = TempDir::new().unwrap();

        // Acquire first lock
        let lock1 = lock::LockFile::acquire(temp.path()).unwrap();

        // Second acquire should fail
        let result = lock::LockFile::acquire(temp.path());
        assert!(
            result.is_err(),
            "Second lock acquire should fail when first lock is held"
        );

        drop(lock1);

        // Now should succeed
        let lock2 = lock::LockFile::acquire(temp.path()).unwrap();
        assert!(
            temp.path().join(".screenshot.lock").exists(),
            "Lock file should exist"
        );
        drop(lock2);
    }

    /// Test README generation with screenshots
    #[test]
    fn test_readme_generation_integration() {
        let temp = TempDir::new().unwrap();

        // Create some test screenshots
        let category_dir = temp.path().join("app-shell");
        fs::create_dir_all(&category_dir).unwrap();
        fs::write(category_dir.join("empty-library-light.png"), "").unwrap();
        fs::write(category_dir.join("empty-library-dark.png"), "").unwrap();

        // Generate README
        readme::generate(temp.path()).unwrap();

        // Verify output files
        assert!(temp.path().join("manifest.json").exists());
        assert!(temp.path().join("README.md").exists());

        // Verify README content
        let readme_content = fs::read_to_string(temp.path().join("README.md")).unwrap();
        assert!(readme_content.contains("# Nodoka UI Screenshots"));
        assert!(readme_content.contains("app-shell"));
    }

    /// Test cleanup of previous artifacts
    #[test]
    fn test_cleanup_previous_artifacts_integration() {
        let temp = TempDir::new().unwrap();
        let ui_dir = temp.path().join("ui");
        fs::create_dir_all(&ui_dir).unwrap();

        // Create some artifacts
        let screenshots = temp.path().join("screenshots");
        let test_results = ui_dir.join("test-results-screenshots");
        fs::create_dir_all(&screenshots).unwrap();
        fs::create_dir_all(&test_results).unwrap();
        fs::write(screenshots.join("test.png"), "data").unwrap();
        fs::write(test_results.join("result.txt"), "data").unwrap();

        // Run cleanup
        cleanup::cleanup_previous_artifacts(temp.path(), &ui_dir).unwrap();

        // Verify artifacts are removed
        assert!(!screenshots.exists());
        assert!(!test_results.exists());
    }

    /// Test stale lock removal
    #[test]
    fn test_stale_lock_removal() {
        let temp = TempDir::new().unwrap();
        let lock_path = temp.path().join(".screenshot.lock");

        // Create a stale lock with a non-existent PID
        fs::write(&lock_path, "99999999").unwrap();

        // Acquire should succeed and remove stale lock
        let lock = lock::LockFile::acquire(temp.path()).unwrap();

        // The lock file should now have our PID, not the stale one
        let contents = fs::read_to_string(&lock_path).unwrap();
        assert_ne!(contents, "99999999", "Stale lock should be replaced");

        drop(lock);
    }

    /// Test idempotent directory creation
    #[test]
    fn test_idempotent_directory_creation() {
        let temp = TempDir::new().unwrap();

        // Create directories multiple times
        organizer::create_directory_structure(temp.path()).unwrap();
        organizer::create_directory_structure(temp.path()).unwrap();
        organizer::create_directory_structure(temp.path()).unwrap();

        // All directories should still exist
        assert!(temp.path().join("app-shell").exists());
    }

    /// Test partial file cleanup
    #[test]
    fn test_partial_file_cleanup_integration() {
        let temp = TempDir::new().unwrap();
        let ui_dir = temp.path().join("ui");
        fs::create_dir_all(&ui_dir).unwrap();

        // Create partial files in screenshots
        let screenshots = temp.path().join("screenshots");
        let nested = screenshots.join("category");
        fs::create_dir_all(&nested).unwrap();
        fs::write(screenshots.join("good.png"), "data").unwrap();
        fs::write(screenshots.join("partial.png.tmp"), "partial").unwrap();
        fs::write(nested.join("nested.tmp"), "partial").unwrap();

        // Run cleanup
        cleanup::cleanup_previous_artifacts(temp.path(), &ui_dir).unwrap();

        // All files should be removed (including partial files since whole dir is removed)
        assert!(!screenshots.exists());
    }
}
