use crate::db::Database;
use crate::error::Error;
use crate::models::Audiobook;
use crate::proxy::AudiobookFileProxy;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

#[allow(clippy::module_name_repetitions)] // Proxy pattern naming is idiomatic
pub struct AudiobookProxy {
    id: i64,
    data: Rc<RefCell<Audiobook>>,
    files_cache: Rc<RefCell<Option<Vec<AudiobookFileProxy>>>>,
    db: Arc<Database>,
}

impl AudiobookProxy {
    /// Creates a new audiobook proxy for the given audiobook ID.
    ///
    /// # Errors
    ///
    /// Returns `Error::Database` if the database query fails.
    /// Returns `Error::AudiobookNotFound` if the audiobook does not exist.
    pub fn new(id: i64, db: Arc<Database>) -> Result<Self, Error> {
        let data = crate::db::queries::get_audiobook_by_id(db.connection(), id)?
            .ok_or_else(|| Error::AudiobookNotFound(id))?;

        Ok(Self {
            id,
            data: Rc::new(RefCell::new(data)),
            files_cache: Rc::new(RefCell::new(None)),
            db,
        })
    }

    /// Retrieves the list of files associated with this audiobook.
    ///
    /// # Errors
    ///
    /// Returns `Error::Database` if the database query fails.
    pub fn get_files(&self) -> Result<Vec<AudiobookFileProxy>, Error> {
        let cache = self.files_cache.borrow();

        if let Some(files) = cache.as_ref() {
            return Ok(files.clone());
        }

        drop(cache);

        // Load from database
        let files = crate::db::queries::get_audiobook_files(self.db.connection(), self.id)?;
        let proxies: Vec<_> = files
            .into_iter()
            .map(|f| AudiobookFileProxy::new(f, Arc::clone(&self.db)))
            .collect();

        *self.files_cache.borrow_mut() = Some(proxies.clone());

        Ok(proxies)
    }

    /// Updates the completeness percentage of this audiobook based on its files.
    ///
    /// # Errors
    ///
    /// Returns `Error::Database` if the database query fails.
    /// Returns `Error::ConversionError` if the file count cannot be converted to i32.
    pub fn update_completeness(&self) -> Result<(), Error> {
        let files = self.get_files()?;

        if files.is_empty() {
            return Ok(());
        }

        let total: i32 = files
            .iter()
            .map(super::audiobook_file_proxy::AudiobookFileProxy::completeness)
            .sum();
        let avg = total / i32::try_from(files.len()).map_err(|_| Error::ConversionError)?;

        self.data.borrow_mut().completeness = avg;

        crate::db::queries::update_audiobook_completeness(self.db.connection(), self.id, avg)
    }

    /// Retrieves a copy of the audiobook data.
    #[must_use]
    pub fn get_data(&self) -> Audiobook {
        self.data.borrow().clone()
    }

    #[must_use]
    pub const fn id(&self) -> i64 {
        self.id
    }
}

impl Clone for AudiobookProxy {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            data: Rc::clone(&self.data),
            files_cache: Rc::clone(&self.files_cache),
            db: Arc::clone(&self.db),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;
    use crate::db::queries;
    use crate::models::{Audiobook, AudiobookFile, Directory};
    use chrono::Utc;

    #[allow(clippy::arc_with_non_send_sync)] // Test helper function
    fn create_test_db_with_audiobook() -> Result<(Arc<Database>, i64), Error> {
        let database = Database::new_in_memory()?;
        db::initialize(database.connection())?;

        let dir = Directory {
            full_path: "/test/audiobooks".to_string(),
            created_at: Utc::now(),
            last_scanned: None,
        };
        queries::insert_directory(database.connection(), &dir)?;

        let audiobook = Audiobook::new(
            "/test/audiobooks".to_string(),
            "Test Audiobook".to_string(),
            "/test/audiobooks/Test Audiobook".to_string(),
            0,
        );
        let id = queries::insert_audiobook(database.connection(), &audiobook)?;

        Ok((Arc::new(database), id))
    }

    #[test]
    fn test_audiobook_proxy_creation() -> Result<(), Error> {
        let (db, id) = create_test_db_with_audiobook()?;
        let proxy = AudiobookProxy::new(id, db)?;
        assert_eq!(proxy.id(), id);
        let data = proxy.get_data();
        assert_eq!(data.name, "Test Audiobook");
        Ok(())
    }

    #[test]
    #[allow(clippy::arc_with_non_send_sync)] // Test code
    fn test_audiobook_proxy_nonexistent() -> Result<(), Error> {
        let database = Database::new_in_memory()?;
        db::initialize(database.connection())?;
        let db = Arc::new(database);

        let result = AudiobookProxy::new(999, db);
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn test_audiobook_proxy_get_files() -> Result<(), Error> {
        let (db, id) = create_test_db_with_audiobook()?;

        let mut file = AudiobookFile::new(
            id,
            "file1.mp3".to_string(),
            "/test/file1.mp3".to_string(),
            0,
        );
        file.completeness = 50;
        queries::insert_audiobook_file(db.connection(), &file)?;

        let proxy = AudiobookProxy::new(id, Arc::clone(&db))?;
        let files = proxy.get_files()?;
        assert_eq!(files.len(), 1);
        let first_file = files
            .first()
            .ok_or(Error::InvalidState("No files found".to_string()))?;
        assert_eq!(first_file.data().full_path, "/test/file1.mp3");

        let files_cached = proxy.get_files()?;
        assert_eq!(files_cached.len(), 1);
        Ok(())
    }

    #[test]
    fn test_audiobook_proxy_update_completeness() -> Result<(), Error> {
        let (db, id) = create_test_db_with_audiobook()?;

        let mut file1 = AudiobookFile::new(
            id,
            "file1.mp3".to_string(),
            "/test/file1.mp3".to_string(),
            0,
        );
        file1.completeness = 50;
        queries::insert_audiobook_file(db.connection(), &file1)?;

        let mut file2 = AudiobookFile::new(
            id,
            "file2.mp3".to_string(),
            "/test/file2.mp3".to_string(),
            1,
        );
        file2.completeness = 100;
        queries::insert_audiobook_file(db.connection(), &file2)?;

        let proxy = AudiobookProxy::new(id, Arc::clone(&db))?;
        proxy.update_completeness()?;

        let data = proxy.get_data();
        assert_eq!(data.completeness, 75);
        Ok(())
    }
}
