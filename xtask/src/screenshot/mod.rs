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
//! # Self-Sufficient Operation
//!
//! The command is fully self-sufficient and will:
//! - Validate the environment (Node.js, npm, disk space)
//! - Install npm dependencies if stale or missing
//! - Install Playwright Chromium browser if missing
//! - Clean up stale processes from previous failed runs
//! - Prevent concurrent runs with a lock file
//! - Start and stop the Vite dev server automatically
//! - Clean up all resources on success, failure, or interruption
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

mod cleanup;
mod deps;
mod lock;
mod organizer;
mod preflight;
mod readme;
mod server;
mod signal;
mod state;

use anyhow::{bail, Context, Result};
use clap::Args as ClapArgs;
use std::path::Path;
use std::process::Command;
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
    // Set up signal handler first
    signal::setup_signal_handler()?;

    let project_root =
        std::env::current_dir().context("Failed to get current directory")?;
    let ui_dir = project_root.join("ui");
    let output_dir = project_root.join(&args.output);

    // Register UI directory for emergency cleanup on interrupt
    state::register_ui_dir(ui_dir.clone());

    println!("Screenshot Generation for Nodoka");
    println!("==================================");
    println!();

    // === PHASE 1: PRE-FLIGHT CHECKS ===
    println!("Pre-flight checks...");
    preflight::run_preflight_checks(&project_root)?;
    println!("Pre-flight checks passed");
    println!();

    // === PHASE 2: DEPENDENCY SETUP ===
    println!("Step 1: Checking dependencies...");
    deps::ensure_npm_dependencies(&ui_dir)?;
    deps::ensure_playwright_browser(&ui_dir)?;
    println!("Dependencies ready");
    println!();

    // === PHASE 3: CLEANUP ===
    println!("Step 2: Cleaning up stale processes...");
    cleanup::kill_stale_dev_server()?;
    cleanup::kill_orphaned_playwright_processes()?;
    println!("Cleanup complete");
    println!();

    // === PHASE 4: LOCK ACQUISITION ===
    println!("Step 3: Acquiring lock...");
    let _lock = lock::LockFile::acquire(&ui_dir)?;
    println!();

    // === PHASE 5: ARTIFACT CLEANUP ===
    println!("Step 4: Cleaning previous artifacts...");
    cleanup::cleanup_previous_artifacts(&project_root, &ui_dir)?;
    println!();

    // === PHASE 6: DIRECTORY STRUCTURE ===
    println!("Step 5: Creating output directory structure...");
    organizer::create_directory_structure(&output_dir)
        .context("Failed to create output directory structure")?;
    println!();

    // === PHASE 7: SERVER STARTUP ===
    println!("Step 6: Starting Vite dev server...");
    let mut dev_server =
        server::DevServer::start(&ui_dir, Duration::from_secs(60))?;
    println!();

    // === PHASE 8: PLAYWRIGHT EXECUTION ===
    println!("Step 7: Running Playwright screenshot generation...");
    let playwright_result =
        run_playwright_screenshots(&ui_dir, &output_dir, &args);
    println!();

    // === PHASE 9: CLEANUP ===
    println!("Step 8: Cleaning up...");
    if let Err(e) = dev_server.stop() {
        eprintln!("Warning: Failed to stop dev server gracefully: {}", e);
    }

    // Handle Playwright result
    if let Err(e) = playwright_result {
        println!();
        println!("Error: {}", e);
        println!();
        println!("Debug information:");
        println!("├── Playwright report: ui/playwright-report-screenshots/index.html");
        println!("├── Test results: ui/test-results-screenshots/");
        println!("└── Run with --headed flag to see browser during tests");
        println!();
        println!("To retry: cargo xtask screenshot");
        bail!("Playwright tests failed");
    }

    // === PHASE 10: README GENERATION ===
    if !args.skip_readme {
        println!("Step 9: Generating README index...");
        readme::generate(&output_dir).context("Failed to generate README index")?;

        let readme_path = output_dir.join("README.md");
        if !readme_path.exists() {
            bail!("README.md was not generated at {}", readme_path.display());
        }
        println!("README generated at: {}", readme_path.display());
    }

    println!();
    println!("Screenshot generation complete!");
    println!("Screenshots saved to: {}", output_dir.display());

    Ok(())
}

/// Run Playwright tests for screenshot generation
fn run_playwright_screenshots(
    ui_dir: &Path,
    output_dir: &Path,
    args: &Args,
) -> Result<()> {
    let mut cmd = Command::new("npx");
    cmd.args([
        "playwright",
        "test",
        "--config=playwright.screenshot.config.ts",
        "--project",
        &args.project,
    ])
    .env(
        "SCREENSHOT_OUTPUT_DIR",
        output_dir.to_string_lossy().as_ref(),
    )
    .current_dir(ui_dir);

    if args.headed {
        cmd.arg("--headed");
    }

    let status = cmd.status().context("Failed to run Playwright tests")?;

    if status.success() {
        Ok(())
    } else {
        bail!("Playwright tests failed with status: {}", status)
    }
}
