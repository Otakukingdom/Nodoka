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
//! - **Model**: Application state stored in [`ui::NodokaState`]
//! - **Update**: Message handling in [`ui::update()`]
//! - **View**: UI rendering in [`ui::main_window::view()`]
//!
//! ### Key Components
//!
//! - [`app::NodokaApp`]: Main application entry point
//! - [`db::Database`]: `SQLite` database wrapper for progress tracking
//! - [`player::ConcretePlayer`]: `VLC`-based media player
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
//! nodoka::db::initialize_schema(db.connection())?;
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

pub use app::NodokaApp;
pub use db::Database;
pub use error::{NodokaError, Result};
