use crate::player::PlaybackState;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum Message {
    // Player controls
    PlayPause,
    Stop,
    SeekTo(f64),
    VolumeChanged(i32),
    SpeedChanged(f32),

    // Sleep timer
    SleepTimerSetDurationSeconds(i64),
    SleepTimerSetEndOfChapter,
    SleepTimerExtendSeconds(i64),
    SleepTimerCancel,

    // Shortcuts
    CreateBookmark,

    // Bookmarks UI
    BookmarkEdit(i64),
    BookmarkDelete(i64),
    BookmarkJump(i64),
    BookmarkEditorLabelChanged(String),
    BookmarkEditorNoteChanged(String),
    BookmarkEditorSave,
    BookmarkEditorCancel,

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
    PlayerStateChanged(PlaybackState),
    PlayerTimeUpdated(f64),
    PlayerTick,
    PlayerMediaParsed,

    // Background tasks
    ScanComplete(String, Vec<crate::tasks::DiscoveredAudiobook>),
    ScanError(String),
    ChecksumCalculated(String, String),
    CoverThumbnailGenerated(i64, Option<PathBuf>),

    // Initial load
    InitialLoadComplete,

    // No-op
    None,
}
