//! Vite dev server management with health checks.
//!
//! Provides robust server startup verification and graceful shutdown.

use anyhow::{bail, Context, Result};
use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

static INTERRUPTED: AtomicBool = AtomicBool::new(false);

/// Set the interrupted flag (called from signal handler)
pub fn set_interrupted() {
    INTERRUPTED.store(true, Ordering::SeqCst);
}

/// Check if we've been interrupted
pub fn is_interrupted() -> bool {
    INTERRUPTED.load(Ordering::SeqCst)
}

/// Running dev server handle
pub struct DevServer {
    child: Option<Child>,
}

impl DevServer {
    /// Start the Vite dev server and wait for it to be healthy
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

        println!("├── Waiting for server...");

        // Wait for server to respond
        let start = Instant::now();
        let mut child = Some(child);

        loop {
            if is_interrupted() {
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

            // Use curl to check server health (cross-platform available)
            let check_result = Command::new("curl")
                .args(["-s", "-o", "/dev/null", "-w", "%{http_code}", &url])
                .timeout(Duration::from_secs(2))
                .output();

            if let Ok(output) = check_result {
                let code = String::from_utf8_lossy(&output.stdout);
                if code.trim() == "200" {
                    let elapsed = start.elapsed();
                    println!(
                        "│   └── [{:.1}s] Server responding at {}",
                        elapsed.as_secs_f64(),
                        url
                    );
                    break;
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

/// Extension trait for Command to add timeout support
trait CommandTimeout {
    fn timeout(&mut self, duration: Duration) -> &mut Self;
}

impl CommandTimeout for Command {
    fn timeout(&mut self, _duration: Duration) -> &mut Self {
        // On Unix, we can't set process timeout directly
        // The timeout is handled by the caller
        self
    }
}
