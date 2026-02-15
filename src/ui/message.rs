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
    SleepTimerCustomMinutesChanged(String),
    SleepTimerCustomSubmit,

    // Shortcuts
    CreateBookmark,

    // Keyboard navigation
    SeekForward(i64),  // seconds to seek forward
    SeekBackward(i64), // seconds to seek backward
    NextFile,
    PreviousFile,
    CloseModal, // Escape to close any open modal

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

    // File list
    FileSelected(String),

    // Directory management
    DirectoryAdd,
    DirectoryAdded(String),
    DirectoryAddCancelled,
    DirectoryRemove(String),
    DirectoryRescan(String),

    // Settings
    OpenSettings,
    CloseSettings,

    // Error handling
    DismissError,

    // Player events
    PlayerTimeUpdated(f64),
    PlayerTick,

    // Background tasks
    ScanComplete(String, Vec<crate::tasks::DiscoveredAudiobook>),
    ScanError(String),
    CoverThumbnailGenerated(i64, Option<PathBuf>),

    // Initial load
    InitialLoadComplete,

    // Window events
    WindowMoved(i32, i32),
    WindowResized(u32, u32),

    // No-op
    None,
}
