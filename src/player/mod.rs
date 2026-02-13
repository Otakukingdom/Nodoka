//! VLC-based media player abstraction.
//!
//! This module provides a safe wrapper around the VLC media player library
//! for audiobook playback with progress tracking and event notification.
//!
//! ## Components
//!
//! - [`ConcretePlayer`]: Main player for playback with full controls (play, pause, seek, volume, speed)
//! - [`ScanPlayer`]: Lightweight player for extracting media metadata during directory scanning
//! - [`PlayerEvent`]: Events emitted by the player (state changes, time updates)
//! - [`PlayerState`]: Current playback state (playing, paused, stopped, ended)
//!
//! ## Usage
//!
//! ```no_run
//! # use nodoka::player::ConcretePlayer;
//! # use nodoka::error::Result;
//! # use std::path::Path;
//! # fn example() -> Result<()> {
//! // Create player
//! let mut player = ConcretePlayer::new()?;
//!
//! // Load and play media
//! player.load_media(Path::new("audiobook.mp3"))?;
//! player.play()?;
//!
//! // Control playback
//! player.set_volume(80)?;
//! player.set_rate(1.25)?;
//! player.set_time(5000)?; // Seek to 5 seconds
//!
//! // Query state
//! let time = player.get_time()?;
//! let length = player.get_length()?;
//! let state = player.get_state();
//! # Ok(())
//! # }
//! ```
//!
//! ## State Monitoring
//!
//! Query the player state at any time:
//!
//! ```no_run
//! # use nodoka::player::{ConcretePlayer, PlayerState};
//! # use nodoka::error::Result;
//! # fn example() -> Result<()> {
//! let player = ConcretePlayer::new()?;
//!
//! // Check current state
//! let state = player.get_state();
//! match state {
//!     PlayerState::Playing => {
//!         // Player is playing
//!     }
//!     PlayerState::Paused => {
//!         // Player is paused
//!     }
//!     PlayerState::Stopped => {
//!         // Player is stopped
//!     }
//!     PlayerState::Ended => {
//!         // Playback finished
//!     }
//!     _ => {
//!         // Other states (buffering, opening, error, etc.)
//!     }
//! }
//! # Ok(())
//! # }
//! ```

mod concrete_player;
mod events;
mod scan_player;

pub use concrete_player::ConcretePlayer;
pub use events::{PlayerEvent, PlayerState};
pub use scan_player::ScanPlayer;
