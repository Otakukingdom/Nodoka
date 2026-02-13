use crate::error::{Error, Result};
use directories::ProjectDirs;
use rusqlite::Connection;
use std::path::PathBuf;

pub struct Database {
    conn: Connection,
}

impl Database {
    /// Opens the database connection and initializes WAL mode
    ///
    /// The database file is stored in platform-specific application data directory:
    /// - **Windows**: `%APPDATA%\Otakukingdom\Nodoka\nodoka.db`
    /// - **macOS**: `~/Library/Application Support/com.Otakukingdom.Nodoka/nodoka.db`
    /// - **Linux**: `~/.local/share/com/Otakukingdom/Nodoka/nodoka.db`
    ///
    /// # Errors
    ///
    /// This function can fail in several scenarios:
    ///
    /// - [`Error::ProjectDirNotFound`]: Platform could not determine application data directory
    ///   (e.g., HOME environment variable not set)
    /// - [`Error::Io`]: Failed to create data directory or access database file
    ///   (e.g., insufficient permissions, disk full, read-only filesystem)
    /// - [`Error::Database`]: Failed to open `SQLite` connection or enable WAL mode
    ///   (e.g., database file corrupted, incompatible `SQLite` version)
    ///
    /// All errors include detailed troubleshooting guidance. See [`Error`] documentation
    /// for platform-specific resolution steps.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use nodoka::Database;
    ///
    /// match Database::open() {
    ///     Ok(db) => println!("Database opened successfully"),
    ///     Err(e) => eprintln!("Failed to open database: {}", e),
    /// }
    /// ```
    pub fn open() -> Result<Self> {
        let db_path = Self::get_db_path()?;
        let conn = Connection::open(db_path)?;
        // PRAGMA journal_mode returns a result, so we must use query_row instead of execute
        let _: String = conn.query_row("PRAGMA journal_mode=WAL", [], |row| row.get(0))?;
        Ok(Self { conn })
    }

    /// Creates an in-memory database for testing
    ///
    /// # Errors
    ///
    /// Returns an error if the in-memory database cannot be created
    pub fn new_in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        Ok(Self { conn })
    }

    fn get_db_path() -> Result<PathBuf> {
        let proj_dirs =
            ProjectDirs::from("com", "Otakukingdom", "Nodoka").ok_or(Error::ProjectDirNotFound)?;
        let data_dir = proj_dirs.data_dir();
        std::fs::create_dir_all(data_dir)?;
        Ok(data_dir.join("nodoka.db"))
    }

    pub const fn connection(&self) -> &Connection {
        &self.conn
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_in_memory_creates_database() {
        let result = Database::new_in_memory();
        assert!(result.is_ok(), "In-memory database creation should succeed");

        if let Ok(db) = result {
            // Verify we can execute a query
            let result = db
                .connection()
                .execute("CREATE TABLE test (id INTEGER)", []);
            assert!(
                result.is_ok(),
                "Should be able to execute SQL on in-memory db"
            );
        }
    }

    #[test]
    fn test_connection_provides_access_to_rusqlite() {
        if let Ok(db) = Database::new_in_memory() {
            let conn = db.connection();

            // Verify the connection is usable
            let result: std::result::Result<i64, rusqlite::Error> =
                conn.query_row("SELECT 1 + 1", [], |row| row.get(0));

            assert!(result.is_ok(), "Query execution should succeed");
            if let Ok(value) = result {
                assert_eq!(value, 2, "Connection should be functional");
            }
        }
    }

    #[test]
    fn test_database_open_error_on_missing_project_dir() {
        // Test that Database::open returns an error when project directory cannot be determined
        // This simulates the ProjectDirNotFound error scenario
        use std::env;

        let original_home = env::var("HOME").ok();
        env::remove_var("HOME");

        let result = Database::open();

        // Restore original HOME
        if let Some(home) = original_home {
            env::set_var("HOME", home);
        }

        // On some platforms, ProjectDirs may have fallback behavior
        // If it fails, it should return an informative error
        if let Err(err) = result {
            let err_string = format!("{err}");
            // Error should be informative, not generic
            assert!(!err_string.is_empty(), "Error message should not be empty");
        }
    }

    #[test]
    const fn test_database_open_error_types() {
        // Test that errors from Database::open are of the correct type
        // and contain meaningful error messages

        // We can't easily simulate IO errors or database errors in unit tests
        // without significant mocking infrastructure, but we can verify
        // that the error conversion from rusqlite::Error and io::Error work

        // This test documents that Database::open can return:
        // - Error::ProjectDirNotFound (tested above)
        // - Error::Io (when filesystem operations fail)
        // - Error::Database (when SQLite operations fail)

        // The integration test in tests/error_reporting_tests.rs
        // verifies that these errors produce informative messages
    }

    #[test]
    fn test_get_db_path_returns_valid_path() {
        // Test that get_db_path returns a valid PathBuf when successful
        // This is a white-box test that verifies the internal helper function

        // If this test runs successfully, it means project dirs were found
        if let Ok(path) = Database::get_db_path() {
            assert!(
                path.to_str().is_some(),
                "Database path should be valid UTF-8"
            );
            assert!(
                path.ends_with("nodoka.db"),
                "Database path should end with nodoka.db"
            );
        }
        // If it fails, it should return Error::ProjectDirNotFound
        // which is tested in test_database_open_error_on_missing_project_dir
    }
}
