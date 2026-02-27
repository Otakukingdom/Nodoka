//! xtask automation tool for Nodoka development tasks.
//!
//! # Usage
//!
//! ```bash
//! cargo run -p xtask -- screenshot
//! ```
//!
//! This tool provides automated screenshot capture for all UI scenarios,
//! generating images for both light and dark themes. Screenshots are saved
//! to the `screenshots/` directory (gitignored) with a manifest.json file
//! documenting what was captured.
//!
//! ## Commands
//!
//! - `screenshot` - Capture UI screenshots for all scenarios in light and dark themes
//!
//! ## Output Structure
//!
//! ```text
//! screenshots/
//!   light/
//!     01-empty-library.png
//!     02-library-with-audiobooks.png
//!     ...
//!   dark/
//!     01-empty-library.png
//!     02-library-with-audiobooks.png
//!     ...
//!   manifest.json
//! ```

mod capture;
mod scenarios;

use std::env;
use std::path::PathBuf;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        process::exit(1);
    }

    match args.get(1).map(std::string::String::as_str) {
        Some("screenshot") => {
            if let Err(e) = run_screenshot_capture() {
                eprintln!("Error: Failed to capture screenshots: {e}");
                process::exit(1);
            }
        }
        Some(cmd) => {
            eprintln!("Error: Unknown command '{cmd}'");
            print_usage();
            process::exit(1);
        }
        None => {
            print_usage();
            process::exit(1);
        }
    }
}

fn print_usage() {
    eprintln!("Usage: cargo xtask <command>");
    eprintln!();
    eprintln!("Commands:");
    eprintln!("  screenshot    Capture UI screenshots for all scenarios");
}

fn run_screenshot_capture() -> Result<(), Box<dyn std::error::Error>> {
    let output_dir = PathBuf::from("screenshots");

    println!("Capturing screenshots to: {}", output_dir.display());
    println!();

    let manifest = capture::capture_all_screenshots(&output_dir)?;

    let manifest_path = output_dir.join("manifest.json");
    manifest.save_to_file(&manifest_path)?;

    println!();
    println!("✓ Captured {} screenshots", manifest.scenarios.len());
    println!("✓ Manifest saved to: {}", manifest_path.display());

    Ok(())
}
