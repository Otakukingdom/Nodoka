//! Screenshot generation for Nodoka UI documentation and visual QA.
//!
//! This module provides automated screenshot capture of all meaningful UI states
//! without requiring the Tauri backend to be running.
//!
//! # Usage
//!
//! Run screenshot generation from the project root:
//!
//! ```bash
//! cargo xtask screenshot
//! ```
//!
//! # Concurrent Execution
//!
//! Screenshots are captured concurrently using a pool of 5 Playwright headless
//! browser instances for optimal performance. The pool size can be configured
//! via the `SCREENSHOT_WORKERS` environment variable.
//!
//! The Vite dev server is started automatically with `VITE_PLAYWRIGHT_TEST=true`
//! to enable mock backend mode, allowing screenshots without the Rust backend.
//!
//! # Output Structure
//!
//! Screenshots are organized into the following directory structure under `/screenshots/`:
//!
//! ```text
//! screenshots/
//! ├── app-shell/           # Full application states
//! ├── library-panel/       # Library panel states
//! ├── details-panel/       # Details panel states
//! ├── player-controls/     # Player control states
//! ├── modals/              # Modal dialog states
//! │   ├── settings/
//! │   ├── keyboard-help/
//! │   ├── sleep-timer/
//! │   ├── bookmark-editor/
//! │   └── confirmation/
//! ├── notifications/       # Toast notification states
//! ├── responsive/          # Responsive layout variants
//! ├── accessibility/       # Focus ring and accessibility states
//! ├── special-states/      # Drag-drop, scan progress, focus mode
//! └── edge-cases/          # Long titles, many chapters, etc.
//! ```
//!
//! # Naming Convention
//!
//! Screenshots follow the pattern: `{state}-{theme}.png`
//!
//! Examples:
//! - `empty-library-light.png`
//! - `empty-library-dark.png`
//! - `player-playing.png`
//!
//! # Adding New Screenshot Scenarios
//!
//! 1. Add mock data to `ui/e2e/fixtures/screenshot-scenarios.ts` if new data is needed
//! 2. Add state manipulation helper to `ui/e2e/screenshot-helpers.ts` for complex UI state
//! 3. Add test case to `ui/e2e/screenshot-generation.spec.ts` in the appropriate describe block
//! 4. If adding a new category, update `CATEGORIES` in `xtask/src/screenshot/organizer.rs`
//! 5. Run `cargo xtask screenshot` to generate the new screenshots
//!
//! # Command Options
//!
//! - `--output <dir>`: Custom output directory (default: screenshots)
//! - `--project <name>`: Playwright project to run (default: screenshot-chromium)
//! - `--headed`: Run in headed mode for debugging
//! - `--timeout <secs>`: Custom timeout for Playwright tests
//! - `--skip-readme`: Skip README index generation
//!
//! # Environment Variables
//!
//! - `SCREENSHOT_OUTPUT_DIR`: Override output directory
//! - `SCREENSHOT_WORKERS`: Number of concurrent browser instances (default: 5)
//!
//! # Troubleshooting
//!
//! - **Screenshots not generated**: Ensure npm and Playwright are installed.
//!   Run `npx playwright install chromium` to install the browser.
//! - **Blank screenshots**: Check that `VITE_PLAYWRIGHT_TEST=true` is set when
//!   the dev server starts. This exposes the Redux store for state manipulation.
//! - **Timeout errors**: Increase timeout with `--timeout 900` for slower machines.
//! - **Port already in use**: Kill any process using port 5173 before running.

mod organizer;
mod readme;

use anyhow::{Context, Result};
use clap::Args as ClapArgs;
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::Duration;

/// Screenshot generation arguments
#[derive(ClapArgs, Debug)]
pub struct Args {
    /// Output directory for screenshots (default: /screenshots)
    #[arg(short, long, default_value = "screenshots")]
    pub output: String,

    /// Playwright project to run (default: screenshot-chromium)
    #[arg(short, long, default_value = "screenshot-chromium")]
    pub project: String,

    /// Skip generating README index
    #[arg(long)]
    pub skip_readme: bool,

    /// Run Playwright in headed mode for debugging
    #[arg(long)]
    pub headed: bool,

    /// Additional timeout in seconds for Playwright tests
    #[arg(long, default_value = "600")]
    pub timeout: u64,
}

/// Run the screenshot generation command
pub fn run(args: Args) -> Result<()> {
    let project_root = std::env::current_dir()
        .context("Failed to get current directory")?;

    let ui_dir = project_root.join("ui");
    let output_dir = project_root.join(&args.output);

    println!("Screenshot Generation for Nodoka");
    println!("==================================");
    println!("Output directory: {}", output_dir.display());
    println!("Project: {}", args.project);
    println!();

    // Step 1: Create output directory structure
    println!("[1/4] Creating output directory structure...");
    organizer::create_directory_structure(&output_dir)
        .context("Failed to create output directory structure")?;

    // Step 2: Start Vite dev server
    println!("[2/4] Starting Vite dev server...");
    let mut dev_server = start_dev_server(&ui_dir)
        .context("Failed to start dev server")?;

    // Give the server time to start
    std::thread::sleep(Duration::from_secs(5));

    // Step 3: Run Playwright screenshot tests
    println!("[3/4] Running Playwright screenshot generation...");
    let result = run_playwright_screenshots(&ui_dir, &output_dir, &args);

    // Clean up: stop the dev server
    println!("[4/4] Cleaning up...");
    if let Err(e) = stop_dev_server(&mut dev_server) {
        eprintln!("Warning: Failed to stop dev server gracefully: {}", e);
    }

    // Check Playwright result
    result.context("Playwright screenshot generation failed")?;

    // Step 5: Generate README index
    if !args.skip_readme {
        println!("Generating README index...");
        readme::generate(&output_dir)
            .context("Failed to generate README index")?;
    }

    println!();
    println!("Screenshot generation complete!");
    println!("Screenshots saved to: {}", output_dir.display());

    Ok(())
}

/// Start the Vite dev server with Playwright test mode enabled
fn start_dev_server(ui_dir: &Path) -> Result<std::process::Child> {
    let mut cmd = Command::new("npm");
    cmd.args(["run", "dev"])
        .env("VITE_PLAYWRIGHT_TEST", "true")
        .current_dir(ui_dir)
        .stdout(Stdio::null())
        .stderr(Stdio::null());

    let child = cmd.spawn()
        .context("Failed to spawn npm run dev. Make sure npm is available.")?;

    Ok(child)
}

/// Stop the dev server gracefully with cross-platform support
fn stop_dev_server(child: &mut std::process::Child) -> Result<()> {
    #[cfg(unix)]
    {
        // Send SIGTERM for graceful shutdown on Unix
        let pid = child.id() as i32;
        // SAFETY: libc::kill is a standard POSIX call for process signaling
        unsafe {
            libc::kill(pid, libc::SIGTERM);
        }
        // Wait a bit for graceful shutdown
        std::thread::sleep(Duration::from_secs(2));
    }

    #[cfg(windows)]
    {
        // On Windows, taskkill is more reliable for process tree termination
        let pid = child.id().to_string();
        let _ = std::process::Command::new("taskkill")
            .args(["/PID", &pid, "/F", "/T"])
            .output();
        std::thread::sleep(Duration::from_secs(1));
    }

    // Kill if still running
    let _ = child.kill();
    let _ = child.wait();

    Ok(())
}

/// Run Playwright tests for screenshot generation
fn run_playwright_screenshots(ui_dir: &Path, output_dir: &Path, args: &Args) -> Result<()> {
    let mut cmd = Command::new("npx");
    cmd.args([
        "playwright",
        "test",
        "--config=playwright.screenshot.config.ts",
        "--project",
        &args.project,
    ])
    .env("SCREENSHOT_OUTPUT_DIR", output_dir.to_string_lossy().as_ref())
    .current_dir(ui_dir);

    if args.headed {
        cmd.arg("--headed");
    }

    let status = cmd.status()
        .context("Failed to run Playwright tests. Make sure npx and playwright are available.")?;

    if status.success() {
        Ok(())
    } else {
        anyhow::bail!("Playwright tests failed with status: {}", status)
    }
}
