use nodoka::{Database, NodokaError};
use std::process;

fn main() {
    // Initialize tracing
    init_logging();

    // Single instance guard
    match check_single_instance() {
        Ok(true) => {}
        Ok(false) => {
            eprintln!("Error: Cannot launch multiple instances of Nodoka Player");
            process::exit(1);
        }
        Err(e) => {
            eprintln!("Error checking instance: {e}");
            process::exit(1);
        }
    }

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

fn check_single_instance() -> Result<bool, NodokaError> {
    use directories::ProjectDirs;
    use std::fs::OpenOptions;
    use std::io::Write;

    let proj_dirs = ProjectDirs::from("com", "Otakukingdom", "Nodoka")
        .ok_or(NodokaError::ProjectDirNotFound)?;

    let lock_file_path = proj_dirs.data_dir().join(".nodoka.lock");

    // Try to create lock file
    match OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&lock_file_path)
    {
        Ok(mut file) => {
            file.write_all(std::process::id().to_string().as_bytes())?;
            Ok(true)
        }
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => Ok(false),
        Err(e) => Err(NodokaError::Io(e)),
    }
}
