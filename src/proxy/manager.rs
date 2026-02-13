use crate::db::Database;
use crate::error::Error;
use crate::proxy::AudiobookProxy;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;

#[allow(clippy::module_name_repetitions)] // Manager pattern naming is idiomatic for manager module
pub struct ProxyManager {
    db: Arc<Database>,
    audiobook_cache: Rc<RefCell<HashMap<i64, AudiobookProxy>>>,
}

impl ProxyManager {
    #[must_use]
    pub fn new(db: Arc<Database>) -> Self {
        Self {
            db,
            audiobook_cache: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    /// Retrieves an audiobook proxy by ID, using the cache if available.
    ///
    /// # Errors
    ///
    /// Returns `Error::Database` if the database query fails.
    /// Returns `Error::AudiobookNotFound` if the audiobook does not exist.
    pub fn get_audiobook(&self, id: i64) -> Result<AudiobookProxy, Error> {
        let cache = self.audiobook_cache.borrow();

        if let Some(proxy) = cache.get(&id) {
            return Ok(proxy.clone());
        }

        drop(cache);

        let proxy = AudiobookProxy::new(id, Arc::clone(&self.db))?;

        self.audiobook_cache.borrow_mut().insert(id, proxy.clone());

        Ok(proxy)
    }

    /// Clears the audiobook proxy cache.
    pub fn clear_cache(&self) {
        self.audiobook_cache.borrow_mut().clear();
    }
}

impl Clone for ProxyManager {
    fn clone(&self) -> Self {
        Self {
            db: Arc::clone(&self.db),
            audiobook_cache: Rc::clone(&self.audiobook_cache),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;
    use crate::db::queries;
    use crate::models::{Audiobook, Directory};
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
    #[allow(clippy::arc_with_non_send_sync)] // Test code
    fn test_manager_creation() -> Result<(), Error> {
        let database = Database::new_in_memory()?;
        let db = Arc::new(database);
        let _manager = ProxyManager::new(db);
        Ok(())
    }

    #[test]
    fn test_manager_get_audiobook() -> Result<(), Error> {
        let (db, id) = create_test_db_with_audiobook()?;
        let manager = ProxyManager::new(Arc::clone(&db));

        let proxy = manager.get_audiobook(id)?;
        assert_eq!(proxy.id(), id);
        let data = proxy.get_data();
        assert_eq!(data.name, "Test Audiobook");
        Ok(())
    }

    #[test]
    fn test_manager_caching() -> Result<(), Error> {
        let (db, id) = create_test_db_with_audiobook()?;
        let manager = ProxyManager::new(Arc::clone(&db));

        let proxy1 = manager.get_audiobook(id)?;
        let proxy2 = manager.get_audiobook(id)?;

        assert_eq!(proxy1.id(), proxy2.id());
        Ok(())
    }

    #[test]
    fn test_manager_clear_cache() -> Result<(), Error> {
        let (db, id) = create_test_db_with_audiobook()?;
        let manager = ProxyManager::new(Arc::clone(&db));

        let _proxy1 = manager.get_audiobook(id)?;
        manager.clear_cache();

        let proxy2 = manager.get_audiobook(id)?;
        assert_eq!(proxy2.id(), id);
        Ok(())
    }

    #[test]
    #[allow(clippy::arc_with_non_send_sync)] // Test code
    fn test_manager_nonexistent_audiobook() -> Result<(), Error> {
        let database = Database::new_in_memory()?;
        db::initialize(database.connection())?;
        let db = Arc::new(database);
        let manager = ProxyManager::new(db);

        let result = manager.get_audiobook(999);
        assert!(result.is_err());
        Ok(())
    }
}
