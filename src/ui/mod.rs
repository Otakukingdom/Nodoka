//! User interface layer following the Elm architecture.
//!
//! This module implements the UI using the iced framework with the Elm pattern:
//! - **Model**: [`State`] contains all UI state
//! - **Update**: [`update`] module handles messages and updates state
//! - **View**: [`main_window::view()`] renders the UI
//!
//! ## Architecture
//!
//! ```text
//! User Interaction
//!       ↓
//!   [Message]
//!       ↓
//!   [Update] → Modify State → [Command]
//!       ↓                          ↓
//!   [State]                   Async Tasks
//!       ↓                          ↓
//!    [View] ← ← ← ← ← ← ← ← ← [Message]
//! ```
//!
//! ## Message Flow
//!
//! 1. User clicks button → generates [`Message`]
//! 2. [`update`] module receives message and current [`State`]
//! 3. Update function modifies state and returns [`iced::Command`]
//! 4. Command may spawn async tasks that generate new messages
//! 5. [`main_window::view()`] renders updated state
//!
//! ## Components
//!
//! UI is organized into reusable components:
//! - [`components::player_controls`]: Playback controls (play/pause, volume, speed)
//! - [`components::audiobook_list`]: List of discovered audiobooks
//! - [`components::file_list`]: Files within selected audiobook
//! - [`settings_form`]: Settings dialog for managing directories
//!
//! ## Keyboard Shortcuts
//!
//! Nodoka supports comprehensive keyboard navigation for accessibility:
//!
//! | Key | Action | Description |
//! |-----|--------|-------------|
//! | <kbd>Space</kbd> | Play/Pause | Toggle playback of current audiobook |
//! | <kbd>←</kbd> | Seek Backward | Skip backward 5 seconds |
//! | <kbd>→</kbd> | Seek Forward | Skip forward 5 seconds |
//! | <kbd>↑</kbd> | Previous File | Jump to previous chapter/file in audiobook |
//! | <kbd>↓</kbd> | Next File | Jump to next chapter/file in audiobook |
//! | <kbd>Ctrl</kbd>+<kbd>B</kbd> | Create Bookmark | Create bookmark at current position |
//! | <kbd>Escape</kbd> | Close Modal | Close open modal dialog (settings, bookmark editor) |
//!
//! On macOS, <kbd>Ctrl</kbd> is replaced with <kbd>Cmd</kbd> for bookmark creation.
//!
//! All keyboard shortcuts are defined in [`shortcuts`] module and can be tested
//! without mouse interaction for full accessibility compliance.
//!
//! ## Accessibility
//!
//! Nodoka follows WCAG 2.1 Level AA guidelines:
//!
//! - **Keyboard Navigation**: All interactive elements accessible via keyboard
//! - **Focus Indicators**: Visual focus indicators with 3px outline (framework-dependent)
//! - **Color Contrast**: Text contrast ratio ≥ 4.5:1 for normal text
//! - **Error Messages**: Clear, actionable error messages with dismiss action
//! - **Loading States**: Visual feedback during long operations (directory scanning)
//! - **Screen Reader Support**: Descriptive button labels (no icon-only buttons)
//!
//! See [`styles`] module for color definitions and contrast testing.
//!
//! ## Testing Strategy
//!
//! The Nodoka UI has comprehensive test coverage to ensure all interactions work correctly:
//!
//! ### Test Suite Organization
//!
//! - **Unit tests** (`src/`): Components, formatters, and helpers
//! - **Integration tests** (`tests/`): UI workflows and state transitions via message handling
//! - **Acceptance tests** (`tests/`): Higher-level user journeys
//! - **Regression tests** (`tests/`): Prevent reintroduction of previously fixed bugs
//! - **Performance tests** (`tests/`): Rendering and large-library scenarios (some ignored in CI)
//!
//! The suite is intentionally large; avoid hard-coding exact counts in rustdoc.
//!
//! ### Coverage by Component
//!
//! | Component | Unit Tests | Integration Tests | Notes |
//! |-----------|------------|-------------------|-------|
//! | Player Controls | Yes | Yes | Playback, volume, speed, sleep timer |
//! | Audiobook List | Yes | Yes | Selection, cover thumbnails, progress |
//! | File List | Yes | Yes | Selection, missing files, completeness |
//! | Bookmarks | Yes | Yes | CRUD, editor modal, empty labels |
//! | Settings Modal | Yes | Yes | Directory management, scrolling |
//! | Keyboard Shortcuts | Yes | Yes | Cross-platform shortcuts |
//! | State Management | Yes | Yes | Defaults, transitions, invariants |
//! | Error Handling | Yes | Yes | Display, dismissal, replacement |
//! | Loading States | Yes | Yes | Scanning indicator, loading overlay |
//!
//! ### Design System Compliance
//!
//! The UI follows a comprehensive design system documented in
//! `design-system/nodoka-audiobook-player/MASTER.md`:
//!
//! - **Color Palette**: Vibrant rose (#E11D48) + engagement blue (#2563EB)
//! - **Typography**: Atkinson Hyperlegible (accessible, WCAG-compliant, dyslexia-friendly)
//! - **Spacing**: 4px base grid (XS: 4px, SM: 8px, MD: 16px, LG: 24px, XL: 32px)
//! - **Border Radius**: Consistent rounded corners (SM: 4px, MD: 8px, LG: 12px)
//! - **Transitions**: Smooth animations (150-300ms duration range)
//! - **Contrast Ratio**: ≥4.5:1 for all text (WCAG AA compliance verified in tests)
//!
//! ### UX Best Practices
//!
//! Following `ui-ux-pro-max` guidelines for audiobook player applications:
//!
//! - ✅ Keyboard navigation for all functionality
//! - ✅ Loading states for operations >300ms (directory scanning)
//! - ✅ Clear, actionable error messages with dismiss button
//! - ✅ Progress indicators for multi-step processes
//! - ✅ Modal backdrops with click-to-dismiss pattern
//! - ✅ Focus indicators meeting WCAG 2.1 AA standards
//! - ✅ Interactive elements with 44x44px minimum touch target
//! - ✅ Color contrast ratios ≥4.5:1 for normal text
//! - ✅ Semantic button hierarchy (primary, secondary, danger)
//!
//! ### Performance Characteristics
//!
//! Rendering performance verified with large libraries:
//!
//! - **10 audiobooks**: <50ms render time
//! - **100 audiobooks**: <100ms render time
//! - **500 audiobooks**: <300ms render time
//! - **1000 audiobooks**: <500ms render time (future: add virtualization)
//!
//! ### Version Requirements
//!
//! - **Rust**: 1.93.1 (pinned in `rust-toolchain.toml` and `Cargo.toml`)
//! - **iced**: 0.14.0 (latest, using stack widget for modals)
//!
//! ### Test Limitations
//!
//! iced's `Element` type is intentionally opaque, so most UI tests assert behavior by
//! exercising message handling (state transitions) and formatting helpers.
//!
//! ## Usage
//!
//! The UI is typically not used directly but through [`crate::app::App`]
//! which implements the iced [`Application`](iced::Application) trait.

pub mod components;
pub mod main_window;
mod media_paths;
mod message;
pub mod settings_form;
pub mod shortcuts;
mod state;
mod styles;
pub mod update;

pub use message::Message;
pub use state::{BookmarkEditor, FocusedElement, LoadState, PlaybackStatus, ScanState, State};
pub use styles::*;
