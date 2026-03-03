//! Shared state for coordinating cleanup across modules.
//!
//! Provides thread-safe tracking of active resources so the signal
//! handler can trigger comprehensive cleanup on interrupt.

use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Mutex;

static INTERRUPTED: AtomicBool = AtomicBool::new(false);
static SERVER_PID: AtomicU32 = AtomicU32::new(0);
static LOCK_PATH: Mutex<Option<PathBuf>> = Mutex::new(None);
static UI_DIR: Mutex<Option<PathBuf>> = Mutex::new(None);

/// Set the interrupted flag
pub fn set_interrupted() {
    INTERRUPTED.store(true, Ordering::SeqCst);
}

/// Check if we've been interrupted
pub fn is_interrupted() -> bool {
    INTERRUPTED.load(Ordering::SeqCst)
}

/// Register the dev server PID for cleanup
pub fn register_server_pid(pid: u32) {
    SERVER_PID.store(pid, Ordering::SeqCst);
}

/// Get the registered server PID
pub fn get_server_pid() -> u32 {
    SERVER_PID.load(Ordering::SeqCst)
}

/// Register the lock file path for cleanup
pub fn register_lock_path(path: PathBuf) {
    if let Ok(mut lock) = LOCK_PATH.lock() {
        *lock = Some(path);
    }
}

/// Register the UI directory for artifact cleanup
pub fn register_ui_dir(path: PathBuf) {
    if let Ok(mut dir) = UI_DIR.lock() {
        *dir = Some(path);
    }
}

/// Get the registered UI directory
pub fn get_ui_dir() -> Option<PathBuf> {
    UI_DIR.lock().ok().and_then(|dir| dir.clone())
}

/// Perform comprehensive cleanup (called from signal handler)
pub fn emergency_cleanup() {
    // Kill server if running
    let server_pid = get_server_pid();
    if server_pid != 0 {
        #[cfg(unix)]
        unsafe {
            libc::kill(server_pid as i32, libc::SIGTERM);
        }
        #[cfg(windows)]
        {
            let _ = std::process::Command::new("taskkill")
                .args(["/PID", &server_pid.to_string(), "/F"])
                .output();
        }
    }

    // Remove lock file
    if let Ok(lock) = LOCK_PATH.lock() {
        if let Some(ref path) = *lock {
            let _ = std::fs::remove_file(path);
        }
    }

    // Clean up partial artifacts from UI directory
    if let Some(ui_dir) = get_ui_dir() {
        // Remove test artifacts directories (best effort)
        let artifacts = [
            ui_dir.join("test-results-screenshots"),
            ui_dir.join("playwright-report-screenshots"),
        ];
        for artifact in &artifacts {
            if artifact.exists() {
                let _ = std::fs::remove_dir_all(artifact);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interrupt_flag() {
        // Reset first
        INTERRUPTED.store(false, Ordering::SeqCst);
        assert!(!is_interrupted());
        set_interrupted();
        assert!(is_interrupted());
        // Reset for other tests
        INTERRUPTED.store(false, Ordering::SeqCst);
    }

    #[test]
    fn test_server_pid_registration() {
        // Reset first
        SERVER_PID.store(0, Ordering::SeqCst);
        assert_eq!(get_server_pid(), 0);
        register_server_pid(12345);
        assert_eq!(get_server_pid(), 12345);
        // Reset
        SERVER_PID.store(0, Ordering::SeqCst);
    }

    #[test]
    fn test_register_lock_path() {
        // Reset first
        {
            let mut lock = LOCK_PATH.lock().unwrap();
            *lock = None;
        }

        register_lock_path(PathBuf::from("/tmp/test.lock"));

        let lock = LOCK_PATH.lock().unwrap();
        assert_eq!(*lock, Some(PathBuf::from("/tmp/test.lock")));

        // Reset
        drop(lock);
        {
            let mut lock = LOCK_PATH.lock().unwrap();
            *lock = None;
        }
    }

    #[test]
    fn test_register_ui_dir() {
        // Reset first
        {
            let mut dir = UI_DIR.lock().unwrap();
            *dir = None;
        }

        assert!(get_ui_dir().is_none());

        register_ui_dir(PathBuf::from("/path/to/ui"));

        assert_eq!(get_ui_dir(), Some(PathBuf::from("/path/to/ui")));

        // Reset
        {
            let mut dir = UI_DIR.lock().unwrap();
            *dir = None;
        }
    }
}
