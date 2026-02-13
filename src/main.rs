//! Nodoka Audiobook Reader - Main entry point
//!
//! A cross-platform audiobook player built with Rust and iced.
//! Provides automatic progress tracking, VLC-powered playback, and a clean UI.

use nodoka::error::Error;
use nodoka::Database;
use std::process;

fn main() {
    // Initialize tracing
    init_logging();

    // Single instance guard
    let _instance_guard = match check_single_instance() {
        Ok(guard) => guard,
        Err(Error::LockError) => {
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

    if let Err(e) = nodoka::db::initialize(db.connection()) {
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

fn check_single_instance() -> Result<SingleInstanceGuard, Error> {
    use directories::ProjectDirs;

    let proj_dirs =
        ProjectDirs::from("com", "Otakukingdom", "Nodoka").ok_or(Error::ProjectDirNotFound)?;

    let data_dir = proj_dirs.data_dir();
    std::fs::create_dir_all(data_dir)?;
    let lock_file_path = data_dir.join(".nodoka.lock");

    match try_create_lock(&lock_file_path) {
        Ok(guard) => Ok(guard),
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
            if let Some(pid) = read_lock_pid(&lock_file_path) {
                if is_pid_running(pid) {
                    return Err(Error::LockError);
                }
            }

            if let Err(e) = std::fs::remove_file(&lock_file_path) {
                tracing::warn!(
                    "Failed to remove stale lock file {}: {e}",
                    lock_file_path.display()
                );
            }

            try_create_lock(&lock_file_path).map_err(Error::Io)
        }
        Err(e) => Err(Error::Io(e)),
    }
}

fn try_create_lock(
    lock_file_path: &std::path::Path,
) -> Result<SingleInstanceGuard, std::io::Error> {
    use std::io::Write;

    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(lock_file_path)?;
    file.write_all(std::process::id().to_string().as_bytes())?;
    Ok(SingleInstanceGuard {
        lock_file_path: lock_file_path.to_path_buf(),
    })
}

fn read_lock_pid(lock_file_path: &std::path::Path) -> Option<u32> {
    let contents = std::fs::read_to_string(lock_file_path).ok()?;
    contents.trim().parse::<u32>().ok()
}

fn is_pid_running(pid: u32) -> bool {
    #[cfg(unix)]
    {
        use std::process::Command;
        let output = Command::new("ps").arg("-p").arg(pid.to_string()).output();

        if let Ok(output) = output {
            if !output.status.success() {
                return false;
            }
            let stdout = String::from_utf8_lossy(&output.stdout);
            return stdout.lines().nth(1).is_some();
        }

        false
    }

    #[cfg(windows)]
    {
        use std::process::Command;
        let filter = format!("PID eq {pid}");
        let output = Command::new("tasklist").arg("/FI").arg(filter).output();

        if let Ok(output) = output {
            if !output.status.success() {
                return false;
            }
            let stdout = String::from_utf8_lossy(&output.stdout);
            if stdout.contains("No tasks are running") {
                return false;
            }
            let pid_str = pid.to_string();
            return stdout.lines().any(|line| {
                line.split_whitespace()
                    .any(|field| field == pid_str.as_str())
            });
        }

        false
    }
}
