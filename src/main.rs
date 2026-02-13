//! Nodoka Audiobook Reader - Main entry point
//!
//! A cross-platform audiobook player built with Rust and iced.
//! Provides automatic progress tracking, VLC-powered playback, and a clean UI.

use nodoka::{Database, NodokaError};
use std::process;

fn main() {
    // Initialize tracing
    init_logging();

    // Single instance guard
    let _instance_guard = match check_single_instance() {
        Ok(guard) => guard,
        Err(NodokaError::LockError) => {
            eprintln!("Error: Cannot launch multiple instances of Nodoka Player");
            process::exit(1);
        }
        Err(e) => {
            eprintln!("Error checking instance: {e}");
            process::exit(1);
        }
    };

    // Initialize database
    let Ok(db) = Database::open() else {
        eprintln!("Error: Failed to load the config file");
        process::exit(1);
    };

    if let Err(e) = nodoka::db::initialize_schema(db.connection()) {
        eprintln!("Error: Failed to initialize database schema: {e}");
        process::exit(1);
    }

    tracing::info!("Nodoka Audiobook Reader starting...");
    tracing::info!("Database initialized successfully");

    // Run iced application
    if let Err(e) = nodoka::app::run(db) {
        eprintln!("Error: Application failed to run: {e}");
        process::exit(1);
    }
}

fn init_logging() {
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "nodoka=info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();
}

struct SingleInstanceGuard {
    lock_file_path: std::path::PathBuf,
}

impl Drop for SingleInstanceGuard {
    fn drop(&mut self) {
        if let Err(e) = std::fs::remove_file(&self.lock_file_path) {
            tracing::warn!(
                "Failed to remove lock file {}: {e}",
                self.lock_file_path.display()
            );
        }
    }
}

fn check_single_instance() -> Result<SingleInstanceGuard, NodokaError> {
    use directories::ProjectDirs;
    use std::fs::OpenOptions;
    use std::io::Write;

    let proj_dirs = ProjectDirs::from("com", "Otakukingdom", "Nodoka")
        .ok_or(NodokaError::ProjectDirNotFound)?;

    let data_dir = proj_dirs.data_dir();
    std::fs::create_dir_all(data_dir)?;
    let lock_file_path = data_dir.join(".nodoka.lock");

    // Try to create lock file
    match OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&lock_file_path)
    {
        Ok(mut file) => {
            file.write_all(std::process::id().to_string().as_bytes())?;
            Ok(SingleInstanceGuard { lock_file_path })
        }
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => Err(NodokaError::LockError),
        Err(e) => Err(NodokaError::Io(e)),
    }
}
