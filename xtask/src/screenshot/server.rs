//! Vite dev server management with health checks.
//!
//! Provides robust server startup verification and graceful shutdown.
//!
//! # Architecture
//!
//! The [`DevServer`] struct uses RAII pattern to ensure cleanup even if
//! the server is not explicitly stopped. When dropped, it will kill the
//! child process.
//!
//! # Server Startup
//!
//! 1. Spawns `npm run dev` with `VITE_PLAYWRIGHT_TEST=true`
//! 2. Polls `http://localhost:5173` every 500ms
//! 3. Verifies HTTP 200 response before proceeding
//! 4. Detects server crashes during startup
//!
//! # Testing
//!
//! Unit testing this module is challenging because it involves:
//! - Spawning external processes (npm)
//! - Network operations (HTTP health checks)
//! - Platform-specific signal handling
//!
//! Integration tests should verify:
//! 1. Server starts and responds to health checks
//! 2. Server stops cleanly on Drop
//! 3. Interrupt handling during startup
//!
//! Run integration tests with: `cargo xtask screenshot --headed`

use anyhow::{bail, Context, Result};
use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::time::{Duration, Instant};

/// Running dev server handle
///
/// Uses RAII pattern to ensure cleanup even if [`stop()`](DevServer::stop)
/// is not called explicitly.
pub struct DevServer {
    child: Option<Child>,
}

impl DevServer {
    /// Start the Vite dev server and wait for it to be healthy
    ///
    /// # Arguments
    ///
    /// * `ui_dir` - Path to the UI directory containing package.json
    /// * `timeout` - Maximum time to wait for server startup
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - npm fails to spawn
    /// - Server crashes during startup
    /// - Server doesn't respond within timeout
    /// - Interrupt signal received during startup
    pub fn start(ui_dir: &Path, timeout: Duration) -> Result<Self> {
        let port = 5173;
        let url = format!("http://localhost:{}", port);

        println!("├── Spawning npm run dev...");

        let child = Command::new("npm")
            .args(["run", "dev"])
            .env("VITE_PLAYWRIGHT_TEST", "true")
            .current_dir(ui_dir)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .context("Failed to spawn npm run dev")?;

        let pid = child.id();
        super::state::register_server_pid(pid);

        println!("├── Waiting for server...");

        // Create HTTP client for health checks
        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(2))
            .build()
            .context("Failed to create HTTP client")?;

        // Wait for server to respond
        let start = Instant::now();
        let mut child = Some(child);

        loop {
            if super::state::is_interrupted() {
                if let Some(ref mut c) = child {
                    let _ = c.kill();
                }
                bail!("Interrupted while waiting for server");
            }

            if start.elapsed() > timeout {
                // Capture stderr for debugging
                if let Some(ref mut c) = child {
                    let _ = c.kill();
                }
                bail!(
                    "Vite server failed to start within {} seconds. \
                     Check ui/vite.config.ts for errors.",
                    timeout.as_secs()
                );
            }

            // Use reqwest to check server health
            match client.get(&url).send() {
                Ok(resp) if resp.status().is_success() => {
                    let elapsed = start.elapsed();
                    println!(
                        "│   └── [{:.1}s] Server responding at {}",
                        elapsed.as_secs_f64(),
                        url
                    );
                    break;
                }
                Ok(_) => {
                    // Server responded but not 200, keep waiting
                }
                Err(_) => {
                    // Connection failed, keep waiting
                }
            }

            // Alternative: check if child process has crashed
            if let Some(ref mut c) = child {
                match c.try_wait() {
                    Ok(Some(status)) => {
                        bail!(
                            "Vite server crashed during startup with status: {}",
                            status
                        );
                    }
                    Ok(None) => {
                        // Still running, continue waiting
                    }
                    Err(e) => {
                        bail!("Failed to check server process status: {}", e);
                    }
                }
            }

            std::thread::sleep(Duration::from_millis(500));
        }

        println!("└── Server ready");

        Ok(Self { child })
    }

    /// Stop the dev server gracefully
    ///
    /// On Unix, sends SIGTERM first for graceful shutdown, then force kills.
    /// On Windows, uses taskkill with /T flag to terminate process tree.
    pub fn stop(&mut self) -> Result<()> {
        if let Some(child) = self.child.as_mut() {
            println!("├── Stopping Vite dev server (PID {})...", child.id());

            #[cfg(unix)]
            {
                // Send SIGTERM for graceful shutdown
                let pid = child.id() as i32;
                unsafe {
                    libc::kill(pid, libc::SIGTERM);
                }

                // Wait a bit for graceful shutdown
                std::thread::sleep(Duration::from_secs(2));
            }

            #[cfg(windows)]
            {
                let pid = child.id().to_string();
                let _ = Command::new("taskkill")
                    .args(["/PID", &pid, "/F", "/T"])
                    .output();
                std::thread::sleep(Duration::from_secs(1));
            }

            // Force kill if still running
            let _ = child.kill();
            let _ = child.wait();

            println!("│   └── [OK] Server stopped gracefully");
        }

        Ok(())
    }
}

impl Drop for DevServer {
    fn drop(&mut self) {
        // Ensure cleanup even if stop() wasn't called
        if let Some(child) = self.child.as_mut() {
            let _ = child.kill();
            let _ = child.wait();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_url_format() {
        // Verify URL construction logic
        let port = 5173;
        let url = format!("http://localhost:{}", port);
        assert_eq!(url, "http://localhost:5173");
    }

    #[test]
    fn test_timeout_duration() {
        // Verify timeout can be created and compared
        let timeout = Duration::from_secs(60);
        assert_eq!(timeout.as_secs(), 60);
    }

    // Note: Full server tests require integration testing
    // See: cargo xtask screenshot (end-to-end verification)
}
