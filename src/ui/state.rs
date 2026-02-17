use crate::models::{Audiobook, AudiobookFile, Bookmark, Directory, SleepTimer};
use crate::ui::Message;
use crate::ui::styles::animations::modal::ModalState;
use crate::ui::tooltip::TooltipState;
use iced::Point;
use std::collections::HashMap;
use std::path::PathBuf;

/// Notification message for user feedback
///
/// Provides non-intrusive toast notifications for user actions.
/// Per ui-ux-pro-max guidelines, notifications should auto-dismiss
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
    pub fn error_with_action(message: String, action_label: String, action_message: Message) -> Self {
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
    /// Uses Unicode symbols suitable for iced framework:
    /// - Success: ✓ (U+2713)
    /// - Info: ℹ (U+2139)
    /// - Warning: ⚠ (U+26A0)
    /// - Error: ✕ (U+2715)
    #[must_use]
    pub const fn icon(&self) -> &'static str {
        match self.kind {
            NotificationType::Success => "✓",
            NotificationType::Info => "ℹ",
            NotificationType::Warning => "⚠",
            NotificationType::Error => "✕",
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
/// Per ui-ux-pro-max: use ease-in-out for most transitions (200-300ms).
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

fn u64_to_f32(value: u64) -> f32 {
    let clamped = u32::try_from(value).unwrap_or(u32::MAX);
    clamped as f32
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
    #[default]
    None,
    PlayPauseButton,
    StopButton,
    VolumeSlider,
    SpeedSlider,
    ProgressSlider,
    AudiobookList,
    FileList,
    BookmarkList,
    SettingsButton,
    SettingsAddDirectoryButton,
    SettingsCloseButton,
    BookmarkEditorCancelButton,
    BookmarkEditorSaveButton,
    KeyboardHelpCloseButton,
}

/// Ripple animation target identifiers for interactive feedback.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RippleTarget {
    PlayPauseButton,
    StopButton,
}

/// Active modal overlay state
///
/// Tracks which modal (if any) is currently displayed. Only one modal
/// can be active at a time to maintain UI clarity and focus management.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ActiveModal {
    #[default]
    None,
    Settings,
    BookmarkEditor,
    KeyboardHelp,
}

#[derive(Debug, Clone)]
pub struct BookmarkEditor {
    pub id: Option<i64>,
    pub audiobook_id: i64,
    pub file_path: String,
    pub position_ms: i64,
    pub label: String,
    pub note: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PlaybackStatus {
    Playing,
    #[default]
    Paused,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LoadState {
    #[default]
    Loading,
    Ready,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ScanState {
    #[default]
    Idle,
    Scanning {
        directory: Option<String>,
    },
}

#[derive(Debug, Clone)]
pub struct State {
    pub audiobooks: Vec<Audiobook>,
    pub selected_audiobook: Option<i64>,
    pub current_files: Vec<AudiobookFile>,
    pub selected_file: Option<String>,
    pub directories: Vec<Directory>,

    pub cover_thumbnails: HashMap<i64, PathBuf>,

    pub bookmarks: Vec<Bookmark>,
    pub bookmark_editor: Option<BookmarkEditor>,

    pub playback: PlaybackStatus,
    pub current_time: f64,
    pub total_duration: f64,
    pub volume: i32,
    pub speed: f32,

    pub sleep_timer: Option<SleepTimer>,
    pub sleep_timer_base_volume: Option<i32>,
    pub sleep_timer_custom_minutes: String,
    pub sleep_timer_custom_error: Option<String>,

    pub active_modal: ActiveModal,
    pub load_state: LoadState,

    // Directory scanning state
    pub scan_state: ScanState,

    // Focus tracking for keyboard navigation accessibility
    pub focused_element: FocusedElement,

    // Operation in progress flag to prevent duplicate operations from rapid input
    pub operation_in_progress: bool,

    // Reduced motion preference for accessibility (WCAG 2.1 2.3.3)
    pub prefers_reduced_motion: bool,

    // Notification system for user feedback (max 3 concurrent)
    pub notification_queue: Vec<Notification>,

    // Animation tracking for smooth transitions
    pub pulse_animations: HashMap<String, crate::ui::styles::animations::PulseAnimation>,
    pub transition_states: HashMap<String, TransitionState>,
    pub ripple_animations: HashMap<RippleTarget, crate::ui::styles::interactive::RippleAnimation>,
    pub ripple_origins: HashMap<RippleTarget, Point>,

    // Modal animation state and close coordination
    pub modal_state: ModalState,
    pub modal_pending_close: bool,

    // UI animation timing (milliseconds since epoch, updated via UiTick)
    pub ui_time_ms: u64,
    pub audiobook_list_animation_start_ms: Option<u64>,
    pub file_list_animation_start_ms: Option<u64>,
    pub bookmark_list_animation_start_ms: Option<u64>,

    // Tooltip tracking for discoverability
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
            bookmarks: Vec::new(),
            bookmark_editor: None,
            playback: PlaybackStatus::Paused,
            current_time: 0.0,
            total_duration: 0.0,
            volume: 100,
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
            modal_pending_close: false,
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
