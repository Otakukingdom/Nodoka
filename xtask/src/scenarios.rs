//! UI scenario definitions for screenshot capture.
//!
//! Each scenario represents a distinct UI state that should be captured
//! for visual auditing and regression testing.

use chrono::Utc;
use nodoka::db::Database;
use nodoka::models::{Audiobook, AudiobookFile, Bookmark, Directory};
use nodoka::ui::State;

mod apply;

/// All screenshot scenarios to capture.
#[derive(Debug, Clone, Copy)]
pub enum Scenario {
    /// Empty library (no audiobooks).
    EmptyLibrary,
    /// Library with multiple audiobooks.
    LibraryWithAudiobooks,
    /// Audiobook selected with file list visible.
    AudiobookSelected,
    /// Playing state with progress.
    PlayingState,
    /// Paused state.
    PausedState,
    /// Focus mode enabled (library hidden).
    FocusModeEnabled,
    /// Settings modal open.
    SettingsModalOpen,
    /// Bookmark editor open.
    BookmarkEditorOpen,
    /// Keyboard help overlay visible.
    KeyboardHelpOverlay,
    /// Notification visible.
    NotificationVisible,
    /// Volume slider expanded.
    VolumeExpanded,
    /// Search active with query.
    SearchActive,
    /// Hover states on interactive elements.
    HoverStates,
    /// Error state with missing files.
    ErrorState,
    /// Multiple bookmarks for comprehensive list view.
    MultipleBookmarks,
    /// Long title handling for text overflow.
    LongTitle,
    /// Loading state with skeleton UI.
    LoadingState,
    /// Scrollbar visible with many files.
    ScrollbarVisible,
    /// Many audiobooks for scrolling demonstration.
    ManyItems,
    /// Bookmark selected with highlight styling.
    BookmarkSelected,
    /// File selected with active styling.
    FileSelected,
    /// Playback speed changed to non-1.0 value.
    SpeedChanged,
    /// Volume muted at 0%.
    VolumeMuted,
    /// Sleep timer active with countdown.
    SleepTimerActive,
}

impl Scenario {
    /// Returns all scenarios in capture order.
    #[must_use]
    pub const fn all() -> &'static [Self] {
        &[
            Self::EmptyLibrary,
            Self::LibraryWithAudiobooks,
            Self::AudiobookSelected,
            Self::PlayingState,
            Self::PausedState,
            Self::FocusModeEnabled,
            Self::SettingsModalOpen,
            Self::BookmarkEditorOpen,
            Self::KeyboardHelpOverlay,
            Self::NotificationVisible,
            Self::VolumeExpanded,
            Self::SearchActive,
            Self::HoverStates,
            Self::ErrorState,
            Self::MultipleBookmarks,
            Self::LongTitle,
            Self::LoadingState,
            Self::ScrollbarVisible,
            Self::ManyItems,
            Self::BookmarkSelected,
            Self::FileSelected,
            Self::SpeedChanged,
            Self::VolumeMuted,
            Self::SleepTimerActive,
        ]
    }

    /// Returns the output filename for this scenario.
    #[must_use]
    pub const fn filename(self) -> &'static str {
        match self {
            Self::EmptyLibrary => "01-empty-library.png",
            Self::LibraryWithAudiobooks => "02-library-with-audiobooks.png",
            Self::AudiobookSelected => "03-audiobook-selected.png",
            Self::PlayingState => "04-playing-state.png",
            Self::PausedState => "05-paused-state.png",
            Self::FocusModeEnabled => "06-focus-mode-enabled.png",
            Self::SettingsModalOpen => "07-settings-modal-open.png",
            Self::BookmarkEditorOpen => "08-bookmark-editor-open.png",
            Self::KeyboardHelpOverlay => "09-keyboard-help-overlay.png",
            Self::NotificationVisible => "10-notification-visible.png",
            Self::VolumeExpanded => "11-volume-expanded.png",
            Self::SearchActive => "12-search-active.png",
            Self::HoverStates => "13-hover-states.png",
            Self::ErrorState => "14-error-state.png",
            Self::MultipleBookmarks => "15-multiple-bookmarks.png",
            Self::LongTitle => "16-long-title.png",
            Self::LoadingState => "17-loading-state.png",
            Self::ScrollbarVisible => "18-scrollbar-visible.png",
            Self::ManyItems => "19-many-items.png",
            Self::BookmarkSelected => "20-bookmark-selected.png",
            Self::FileSelected => "21-file-selected.png",
            Self::SpeedChanged => "22-speed-changed.png",
            Self::VolumeMuted => "23-volume-muted.png",
            Self::SleepTimerActive => "24-sleep-timer-active.png",
        }
    }

    /// Applies this scenario to a State struct, setting up the appropriate UI state.
    pub fn apply_to_state(self, state: &mut State, db: &Database) {
        apply::apply_to_state(self, state, db);
    }
}

/// Populates state with fixture audiobooks and selects the first one.
///
/// Returns the selected audiobook's database ID, or `None` if no fixture
/// audiobooks could be created.
fn setup_with_selected_audiobook(state: &mut State, db: &Database) -> Option<i64> {
    state.audiobooks = create_fixture_audiobooks(db);
    let id = state.audiobooks.first().and_then(|ab| ab.id)?;
    state.selected_audiobook = Some(id);
    state.current_files = create_fixture_files(id);
    Some(id)
}

/// Creates fixture audiobooks for testing.
fn create_fixture_audiobooks(db: &Database) -> Vec<Audiobook> {
    let audiobooks = vec![
        Audiobook {
            id: Some(1),
            directory: "/audiobooks".to_string(),
            name: "Foundation".to_string(),
            author: Some("Isaac Asimov".to_string()),
            full_path: "/audiobooks/Foundation".to_string(),
            completeness: 35,
            default_order: 0,
            selected_file: Some("/audiobooks/Foundation/chapter1.mp3".to_string()),
            total_duration_ms: Some(18_000_000),
            remaining_duration_ms: Some(11_700_000),
            created_at: Utc::now(),
            last_listened: Some(Utc::now()),
        },
        Audiobook {
            id: Some(2),
            directory: "/audiobooks".to_string(),
            name: "Dune".to_string(),
            author: Some("Frank Herbert".to_string()),
            full_path: "/audiobooks/Dune".to_string(),
            completeness: 0,
            default_order: 1,
            selected_file: None,
            total_duration_ms: Some(21_600_000),
            remaining_duration_ms: Some(21_600_000),
            created_at: Utc::now(),
            last_listened: None,
        },
        Audiobook {
            id: Some(3),
            directory: "/audiobooks".to_string(),
            name: "The Hobbit".to_string(),
            author: Some("J.R.R. Tolkien".to_string()),
            full_path: "/audiobooks/The Hobbit".to_string(),
            completeness: 100,
            default_order: 2,
            selected_file: None,
            total_duration_ms: Some(10_800_000),
            remaining_duration_ms: Some(0),
            created_at: Utc::now(),
            last_listened: Some(Utc::now()),
        },
    ];

    for audiobook in &audiobooks {
        // Allow duplicate insertions to succeed silently
        let _result = nodoka::db::queries::insert_audiobook(db.connection(), audiobook);
    }

    audiobooks
}

/// Creates fixture audio files for an audiobook.
fn create_fixture_files(audiobook_id: i64) -> Vec<AudiobookFile> {
    vec![
        AudiobookFile {
            audiobook_id,
            name: "Chapter 1 - The Beginning".to_string(),
            full_path: "/audiobooks/Foundation/chapter1.mp3".to_string(),
            length_of_file: Some(300_000),
            seek_position: Some(45_000),
            checksum: Some("abc123".to_string()),
            position: 0,
            completeness: 15,
            file_exists: true,
            created_at: Utc::now(),
        },
        AudiobookFile {
            audiobook_id,
            name: "Chapter 2 - The Journey".to_string(),
            full_path: "/audiobooks/Foundation/chapter2.mp3".to_string(),
            length_of_file: Some(360_000),
            seek_position: Some(0),
            checksum: Some("def456".to_string()),
            position: 1,
            completeness: 0,
            file_exists: true,
            created_at: Utc::now(),
        },
        AudiobookFile {
            audiobook_id,
            name: "Chapter 3 - The Arrival".to_string(),
            full_path: "/audiobooks/Foundation/chapter3.mp3".to_string(),
            length_of_file: Some(420_000),
            seek_position: Some(0),
            checksum: Some("ghi789".to_string()),
            position: 2,
            completeness: 0,
            file_exists: true,
            created_at: Utc::now(),
        },
    ]
}

/// Creates fixture bookmarks for an audiobook.
fn create_fixture_bookmarks(audiobook_id: i64) -> Vec<Bookmark> {
    vec![
        Bookmark {
            id: Some(1),
            audiobook_id,
            file_path: "/audiobooks/Foundation/chapter1.mp3".to_string(),
            position_ms: 60_000,
            label: "Key moment".to_string(),
            note: Some("This is where everything changes".to_string()),
            created_at: Utc::now(),
        },
        Bookmark {
            id: Some(2),
            audiobook_id,
            file_path: "/audiobooks/Foundation/chapter2.mp3".to_string(),
            position_ms: 120_000,
            label: "Important quote".to_string(),
            note: None,
            created_at: Utc::now(),
        },
    ]
}

/// Creates fixture directories.
fn create_fixture_directories() -> Vec<Directory> {
    vec![
        Directory {
            full_path: "/audiobooks".to_string(),
            created_at: Utc::now(),
            last_scanned: Some(Utc::now()),
        },
        Directory {
            full_path: "/more-audiobooks".to_string(),
            created_at: Utc::now(),
            last_scanned: None,
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use nodoka::ui::{LoadState, PlaybackStatus};

    #[test]
    fn test_empty_library_scenario() -> Result<(), Box<dyn std::error::Error>> {
        let db = Database::new_in_memory()?;
        nodoka::db::initialize(db.connection())?;

        let mut state = State::default();
        Scenario::EmptyLibrary.apply_to_state(&mut state, &db);

        assert!(state.audiobooks.is_empty());
        assert_eq!(state.load_state, LoadState::Ready);
        Ok(())
    }

    #[test]
    fn test_library_with_audiobooks_scenario() -> Result<(), Box<dyn std::error::Error>> {
        let db = Database::new_in_memory()?;
        nodoka::db::initialize(db.connection())?;

        let mut state = State::default();
        Scenario::LibraryWithAudiobooks.apply_to_state(&mut state, &db);

        assert!(!state.audiobooks.is_empty());
        assert_eq!(state.audiobooks.len(), 3);
        Ok(())
    }

    #[test]
    fn test_playing_state_scenario() -> Result<(), Box<dyn std::error::Error>> {
        let db = Database::new_in_memory()?;
        nodoka::db::initialize(db.connection())?;

        let mut state = State::default();
        Scenario::PlayingState.apply_to_state(&mut state, &db);

        assert_eq!(state.playback, PlaybackStatus::Playing);
        assert!(state.current_time > 0.0);
        assert!(state.total_duration > 0.0);
        Ok(())
    }

    #[test]
    fn test_all_scenarios_have_unique_filenames() {
        let mut filenames = std::collections::HashSet::new();
        for scenario in Scenario::all() {
            assert!(
                filenames.insert(scenario.filename()),
                "Duplicate filename: {}",
                scenario.filename()
            );
        }
        assert_eq!(filenames.len(), 24); // Updated to 24 scenarios
    }

    #[test]
    fn test_audiobook_selected_scenario() -> Result<(), Box<dyn std::error::Error>> {
        let db = Database::new_in_memory()?;
        nodoka::db::initialize(db.connection())?;

        let mut state = State::default();
        Scenario::AudiobookSelected.apply_to_state(&mut state, &db);

        assert!(!state.audiobooks.is_empty());
        assert!(state.selected_audiobook.is_some());
        assert!(!state.current_files.is_empty());
        Ok(())
    }

    #[test]
    fn test_focus_mode_enabled_scenario() -> Result<(), Box<dyn std::error::Error>> {
        let db = Database::new_in_memory()?;
        nodoka::db::initialize(db.connection())?;

        let mut state = State::default();
        Scenario::FocusModeEnabled.apply_to_state(&mut state, &db);

        assert!(state.focus_mode.is_enabled());
        assert_eq!(state.playback, PlaybackStatus::Playing);
        Ok(())
    }
}
