use crate::db::queries::{get_metadata, set_metadata};
use crate::error::Result;
use rusqlite::Connection;

pub struct Settings<'a> {
    conn: &'a Connection,
}

impl<'a> Settings<'a> {
    #[must_use]
    pub const fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    /// Gets the volume setting
    ///
    /// # Errors
    ///
    /// Returns an error if the database query fails
    pub fn get_volume(&self) -> Result<i32> {
        (get_metadata(self.conn, "volume")?).map_or_else(|| Ok(100), |v| v.parse().or(Ok(100)))
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
        (get_metadata(self.conn, "speed")?).map_or_else(|| Ok(1.0), |v| v.parse().or(Ok(1.0)))
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
        (get_metadata(self.conn, "current_audiobook_id")?).map_or_else(
            || Ok(None),
            |v| {
                v.parse().ok().map(Some).ok_or_else(|| {
                    crate::error::Error::InvalidState("Invalid audiobook ID".to_string())
                })
            },
        )
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::Database;

    fn create_test_db() -> Result<Database> {
        let db = Database::new_in_memory()?;
        crate::db::initialize(db.connection())?;
        Ok(db)
    }

    #[test]
    fn test_volume_default_value() -> Result<()> {
        let db = create_test_db()?;
        let settings = Settings::new(db.connection());
        assert_eq!(settings.get_volume()?, 100);
        Ok(())
    }

    #[test]
    fn test_volume_persistence() -> Result<()> {
        let db = create_test_db()?;
        let settings = Settings::new(db.connection());

        settings.set_volume(75)?;
        assert_eq!(settings.get_volume()?, 75);

        settings.set_volume(0)?;
        assert_eq!(settings.get_volume()?, 0);

        settings.set_volume(200)?;
        assert_eq!(settings.get_volume()?, 200);

        Ok(())
    }

    #[test]
    fn test_speed_default_value() -> Result<()> {
        let db = create_test_db()?;
        let settings = Settings::new(db.connection());
        assert!((settings.get_speed()? - 1.0).abs() < f32::EPSILON);
        Ok(())
    }

    #[test]
    fn test_speed_persistence() -> Result<()> {
        let db = create_test_db()?;
        let settings = Settings::new(db.connection());

        settings.set_speed(1.5)?;
        assert!((settings.get_speed()? - 1.5).abs() < f32::EPSILON);

        settings.set_speed(0.5)?;
        assert!((settings.get_speed()? - 0.5).abs() < f32::EPSILON);

        settings.set_speed(2.0)?;
        assert!((settings.get_speed()? - 2.0).abs() < f32::EPSILON);

        Ok(())
    }

    #[test]
    fn test_current_audiobook_default_value() -> Result<()> {
        let db = create_test_db()?;
        let settings = Settings::new(db.connection());
        assert_eq!(settings.get_current_audiobook()?, None);
        Ok(())
    }

    #[test]
    fn test_current_audiobook_persistence() -> Result<()> {
        let db = create_test_db()?;
        let settings = Settings::new(db.connection());

        settings.set_current_audiobook(42)?;
        assert_eq!(settings.get_current_audiobook()?, Some(42));

        settings.set_current_audiobook(100)?;
        assert_eq!(settings.get_current_audiobook()?, Some(100));

        Ok(())
    }

    #[test]
    fn test_current_file_default_value() -> Result<()> {
        let db = create_test_db()?;
        let settings = Settings::new(db.connection());
        assert_eq!(settings.get_current_file()?, None);
        Ok(())
    }

    #[test]
    fn test_current_file_persistence() -> Result<()> {
        let db = create_test_db()?;
        let settings = Settings::new(db.connection());

        settings.set_current_file("/path/to/file.mp3")?;
        assert_eq!(
            settings.get_current_file()?,
            Some("/path/to/file.mp3".to_string())
        );

        settings.set_current_file("/another/path.mp3")?;
        assert_eq!(
            settings.get_current_file()?,
            Some("/another/path.mp3".to_string())
        );

        Ok(())
    }
}
