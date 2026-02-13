use crate::db::Database;
use crate::models::AudiobookFile;
use std::sync::Arc;

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
