use serde::{Deserialize, Serialize};

/// Playback state reported by the underlying media player.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PlaybackState {
    /// No active media or no meaningful state.
    NothingSpecial,
    /// Media is opening and preparing for playback.
    Opening,
    /// Media is buffering.
    Buffering,
    /// Media is currently playing.
    Playing,
    /// Media is paused.
    Paused,
    /// Playback is stopped.
    Stopped,
    /// Playback reached the end of the media.
    Ended,
    /// Playback encountered an error.
    Error,
}

/// Discrete events emitted by the player for UI/state synchronization.
#[derive(Debug, Clone)]
pub enum PlaybackEvent {
    /// Player state changed.
    StateChanged(PlaybackState),
    /// Playback time changed (milliseconds).
    TimeChanged(i64),
    /// Media parsing completed.
    MediaParsed,
    /// Playback finished.
    Finished,
    /// Playback error with a human-readable message.
    Error(String),
}
