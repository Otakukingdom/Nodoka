//! Test-only support utilities.
//!
//! This module is compiled only for unit tests.

use std::env;
use std::ffi::OsString;
use std::sync::{Mutex, MutexGuard};

static ENV_MUTEX: Mutex<()> = Mutex::new(());

pub fn env_lock() -> MutexGuard<'static, ()> {
    match ENV_MUTEX.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    }
}

pub struct EnvVarGuard {
    key: &'static str,
    previous: Option<OsString>,
}

impl EnvVarGuard {
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
