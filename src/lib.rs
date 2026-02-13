//! # Nodoka Audiobook Reader
//!
//! A free and open source cross-platform audiobook player with automatic progress tracking.
//! Built with the iced UI framework and VLC for media playback.
//!
//! ## Features
//!
//! - **Cross-platform**: Works on Windows, macOS, and Linux
//! - **VLC-powered playback**: Supports all VLC-compatible audio formats (MP3, M4A, M4B, OGG, FLAC, WAV, WMA)
//! - **Automatic progress tracking**: Never lose your place with SQLite-based persistence
//! - **Directory scanning**: Automatically discovers audiobooks from your library folders
//! - **Playback controls**: Adjustable speed (0.5x-2.0x) and volume
//! - **Resume functionality**: Picks up exactly where you left off
//!
//! ## Installation
//!
//! ### Prerequisites
//!
//! **IMPORTANT**: Nodoka requires VLC media player 3.x or later to be installed.
//!
//! #### Windows
//! 1. Download and install VLC from [videolan.org](https://www.videolan.org/vlc/)
//! 2. Install to the default location: `C:\Program Files\VideoLAN\VLC`
//!
//! #### macOS
//! Install via Homebrew:
//! ```sh
//! brew install --cask vlc
//! ```
//!
//! #### Linux
//! ```sh
//! # Ubuntu/Debian
//! sudo apt install vlc
//!
//! # Fedora
//! sudo dnf install vlc
//!
//! # Arch
//! sudo pacman -S vlc
//! ```
//!
//! ### Installing Nodoka
//!
//! #### Windows
//! 1. Download `nodoka-0.2.0-x64.msi` from the [releases page](https://github.com/your-username/nodoka/releases)
//! 2. Run the installer and follow the wizard
//! 3. Launch Nodoka from the Start Menu
//!
//! #### macOS
//! 1. Download `Nodoka-0.2.0.dmg` from the [releases page](https://github.com/your-username/nodoka/releases)
//! 2. Open the DMG and drag Nodoka to your Applications folder
//! 3. If you see a "damaged app" warning, run in Terminal:
//!    ```sh
//!    xattr -cr /Applications/Nodoka.app
//!    ```
//!
//! #### Linux
//! 1. Download `nodoka_0.2.0_amd64.deb` from the [releases page](https://github.com/your-username/nodoka/releases)
//! 2. Install with:
//!    ```sh
//!    sudo dpkg -i nodoka_0.2.0_amd64.deb
//!    ```
//! 3. Launch from your application menu or run `nodoka` in terminal
//!
//! ### Building from Source
//!
//! ```sh
//! # Clone the repository
//! git clone https://github.com/your-username/nodoka.git
//! cd nodoka
//!
//! # Build release binary
//! cargo build --release
//!
//! # Run
//! ./target/release/nodoka
//! ```
//!
//! ## Quick Start
//!
//! 1. Launch Nodoka
//! 2. Click **Settings** → **Directories** tab
//! 3. Click **Add Directory** and select your audiobooks folder
//! 4. Wait for scanning to complete
//! 5. Select an audiobook from the list and click Play!
//!
//! ## Architecture
//!
//! Nodoka follows the Elm architecture pattern via the iced framework:
//!
//! - **Model**: Application state stored in [`ui::State`]
//! - **Update**: Message handling in [`ui::update`]
//! - **View**: UI rendering in [`ui::main_window::view()`]
//!
//! ### Key Components
//!
//! - [`app::App`]: Main application entry point
//! - [`db::Database`]: `SQLite` database wrapper for progress tracking
//! - [`player::VlcPlayer`]: VLC-based media player
//! - [`tasks`]: Async operations for directory scanning and file processing
//! - [`models`]: Domain types for audiobooks, files, and directories
//!
//! ## Usage Example
//!
//! ```no_run
//! use nodoka::Database;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Open database
//! let db = Database::open()?;
//!
//! // Initialize schema
//! nodoka::db::initialize(db.connection())?;
//!
//! // Run application
//! nodoka::app::run(db)?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Organizing Your Audiobooks
//!
//! Nodoka works best with this folder structure:
//!
//! ```text
//! Audiobooks/
//! ├── The Great Gatsby/
//! │   ├── Chapter 01.mp3
//! │   ├── Chapter 02.mp3
//! │   └── Chapter 03.mp3
//! ├── 1984/
//! │   └── 1984 - Complete.m4b
//! └── Harry Potter/
//!     ├── Part 1.mp3
//!     └── Part 2.mp3
//! ```
//!
//! - Each audiobook in its own folder
//! - Folder name becomes the audiobook title
//! - Files are sorted alphabetically
//! - Single-file audiobooks work perfectly
//!
//! ## Database Location
//!
//! Your progress is stored in:
//! - **Windows**: `%APPDATA%\Otakukingdom\Nodoka\nodoka.db`
//! - **macOS**: `~/Library/Application Support/com.Otakukingdom.Nodoka/nodoka.db`
//! - **Linux**: `~/.local/share/com/Otakukingdom/Nodoka/nodoka.db`
//!
//! Back up this file to preserve your progress!
//!
//! ## Supported File Formats
//!
//! All VLC-compatible audio formats:
//! - MP3 (.mp3)
//! - M4A/M4B (.m4a, .m4b) - Apple audiobook format
//! - OGG Vorbis (.ogg)
//! - FLAC (.flac)
//! - WAV (.wav)
//! - WMA (.wma)
//!
//! ## Performance Tips
//!
//! - Organize audiobooks before adding directories
//! - Avoid adding entire drives (scan only audiobook folders)
//! - Use local drives instead of network shares for better performance
//! - Large libraries (1000+ files) may take several minutes to scan
//!
//! ## License
//!
//! MIT License - see LICENSE file for details

pub mod app;
pub mod conversions;
pub mod db;
pub mod error;
pub mod models;
pub mod player;
pub mod proxy;
pub mod settings;
pub mod tasks;
pub mod ui;

pub use app::App;
pub use db::Database;
pub use error::{Error, Result};

/// # Contributing to Nodoka
///
/// Thank you for considering contributing to Nodoka! This module outlines the process and standards for contributing.
///
/// ## Code of Conduct
///
/// Be respectful, inclusive, and constructive in all interactions.
///
/// ## Getting Started
///
/// 1. Fork the repository
/// 2. Clone your fork: `git clone https://github.com/your-username/nodoka.git`
/// 3. Create a branch from `main`: `git checkout -b feature/your-feature-name`
/// 4. Make your changes following the code standards below
/// 5. Submit a pull request
///
/// **Note**: Version 0.2.0 represents the complete Rust rewrite baseline. All contributions should build upon this codebase.
///
/// ## Code Standards (STRICT)
///
/// This project enforces **exceptionally strict** linting rules:
///
/// ### Forbidden Patterns
/// - ❌ `unwrap()` - Use proper error handling with `?` or `match`
/// - ❌ `expect()` - Same as unwrap, use Result types
/// - ❌ `panic!()` - Handle errors gracefully
/// - ❌ `#[allow(...)]` - Do not suppress warnings without inline justification
/// - ❌ `unsafe` - Use safe Rust patterns
/// - ❌ Dead code - Remove unused functions and imports
///
/// ### Required Practices
/// - ✅ All errors returned as `Result<T, Error>`
/// - ✅ Doc comments on all public APIs with `/// Errors` and `/// Panics` sections
/// - ✅ Clippy passes with `-D warnings` flag
/// - ✅ All tests pass: `cargo test`
/// - ✅ Code formatted: `cargo fmt`
///
/// ## Testing Requirements
///
/// - Add tests for new functionality
/// - Maintain 100% test pass rate
/// - Use temp-dir crate for integration tests requiring file system access
/// - Run the full test suite before submitting: `cargo test --all`
///
/// ## Pull Request Process
///
/// 1. Ensure your code passes all checks:
///    ```bash
///    cargo fmt --check
///    cargo clippy --all-targets --all-features -- -D warnings
///    cargo test --all
///    ```
/// 2. Update documentation if adding public APIs
/// 3. Add entry to `CHANGELOG.md` under `[Unreleased]`
/// 4. Reference any related issues in PR description
/// 5. Wait for CI/CD to pass (GitHub Actions)
/// 6. Request review from maintainers
///
/// ## Areas for Contribution
///
/// - 🐛 Bug fixes
/// - 📚 Documentation improvements
/// - ✨ New features (discuss in issue first)
/// - 🧪 Additional tests
/// - 🌐 Translations (when i18n support is added)
/// - ♿ Accessibility improvements
///
/// ## Development Setup
///
/// See the main documentation for build instructions and dependencies.
///
/// ## Linting Configuration
///
/// The project uses strict linting enforced in `Cargo.toml`:
///
/// ```toml
/// [lints.clippy]
/// all = { level = "deny", priority = -1 }
/// unwrap_used = { level = "deny", priority = 0 }
/// expect_used = { level = "deny", priority = 0 }
/// panic = { level = "deny", priority = 0 }
/// ```
///
/// Function-level allows are permitted only with inline justification for unavoidable framework interoperability.
///
/// ## Error Handling
///
/// All fallible operations must return `Result<T, Error>`:
///
/// **❌ Bad** (don't do this):
/// ```rust,no_run
/// fn read_file_bad() -> String {
///     std::fs::read_to_string("file.txt").unwrap()
/// }
/// ```
///
/// **✅ Good** (do this instead):
/// ```rust,no_run
/// # use nodoka::error::Result;
/// fn read_file() -> Result<String> {
///     Ok(std::fs::read_to_string("file.txt")?)
/// }
/// ```
///
/// ## Documentation
///
/// Add doc comments to all public items:
///
/// ```rust,no_run
/// # use std::path::Path;
/// /// Calculates the SHA-256 checksum of a file.
/// ///
/// /// # Errors
/// ///
/// /// Returns an error if the file cannot be read or if I/O fails during hashing.
/// ///
/// /// # Examples
/// ///
/// /// ```no_run
/// /// # use std::path::Path;
/// /// # async fn example() -> Result<(), std::io::Error> {
/// /// # let calculate_checksum = |_: &Path| async { Ok("abc".to_string()) };
/// /// let checksum = calculate_checksum(Path::new("audio.mp3")).await?;
/// /// assert_eq!(checksum.len(), 64); // SHA-256 is 64 hex characters
/// /// # Ok(())
/// /// # }
/// /// ```
/// pub async fn calculate_checksum(path: &Path) -> Result<String, std::io::Error> {
///     // implementation
/// #   Ok(String::new())
/// }
/// ```
///
/// ## Testing
///
/// Write tests for new functionality:
///
/// ```rust,no_run
/// #[cfg(test)]
/// mod tests {
///     use super::*;
///     use temp_dir::TempDir;
///     # use nodoka::error::Result;
///
///     #[test]
///     fn test_my_feature() -> Result<()> {
///         let temp = TempDir::new()?;
///         // test implementation
///         Ok(())
///     }
/// }
/// ```
///
/// ## Commit Messages
///
/// Write clear, concise commit messages:
///
/// - Use present tense ("Add feature" not "Added feature")
/// - Capitalize first letter
/// - No period at the end
/// - Reference issues when applicable (#123)
///
/// Examples:
/// - `Add volume control to player interface`
/// - `Fix database connection leak on error`
/// - `Refactor audiobook scanning for better performance`
/// - `Update README with troubleshooting section`
///
/// ## Questions?
///
/// Open an issue with the "question" label or start a discussion.
///
/// ## License
///
/// By contributing, you agree that your contributions will be licensed under the MIT License.
pub mod contributing {}

/// # Security Policy
///
/// ## Supported Versions
///
/// | Version | Supported          |
/// | ------- | ------------------ |
/// | 0.2.x   | ✅                 |
/// | 0.1.x   | ❌                 |
///
/// ## Reporting a Vulnerability
///
/// If you discover a security vulnerability in Nodoka, please report it by:
///
/// 1. **DO NOT** open a public GitHub issue
/// 2. Email security concerns to: (add email when available)
/// 3. Include detailed information:
///    - Description of the vulnerability
///    - Steps to reproduce
///    - Potential impact
///    - Suggested fix (if any)
///
/// We will respond within 48 hours and provide a timeline for a fix.
///
/// ## Security Best Practices
///
/// ### Dependencies
///
/// Nodoka uses the following security measures for dependencies:
///
/// - **Stable versions only**: No alpha/beta/rc dependencies in production
/// - **Minimal dependency tree**: ~26 production dependencies, all from trusted sources
/// - **Bundled `SQLite`**: Using rusqlite with bundled feature to avoid system library vulnerabilities
/// - **Regular updates**: Dependencies reviewed and updated quarterly
///
/// ### Code Quality
///
/// - **No unsafe code**: `#[deny(unsafe_code)]` enforced at compile time
/// - **No unwrap/expect**: All errors handled with Result types
/// - **No panic**: Graceful error handling throughout
/// - **Strict clippy lints**: Enforced at compile time with `-D warnings`
///
/// ### Runtime Security
///
/// - **Single instance guard**: Prevents multiple instances from corrupting database
/// - **Database file permissions**: User-only read/write on database files
/// - **No network access**: Application does not make network requests
/// - **Local data only**: All data stored locally in user's config directory
///
/// ### VLC Integration
///
/// - **Dynamic linking**: VLC loaded from system installation
/// - **Plugin restrictions**: VLC plugin loading restricted to trusted paths
/// - **Version requirements**: VLC 3.x required (tested versions)
///
/// ## Known Limitations
///
/// 1. **VLC 4.x compatibility**: Not yet tested, use VLC 3.x
/// 2. **Unsigned binaries**: macOS and Windows installers are not code-signed
///    - macOS: May require `xattr -cr` to bypass Gatekeeper
///    - Windows: May trigger `SmartScreen` warnings
/// 3. **No sandboxing**: Application runs with full user permissions
///
/// ## Audit History
///
/// - **2026-02-12**: Initial security review for v0.2.0 release
///   - Dependency tree reviewed (26 stable dependencies)
///   - No known vulnerabilities in dependencies
///   - All security best practices implemented
///
/// ## Security Checklist (for contributors)
///
/// Before submitting changes:
///
/// - [ ] No new unsafe code introduced
/// - [ ] All errors handled with Result types
/// - [ ] No unwrap/expect/panic in source code
/// - [ ] Clippy passes with `-D warnings`
/// - [ ] No new dependencies without justification
/// - [ ] New dependencies are from trusted sources
/// - [ ] File operations use proper error handling
/// - [ ] No hardcoded credentials or secrets
/// - [ ] No logging of sensitive user data
///
/// ## Future Improvements
///
/// - [ ] Implement code signing for macOS and Windows
/// - [ ] Add automated dependency vulnerability scanning in CI/CD
/// - [ ] Implement application sandboxing on supported platforms
/// - [ ] Add integrity verification for database files
/// - [ ] Implement automatic backup of database
pub mod security {}
