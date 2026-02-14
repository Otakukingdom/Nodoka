use crate::db::Database;
use crate::error::Error;
use crate::proxy::AudiobookHandle;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Cache {
    db: Rc<Database>,
    audiobook_cache: Rc<RefCell<HashMap<i64, AudiobookHandle>>>,
}

impl Cache {
    #[must_use]
    pub fn new(db: Rc<Database>) -> Self {
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
    pub fn get_audiobook(&self, id: i64) -> Result<AudiobookHandle, Error> {
        let cache = self.audiobook_cache.borrow();

        if let Some(proxy) = cache.get(&id) {
            return Ok(proxy.clone());
        }

        drop(cache);

        let proxy = AudiobookHandle::new(id, Rc::clone(&self.db))?;

        self.audiobook_cache.borrow_mut().insert(id, proxy.clone());

        Ok(proxy)
    }

    /// Clears the audiobook proxy cache.
    pub fn clear_cache(&self) {
        self.audiobook_cache.borrow_mut().clear();
    }
}

impl Clone for Cache {
    fn clone(&self) -> Self {
        Self {
            db: Rc::clone(&self.db),
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
    use temp_dir::TempDir;

    fn create_test_db_with_audiobook() -> Result<(TempDir, Rc<Database>, i64, String), Error> {
        let database = Database::new_in_memory()?;
        db::initialize(database.connection())?;

        let temp_dir = TempDir::new().map_err(Error::from)?;
        let dir_path = temp_dir
            .path()
            .to_str()
            .ok_or_else(|| Error::InvalidState("Temp dir path not UTF-8".to_string()))?
            .to_string();

        let dir = Directory {
            full_path: dir_path.clone(),
            created_at: Utc::now(),
            last_scanned: None,
        };
        queries::insert_directory(database.connection(), &dir)?;

        let audiobook = Audiobook::new(
            dir_path.clone(),
            "Test Audiobook".to_string(),
            format!("{dir_path}/Test Audiobook"),
            0,
        );
        let id = queries::insert_audiobook(database.connection(), &audiobook)?;

        Ok((temp_dir, Rc::new(database), id, dir_path))
    }

    #[test]
    fn test_manager_creation() -> Result<(), Error> {
        let database = Database::new_in_memory()?;
        let db = Rc::new(database);
        let _manager = Cache::new(db);
        Ok(())
    }

    #[test]
    fn test_manager_get_audiobook() -> Result<(), Error> {
        let (_temp_dir, db, id, _dir_path) = create_test_db_with_audiobook()?;
        let manager = Cache::new(Rc::clone(&db));

        let proxy = manager.get_audiobook(id)?;
        assert_eq!(proxy.id(), id);
        let data = proxy.get_data();
        assert_eq!(data.name, "Test Audiobook");
        Ok(())
    }

    #[test]
    fn test_manager_caching() -> Result<(), Error> {
        let (_temp_dir, db, id, _dir_path) = create_test_db_with_audiobook()?;
        let manager = Cache::new(Rc::clone(&db));

        let proxy1 = manager.get_audiobook(id)?;
        let proxy2 = manager.get_audiobook(id)?;

        assert_eq!(proxy1.id(), proxy2.id());
        Ok(())
    }

    #[test]
    fn test_manager_clear_cache() -> Result<(), Error> {
        let (_temp_dir, db, id, _dir_path) = create_test_db_with_audiobook()?;
        let manager = Cache::new(Rc::clone(&db));

        let _proxy1 = manager.get_audiobook(id)?;
        manager.clear_cache();

        let proxy2 = manager.get_audiobook(id)?;
        assert_eq!(proxy2.id(), id);
        Ok(())
    }

    #[test]
    fn test_manager_nonexistent_audiobook() -> Result<(), Error> {
        let database = Database::new_in_memory()?;
        db::initialize(database.connection())?;
        let db = Rc::new(database);
        let manager = Cache::new(db);

        let result = manager.get_audiobook(999);
        assert!(result.is_err());
        Ok(())
    }
}
