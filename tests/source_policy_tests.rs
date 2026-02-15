//! Source policy regression tests.
//!
//! These tests enforce repository constraints from `AGENTS.md` that are easy to
//! accidentally violate during refactors.

use std::fs;
use std::path::{Path, PathBuf};
use std::{error::Error, result};

fn assert_lines_under_1000(path: &Path) -> result::Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(path)?;
    let lines = content.lines().count();
    assert!(
        lines <= 1000,
        "{} must be <= 1000 lines (found {lines})",
        path.display()
    );

    Ok(())
}

fn repo_path(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(rel)
}

#[test]
fn test_ui_update_rs_under_1000_lines() -> result::Result<(), Box<dyn Error>> {
    assert_lines_under_1000(&repo_path("src/ui/update.rs"))
}

#[test]
fn test_ui_update_test_modules_under_1000_lines() -> result::Result<(), Box<dyn Error>> {
    let dir = repo_path("src/ui/update/tests");

    for entry in fs::read_dir(&dir)? {
        let path = entry?.path();

        if path.extension().and_then(|s| s.to_str()) != Some("rs") {
            continue;
        }

        assert_lines_under_1000(&path)?;
    }

    Ok(())
}
