//! Application message type used by the iced update loop.

use std::path::PathBuf;

/// All events that can change application state.
///
/// Each variant is produced by a UI widget interaction, a background task,
/// or a player event and is dispatched to [`crate::ui::update::update`].
#[derive(Debug, Clone)]
pub enum Message {
    // Player controls
    /// Toggle play/pause for the current media.
    PlayPause,
    /// Stop playback and reset position.
    Stop,
    /// Seek to a specific position in milliseconds.
    SeekTo(f64),
    /// Change the playback volume to the given level (0–200).
    VolumeChanged(i32),
    /// Change the playback speed to the given multiplier.
    SpeedChanged(f32),

    // Sleep timer
    /// Set a duration-based sleep timer (seconds until pause).
    SleepTimerSetDurationSeconds(i64),
    /// Set the sleep timer to pause at the end of the current file.
    SleepTimerSetEndOfChapter,
    /// Extend the active sleep timer by the given number of seconds.
    SleepTimerExtendSeconds(i64),
    /// Cancel the active sleep timer.
    SleepTimerCancel,
    /// Update the custom-minutes text field in the sleep timer UI.
    SleepTimerCustomMinutesChanged(String),
    /// Submit the custom sleep-timer duration entered by the user.
    SleepTimerCustomSubmit,

    // Shortcuts
    /// Create a bookmark at the current playback position.
    CreateBookmark,

    // Keyboard navigation
    /// Seek forward by the given number of seconds.
    SeekForward(i64),
    /// Seek backward by the given number of seconds.
    SeekBackward(i64),
    /// Jump to the next audio file in the audiobook.
    NextFile,
    /// Jump to the previous audio file in the audiobook.
    PreviousFile,
    /// Close whichever modal is currently open (Escape key).
    CloseModal,
    /// Move keyboard focus to the next interactive element.
    FocusNext,
    /// Move keyboard focus to the previous interactive element.
    FocusPrevious,

    // Bookmarks UI
    /// Open the bookmark editor for the bookmark with the given ID.
    BookmarkEdit(i64),
    /// Delete the bookmark with the given ID.
    BookmarkDelete(i64),
    /// Jump playback to the position stored in the given bookmark ID.
    BookmarkJump(i64),
    /// Update the label field in the open bookmark editor.
    BookmarkEditorLabelChanged(String),
    /// Update the note field in the open bookmark editor.
    BookmarkEditorNoteChanged(String),
    /// Save the changes in the open bookmark editor.
    BookmarkEditorSave,
    /// Discard changes and close the bookmark editor.
    BookmarkEditorCancel,

    // Audiobook list
    /// Select the audiobook with the given database ID.
    AudiobookSelected(i64),

    // File list
    /// Select the audio file at the given path for playback.
    FileSelected(String),

    // Directory management
    /// Open the OS directory picker to add a new library directory.
    DirectoryAdd,
    /// Confirm that a directory was successfully added at the given path.
    DirectoryAdded(String),
    /// The user cancelled the directory picker without selecting a directory.
    DirectoryAddCancelled,
    /// Remove the library directory at the given path.
    DirectoryRemove(String),
    /// Re-scan the library directory at the given path.
    DirectoryRescan(String),

    // Settings
    /// Open the settings modal.
    OpenSettings,
    /// Close the settings modal.
    CloseSettings,

    // Help overlay
    /// Show the keyboard shortcuts help overlay.
    ShowKeyboardHelp,
    /// Hide the keyboard shortcuts help overlay.
    HideKeyboardHelp,

    // Error handling
    /// Dismiss the notification with the given ID.
    DismissNotification(u64),

    // Tooltip hover tracking
    /// Start showing the tooltip for the given target element.
    TooltipHoverStart(crate::ui::tooltip::TooltipTarget),
    /// Stop showing the active tooltip.
    TooltipHoverEnd,
    /// Update the pointer position for a ripple animation on the given target.
    RipplePointerMoved(crate::ui::state::RippleTarget, iced::Point),

    // Player events
    /// The player reported a new playback position (in milliseconds).
    PlayerTimeUpdated(f64),
    /// Periodic tick from the player subscription loop.
    PlayerTick,
    /// Periodic UI-driven animation tick.
    UiTick,

    // Background tasks
    /// A directory scan completed; carries the scanned path and discovered audiobooks.
    ScanComplete(String, Vec<crate::tasks::DiscoveredAudiobook>),
    /// A directory scan failed with the given error message.
    ScanError(String),
    /// A cover thumbnail was generated for the given audiobook ID.
    CoverThumbnailGenerated(i64, Option<PathBuf>),

    // Initial load
    /// The initial database load completed and the library is ready to display.
    InitialLoadComplete,

    // Window events
    /// The application window was moved to the given screen coordinates.
    WindowMoved(i32, i32),
    /// The application window was resized to the given dimensions in pixels.
    WindowResized(u32, u32),

    // No-op
    /// A no-op message used as a placeholder where a message is required but no action is needed.
    None,
}
