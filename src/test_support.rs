//! Test-only support utilities.
//!
//! This module provides utilities for writing thread-safe tests that modify
//! environment variables. It is compiled only for unit tests within the crate.
//!
//! Integration tests in `tests/` cannot access this module and must define
//! their own copies of these utilities.

use std::env;
use std::ffi::OsString;
use std::sync::{Mutex, MutexGuard};

static ENV_MUTEX: Mutex<()> = Mutex::new(());

/// Acquires a global lock for tests that modify environment variables.
///
/// This prevents parallel test execution from causing race conditions when
/// multiple tests modify the same environment variables (e.g., `VLC_PLUGIN_PATH`).
///
/// # Example
///
/// ```rust,no_run
/// use crate::test_support::env_lock;
///
/// fn example() {
///     let _lock = env_lock();
///     std::env::set_var("VLC_PLUGIN_PATH", "/test/path");
///     // Test code that depends on VLC_PLUGIN_PATH
/// }
/// ```
pub fn env_lock() -> MutexGuard<'static, ()> {
    match ENV_MUTEX.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    }
}

/// RAII guard that captures and restores an environment variable.
///
/// When the guard is created, it captures the current value of an environment
/// variable. When the guard is dropped, it restores the variable to its
/// original value (or removes it if it wasn't set).
///
/// This is useful for tests that need to temporarily modify environment
/// variables without affecting other tests or leaving the environment dirty.
///
/// # Example
///
/// ```rust,no_run
/// use crate::test_support::{env_lock, EnvVarGuard};
///
/// fn example() {
///     let _lock = env_lock();
///     let _guard = EnvVarGuard::capture("VLC_PLUGIN_PATH");
///
///     std::env::set_var("VLC_PLUGIN_PATH", "/test/path");
///     // Test code that depends on modified VLC_PLUGIN_PATH
///
///     // When `_guard` drops, `VLC_PLUGIN_PATH` is restored to original value.
/// }
/// ```
pub struct EnvVarGuard {
    key: &'static str,
    previous: Option<OsString>,
}

impl EnvVarGuard {
    /// Captures the current value of an environment variable.
    ///
    /// # Arguments
    ///
    /// * `key` - The name of the environment variable to capture
    pub fn capture(key: &'static str) -> Self {
        let previous = env::var_os(key);
        Self { key, previous }
    }
}

impl Drop for EnvVarGuard {
    fn drop(&mut self) {
        match &self.previous {
            Some(value) => env::set_var(self.key, value),
            None => env::remove_var(self.key),
        }
    }
}
