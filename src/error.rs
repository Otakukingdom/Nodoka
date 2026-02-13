use thiserror::Error;

#[derive(Error, Debug)]
pub enum NodokaError {
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("VLC error: {0}")]
    Vlc(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Media parsing failed: {0}")]
    MediaParse(String),

    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("Invalid state: {0}")]
    InvalidState(String),

    #[error("Project directory not found")]
    ProjectDirNotFound,

    #[error("Lock error occurred")]
    LockError,

    #[error("Audiobook not found: {0}")]
    AudiobookNotFound(i64),

    #[error("Conversion error")]
    ConversionError,
}

pub type Result<T> = std::result::Result<T, NodokaError>;
