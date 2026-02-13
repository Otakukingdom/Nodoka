use crate::error::{NodokaError, Result};
use directories::ProjectDirs;
use rusqlite::Connection;
use std::path::PathBuf;

pub struct Database {
    conn: Connection,
}

impl Database {
    /// Opens the database connection and initializes WAL mode
    ///
    /// # Errors
    ///
    /// Returns an error if the database cannot be opened or WAL mode cannot be set
    pub fn open() -> Result<Self> {
        let db_path = Self::get_db_path()?;
        let conn = Connection::open(db_path)?;
        conn.execute("PRAGMA journal_mode=WAL", [])?;
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
        let proj_dirs = ProjectDirs::from("com", "Otakukingdom", "Nodoka")
            .ok_or(NodokaError::ProjectDirNotFound)?;
        let data_dir = proj_dirs.data_dir();
        std::fs::create_dir_all(data_dir)?;
        Ok(data_dir.join("nodoka.db"))
    }

    pub const fn connection(&self) -> &Connection {
        &self.conn
    }
}
