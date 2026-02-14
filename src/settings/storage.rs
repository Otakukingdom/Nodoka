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

    /// Gets the last audiobook ID (alias for get_current_audiobook)
    ///
    /// # Errors
    ///
    /// Returns an error if the database query fails
    pub fn get_last_audiobook_id(&self) -> Result<Option<i64>> {
        self.get_current_audiobook()
    }

    /// Sets the last audiobook ID (alias for set_current_audiobook)
    ///
    /// # Errors
    ///
    /// Returns an error if the database update fails
    pub fn set_last_audiobook_id(&self, id: i64) -> Result<()> {
        self.set_current_audiobook(id)
    }

    /// Gets the default speed setting (alias for get_speed)
    ///
    /// # Errors
    ///
    /// Returns an error if the database query fails
    pub fn get_default_speed(&self) -> Result<Option<f32>> {
        Ok(Some(self.get_speed()?))
    }

    /// Sets the default speed setting (alias for set_speed)
    ///
    /// # Errors
    ///
    /// Returns an error if the database update fails or speed is out of range
    pub fn set_default_speed(&self, speed: f32) -> Result<()> {
        if !(0.5..=2.0).contains(&speed) {
            return Err(crate::error::Error::InvalidState(format!(
                "Speed {} out of range (0.5-2.0)",
                speed
            )));
        }
        self.set_speed(speed)
    }

    /// Gets the default volume setting (alias for get_volume)
    ///
    /// # Errors
    ///
    /// Returns an error if the database query fails
    pub fn get_default_volume(&self) -> Result<Option<i32>> {
        Ok(Some(self.get_volume()?))
    }

    /// Sets the default volume setting (alias for set_volume)
    ///
    /// # Errors
    ///
    /// Returns an error if the database update fails or volume is out of range
    pub fn set_default_volume(&self, volume: i32) -> Result<()> {
        if !(0..=200).contains(&volume) {
            return Err(crate::error::Error::InvalidState(format!(
                "Volume {} out of range (0-200)",
                volume
            )));
        }
        self.set_volume(volume)
    }

    /// Gets window position
    ///
    /// # Errors
    ///
    /// Returns an error if the database query fails
    pub fn get_window_position(&self) -> Result<Option<(i32, i32)>> {
        let x = get_metadata(self.conn, "window_x")?;
        let y = get_metadata(self.conn, "window_y")?;

        match (x, y) {
            (Some(x_str), Some(y_str)) => {
                let x_val: i32 = x_str.parse().unwrap_or(0);
                let y_val: i32 = y_str.parse().unwrap_or(0);
                Ok(Some((x_val, y_val)))
            }
            _ => Ok(None),
        }
    }

    /// Sets window position
    ///
    /// # Errors
    ///
    /// Returns an error if the database update fails
    pub fn set_window_position(&self, x: i32, y: i32) -> Result<()> {
        set_metadata(self.conn, "window_x", &x.to_string())?;
        set_metadata(self.conn, "window_y", &y.to_string())?;
        Ok(())
    }

    /// Gets window size
    ///
    /// # Errors
    ///
    /// Returns an error if the database query fails
    pub fn get_window_size(&self) -> Result<Option<(i32, i32)>> {
        let w = get_metadata(self.conn, "window_width")?;
        let h = get_metadata(self.conn, "window_height")?;

        match (w, h) {
            (Some(w_str), Some(h_str)) => {
                let w_val: i32 = w_str.parse().unwrap_or(800);
                let h_val: i32 = h_str.parse().unwrap_or(600);
                Ok(Some((w_val, h_val)))
            }
            _ => Ok(None),
        }
    }

    /// Sets window size
    ///
    /// # Errors
    ///
    /// Returns an error if the database update fails
    pub fn set_window_size(&self, width: i32, height: i32) -> Result<()> {
        set_metadata(self.conn, "window_width", &width.to_string())?;
        set_metadata(self.conn, "window_height", &height.to_string())?;
        Ok(())
    }

    /// Sets skip forward duration in seconds
    ///
    /// # Errors
    ///
    /// Returns an error if the database update fails
    pub fn set_skip_forward_duration(&self, seconds: i32) -> Result<()> {
        set_metadata(self.conn, "skip_forward_duration", &seconds.to_string())
    }

    /// Gets skip forward duration in seconds
    ///
    /// # Errors
    ///
    /// Returns an error if the database query fails
    pub fn get_skip_forward_duration(&self) -> Result<Option<i32>> {
        Ok(get_metadata(self.conn, "skip_forward_duration")?.and_then(|s| s.parse().ok()))
    }

    /// Sets skip backward duration in seconds
    ///
    /// # Errors
    ///
    /// Returns an error if the database update fails
    pub fn set_skip_backward_duration(&self, seconds: i32) -> Result<()> {
        set_metadata(self.conn, "skip_backward_duration", &seconds.to_string())
    }

    /// Gets skip backward duration in seconds
    ///
    /// # Errors
    ///
    /// Returns an error if the database query fails
    pub fn get_skip_backward_duration(&self) -> Result<Option<i32>> {
        Ok(get_metadata(self.conn, "skip_backward_duration")?.and_then(|s| s.parse().ok()))
    }

    /// Sets auto-save interval in seconds
    ///
    /// # Errors
    ///
    /// Returns an error if the database update fails or interval is out of range
    pub fn set_auto_save_interval(&self, seconds: i32) -> Result<()> {
        if !(1..=60).contains(&seconds) {
            return Err(crate::error::Error::InvalidState(format!(
                "Auto-save interval {} out of range (1-60 seconds)",
                seconds
            )));
        }
        set_metadata(self.conn, "auto_save_interval", &seconds.to_string())
    }

    /// Gets auto-save interval in seconds
    ///
    /// # Errors
    ///
    /// Returns an error if the database query fails
    pub fn get_auto_save_interval(&self) -> Result<Option<i32>> {
        Ok(get_metadata(self.conn, "auto_save_interval")?.and_then(|s| s.parse().ok()))
    }

    /// Resets all settings to defaults
    ///
    /// # Errors
    ///
    /// Returns an error if the database update fails
    pub fn reset_to_defaults(&self) -> Result<()> {
        self.set_speed(1.0)?;
        self.set_volume(100)?;
        Ok(())
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
