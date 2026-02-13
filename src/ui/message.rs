use crate::player::PlayerState;

#[derive(Debug, Clone)]
pub enum Message {
    // Player controls
    PlayPause,
    Stop,
    SeekTo(i64),
    VolumeChanged(i32),
    SpeedChanged(f32),

    // Audiobook list
    AudiobookSelected(i64),
    AudiobookRemove(i64),
    AudiobookRescan(i64),
    AudiobookMarkComplete(i64),
    AudiobookResetProgress(i64),

    // File list
    FileSelected(String),

    // Directory management
    DirectoryAdd,
    DirectoryAdded(String),
    DirectoryAddCancelled,
    DirectoryRemove(String),
    DirectoryRemoved(String),
    DirectoryRescan(String),

    // Settings
    OpenSettings,
    CloseSettings,

    // Player events
    PlayerStateChanged(PlayerState),
    PlayerTimeUpdated(i64),
    PlayerMediaParsed,

    // Background tasks
    ScanComplete(Vec<crate::models::Audiobook>),
    ScanError(String),
    ChecksumCalculated(String, String),

    // Initial load
    InitialLoadComplete,

    // No-op
    None,
}
