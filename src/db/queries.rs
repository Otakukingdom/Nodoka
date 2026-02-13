use crate::error::Result;
use crate::models::{Audiobook, AudiobookFile, Directory};
use chrono::{DateTime, Utc};
use rusqlite::{params, Connection, OptionalExtension};

/// Inserts a new audiobook into the database
///
/// # Errors
///
/// Returns an error if the database insert fails
pub fn insert_audiobook(conn: &Connection, audiobook: &Audiobook) -> Result<i64> {
    conn.execute(
        "INSERT INTO audiobooks (directory, name, full_path, completeness, default_order, selected_file, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            audiobook.directory,
            audiobook.name,
            audiobook.full_path,
            audiobook.completeness,
            audiobook.default_order,
            audiobook.selected_file,
            audiobook.created_at.to_rfc3339(),
        ],
    )?;
    Ok(conn.last_insert_rowid())
}

/// Gets all audiobooks for a specific directory
///
/// # Errors
///
/// Returns an error if the database query fails
pub fn get_audiobooks_by_directory(conn: &Connection, directory: &str) -> Result<Vec<Audiobook>> {
    let mut stmt = conn.prepare(
        "SELECT id, directory, name, full_path, completeness, default_order, selected_file, created_at
         FROM audiobooks WHERE directory = ?1 ORDER BY default_order"
    )?;

    let rows = stmt.query_map([directory], |row| {
        let created_str: String = row.get(7)?;
        let created_at = DateTime::parse_from_rfc3339(&created_str)
            .map_or_else(|_| Utc::now(), |dt| dt.with_timezone(&Utc));

        Ok(Audiobook {
            id: Some(row.get(0)?),
            directory: row.get(1)?,
            name: row.get(2)?,
            full_path: row.get(3)?,
            completeness: row.get(4)?,
            default_order: row.get(5)?,
            selected_file: row.get(6)?,
            created_at,
        })
    })?;

    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(Into::into)
}

/// Gets all audiobooks from all directories
///
/// # Errors
///
/// Returns an error if the database query fails
pub fn get_all_audiobooks(conn: &Connection) -> Result<Vec<Audiobook>> {
    let mut stmt = conn.prepare(
        "SELECT id, directory, name, full_path, completeness, default_order, selected_file, created_at
         FROM audiobooks ORDER BY default_order"
    )?;

    let rows = stmt.query_map([], |row| {
        let created_str: String = row.get(7)?;
        let created_at = DateTime::parse_from_rfc3339(&created_str)
            .map_or_else(|_| Utc::now(), |dt| dt.with_timezone(&Utc));

        Ok(Audiobook {
            id: Some(row.get(0)?),
            directory: row.get(1)?,
            name: row.get(2)?,
            full_path: row.get(3)?,
            completeness: row.get(4)?,
            default_order: row.get(5)?,
            selected_file: row.get(6)?,
            created_at,
        })
    })?;

    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(Into::into)
}

/// Gets a specific audiobook by ID
///
/// # Errors
///
/// Returns an error if the database query fails
pub fn get_audiobook_by_id(conn: &Connection, id: i64) -> Result<Option<Audiobook>> {
    let result = conn.query_row(
        "SELECT id, directory, name, full_path, completeness, default_order, selected_file, created_at
         FROM audiobooks WHERE id = ?1",
        [id],
        |row| {
            let created_str: String = row.get(7)?;
            let created_at = DateTime::parse_from_rfc3339(&created_str)
                .map_or_else(|_| Utc::now(), |dt| dt.with_timezone(&Utc));

            Ok(Audiobook {
                id: Some(row.get(0)?),
                directory: row.get(1)?,
                name: row.get(2)?,
                full_path: row.get(3)?,
                completeness: row.get(4)?,
                default_order: row.get(5)?,
                selected_file: row.get(6)?,
                created_at,
            })
        }
    ).optional()?;

    Ok(result)
}

/// Updates audiobook completeness
///
/// # Errors
///
/// Returns an error if the database update fails
pub fn update_audiobook_completeness(conn: &Connection, id: i64, completeness: i32) -> Result<()> {
    conn.execute(
        "UPDATE audiobooks SET completeness = ?1 WHERE id = ?2",
        params![completeness, id],
    )?;
    Ok(())
}

/// Updates the selected file for an audiobook
///
/// # Errors
///
/// Returns an error if the database update fails
pub fn update_audiobook_selected_file(
    conn: &Connection,
    id: i64,
    file_path: Option<&str>,
) -> Result<()> {
    conn.execute(
        "UPDATE audiobooks SET selected_file = ?1 WHERE id = ?2",
        params![file_path, id],
    )?;
    Ok(())
}

/// Deletes an audiobook and all its files
///
/// # Errors
///
/// Returns an error if the database delete fails
pub fn delete_audiobook(conn: &Connection, id: i64) -> Result<()> {
    conn.execute("DELETE FROM audiobook_file WHERE audiobook_id = ?1", [id])?;
    conn.execute("DELETE FROM audiobooks WHERE id = ?1", [id])?;
    Ok(())
}

/// Inserts a new audiobook file
///
/// # Errors
///
/// Returns an error if the database insert fails
pub fn insert_audiobook_file(conn: &Connection, file: &AudiobookFile) -> Result<()> {
    conn.execute(
        "INSERT OR REPLACE INTO audiobook_file 
         (audiobook_id, name, full_path, length_of_file, seek_position, position, completeness, file_exists, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        params![
            file.audiobook_id,
            file.name,
            file.full_path,
            file.length_of_file.map(|v| v.to_string()),
            file.seek_position.map(|v| v.to_string()),
            file.position,
            file.completeness,
            file.file_exists,
            file.created_at.to_rfc3339(),
        ],
    )?;
    Ok(())
}

/// Gets all files for a specific audiobook
///
/// # Errors
///
/// Returns an error if the database query fails
pub fn get_audiobook_files(conn: &Connection, audiobook_id: i64) -> Result<Vec<AudiobookFile>> {
    let mut stmt = conn.prepare(
        "SELECT audiobook_id, name, full_path, length_of_file, seek_position, position, completeness, file_exists, created_at
         FROM audiobook_file WHERE audiobook_id = ?1 ORDER BY position"
    )?;

    let rows = stmt.query_map([audiobook_id], |row| {
        let length_str: Option<String> = row.get(3)?;
        let seek_str: Option<String> = row.get(4)?;
        let created_str: String = row.get(8)?;
        let created_at = DateTime::parse_from_rfc3339(&created_str)
            .map_or_else(|_| Utc::now(), |dt| dt.with_timezone(&Utc));

        Ok(AudiobookFile {
            audiobook_id: row.get(0)?,
            name: row.get(1)?,
            full_path: row.get(2)?,
            length_of_file: length_str.and_then(|s| s.parse().ok()),
            seek_position: seek_str.and_then(|s| s.parse().ok()),
            position: row.get(5)?,
            completeness: row.get(6)?,
            file_exists: row.get(7)?,
            created_at,
        })
    })?;

    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(Into::into)
}

/// Updates file seek position and completeness
///
/// # Errors
///
/// Returns an error if the database update fails
pub fn update_file_progress(
    conn: &Connection,
    full_path: &str,
    seek_position: f64,
    completeness: i32,
) -> Result<()> {
    let position_ms = seek_position.round() as i64;
    let position_text = position_ms.to_string();
    conn.execute(
        "UPDATE audiobook_file SET seek_position = ?1, completeness = ?2 WHERE full_path = ?3",
        params![position_text, completeness, full_path],
    )?;
    Ok(())
}

/// Inserts a new directory
///
/// # Errors
///
/// Returns an error if the database insert fails
pub fn insert_directory(conn: &Connection, directory: &Directory) -> Result<()> {
    conn.execute(
        "INSERT OR REPLACE INTO directories (full_path, created_at, last_scanned)
         VALUES (?1, ?2, ?3)",
        params![
            directory.full_path,
            directory.created_at.to_rfc3339(),
            directory
                .last_scanned
                .as_ref()
                .map(chrono::DateTime::to_rfc3339),
        ],
    )?;
    Ok(())
}

/// Gets all directories
///
/// # Errors
///
/// Returns an error if the database query fails
pub fn get_all_directories(conn: &Connection) -> Result<Vec<Directory>> {
    let mut stmt = conn.prepare("SELECT full_path, created_at, last_scanned FROM directories")?;

    let rows = stmt.query_map([], |row| {
        let created_str: String = row.get(1)?;
        let created_at = DateTime::parse_from_rfc3339(&created_str)
            .map_or_else(|_| Utc::now(), |dt| dt.with_timezone(&Utc));

        let last_scanned_str: Option<String> = row.get(2)?;
        let last_scanned = last_scanned_str.and_then(|s| {
            DateTime::parse_from_rfc3339(&s)
                .map(|dt| dt.with_timezone(&Utc))
                .ok()
        });

        Ok(Directory {
            full_path: row.get(0)?,
            created_at,
            last_scanned,
        })
    })?;

    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(Into::into)
}

/// Deletes a directory and all associated audiobooks
///
/// # Errors
///
/// Returns an error if the database delete fails
pub fn delete_directory(conn: &Connection, path: &str) -> Result<()> {
    // First get all audiobook IDs in this directory
    let mut stmt = conn.prepare("SELECT id FROM audiobooks WHERE directory = ?1")?;
    let ids: Vec<i64> = stmt
        .query_map([path], |row| row.get(0))?
        .collect::<rusqlite::Result<Vec<_>>>()?;

    // Delete all files for these audiobooks
    for id in ids {
        conn.execute("DELETE FROM audiobook_file WHERE audiobook_id = ?1", [id])?;
    }

    // Delete all audiobooks in this directory
    conn.execute("DELETE FROM audiobooks WHERE directory = ?1", [path])?;

    // Delete the directory itself
    conn.execute("DELETE FROM directories WHERE full_path = ?1", [path])?;

    Ok(())
}

/// Gets a metadata value
///
/// # Errors
///
/// Returns an error if the database query fails
pub fn get_metadata(conn: &Connection, key: &str) -> Result<Option<String>> {
    let result = conn
        .query_row("SELECT value FROM metadata WHERE key = ?1", [key], |row| {
            row.get(0)
        })
        .optional()?;

    Ok(result)
}

/// Sets a metadata value
///
/// # Errors
///
/// Returns an error if the database insert/update fails
pub fn set_metadata(conn: &Connection, key: &str, value: &str) -> Result<()> {
    conn.execute(
        "INSERT OR REPLACE INTO metadata (key, value) VALUES (?1, ?2)",
        params![key, value],
    )?;
    Ok(())
}

/// Deletes a metadata value
///
/// # Errors
///
/// Returns an error if the database delete fails
pub fn delete_metadata(conn: &Connection, key: &str) -> Result<()> {
    conn.execute("DELETE FROM metadata WHERE key = ?1", [key])?;
    Ok(())
}

/// Updates the last scanned timestamp for a directory
///
/// # Errors
///
/// Returns an error if the database update fails
pub fn update_directory_last_scanned(conn: &Connection, path: &str) -> Result<()> {
    conn.execute(
        "UPDATE directories SET last_scanned = ?1 WHERE full_path = ?2",
        params![Utc::now().to_rfc3339(), path],
    )?;
    Ok(())
}

/// Gets audiobook by full path
///
/// # Errors
///
/// Returns an error if the database query fails
pub fn get_audiobook_by_path(conn: &Connection, path: &str) -> Result<Option<Audiobook>> {
    let result = conn.query_row(
        "SELECT id, directory, name, full_path, completeness, default_order, selected_file, created_at
         FROM audiobooks WHERE full_path = ?1",
        [path],
        |row| {
            let created_str: String = row.get(7)?;
            let created_at = DateTime::parse_from_rfc3339(&created_str)
                .map_or_else(|_| Utc::now(), |dt| dt.with_timezone(&Utc));

            Ok(Audiobook {
                id: Some(row.get(0)?),
                directory: row.get(1)?,
                name: row.get(2)?,
                full_path: row.get(3)?,
                completeness: row.get(4)?,
                default_order: row.get(5)?,
                selected_file: row.get(6)?,
                created_at,
            })
        }
    ).optional()?;

    Ok(result)
}

/// Gets a single audiobook file by full path
///
/// # Errors
///
/// Returns an error if the database query fails
pub fn get_audiobook_file_by_path(conn: &Connection, path: &str) -> Result<Option<AudiobookFile>> {
    let result = conn.query_row(
        "SELECT audiobook_id, name, full_path, length_of_file, seek_position, position, completeness, file_exists, created_at
         FROM audiobook_file WHERE full_path = ?1",
        [path],
        |row| {
            let length_str: Option<String> = row.get(3)?;
            let seek_str: Option<String> = row.get(4)?;
            let created_str: String = row.get(8)?;
        let created_at = DateTime::parse_from_rfc3339(&created_str)
            .map_or_else(|_| Utc::now(), |dt| dt.with_timezone(&Utc));

            Ok(AudiobookFile {
                audiobook_id: row.get(0)?,
                name: row.get(1)?,
                full_path: row.get(2)?,
                length_of_file: length_str.and_then(|s| s.parse().ok()),
                seek_position: seek_str.and_then(|s| s.parse().ok()),
                position: row.get(5)?,
                completeness: row.get(6)?,
                file_exists: row.get(7)?,
                created_at,
            })
        }
    ).optional()?;

    Ok(result)
}

/// Marks all files in an audiobook as not existing (for rescan)
///
/// # Errors
///
/// Returns an error if the database update fails
pub fn mark_audiobook_files_missing(conn: &Connection, audiobook_id: i64) -> Result<()> {
    conn.execute(
        "UPDATE audiobook_file SET file_exists = 0 WHERE audiobook_id = ?1",
        [audiobook_id],
    )?;
    Ok(())
}

/// Marks a specific file as existing
///
/// # Errors
///
/// Returns an error if the database update fails
pub fn mark_file_exists(conn: &Connection, path: &str, exists: bool) -> Result<()> {
    conn.execute(
        "UPDATE audiobook_file SET file_exists = ?1 WHERE full_path = ?2",
        params![exists, path],
    )?;
    Ok(())
}

/// Updates the length of a file
///
/// # Errors
///
/// Returns an error if the database update fails
pub fn update_file_length(conn: &Connection, path: &str, length_ms: i64) -> Result<()> {
    conn.execute(
        "UPDATE audiobook_file SET length_of_file = ?1 WHERE full_path = ?2",
        params![length_ms.to_string(), path],
    )?;
    Ok(())
}

/// Resets progress for an audiobook (all files back to 0%)
///
/// # Errors
///
/// Returns an error if the database update fails
pub fn reset_audiobook_progress(conn: &Connection, audiobook_id: i64) -> Result<()> {
    conn.execute(
        "UPDATE audiobook_file SET seek_position = NULL, completeness = 0 WHERE audiobook_id = ?1",
        [audiobook_id],
    )?;
    conn.execute(
        "UPDATE audiobooks SET completeness = 0, selected_file = NULL WHERE id = ?1",
        [audiobook_id],
    )?;
    Ok(())
}

/// Marks an audiobook as complete
///
/// # Errors
///
/// Returns an error if the database update fails
pub fn mark_audiobook_complete(conn: &Connection, audiobook_id: i64) -> Result<()> {
    conn.execute(
        "UPDATE audiobook_file SET completeness = 100 WHERE audiobook_id = ?1",
        [audiobook_id],
    )?;
    conn.execute(
        "UPDATE audiobooks SET completeness = 100 WHERE id = ?1",
        [audiobook_id],
    )?;
    Ok(())
}

/// Gets the count of audiobooks
///
/// # Errors
///
/// Returns an error if the database query fails
pub fn count_audiobooks(conn: &Connection) -> Result<i64> {
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM audiobooks", [], |row| row.get(0))?;
    Ok(count)
}

/// Gets the count of files for an audiobook
///
/// # Errors
///
/// Returns an error if the database query fails
pub fn count_audiobook_files(conn: &Connection, audiobook_id: i64) -> Result<i64> {
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM audiobook_file WHERE audiobook_id = ?1",
        [audiobook_id],
        |row| row.get(0),
    )?;
    Ok(count)
}
