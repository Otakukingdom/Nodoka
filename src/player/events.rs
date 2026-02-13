use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PlaybackState {
    NothingSpecial,
    Opening,
    Buffering,
    Playing,
    Paused,
    Stopped,
    Ended,
    Error,
}

#[derive(Debug, Clone)]
pub enum PlaybackEvent {
    StateChanged(PlaybackState),
    TimeChanged(i64),
    MediaParsed,
    Finished,
    Error(String),
}
