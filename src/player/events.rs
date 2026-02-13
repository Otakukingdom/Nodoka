use serde::{Deserialize, Serialize};

#[allow(clippy::module_name_repetitions)] // "PlayerState" is idiomatic for player module state enum
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PlayerState {
    NothingSpecial,
    Opening,
    Buffering,
    Playing,
    Paused,
    Stopped,
    Ended,
    Error,
}

#[allow(clippy::module_name_repetitions)] // "PlayerEvent" is idiomatic for player module event enum
#[derive(Debug, Clone)]
pub enum PlayerEvent {
    StateChanged(PlayerState),
    TimeChanged(i64),
    MediaParsed,
    Finished,
    Error(String),
}
