use crate::error::Result;
use rusqlite::Connection;

/// Initializes the database schema
///
/// Creates all required tables and indices if they don't already exist.
/// This function is idempotent and safe to call multiple times - existing
/// schema elements will not be modified or recreated.
///
/// ## Tables Created
///
/// - `metadata`: Key-value storage for application settings
/// - `directories`: Tracked audiobook directories
/// - `audiobooks`: Audiobook metadata and progress
/// - `audiobook_file`: Individual audio file tracking
///
/// ## Indices Created
///
/// - `audiobook_dir_index`: Index on `audiobooks.directory`
/// - `audiobook_full_path_index`: Index on `audiobooks.full_path`
/// - `audiobook_ab_id_index`: Index on `audiobook_file.audiobook_id`
/// - `audiobook_file_dir_index`: Index on `audiobook_file.full_path`
///
/// # Errors
///
/// Returns an error if any table or index creation fails due to:
/// - Database connection issues
/// - Insufficient permissions
/// - Disk space limitations
/// - Database corruption
///
/// # Example
///
/// ```no_run
/// # use nodoka::Database;
/// # use nodoka::error::Result;
/// # fn example() -> Result<()> {
/// let db = Database::open()?;
/// nodoka::db::initialize(db.connection())?;
/// # Ok(())
/// # }
/// ```
pub fn initialize(conn: &Connection) -> Result<()> {
    // Create metadata table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS metadata (
            key text PRIMARY KEY,
            value text
        )",
        [],
    )?;

    // Create directories table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS directories (
            full_path text PRIMARY KEY,
            created_at text,
            last_scanned text
        )",
        [],
    )?;

    // Create audiobooks table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS audiobooks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            directory TEXT,
            name TEXT,
            full_path TEXT,
            completeness INTEGER,
            default_order INTEGER,
            selected_file TEXT,
            created_at TEXT
        )",
        [],
    )?;

    // Create audiobook_file table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS audiobook_file (
            audiobook_id INTEGER,
            name TEXT,
            full_path TEXT PRIMARY KEY,
            length_of_file TEXT,
            seek_position TEXT,
            position INTEGER,
            completeness INTEGER,
            file_exists BOOL,
            created_at TEXT
        )",
        [],
    )?;

    // Create bookmarks table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS bookmarks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            audiobook_id INTEGER NOT NULL,
            file_path TEXT NOT NULL,
            position_ms INTEGER NOT NULL,
            label TEXT NOT NULL,
            note TEXT,
            created_at TEXT NOT NULL,
            FOREIGN KEY (audiobook_id) REFERENCES audiobooks(id) ON DELETE CASCADE
        )",
        [],
    )?;

    // Create indices using execute_batch to handle DDL statements correctly
    conn.execute_batch(
        "CREATE INDEX IF NOT EXISTS audiobook_dir_index ON audiobooks(directory);
         CREATE INDEX IF NOT EXISTS audiobook_full_path_index ON audiobooks(full_path);
         CREATE INDEX IF NOT EXISTS audiobook_ab_id_index ON audiobook_file(audiobook_id);
         CREATE INDEX IF NOT EXISTS audiobook_file_dir_index ON audiobook_file(full_path);
         CREATE INDEX IF NOT EXISTS bookmark_audiobook_id_index ON bookmarks(audiobook_id);",
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_creates_all_tables() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        initialize(&conn)?;

        // Verify all tables exist
        let tables: Vec<String> = conn
            .prepare("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name")?
            .query_map([], |row| row.get(0))?
            .collect::<std::result::Result<Vec<_>, _>>()?;

        assert!(tables.contains(&"metadata".to_string()));
        assert!(tables.contains(&"directories".to_string()));
        assert!(tables.contains(&"audiobooks".to_string()));
        assert!(tables.contains(&"audiobook_file".to_string()));

        Ok(())
    }

    #[test]
    fn test_initialize_creates_all_indices() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        initialize(&conn)?;

        // Verify all indices exist
        let indices: Vec<String> = conn
            .prepare("SELECT name FROM sqlite_master WHERE type='index' AND name NOT LIKE 'sqlite_%' ORDER BY name")?
            .query_map([], |row| row.get(0))?
            .collect::<std::result::Result<Vec<_>, _>>()?;

        assert!(indices.contains(&"audiobook_dir_index".to_string()));
        assert!(indices.contains(&"audiobook_full_path_index".to_string()));
        assert!(indices.contains(&"audiobook_ab_id_index".to_string()));
        assert!(indices.contains(&"audiobook_file_dir_index".to_string()));

        Ok(())
    }

    #[test]
    fn test_initialize_is_idempotent() -> Result<()> {
        let conn = Connection::open_in_memory()?;

        // First call
        initialize(&conn)?;

        // Second call should not fail
        initialize(&conn)?;

        // Third call should also succeed
        initialize(&conn)?;

        Ok(())
    }

    #[test]
    fn test_initialize_file_based_database() -> Result<()> {
        // Test with file-based database to ensure it works in production scenario
        let temp_dir = std::env::temp_dir();
        let db_path = temp_dir.join(format!("nodoka_schema_test_{}.db", std::process::id()));

        // Clean up any existing test database
        let _ = std::fs::remove_file(&db_path);

        let conn = Connection::open(&db_path)?;

        // Enable WAL mode like production does - use query instead of execute
        // because PRAGMA returns results
        let _: String = conn.query_row("PRAGMA journal_mode=WAL", [], |row| row.get(0))?;

        // Initialize should work on file-based database
        let result = initialize(&conn);

        // Clean up
        drop(conn);
        let _ = std::fs::remove_file(&db_path);
        let _ = std::fs::remove_file(
            temp_dir.join(format!("nodoka_schema_test_{}.db-wal", std::process::id())),
        );
        let _ = std::fs::remove_file(
            temp_dir.join(format!("nodoka_schema_test_{}.db-shm", std::process::id())),
        );

        result?;

        Ok(())
    }
}
