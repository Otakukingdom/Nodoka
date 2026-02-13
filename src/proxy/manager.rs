use crate::db::Database;
use crate::proxy::AudiobookProxy;
use crate::NodokaError;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;

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
    /// Returns `NodokaError::Database` if the database query fails.
    /// Returns `NodokaError::AudiobookNotFound` if the audiobook does not exist.
    pub fn get_audiobook(&self, id: i64) -> Result<AudiobookProxy, NodokaError> {
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
