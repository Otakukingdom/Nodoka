use crate::db::Database;
use crate::models::Audiobook;
use crate::proxy::AudiobookFileProxy;
use crate::NodokaError;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

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
    /// Returns `NodokaError::Database` if the database query fails.
    /// Returns `NodokaError::AudiobookNotFound` if the audiobook does not exist.
    pub fn new(id: i64, db: Arc<Database>) -> Result<Self, NodokaError> {
        let data = crate::db::queries::get_audiobook_by_id(db.connection(), id)?
            .ok_or_else(|| NodokaError::AudiobookNotFound(id))?;

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
    /// Returns `NodokaError::Database` if the database query fails.
    pub fn get_files(&self) -> Result<Vec<AudiobookFileProxy>, NodokaError> {
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
    /// Returns `NodokaError::Database` if the database query fails.
    /// Returns `NodokaError::ConversionError` if the file count cannot be converted to i32.
    pub fn update_completeness(&self) -> Result<(), NodokaError> {
        let files = self.get_files()?;

        if files.is_empty() {
            return Ok(());
        }

        let total: i32 = files
            .iter()
            .map(super::audiobook_file_proxy::AudiobookFileProxy::completeness)
            .sum();
        let avg = total / i32::try_from(files.len()).map_err(|_| NodokaError::ConversionError)?;

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
