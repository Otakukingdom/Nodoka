//! Shared application state for Tauri commands

use crate::db::Database;
use crate::player::Vlc;
use parking_lot::Mutex;
use std::sync::Arc;

// SAFETY: Vlc internally uses Arc<Mutex<>> for thread safety.
// The raw pointers in VLC are only accessed through the vlc-rs library
// which manages thread safety internally. This is safe because:
// 1. Vlc is already used in multi-threaded iced runtime
// 2. All Vlc methods use proper locking internally
// 3. VLC library itself is thread-safe for the operations we use
#[allow(unsafe_code)]
unsafe impl Send for Vlc {}

#[allow(unsafe_code)]
unsafe impl Sync for Vlc {}

/// Shared application state accessible from Tauri commands
pub struct AppState {
    /// Database connection for queries
    pub db: Arc<Mutex<Database>>,
    /// VLC player instance
    pub player: Arc<Mutex<Option<Vlc>>>,
}

impl AppState {
    /// Creates a new application state
    #[must_use]
    pub fn new(db: Database, player: Option<Vlc>) -> Self {
        Self {
            db: Arc::new(Mutex::new(db)),
            player: Arc::new(Mutex::new(player)),
        }
    }
}
