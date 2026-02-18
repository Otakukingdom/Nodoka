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

    // Setup VLC environment early, before any VLC instances are created
    // This must happen before Database::open() and app initialization
    nodoka::player::setup_vlc_environment();

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
    let db = match Database::open() {
        Ok(db) => db,
        Err(e) => {
            eprintln!("Error: Failed to initialize database: {e}");
            eprintln!("See the error message above for details on how to resolve this issue.");
            process::exit(1);
        }
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

/// Initialises the `tracing` logging subscriber.
///
/// The log level is controlled by the `RUST_LOG` environment variable.
/// When `RUST_LOG` is not set the subscriber defaults to `nodoka=info`,
/// which shows informational messages from this crate only.
///
/// # Example
///
/// ```sh
/// RUST_LOG=nodoka=debug ./nodoka   # verbose logging for the nodoka crate
/// RUST_LOG=trace ./nodoka          # full trace logging for all crates
/// ```
fn init_logging() {
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "nodoka=info".to_string()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();
}

/// RAII guard that holds an exclusive application-wide lock file.
///
/// The guard is created by [`check_single_instance`] and must be kept alive
/// for the duration of the process.  When the guard is dropped (on normal
/// exit or on panic) the lock file is removed from disk, allowing a new
/// instance to start.
struct SingleInstanceGuard {
    /// Absolute path to the lock file that this guard owns.
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

/// Ensures that only one instance of Nodoka is running.
///
/// Creates a PID lock file in the application's data directory.  If a lock
/// file already exists the function reads the stored PID and checks whether
/// that process is still alive.  A stale lock (dead PID) is removed and a
/// fresh lock is created.  A live lock causes [`Error::LockError`] to be
/// returned so the caller can exit gracefully.
///
/// # Errors
///
/// - [`Error::ProjectDirNotFound`] – platform project directories could not be determined.
/// - [`Error::LockError`] – another live Nodoka instance holds the lock.
/// - [`Error::Io`] – an I/O error occurred while creating or reading the lock file.
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

/// Attempts to create a new lock file at `lock_file_path` using an exclusive
/// create (`O_CREAT` | `O_EXCL`), writing the current process ID into it.
///
/// Returns the owning [`SingleInstanceGuard`] on success.  If the file
/// already exists [`std::io::ErrorKind::AlreadyExists`] is returned and the
/// caller should decide whether the existing lock is stale.
///
/// # Errors
///
/// Returns any [`std::io::Error`] produced by opening or writing the file.
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

/// Reads the PID stored in an existing lock file.
///
/// The file is expected to contain a single decimal integer (the PID of the
/// instance that created the lock).  Returns `None` if the file cannot be
/// read or its contents cannot be parsed as a [`u32`].
fn read_lock_pid(lock_file_path: &std::path::Path) -> Option<u32> {
    let contents = std::fs::read_to_string(lock_file_path).ok()?;
    contents.trim().parse::<u32>().ok()
}

/// Returns `true` when a process with the given `pid` is currently alive.
///
/// The check is platform-specific:
///
/// - **Unix**: runs `ps -p <pid>` and inspects whether a second line (the
///   process row) is present in the output.
/// - **Windows**: runs `tasklist /FI "PID eq <pid>"` and checks that the
///   output does not contain the "No tasks are running" sentinel string.
///
/// Both approaches have a small TOCTOU window but are sufficient for the
/// single-instance guard use-case, where false positives (claiming a dead
/// process is alive) cause only a temporary denial of launch.
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
