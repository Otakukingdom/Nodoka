//! VLC-based media player abstraction.
//!
//! This module provides a safe wrapper around the VLC media player library
//! for audiobook playback with progress tracking and event notification.
//!
//! ## Components
//!
//! - [`Vlc`]: Main player for playback with full controls (play, pause, seek, volume, speed)
//! - [`Scanner`]: Lightweight scanner for extracting media metadata during directory scanning
//! - [`PlaybackEvent`]: Events emitted by the player (state changes, time updates)
//! - [`PlaybackState`]: Current playback state (playing, paused, stopped, ended)
//!
//! ## Usage
//!
//! ```no_run
//! # use nodoka::player::Vlc;
//! # use nodoka::error::Result;
//! # use std::path::Path;
//! # fn example() -> Result<()> {
//! // Create player
//! let mut player = Vlc::new()?;
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
//! # use nodoka::player::{PlaybackState, Vlc};
//! # use nodoka::error::Result;
//! # fn example() -> Result<()> {
//! let player = Vlc::new()?;
//!
//! // Check current state
//! let state = player.get_state();
//! match state {
//!     PlaybackState::Playing => {
//!         // Player is playing
//!     }
//!     PlaybackState::Paused => {
//!         // Player is paused
//!     }
//!     PlaybackState::Stopped => {
//!         // Player is stopped
//!     }
//!     PlaybackState::Ended => {
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
mod media_duration;
mod scan_player;
mod vlc_env;

pub use concrete_player::Vlc;
pub use events::{PlaybackEvent, PlaybackState};
pub use scan_player::Scanner;
pub use vlc_env::{setup_vlc_environment, verify_vlc_available};

#[doc(hidden)]
pub use vlc_env::{
    VlcInitEvent, VlcTestHookGuard, __set_vlc_init_observer_for_tests,
    __set_vlc_instance_factory_for_tests,
};

mod legacy {
    use super::{PlaybackEvent, PlaybackState, Vlc};

    pub type VlcPlayer = Vlc;

    pub type PlayerState = PlaybackState;

    pub type PlayerEvent = PlaybackEvent;
}

#[deprecated(
    since = "0.2.0",
    note = "Renamed: use `nodoka::player::Vlc` instead of `VlcPlayer`"
)]
pub use legacy::VlcPlayer;

#[deprecated(
    since = "0.2.0",
    note = "Renamed: use `nodoka::player::PlaybackState` instead of `PlayerState`"
)]
pub use legacy::PlayerState;

#[deprecated(
    since = "0.2.0",
    note = "Renamed: use `nodoka::player::PlaybackEvent` instead of `PlayerEvent`"
)]
pub use legacy::PlayerEvent;
