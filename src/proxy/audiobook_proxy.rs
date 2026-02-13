use crate::db::Database;
use crate::models::Audiobook;
use crate::proxy::AudiobookFileProxy;
use crate::NodokaError;
use std::sync::{Arc, RwLock};

pub struct AudiobookProxy {
    id: i64,
    data: Arc<RwLock<Audiobook>>,
    files_cache: Arc<RwLock<Option<Vec<AudiobookFileProxy>>>>,
    db: Arc<Database>,
}

impl AudiobookProxy {
    pub fn new(id: i64, db: Arc<Database>) -> Result<Self, NodokaError> {
        let data = crate::db::queries::get_audiobook_by_id(db.connection(), id)?
            .ok_or_else(|| NodokaError::AudiobookNotFound(id))?;

        Ok(Self {
            id,
            data: Arc::new(RwLock::new(data)),
            files_cache: Arc::new(RwLock::new(None)),
            db,
        })
    }

    pub fn get_files(&self) -> Result<Vec<AudiobookFileProxy>, NodokaError> {
        let cache = self
            .files_cache
            .read()
            .map_err(|_| NodokaError::LockError)?;

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

        let mut cache = self
            .files_cache
            .write()
            .map_err(|_| NodokaError::LockError)?;
        *cache = Some(proxies.clone());

        Ok(proxies)
    }

    pub fn update_completeness(&self) -> Result<(), NodokaError> {
        let files = self.get_files()?;

        if files.is_empty() {
            return Ok(());
        }

        let total: i32 = files.iter().map(|f| f.completeness()).sum();
        let avg = total / i32::try_from(files.len()).map_err(|_| NodokaError::ConversionError)?;

        let mut data = self.data.write().map_err(|_| NodokaError::LockError)?;
        data.completeness = avg;

        crate::db::queries::update_audiobook_completeness(self.db.connection(), self.id, avg)
    }

    pub fn get_data(&self) -> Result<Audiobook, NodokaError> {
        self.data
            .read()
            .map_err(|_| NodokaError::LockError)
            .map(|d| d.clone())
    }

    pub fn id(&self) -> i64 {
        self.id
    }
}

impl Clone for AudiobookProxy {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            data: Arc::clone(&self.data),
            files_cache: Arc::clone(&self.files_cache),
            db: Arc::clone(&self.db),
        }
    }
}
