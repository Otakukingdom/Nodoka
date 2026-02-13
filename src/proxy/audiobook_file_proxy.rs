use crate::db::Database;
use crate::models::AudiobookFile;
use std::sync::Arc;

#[derive(Clone)]
pub struct AudiobookFileProxy {
    data: AudiobookFile,
    db: Arc<Database>,
}

impl AudiobookFileProxy {
    pub fn new(data: AudiobookFile, db: Arc<Database>) -> Self {
        Self { data, db }
    }

    pub fn completeness(&self) -> i32 {
        self.data.completeness
    }

    pub fn data(&self) -> &AudiobookFile {
        &self.data
    }

    /// Gets a reference to the database connection
    #[must_use]
    pub fn database(&self) -> &Arc<Database> {
        &self.db
    }
}
