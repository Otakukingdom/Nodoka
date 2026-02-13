//! VLC-based media player abstraction.
//!
//! This module provides a safe wrapper around the VLC media player library
//! for audiobook playback with progress tracking and event notification.
//!
//! ## Components
//!
//! - [`VlcPlayer`]: Main player for playback with full controls (play, pause, seek, volume, speed)
//! - [`Scanner`]: Lightweight scanner for extracting media metadata during directory scanning
//! - [`PlayerEvent`]: Events emitted by the player (state changes, time updates)
//! - [`PlayerState`]: Current playback state (playing, paused, stopped, ended)
//!
//! ## Usage
//!
//! ```no_run
//! # use nodoka::player::VlcPlayer;
//! # use nodoka::error::Result;
//! # use std::path::Path;
//! # fn example() -> Result<()> {
//! // Create player
//! let mut player = VlcPlayer::new()?;
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
//! # use nodoka::player::{VlcPlayer, PlayerState};
//! # use nodoka::error::Result;
//! # fn example() -> Result<()> {
//! let player = VlcPlayer::new()?;
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

#[allow(clippy::module_name_repetitions)]
// VlcPlayer is descriptive - VLC is the implementation detail
pub use concrete_player::VlcPlayer;
#[allow(clippy::module_name_repetitions)]
// PlayerEvent and PlayerState are idiomatic event/state patterns
pub use events::{PlayerEvent, PlayerState};
pub use scan_player::Scanner;
