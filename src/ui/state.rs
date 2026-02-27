//! Application UI state and related types.
//!
//! [`State`] is the single source of truth for all UI state.  Every render
//! cycle receives a `&State` and every user interaction produces a [`Message`]
//! that is handled in [`crate::ui::update`] to produce the next `State`.

use crate::models::{Audiobook, AudiobookFile, Bookmark, Directory, SleepTimer};
use crate::ui::styles::animations::modal::ModalState;
use crate::ui::styles::effects::u64_to_f32;
use crate::ui::tooltip::TooltipState;
use crate::ui::Message;
use iced::Point;
use std::collections::HashMap;
use std::path::PathBuf;

/// Notification message for user feedback
///
/// Provides non-intrusive toast notifications for user actions.
/// In Nodoka, notifications auto-dismiss
/// after 3 seconds for success/info, and allow manual dismiss for errors.
///
/// # Enhanced Features (v0.3.0)
///
/// - **Semantic variants**: Success, Info, Warning, Error with distinct icons
/// - **Optional actions**: Add action buttons for contextual operations (Undo, Retry)
/// - **Title + message**: Support for two-line notifications with hierarchy
/// - **Auto-dismiss countdown**: Visual progress bar for timed notifications
/// - **Queue management**: Max 3 concurrent notifications with FIFO eviction
///
/// # Examples
///
/// ```ignore
/// // Basic success notification
/// state.notification_queue.push(Notification::success(
///     "Bookmark saved successfully".to_string()
/// ));
///
/// // Error notification with action
/// state.notification_queue.push(Notification::error_with_action(
///     "Failed to save settings".to_string(),
///     "Retry".to_string(),
///     Message::OpenSettings,
/// ));
///
/// // Info notification with title and message
/// state.notification_queue.push(Notification::info_with_title(
///     "Scan Complete".to_string(),
///     "Found 5 new audiobooks in /path/to/directory".to_string(),
/// ));
/// ```
#[derive(Debug, Clone)]
pub struct Notification {
    /// Unique identifier for this notification
    pub id: u64,
    /// Type of notification affecting styling and icon
    pub kind: NotificationType,
    /// Title text (optional, for two-line notifications)
    pub title: Option<String>,
    /// Message text to display
    pub message: String,
    /// Optional action button label
    pub action_label: Option<String>,
    /// Optional action message for button press handling
    pub action_message: Option<Message>,
    /// Duration in milliseconds before auto-dismiss
    pub duration_ms: u64,
    /// Creation timestamp for expiry calculation
    pub created_at: u64,
}

/// Maximum number of notifications shown concurrently
pub const MAX_NOTIFICATIONS: usize = 3;

/// Global notification ID counter
///
/// Uses atomic operations to ensure thread-safe ID generation.
static NOTIFICATION_ID_COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

impl Notification {
    /// Generates next unique notification ID
    fn next_id() -> u64 {
        NOTIFICATION_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    }

    /// Creates a success notification (auto-dismisses after 3s)
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let notification = Notification::success("File saved successfully".to_string());
    /// ```
    #[must_use]
    pub fn success(message: String) -> Self {
        Self {
            id: Self::next_id(),
            kind: NotificationType::Success,
            title: None,
            message,
            action_label: None,
            action_message: None,
            duration_ms: 3000,
            created_at: current_time_ms(),
        }
    }

    /// Creates an info notification (auto-dismisses after 3s)
    #[must_use]
    pub fn info(message: String) -> Self {
        Self {
            id: Self::next_id(),
            kind: NotificationType::Info,
            title: None,
            message,
            action_label: None,
            action_message: None,
            duration_ms: 3000,
            created_at: current_time_ms(),
        }
    }

    /// Creates a warning notification (longer duration: 5s)
    #[must_use]
    pub fn warning(message: String) -> Self {
        Self {
            id: Self::next_id(),
            kind: NotificationType::Warning,
            title: None,
            message,
            action_label: None,
            action_message: None,
            duration_ms: 5000,
            created_at: current_time_ms(),
        }
    }

    /// Creates an error notification (requires manual dismiss)
    #[must_use]
    pub fn error(message: String) -> Self {
        Self {
            id: Self::next_id(),
            kind: NotificationType::Error,
            title: None,
            message,
            action_label: None,
            action_message: None,
            duration_ms: u64::MAX, // No auto-dismiss for errors
            created_at: current_time_ms(),
        }
    }

    /// Creates a notification with title and message (two-line)
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let notification = Notification::info_with_title(
    ///     "Scan Complete".to_string(),
    ///     "Found 5 new audiobooks".to_string(),
    /// );
    /// ```
    #[must_use]
    pub fn info_with_title(title: String, message: String) -> Self {
        Self {
            id: Self::next_id(),
            kind: NotificationType::Info,
            title: Some(title),
            message,
            action_label: None,
            action_message: None,
            duration_ms: 3000,
            created_at: current_time_ms(),
        }
    }

    /// Creates a warning notification with title and message
    #[must_use]
    pub fn warning_with_title(title: String, message: String) -> Self {
        Self {
            id: Self::next_id(),
            kind: NotificationType::Warning,
            title: Some(title),
            message,
            action_label: None,
            action_message: None,
            duration_ms: 5000,
            created_at: current_time_ms(),
        }
    }

    /// Creates an error notification with title and message
    #[must_use]
    pub fn error_with_title(title: String, message: String) -> Self {
        Self {
            id: Self::next_id(),
            kind: NotificationType::Error,
            title: Some(title),
            message,
            action_label: None,
            action_message: None,
            duration_ms: u64::MAX,
            created_at: current_time_ms(),
        }
    }

    /// Creates a notification with an action button
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let notification = Notification::error_with_action(
    ///     "Failed to save settings".to_string(),
    ///     "Retry".to_string(),
    ///     Message::OpenSettings,
    /// );
    /// ```
    #[must_use]
    pub fn error_with_action(
        message: String,
        action_label: String,
        action_message: Message,
    ) -> Self {
        Self {
            id: Self::next_id(),
            kind: NotificationType::Error,
            title: None,
            message,
            action_label: Some(action_label),
            action_message: Some(action_message),
            duration_ms: u64::MAX,
            created_at: current_time_ms(),
        }
    }

    /// Creates a warning notification with an action button
    #[must_use]
    pub fn warning_with_action(
        message: String,
        action_label: String,
        action_message: Message,
    ) -> Self {
        Self {
            id: Self::next_id(),
            kind: NotificationType::Warning,
            title: None,
            message,
            action_label: Some(action_label),
            action_message: Some(action_message),
            duration_ms: 5000,
            created_at: current_time_ms(),
        }
    }

    /// Checks if notification should be expired
    #[must_use]
    pub fn is_expired(&self) -> bool {
        let now = current_time_ms();
        now.saturating_sub(self.created_at) >= self.duration_ms
    }

    /// Returns progress of auto-dismiss countdown (0.0 to 1.0)
    ///
    /// Returns 1.0 for notifications that don't auto-dismiss.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let progress = notification.dismiss_progress();
    /// // Use progress to render countdown progress bar
    /// ```
    #[must_use]
    pub fn dismiss_progress(&self) -> f32 {
        if self.duration_ms == u64::MAX {
            return 1.0; // No countdown for manual dismiss
        }
        let now = current_time_ms();
        let elapsed = now.saturating_sub(self.created_at);
        if self.duration_ms == 0 {
            return 1.0;
        }
        let elapsed_f = u64_to_f32(elapsed);
        let duration_f = u64_to_f32(self.duration_ms);
        (elapsed_f / duration_f).min(1.0)
    }

    /// Returns progress portions for auto-dismiss countdown bar
    ///
    /// Returns (100, 0) for notifications that don't auto-dismiss.
    #[must_use]
    pub fn dismiss_progress_portions(&self) -> (u16, u16) {
        if self.duration_ms == u64::MAX || self.duration_ms == 0 {
            return (100, 0);
        }
        let now = current_time_ms();
        let elapsed = now.saturating_sub(self.created_at);
        let progress = elapsed.saturating_mul(100).saturating_div(self.duration_ms);
        let progress = progress.min(100);
        let remaining = 100_u64.saturating_sub(progress);
        let progress_u16 = u16::try_from(progress).unwrap_or(100);
        let remaining_u16 = u16::try_from(remaining).unwrap_or(0);
        (progress_u16, remaining_u16)
    }

    /// Returns semantic icon for notification type
    ///
    /// Uses ASCII symbols to avoid terminal/font rendering issues.
    /// - Success: OK
    /// - Info: i
    /// - Warning: !
    /// - Error: X
    #[must_use]
    pub const fn icon(&self) -> &'static str {
        match self.kind {
            NotificationType::Success => "OK",
            NotificationType::Info => "i",
            NotificationType::Warning => "!",
            NotificationType::Error => "X",
        }
    }
}

/// Type of notification affecting styling and behavior
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NotificationType {
    /// Success notifications (green, auto-dismiss 3s)
    Success,
    /// Informational notifications (blue, auto-dismiss 3s)
    Info,
    /// Warning notifications (yellow, auto-dismiss 5s)
    Warning,
    /// Error notifications (red, manual dismiss only)
    Error,
}

/// Transition state for component animations
///
/// Provides smooth interpolation between values for animated transitions.
/// Use ease-in-out for most transitions (200-300ms).
#[derive(Debug, Clone)]
pub struct TransitionState {
    /// Starting value
    pub from_value: f32,
    /// Target value
    pub to_value: f32,
    /// Progress from 0.0 to 1.0
    pub progress: f32,
    /// Duration in milliseconds
    pub duration_ms: u64,
}

impl TransitionState {
    /// Creates a new transition from one value to another
    #[must_use]
    pub const fn new(from_value: f32, to_value: f32, duration_ms: u64) -> Self {
        Self {
            from_value,
            to_value,
            progress: 0.0,
            duration_ms,
        }
    }

    /// Updates transition progress
    ///
    /// Returns `true` when transition is complete.
    pub fn update(&mut self, delta_ms: u64) -> bool {
        if self.duration_ms == 0 {
            self.progress = 1.0;
            return true;
        }
        let progress_delta = u64_to_f32(delta_ms) / u64_to_f32(self.duration_ms);
        self.progress = (self.progress + progress_delta).min(1.0);
        self.progress >= 1.0
    }

    /// Gets current interpolated value using ease-in-out easing
    #[must_use]
    pub fn current_value(&self) -> f32 {
        use crate::ui::styles::animations::easing;
        let t = easing::ease_in_out(self.progress);
        self.from_value.mul_add(1.0 - t, self.to_value * t)
    }

    /// Checks if transition is complete
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.progress >= 1.0
    }
}

/// Gets current time in milliseconds since UNIX epoch
#[must_use]
pub(crate) fn current_time_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| u64::try_from(d.as_millis()).unwrap_or(u64::MAX))
}

/// Focus tracking for keyboard navigation accessibility
///
/// Tracks which UI element currently has keyboard focus to provide
/// visual indicators for keyboard navigation (WCAG 2.1 AA 2.4.7).
///
/// Note: iced 0.14 does not expose a stable, cross-widget focus API,
/// so we track focus in application state as a workaround.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FocusedElement {
    /// No element is focused.
    #[default]
    None,
    /// The play/pause transport button.
    PlayPauseButton,
    /// The stop transport button.
    StopButton,
    /// The volume slider.
    VolumeSlider,
    /// The playback speed slider.
    SpeedSlider,
    /// The progress/seek slider.
    ProgressSlider,
    /// The audiobook library list.
    AudiobookList,
    /// The audio file list.
    FileList,
    /// The bookmarks list.
    BookmarkList,
    /// The settings open button in the top bar.
    SettingsButton,
    /// The "Add Directory" button in the settings panel.
    SettingsAddDirectoryButton,
    /// The settings close button.
    SettingsCloseButton,
    /// The bookmark editor's cancel button.
    BookmarkEditorCancelButton,
    /// The bookmark editor's save button.
    BookmarkEditorSaveButton,
    /// The keyboard help modal close button.
    KeyboardHelpCloseButton,
}

/// Ripple animation target identifiers for interactive feedback.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RippleTarget {
    /// Ripple on the play/pause button.
    PlayPauseButton,
    /// Ripple on the stop button.
    StopButton,
}

/// Sort order options for audiobook library
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AudiobookSortOrder {
    /// Recently played (last listened timestamp)
    #[default]
    RecentlyPlayed,
    /// Alphabetical by title
    Title,
    /// By progress percentage
    Progress,
    /// By date added
    DateAdded,
}

/// Filter status for audiobook library
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AudiobookFilterStatus {
    /// Show all audiobooks
    #[default]
    All,
    /// Show only in-progress audiobooks
    InProgress,
    /// Show only completed audiobooks
    Complete,
    /// Show only not-started audiobooks
    NotStarted,
}

/// Application theme mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ThemeMode {
    /// Light theme with rose/white palette
    #[default]
    Light,
    /// Dark theme with dark backgrounds
    Dark,
}

/// Whether focus mode is enabled.
///
/// Clippy discourages having many `bool` flags in a single struct. Representing
/// focus mode as an enum keeps the intent clear while avoiding excessive `bool`
/// fields on [`State`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FocusMode {
    /// Focus mode is disabled.
    #[default]
    Disabled,
    /// Focus mode is enabled.
    Enabled,
}

impl FocusMode {
    #[must_use]
    pub const fn is_enabled(self) -> bool {
        matches!(self, Self::Enabled)
    }

    pub const fn toggle(&mut self) {
        *self = match *self {
            Self::Disabled => Self::Enabled,
            Self::Enabled => Self::Disabled,
        };
    }
}

impl From<bool> for FocusMode {
    fn from(value: bool) -> Self {
        if value {
            Self::Enabled
        } else {
            Self::Disabled
        }
    }
}

impl From<FocusMode> for bool {
    fn from(value: FocusMode) -> Self {
        value.is_enabled()
    }
}

/// Whether the volume slider in player controls is expanded.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum VolumeExpansion {
    /// Only the volume button/percentage is visible.
    #[default]
    Collapsed,
    /// The slider is visible.
    Expanded,
}

impl VolumeExpansion {
    #[must_use]
    pub const fn is_expanded(self) -> bool {
        matches!(self, Self::Expanded)
    }

    pub const fn toggle(&mut self) {
        *self = match *self {
            Self::Collapsed => Self::Expanded,
            Self::Expanded => Self::Collapsed,
        };
    }
}

impl From<bool> for VolumeExpansion {
    fn from(value: bool) -> Self {
        if value {
            Self::Expanded
        } else {
            Self::Collapsed
        }
    }
}

impl From<VolumeExpansion> for bool {
    fn from(value: VolumeExpansion) -> Self {
        value.is_expanded()
    }
}

/// Active modal overlay state
///
/// Tracks which modal (if any) is currently displayed. Only one modal
/// can be active at a time to maintain UI clarity and focus management.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ActiveModal {
    /// No modal is open.
    #[default]
    None,
    /// The settings modal is open.
    Settings,
    /// The bookmark editor modal is open.
    BookmarkEditor,
    /// The keyboard shortcuts help modal is open.
    KeyboardHelp,
}

/// In-progress bookmark creation or edit form.
///
/// This struct is populated when the user opens the bookmark editor modal and
/// is cleared when the modal is dismissed (either saved or cancelled).
#[derive(Debug, Clone)]
pub struct BookmarkEditor {
    /// Database ID of an existing bookmark being edited, or `None` when creating a new one.
    pub id: Option<i64>,
    /// ID of the audiobook this bookmark belongs to.
    pub audiobook_id: i64,
    /// Absolute path of the audio file that was playing when the bookmark was created.
    pub file_path: String,
    /// Playback position in milliseconds at which the bookmark was created.
    pub position_ms: i64,
    /// User-supplied short label for the bookmark (displayed in the bookmark list).
    pub label: String,
    /// Optional longer note attached to the bookmark.
    pub note: String,
}

/// Whether the player is currently playing or paused.
///
/// This status drives the play/pause button icon and controls whether
/// the progress slider is updated on each UI tick.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PlaybackStatus {
    /// Audio is actively playing.
    Playing,
    /// Audio is paused or stopped.
    #[default]
    Paused,
}

/// Whether the initial audiobook list has finished loading from the database.
///
/// The UI starts in [`LoadState::Loading`] so that a spinner or placeholder
/// can be shown while the async database query is in flight.  Once the query
/// resolves the state transitions to [`LoadState::Ready`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LoadState {
    /// The audiobook list is being fetched; the UI shows a loading indicator.
    #[default]
    Loading,
    /// The audiobook list has been fetched and is ready for display.
    Ready,
}

/// Whether the application is currently scanning a directory for new audiobooks.
///
/// The `Scanning` variant carries the path that is being scanned so the UI can
/// display it in a progress message.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ScanState {
    /// No scan is in progress.
    #[default]
    Idle,
    /// A directory scan is running.
    Scanning {
        /// The directory path being scanned, or `None` when the path is not known.
        directory: Option<String>,
    },
}

/// Single source of truth for all mutable UI state.
///
/// Every view function in the `ui` module receives a shared reference to this
/// struct and produces an `Element` that may emit [`crate::ui::Message`] values.
/// The application's `update` handler applies those messages to produce the next
/// `State`.
///
/// The initial state has `load_state == LoadState::Loading` because the
/// audiobook list is fetched asynchronously at startup; all view functions must
/// handle this case gracefully.
#[derive(Debug, Clone)]
pub struct State {
    /// Full list of audiobooks loaded from the database.
    pub audiobooks: Vec<Audiobook>,
    /// ID of the currently selected audiobook, if any.
    pub selected_audiobook: Option<i64>,
    /// Audio files belonging to the selected audiobook.
    pub current_files: Vec<AudiobookFile>,
    /// File path of the currently selected audio file.
    pub selected_file: Option<String>,
    /// Watched directories configured by the user.
    pub directories: Vec<Directory>,

    /// Cached cover thumbnail paths keyed by audiobook ID.
    pub cover_thumbnails: HashMap<i64, PathBuf>,

    /// Search query for filtering audiobook list
    pub search_query: String,
    /// Current sort order for audiobooks
    pub sort_order: AudiobookSortOrder,
    /// Filter applied to audiobook list (completion status)
    pub filter_status: AudiobookFilterStatus,
    /// Panel ratio for library (left) vs detail (right) panels (0.0 to 1.0)
    pub panel_ratio: f32,
    /// Whether the panel divider is currently being dragged.
    pub panel_divider_dragging: bool,
    /// Last known cursor X position while dragging the panel divider.
    pub panel_divider_last_cursor_x: Option<f32>,
    /// Focus mode preference (hides library panel during playback)
    pub focus_mode: FocusMode,
    /// Current theme (light or dark mode)
    pub theme: ThemeMode,
    /// Window width in pixels for responsive layout
    pub window_width: u32,
    /// Window height in pixels for responsive layout
    pub window_height: u32,

    /// Bookmarks for the selected audiobook.
    pub bookmarks: Vec<Bookmark>,
    /// Bookmark editor form state when the editor modal is open.
    pub bookmark_editor: Option<BookmarkEditor>,

    /// Whether audio is currently playing or paused.
    pub playback: PlaybackStatus,
    /// Current playback position in milliseconds (interpolated between VLC ticks).
    pub current_time: f64,
    /// Total duration of the current file in milliseconds.
    pub total_duration: f64,
    /// Current volume level in the range [0, 200] (100 = unity gain).
    pub volume: i32,
    /// Whether the volume slider is expanded in the player controls.
    pub volume_expanded: VolumeExpansion,
    /// Current playback speed (1.0 = normal speed).
    pub speed: f32,

    /// Active sleep timer, if set by the user.
    pub sleep_timer: Option<SleepTimer>,
    /// Volume captured before a sleep-timer fade began, used to restore after cancel.
    pub sleep_timer_base_volume: Option<i32>,
    /// Raw text from the custom-duration input field.
    pub sleep_timer_custom_minutes: String,
    /// Validation error for the custom-duration input, if any.
    pub sleep_timer_custom_error: Option<String>,

    /// Which modal overlay is currently visible (at most one at a time).
    pub active_modal: ActiveModal,
    /// Whether the initial audiobook list has been loaded.
    pub load_state: LoadState,

    /// Directory scanning state; updated while a scan task is running.
    pub scan_state: ScanState,

    /// Keyboard focus tracking for accessibility (WCAG 2.1 AA 2.4.7).
    pub focused_element: FocusedElement,

    /// Set to `true` while an async operation is in flight to prevent duplicate
    /// submissions from rapid user input.
    pub operation_in_progress: bool,

    /// Mirrors the user's system-level reduced-motion preference (WCAG 2.1 2.3.3).
    pub prefers_reduced_motion: bool,

    /// Queue of pending toast notifications (capped at [`MAX_NOTIFICATIONS`]).
    pub notification_queue: Vec<Notification>,

    /// Per-element pulse animation state, keyed by a stable string identifier.
    pub pulse_animations: HashMap<String, crate::ui::styles::animations::PulseAnimation>,
    /// Per-element value-transition state, keyed by a stable string identifier.
    pub transition_states: HashMap<String, TransitionState>,
    /// Ripple click-feedback animations keyed by the interactive target.
    pub ripple_animations: HashMap<RippleTarget, crate::ui::styles::interactive::RippleAnimation>,
    /// Screen-space origin of each ripple, used to position the expanding ring.
    pub ripple_origins: HashMap<RippleTarget, Point>,

    /// Lifecycle state of the currently open (or closing) modal overlay.
    pub modal_state: ModalState,

    /// Milliseconds since the UNIX epoch, updated on every [`crate::ui::Message::UiTick`].
    pub ui_time_ms: u64,
    /// Timestamp when the audiobook list entry animation started, if active.
    pub audiobook_list_animation_start_ms: Option<u64>,
    /// Timestamp when the file list entry animation started, if active.
    pub file_list_animation_start_ms: Option<u64>,
    /// Timestamp when the bookmark list entry animation started, if active.
    pub bookmark_list_animation_start_ms: Option<u64>,

    /// Hover and visibility tracking for contextual tooltips.
    pub tooltip_state: TooltipState,
}

impl Default for State {
    fn default() -> Self {
        Self {
            audiobooks: Vec::new(),
            selected_audiobook: None,
            current_files: Vec::new(),
            selected_file: None,
            directories: Vec::new(),
            cover_thumbnails: HashMap::new(),
            search_query: String::new(),
            sort_order: AudiobookSortOrder::default(),
            filter_status: AudiobookFilterStatus::default(),
            panel_ratio: 0.4, // 40% left panel, 60% right panel (2:3 ratio)
            panel_divider_dragging: false,
            panel_divider_last_cursor_x: None,
            focus_mode: FocusMode::Disabled,
            theme: ThemeMode::default(),
            window_width: 1280, // Default window width
            window_height: 800, // Default window height
            bookmarks: Vec::new(),
            bookmark_editor: None,
            playback: PlaybackStatus::Paused,
            current_time: 0.0,
            total_duration: 0.0,
            volume: 100,
            volume_expanded: VolumeExpansion::Collapsed,
            speed: 1.0,
            sleep_timer: None,
            sleep_timer_base_volume: None,
            sleep_timer_custom_minutes: String::new(),
            sleep_timer_custom_error: None,
            active_modal: ActiveModal::None,
            load_state: LoadState::Loading,
            scan_state: ScanState::Idle,
            focused_element: FocusedElement::None,
            operation_in_progress: false,
            prefers_reduced_motion: false,
            notification_queue: Vec::new(),
            pulse_animations: HashMap::new(),
            transition_states: HashMap::new(),
            ripple_animations: HashMap::new(),
            ripple_origins: HashMap::new(),
            modal_state: ModalState::Closed,
            ui_time_ms: 0,
            audiobook_list_animation_start_ms: None,
            file_list_animation_start_ms: None,
            bookmark_list_animation_start_ms: None,
            tooltip_state: TooltipState::new(),
        }
    }
}

#[cfg(test)]
mod tests;
