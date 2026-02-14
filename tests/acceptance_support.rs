//! Test support utilities for acceptance testing
//!
//! This module provides specialized utilities for acceptance tests including:
//! - Test database creation
//! - Fixture file access
//! - Test audiobook directory generation
//! - Custom assertion helpers

#![allow(dead_code)]

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
        format!("{}/{}", directory, name),
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
            .ok_or_else(|| format!("Audiobook '{}' not found", name))?;
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
}
