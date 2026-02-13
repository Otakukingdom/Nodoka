use crate::db::Database;
use crate::proxy::AudiobookProxy;
use crate::NodokaError;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub struct ProxyManager {
    db: Arc<Database>,
    audiobook_cache: Arc<RwLock<HashMap<i64, AudiobookProxy>>>,
}

impl ProxyManager {
    pub fn new(db: Arc<Database>) -> Self {
        Self {
            db,
            audiobook_cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn get_audiobook(&self, id: i64) -> Result<AudiobookProxy, NodokaError> {
        let cache = self
            .audiobook_cache
            .read()
            .map_err(|_| NodokaError::LockError)?;

        if let Some(proxy) = cache.get(&id) {
            return Ok(proxy.clone());
        }

        drop(cache);

        let proxy = AudiobookProxy::new(id, Arc::clone(&self.db))?;

        let mut cache = self
            .audiobook_cache
            .write()
            .map_err(|_| NodokaError::LockError)?;
        cache.insert(id, proxy.clone());

        Ok(proxy)
    }

    pub fn clear_cache(&self) -> Result<(), NodokaError> {
        let mut cache = self
            .audiobook_cache
            .write()
            .map_err(|_| NodokaError::LockError)?;
        cache.clear();
        Ok(())
    }
}

impl Clone for ProxyManager {
    fn clone(&self) -> Self {
        Self {
            db: Arc::clone(&self.db),
            audiobook_cache: Arc::clone(&self.audiobook_cache),
        }
    }
}
