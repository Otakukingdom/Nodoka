use crate::db::Database;
use crate::models::AudiobookFile;
use std::sync::Arc;

#[allow(clippy::module_name_repetitions)] // Proxy pattern naming is idiomatic
#[derive(Clone)]
pub struct AudiobookFileProxy {
    data: AudiobookFile,
    db: Arc<Database>,
}

impl AudiobookFileProxy {
    pub const fn new(data: AudiobookFile, db: Arc<Database>) -> Self {
        Self { data, db }
    }

    #[must_use]
    pub const fn completeness(&self) -> i32 {
        self.data.completeness
    }

    #[must_use]
    pub const fn data(&self) -> &AudiobookFile {
        &self.data
    }

    /// Gets a reference to the database connection
    #[must_use]
    pub const fn database(&self) -> &Arc<Database> {
        &self.db
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;

    #[allow(clippy::arc_with_non_send_sync)] // Test helper function
    fn create_test_db() -> Result<Arc<Database>, crate::error::Error> {
        let database = Database::new_in_memory()?;
        db::initialize(database.connection())?;
        Ok(Arc::new(database))
    }

    #[test]
    fn test_file_proxy_creation() -> Result<(), crate::error::Error> {
        let db = create_test_db()?;
        let file = AudiobookFile::new(1, "file.mp3".to_string(), "/test/file.mp3".to_string(), 0);

        let proxy = AudiobookFileProxy::new(file, db);
        assert_eq!(proxy.completeness(), 0);
        assert_eq!(proxy.data().full_path, "/test/file.mp3");
        Ok(())
    }

    #[test]
    fn test_file_proxy_completeness() -> Result<(), crate::error::Error> {
        let db = create_test_db()?;
        let mut file =
            AudiobookFile::new(1, "file.mp3".to_string(), "/test/file.mp3".to_string(), 0);
        file.completeness = 75;

        let proxy = AudiobookFileProxy::new(file, db);
        assert_eq!(proxy.completeness(), 75);
        Ok(())
    }

    #[test]
    fn test_file_proxy_data_access() -> Result<(), crate::error::Error> {
        let db = create_test_db()?;
        let mut file =
            AudiobookFile::new(1, "file.mp3".to_string(), "/test/file.mp3".to_string(), 1);
        file.completeness = 100;
        file.length_of_file = Some(7200);
        file.seek_position = Some(7200);

        let proxy = AudiobookFileProxy::new(file, db);
        let data = proxy.data();
        assert_eq!(data.full_path, "/test/file.mp3");
        assert_eq!(data.length_of_file, Some(7200));
        assert_eq!(data.seek_position, Some(7200));
        assert_eq!(data.position, 1);
        assert_eq!(data.completeness, 100);
        Ok(())
    }
}
