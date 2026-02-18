//! # Nodoka Audiobook Reader
//!
//! A free and open source cross-platform audiobook player with automatic progress tracking.
//! Built with the iced UI framework and VLC for media playback.
//!
//! ## Features
//!
//! - **Cross-platform**: Works on Windows, macOS, and Linux
//! - **VLC-powered playback**: Supports all VLC-compatible audio formats (MP3, M4A, M4B, OGG, FLAC, OPUS, AAC, WAV, WMA)
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
//! 1. Download `nodoka-0.2.0-x64.msi` from the [releases page](https://github.com/otakukingdom/nodoka/releases)
//! 2. Run the installer and follow the wizard
//! 3. Launch Nodoka from the Start Menu
//!
//! #### macOS
//! 1. Download `Nodoka-0.2.0.dmg` from the [releases page](https://github.com/otakukingdom/nodoka/releases)
//! 2. Open the DMG and drag Nodoka to your Applications folder
//! 3. If you see a "damaged app" warning, run in Terminal:
//!    ```sh
//!    xattr -cr /Applications/Nodoka.app
//!    ```
//!
//! #### Linux
//! 1. Download `nodoka_0.2.0_amd64.deb` from the [releases page](https://github.com/otakukingdom/nodoka/releases)
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
//! git clone https://github.com/otakukingdom/nodoka.git
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
//! ## Keyboard Shortcuts
//!
//! Nodoka supports comprehensive keyboard navigation for improved accessibility:
//!
//! | Key | Action |
//! |-----|--------|
//! | Space | Play/Pause |
//! | ← | Seek backward 5 seconds |
//! | → | Seek forward 5 seconds |
//! | ↑ | Previous file |
//! | ↓ | Next file |
//! | Ctrl+B (Cmd+B on macOS) | Create bookmark |
//! | F1 | Show keyboard help |
//! | Escape | Close modal |
//!
//! All interactive elements support keyboard navigation via Tab and Enter keys.
//!
//! ## Accessibility
//!
//! Nodoka follows WCAG 2.1 Level AA guidelines:
//! - **Keyboard navigation** for all interactive elements
//! - **Color contrast** ratio ≥ 4.5:1 for text
//! - **Clear error messages** and loading states
//! - **Visual feedback** for selection and hover states
//!
//! ### Known Limitations
//!
//! - Focus visibility depends on iced's built-in widget focus styling; not all widgets expose rich focus state for customization.
//! - Screen reader semantics are limited by framework-level accessibility support; Nodoka prioritizes clear text labels and keyboard navigation.
//! - Modal backdrops are implemented using iced 0.14 stack overlays, including click-outside-to-dismiss behavior.
//!
//! ## User Experience Enhancements
//!
//! Nodoka includes comprehensive UX improvements following modern design principles:
//!
//! ### Visual Hierarchy
//! - **Enhanced audiobook list**: 80×80px cover thumbnails with 4px left border selection indicator
//! - **Improved spacing**: Consistent 8px spacing for better touch targets (WCAG compliant)
//! - **Visual feedback**: Hover states with subtle color transitions on all interactive elements
//! - **Loading states**: Skeleton UI with shimmer effects during library scanning
//!
//! ### Micro-Interactions
//! - **Smooth animations**: Easing functions (ease-in, ease-out, ease-in-out) for natural motion
//! - **Reduced motion support**: Respects system `prefers-reduced-motion` preference
//! - **Pulse animations**: Active state indicators for currently playing files
//! - **Progress transitions**: Interpolated progress bar updates with configurable duration
//!
//! ### User Feedback
//! - **Toast notifications**: Non-intrusive success/info messages (3s auto-dismiss)
//! - **Error persistence**: Error notifications require manual dismissal
//! - **Notification stacking**: Maximum 3 concurrent notifications to avoid clutter
//! - **Visual distinction**: Color-coded notification types (success: green, error: red, etc.)
//!
//! ### Enhanced Components
//!
//! #### Audiobook List
//! - Larger cover thumbnails (60→80px) for better visual prominence
//! - Enhanced hover states with subtle elevation simulation
//! - Selection highlight with colored left border accent
//! - Empty state with helpful guidance when library is empty
//!
//! #### Player Controls
//! - Speed preset buttons (0.5×, 0.75×, 1×, 1.25×, 1.5×, 2×) with active state indication
//! - Improved slider sizing for easier interaction
//! - Visual feedback for all control state changes
//!
//! #### File List
//! - File number prefixes (#1, #2, etc.) for better navigation context
//! - Duration display next to each file name
//! - Progress indicators for partially listened files
//! - Empty state guidance for audiobooks without files
//!
//! #### Bookmarks
//! - Timestamp badges with rounded styling and colored backgrounds
//! - Note indicators showing bookmark descriptions
//! - Visual grouping for better organization
//! - Empty state with call-to-action to create first bookmark
//!
//! #### Settings Form
//! - Section headers with visual separators for better scannability
//! - Inline help text explaining each setting's purpose
//! - Improved directory list with enhanced action buttons
//!
//! #### Modals
//! - Fade-in backdrop (50% opacity) for clear focus distinction
//! - Keyboard help modal accessible via F1 key
//! - Slide-up animations for modal cards
//! - Click-outside-to-dismiss functionality
//!
//! ### Design System
//!
//! All enhancements follow a consistent design system:
//! - **Color palette**: WCAG AA compliant (4.5:1 minimum contrast)
//! - **Typography scale**: Monotonic sizes from 12px to 32px
//! - **Spacing scale**: 4px base grid (4, 8, 12, 16, 24, 32px)
//! - **Border radius**: SM (4px), MD (8px), LG (12px), XL (16px)
//! - **Transitions**: Fast (200ms), Normal (250ms), Slow (300ms)
//!
//! ### Migration Guide for UI Components
//!
//! If you're using Nodoka's UI components as a library:
//!
//! #### New State Fields
//! ```rust,no_run
//! use nodoka::ui::State;
//!
//! let state = State::default();
//!
//! // New fields for animations and notifications:
//! // - notification_queue: Vec<Notification>
//! // - active_modal: ActiveModal
//! // - prefers_reduced_motion: bool
//! // - modal_state: ModalState
//! ```
//!
//! #### Notification System
//! ```rust,no_run
//! use nodoka::ui::{Notification, NotificationType};
//! # let mut state = nodoka::ui::State::default();
//!
//! // Create notifications
//! let success = Notification::success("Bookmark saved".to_string());
//! let info = Notification::info("Scanning library...".to_string());
//! let warning = Notification::warning("File not found".to_string());
//! let error = Notification::error("Database error".to_string());
//!
//! // Add to state
//! state.notification_queue.push(success);
//! ```
//!
//! #### Animation Utilities
//! ```rust,no_run
//! use nodoka::ui::animations::{easing, PulseAnimation};
//!
//! // Easing functions for smooth transitions
//! let t = 0.5; // Progress value 0.0-1.0
//! let eased = easing::ease_in_out(t); // Returns smoothed value
//!
//! // Pulse animations for active indicators
//! let mut pulse = PulseAnimation::new(1000); // 1 second cycle
//! pulse.update(16); // Update with frame delta (ms)
//! let opacity = pulse.opacity(); // Returns 0.5-1.0
//! ```
//!
//! #### Transition States
//! ```rust,no_run
//! use nodoka::ui::TransitionState;
//!
//! // Create smooth value transitions
//! let mut transition = TransitionState::new(0.0, 100.0, 300); // from, to, duration_ms
//! transition.update(150); // Update with delta time
//! let current = transition.current_value(); // Returns interpolated value
//! let complete = transition.is_complete(); // Check if finished
//! ```
//!
//! ### UX Testing & Quality
//!
//! All UX enhancements maintain strict quality standards:
//! - **UX compliance tests**: Verify design system adherence and animation behavior
//! - **23 integration tests**: Validate notifications, modals, and interactive states
//! - **WCAG AA compliance**: All color pairs tested for 4.5:1 contrast minimum
//! - **Reduced motion**: Automated tests verify animations disable correctly
//! - **Zero clippy warnings**: All enhancements pass strict linting
//!
//! See `tests/ux_compliance_tests.rs` and `tests/ui_enhanced_interactions_tests.rs` for comprehensive test coverage.
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
//! - [`player::Vlc`]: VLC-based media player
//! - [`tasks`]: Async operations for directory scanning and file processing
//! - [`models`]: Domain types for audiobooks, files, and directories
//!
//! ## VLC Initialization
//!
//! This application uses VLC media player for audio playback. When using Nodoka
//! as a library, you should call [`player::setup_vlc_environment()`] early in
//! your application, before creating any player instances:
//!
//! ```no_run
//! use nodoka::player::setup_vlc_environment;
//!
//! setup_vlc_environment();
//! // ... rest of your application
//! ```
//!
//! Player constructors ([`player::Vlc::new()`] and [`player::Scanner::new()`])
//! call this automatically for convenience, but calling it explicitly at startup
//! provides more reliable initialization.
//!
//! ## Usage Example
//!
//! ```no_run
//! use nodoka::Database;
//! use nodoka::player::setup_vlc_environment;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Setup VLC environment first
//! setup_vlc_environment();
//!
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
//! ## Testing & Quality
//!
//! Nodoka includes a comprehensive acceptance test suite:
//!
//! - Automated acceptance tests covering core flows
//! - **Specification coverage** validated by the acceptance suite
//! - **All 9 audio formats tested** (MP3, M4A, M4B, OGG, FLAC, OPUS, AAC, WAV, WMA)
//! - **Performance validated** for libraries with 1000+ audiobooks
//! - **Cross-platform testing** on Windows, macOS, and Linux
//! - **Zero linting warnings** with strict Clippy configuration
//! - **No unwrap/panic** in production code (enforced)
//!
//! Run tests:
//! ```sh
//! cargo test --tests                # Integration tests (includes acceptance_*)
//! cargo test --all                  # All tests
//! ```
//!
//! See `tests/acceptance_tests.rs` for the acceptance suite overview.
//!
//! ## License
//!
//! MIT License - see LICENSE file for details

pub(crate) mod audio_extensions;
pub mod app;
pub mod conversions;
pub mod cover_art;
pub mod cover_cache;
pub mod db;
pub mod error;
pub mod models;
pub mod player;
pub mod proxy;
pub mod settings;
pub mod tasks;
pub mod ui;

#[cfg(test)]
pub(crate) mod test_support;

pub use app::App;
pub use db::Database;
pub use error::{Error, Result};
