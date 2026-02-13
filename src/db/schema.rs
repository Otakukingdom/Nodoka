use crate::error::Result;
use rusqlite::Connection;

/// Initializes the database schema
///
/// # Errors
///
/// Returns an error if any table or index creation fails
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

    // Create indices
    conn.execute(
        "CREATE INDEX IF NOT EXISTS audiobook_dir_index ON audiobooks(directory)",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS audiobook_full_path_index ON audiobooks(full_path)",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS audiobook_ab_id_index ON audiobook_file(audiobook_id)",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS audiobook_file_dir_index ON audiobook_file(full_path)",
        [],
    )?;

    Ok(())
}
