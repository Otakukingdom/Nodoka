use crate::db::queries::{get_metadata, set_metadata};
use crate::error::Result;
use rusqlite::Connection;

pub struct Settings<'a> {
    conn: &'a Connection,
}

impl<'a> Settings<'a> {
    #[must_use]
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    /// Gets the volume setting
    ///
    /// # Errors
    ///
    /// Returns an error if the database query fails
    pub fn get_volume(&self) -> Result<i32> {
        match get_metadata(self.conn, "volume")? {
            Some(v) => v.parse().or(Ok(100)),
            None => Ok(100),
        }
    }

    /// Sets the volume setting
    ///
    /// # Errors
    ///
    /// Returns an error if the database update fails
    pub fn set_volume(&self, volume: i32) -> Result<()> {
        set_metadata(self.conn, "volume", &volume.to_string())
    }

    /// Gets the playback speed setting
    ///
    /// # Errors
    ///
    /// Returns an error if the database query fails
    pub fn get_speed(&self) -> Result<f32> {
        match get_metadata(self.conn, "speed")? {
            Some(v) => v.parse().or(Ok(1.0)),
            None => Ok(1.0),
        }
    }

    /// Sets the playback speed setting
    ///
    /// # Errors
    ///
    /// Returns an error if the database update fails
    pub fn set_speed(&self, speed: f32) -> Result<()> {
        set_metadata(self.conn, "speed", &speed.to_string())
    }

    /// Gets the current audiobook ID
    ///
    /// # Errors
    ///
    /// Returns an error if the database query fails
    pub fn get_current_audiobook(&self) -> Result<Option<i64>> {
        match get_metadata(self.conn, "current_audiobook_id")? {
            Some(v) => v.parse().ok().map(Some).ok_or_else(|| {
                crate::error::NodokaError::InvalidState("Invalid audiobook ID".to_string())
            }),
            None => Ok(None),
        }
    }

    /// Sets the current audiobook ID
    ///
    /// # Errors
    ///
    /// Returns an error if the database update fails
    pub fn set_current_audiobook(&self, id: i64) -> Result<()> {
        set_metadata(self.conn, "current_audiobook_id", &id.to_string())
    }

    /// Gets the current file path
    ///
    /// # Errors
    ///
    /// Returns an error if the database query fails
    pub fn get_current_file(&self) -> Result<Option<String>> {
        get_metadata(self.conn, "current_file")
    }

    /// Sets the current file path
    ///
    /// # Errors
    ///
    /// Returns an error if the database update fails
    pub fn set_current_file(&self, path: &str) -> Result<()> {
        set_metadata(self.conn, "current_file", path)
    }
}
