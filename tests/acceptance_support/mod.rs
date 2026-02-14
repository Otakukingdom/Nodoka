//! Test support utilities for acceptance testing
//!
//! This module provides specialized utilities for acceptance tests including:
//! - Test database creation
//! - Fixture file access
//! - Test audiobook directory generation
//! - Custom assertion helpers

use nodoka::db::{queries, Database};
use nodoka::models::{Audiobook, AudiobookFile};
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use temp_dir::TempDir;

/// Creates a test database with initialized schema
///
/// # Errors
///
/// Returns an error if database creation or initialization fails
pub fn create_test_db() -> Result<Database, Box<dyn Error>> {
    let db = Database::new_in_memory()?;
    nodoka::db::initialize(db.connection())?;
    Ok(db)
}

/// Fixture paths for test audio files
pub struct TestFixtures {
    pub fixtures_root: PathBuf,
}

impl TestFixtures {
    #[must_use]
    pub fn new() -> Self {
        Self {
            fixtures_root: PathBuf::from("tests/fixtures"),
        }
    }

    /// Get path to an audio fixture file
    #[must_use]
    pub fn audio_path(&self, filename: &str) -> PathBuf {
        self.fixtures_root.join("audio").join(filename)
    }

    /// Get path to an archive fixture file
    #[must_use]
    pub fn archive_path(&self, filename: &str) -> PathBuf {
        self.fixtures_root.join("archives").join(filename)
    }

    /// Get path to an image fixture file
    #[must_use]
    pub fn image_path(&self, filename: &str) -> PathBuf {
        self.fixtures_root.join("images").join(filename)
    }
}

impl Default for TestFixtures {
    fn default() -> Self {
        Self::new()
    }
}

/// Creates a test audiobook directory structure with real files
///
/// # Errors
///
/// Returns an error if directory creation or file copying fails
pub fn create_test_audiobook_directory(
    temp: &TempDir,
    name: &str,
    file_count: usize,
) -> Result<PathBuf, Box<dyn Error>> {
    let audiobook_dir = temp.path().join(name);
    fs::create_dir(&audiobook_dir)?;

    let fixtures = TestFixtures::new();
    let source_file = fixtures.audio_path("sample_mp3.mp3");

    for i in 0..file_count {
        let dest = audiobook_dir.join(format!("chapter_{:02}.mp3", i + 1));
        fs::copy(&source_file, &dest)?;
    }

    Ok(audiobook_dir)
}

/// Creates a test audiobook in the database
///
/// # Errors
///
/// Returns an error if database insertion fails
pub fn create_test_audiobook(
    db: &Database,
    directory: &str,
    name: &str,
) -> Result<i64, Box<dyn Error>> {
    let audiobook = Audiobook::new(
        directory.to_string(),
        name.to_string(),
        format!("{directory}/{name}"),
        0,
    );

    queries::insert_audiobook(db.connection(), &audiobook).map_err(Into::into)
}

/// Inserts a test file into the database
///
/// # Errors
///
/// Returns an error if database insertion fails
pub fn insert_test_file(
    db: &Database,
    audiobook_id: i64,
    file_path: &str,
) -> Result<(), Box<dyn Error>> {
    let mut file = AudiobookFile::new(
        audiobook_id,
        Path::new(file_path)
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or("Invalid filename")?
            .to_string(),
        file_path.to_string(),
        0,
    );
    file.length_of_file = Some(3600);
    file.seek_position = Some(0);

    queries::insert_audiobook_file(db.connection(), &file).map_err(Into::into)
}

/// Test assertion helpers
pub mod assertions {
    use super::*;

    /// Asserts that an audiobook with the given name exists and returns its ID
    ///
    /// # Errors
    ///
    /// Returns an error if the audiobook is not found or query fails
    pub fn assert_audiobook_exists(db: &Database, name: &str) -> Result<i64, Box<dyn Error>> {
        let audiobooks = queries::get_all_audiobooks(db.connection())?;
        let found = audiobooks
            .iter()
            .find(|ab| ab.name == name)
            .ok_or_else(|| format!("Audiobook '{name}' not found"))?;
        found.id.ok_or_else(|| "Audiobook has no ID".into())
    }

    /// Asserts that an audiobook has the expected number of files
    ///
    /// # Errors
    ///
    /// Returns an error if file count doesn't match or query fails
    pub fn assert_file_count(
        db: &Database,
        audiobook_id: i64,
        expected: usize,
    ) -> Result<(), Box<dyn Error>> {
        let files = queries::get_audiobook_files(db.connection(), audiobook_id)?;
        if files.len() != expected {
            return Err(format!("Expected {} files, found {}", expected, files.len()).into());
        }
        Ok(())
    }

    /// Asserts that a file exists in the audiobook
    ///
    /// # Errors
    ///
    /// Returns an error if the file is not found
    pub fn assert_file_exists(
        db: &Database,
        audiobook_id: i64,
        filename: &str,
    ) -> Result<(), Box<dyn Error>> {
        let files = queries::get_audiobook_files(db.connection(), audiobook_id)?;
        let _found = files
            .iter()
            .find(|f| f.name == filename)
            .ok_or_else(|| format!("File '{filename}' not found"))?;
        Ok(())
    }

    /// Asserts that playback position is within expected range
    ///
    /// # Errors
    ///
    /// Returns an error if position is out of range
    pub fn assert_position_near(
        actual: f64,
        expected: f64,
        tolerance: f64,
    ) -> Result<(), Box<dyn Error>> {
        let diff = (actual - expected).abs();
        if diff > tolerance {
            return Err(format!(
                "Position {actual} not near expected {expected} (tolerance {tolerance})"
            )
            .into());
        }
        Ok(())
    }

    /// Asserts that an audiobook has expected completion percentage
    ///
    /// # Errors
    ///
    /// Returns an error if percentage doesn't match
    pub fn assert_completion_percentage(
        db: &Database,
        audiobook_id: i64,
        expected: i32,
        tolerance: i32,
    ) -> Result<(), Box<dyn Error>> {
        let audiobook = queries::get_audiobook_by_id(db.connection(), audiobook_id)?
            .ok_or("Audiobook not found")?;

        let diff = (audiobook.completeness - expected).abs();
        if diff > tolerance {
            return Err(format!(
                "Completion {}% not near expected {}% (tolerance {}%)",
                audiobook.completeness, expected, tolerance
            )
            .into());
        }
        Ok(())
    }

    /// Asserts that a bookmark exists at a specific position
    ///
    /// # Errors
    ///
    /// Returns an error if bookmark not found
    pub fn assert_bookmark_at_position(
        db: &Database,
        audiobook_id: i64,
        position_ms: i64,
        tolerance_ms: i64,
    ) -> Result<(), Box<dyn Error>> {
        let bookmarks = queries::get_bookmarks_for_audiobook(db.connection(), audiobook_id)?;
        let _found = bookmarks
            .iter()
            .find(|b| (b.position_ms - position_ms).abs() < tolerance_ms)
            .ok_or_else(|| format!("No bookmark near position {position_ms}ms"))?;
        Ok(())
    }
}

#[cfg(test)]
mod internal_smoke {
    use super::*;
    use nodoka::models::Bookmark;

    #[test]
    fn test_acceptance_support_helpers_compile_and_run() -> Result<(), Box<dyn Error>> {
        let db = create_test_db()?;

        let fixtures = TestFixtures::new();
        let _ = fixtures.audio_path("sample_mp3.mp3");
        let _ = fixtures.archive_path("sample.zip");
        let _ = fixtures.image_path("cover.jpg");

        let temp = TempDir::new()?;
        let _dir = create_test_audiobook_directory(&temp, "Book", 1)?;

        let audiobook_id = create_test_audiobook(&db, "/test", "Book")?;
        insert_test_file(&db, audiobook_id, "/test/Book/ch1.mp3")?;

        let bookmark = Bookmark::new(
            audiobook_id,
            "/test/Book/ch1.mp3".to_string(),
            1000,
            "Test".to_string(),
        );
        let _ = queries::insert_bookmark(db.connection(), &bookmark)?;

        let resolved = assertions::assert_audiobook_exists(&db, "Book")?;
        assertions::assert_file_count(&db, resolved, 1)?;
        assertions::assert_file_exists(&db, resolved, "ch1.mp3")?;
        assertions::assert_position_near(10.0, 10.0, 0.0)?;
        assertions::assert_completion_percentage(&db, resolved, 0, 0)?;
        assertions::assert_bookmark_at_position(&db, resolved, 1000, 1)?;

        Ok(())
    }
}
