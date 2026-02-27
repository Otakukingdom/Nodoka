//! UI scenario definitions for screenshot capture.
//!
//! Each scenario represents a distinct UI state that should be captured
//! for visual auditing and regression testing.

use chrono::Utc;
use nodoka::db::Database;
use nodoka::models::{Audiobook, AudiobookFile, Bookmark, Directory};
use nodoka::ui::{ActiveModal, BookmarkEditor, LoadState, PlaybackStatus, State};

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
        }
    }

    /// Applies this scenario to a State struct, setting up the appropriate UI state.
    pub fn apply_to_state(self, state: &mut State, db: &Database) {
        *state = State::default();
        state.load_state = LoadState::Ready;

        match self {
            Self::EmptyLibrary => {}
            Self::LibraryWithAudiobooks => {
                state.audiobooks = create_fixture_audiobooks(db);
            }
            Self::AudiobookSelected => {
                setup_with_selected_audiobook(state, db);
            }
            Self::PlayingState => {
                if setup_with_selected_audiobook(state, db).is_some() {
                    state.playback = PlaybackStatus::Playing;
                    state.current_time = 45_000.0;
                    state.total_duration = 300_000.0;
                    state.volume = 75;
                    state.speed = 1.0;
                }
            }
            Self::PausedState => {
                if setup_with_selected_audiobook(state, db).is_some() {
                    state.playback = PlaybackStatus::Paused;
                    state.current_time = 120_000.0;
                    state.total_duration = 300_000.0;
                }
            }
            Self::FocusModeEnabled => {
                if setup_with_selected_audiobook(state, db).is_some() {
                    state.playback = PlaybackStatus::Playing;
                    state.focus_mode = true;
                }
            }
            Self::SettingsModalOpen => {
                state.audiobooks = create_fixture_audiobooks(db);
                state.directories = create_fixture_directories();
                state.active_modal = ActiveModal::Settings;
            }
            Self::BookmarkEditorOpen => {
                if let Some(id) = setup_with_selected_audiobook(state, db) {
                    state.bookmarks = create_fixture_bookmarks(id);
                    state.active_modal = ActiveModal::BookmarkEditor;
                    state.bookmark_editor = Some(BookmarkEditor {
                        id: None,
                        audiobook_id: id,
                        file_path: "/audiobooks/Foundation/chapter1.mp3".to_string(),
                        position_ms: 120_000,
                        label: "Important scene".to_string(),
                        note: "The protagonist discovers the truth".to_string(),
                    });
                }
            }
            Self::KeyboardHelpOverlay => {
                state.audiobooks = create_fixture_audiobooks(db);
                state.active_modal = ActiveModal::KeyboardHelp;
            }
            Self::NotificationVisible => {
                state.audiobooks = create_fixture_audiobooks(db);
                state
                    .notification_queue
                    .push(nodoka::ui::Notification::success(
                        "Bookmark saved successfully".to_string(),
                    ));
            }
            Self::VolumeExpanded => {
                if setup_with_selected_audiobook(state, db).is_some() {
                    state.volume_expanded = true;
                    state.volume = 80;
                }
            }
            Self::SearchActive => {
                state.audiobooks = create_fixture_audiobooks(db);
                state.search_query = "foundation".to_string();
            }
            Self::HoverStates => {
                // Simulate hover states with selected audiobook and active playback
                if setup_with_selected_audiobook(state, db).is_some() {
                    state.playback = PlaybackStatus::Playing;
                    state.current_time = 60_000.0;
                    state.total_duration = 300_000.0;
                    // Set focused element to simulate hover state on file list
                    state.focused_element = nodoka::ui::FocusedElement::FileList;
                }
            }
            Self::ErrorState => {
                // Create audiobooks with missing files for error state
                if let Some(id) = setup_with_selected_audiobook(state, db) {
                    // Mark some files as missing
                    state.current_files = vec![
                        AudiobookFile {
                            audiobook_id: id,
                            name: "Chapter 1 - The Beginning".to_string(),
                            full_path: "/audiobooks/Foundation/chapter1.mp3".to_string(),
                            length_of_file: Some(300_000),
                            seek_position: Some(45_000),
                            checksum: Some("abc123".to_string()),
                            position: 0,
                            completeness: 15,
                            file_exists: false, // Missing file
                            created_at: Utc::now(),
                        },
                        AudiobookFile {
                            audiobook_id: id,
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
                    ];
                    // Add error notification
                    state
                        .notification_queue
                        .push(nodoka::ui::Notification::error(
                            "Some files could not be found".to_string(),
                        ));
                }
            }
            Self::MultipleBookmarks => {
                // Create multiple bookmarks to demonstrate list scrolling
                if let Some(id) = setup_with_selected_audiobook(state, db) {
                    state.bookmarks = vec![
                        Bookmark {
                            id: Some(1),
                            audiobook_id: id,
                            file_path: "/audiobooks/Foundation/chapter1.mp3".to_string(),
                            position_ms: 30_000,
                            label: "Opening scene".to_string(),
                            note: Some("Sets the tone for the entire story".to_string()),
                            created_at: Utc::now(),
                        },
                        Bookmark {
                            id: Some(2),
                            audiobook_id: id,
                            file_path: "/audiobooks/Foundation/chapter1.mp3".to_string(),
                            position_ms: 90_000,
                            label: "First plot point".to_string(),
                            note: Some("Where the main conflict is introduced".to_string()),
                            created_at: Utc::now(),
                        },
                        Bookmark {
                            id: Some(3),
                            audiobook_id: id,
                            file_path: "/audiobooks/Foundation/chapter2.mp3".to_string(),
                            position_ms: 150_000,
                            label: "Character development".to_string(),
                            note: Some("Key moment for protagonist growth".to_string()),
                            created_at: Utc::now(),
                        },
                        Bookmark {
                            id: Some(4),
                            audiobook_id: id,
                            file_path: "/audiobooks/Foundation/chapter2.mp3".to_string(),
                            position_ms: 210_000,
                            label: "Midpoint twist".to_string(),
                            note: None,
                            created_at: Utc::now(),
                        },
                        Bookmark {
                            id: Some(5),
                            audiobook_id: id,
                            file_path: "/audiobooks/Foundation/chapter3.mp3".to_string(),
                            position_ms: 270_000,
                            label: "Climax buildup".to_string(),
                            note: Some("Tension reaches its peak here".to_string()),
                            created_at: Utc::now(),
                        },
                    ];
                }
            }
            Self::LongTitle => {
                // Create audiobooks with very long names to test text handling
                let long_title_audiobooks = vec![
                    Audiobook {
                        id: Some(1),
                        directory: "/audiobooks".to_string(),
                        name: "The Extremely Long and Detailed Title of This Audiobook That Continues On and On to Test Text Overflow Handling in the User Interface".to_string(),
                        author: Some("Author With A Very Long Name That Tests Layout Flexibility".to_string()),
                        full_path: "/audiobooks/Long Title".to_string(),
                        completeness: 50,
                        default_order: 0,
                        selected_file: None,
                        total_duration_ms: Some(18_000_000),
                        remaining_duration_ms: Some(9_000_000),
                        created_at: Utc::now(),
                        last_listened: Some(Utc::now()),
                    },
                    Audiobook {
                        id: Some(2),
                        directory: "/audiobooks".to_string(),
                        name: "Another Book With An Exceptionally Long Name That Might Cause Text Truncation Issues If Not Handled Properly by the Layout Engine".to_string(),
                        author: Some("Jane Doe Smith Johnson Williams The Third".to_string()),
                        full_path: "/audiobooks/Another Long Title".to_string(),
                        completeness: 25,
                        default_order: 1,
                        selected_file: None,
                        total_duration_ms: Some(21_600_000),
                        remaining_duration_ms: Some(16_200_000),
                        created_at: Utc::now(),
                        last_listened: Some(Utc::now()),
                    },
                ];

                for audiobook in &long_title_audiobooks {
                    let _result = nodoka::db::queries::insert_audiobook(db.connection(), audiobook);
                }

                state.audiobooks = long_title_audiobooks;
                let id = state.audiobooks.first().and_then(|ab| ab.id);
                state.selected_audiobook = id;
                if let Some(audiobook_id) = id {
                    state.current_files = vec![
                        AudiobookFile {
                            audiobook_id,
                            name: "Chapter 1: This Chapter Has An Extremely Long Name That Tests The Text Wrapping and Ellipsis Handling Capabilities of the UI Component".to_string(),
                            full_path: "/audiobooks/Long Title/chapter1.mp3".to_string(),
                            length_of_file: Some(300_000),
                            seek_position: Some(45_000),
                            checksum: Some("abc123".to_string()),
                            position: 0,
                            completeness: 15,
                            file_exists: true,
                            created_at: Utc::now(),
                        },
                    ];
                }
            }
            Self::LoadingState => {
                // Set loading state to demonstrate skeleton UI
                state.load_state = LoadState::Loading;
                state.audiobooks = vec![]; // Empty during loading
            }
            Self::ScrollbarVisible => {
                // Create enough files to show scrollbar in file list
                if let Some(id) = setup_with_selected_audiobook(state, db) {
                    // Create 20+ files to ensure scrollbar is visible
                    let mut many_files = Vec::new();
                    for i in 0..25 {
                        many_files.push(AudiobookFile {
                            audiobook_id: id,
                            name: format!("Chapter {} - {}", i + 1, &["The Beginning", "The Journey", "The Discovery", "The Challenge", "The Resolution"][i % 5]),
                            full_path: format!("/audiobooks/Foundation/chapter{}.mp3", i + 1),
                            length_of_file: Some(300_000 + (i as i64 * 10_000)),
                            seek_position: if i == 0 { Some(45_000) } else { Some(0) },
                            checksum: Some(format!("checksum{}", i)),
                            position: i as i32,
                            completeness: if i == 0 { 15 } else { 0 },
                            file_exists: true,
                            created_at: Utc::now(),
                        });
                    }
                    state.current_files = many_files;
                }
            }
            Self::ManyItems => {
                // Create 12+ audiobooks to demonstrate scrolling
                let many_audiobooks = vec![
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
                    Audiobook {
                        id: Some(4),
                        directory: "/audiobooks".to_string(),
                        name: "1984".to_string(),
                        author: Some("George Orwell".to_string()),
                        full_path: "/audiobooks/1984".to_string(),
                        completeness: 62,
                        default_order: 3,
                        selected_file: None,
                        total_duration_ms: Some(12_600_000),
                        remaining_duration_ms: Some(4_788_000),
                        created_at: Utc::now(),
                        last_listened: Some(Utc::now()),
                    },
                    Audiobook {
                        id: Some(5),
                        directory: "/audiobooks".to_string(),
                        name: "Brave New World".to_string(),
                        author: Some("Aldous Huxley".to_string()),
                        full_path: "/audiobooks/Brave New World".to_string(),
                        completeness: 20,
                        default_order: 4,
                        selected_file: None,
                        total_duration_ms: Some(14_400_000),
                        remaining_duration_ms: Some(11_520_000),
                        created_at: Utc::now(),
                        last_listened: Some(Utc::now()),
                    },
                    Audiobook {
                        id: Some(6),
                        directory: "/audiobooks".to_string(),
                        name: "Fahrenheit 451".to_string(),
                        author: Some("Ray Bradbury".to_string()),
                        full_path: "/audiobooks/Fahrenheit 451".to_string(),
                        completeness: 85,
                        default_order: 5,
                        selected_file: None,
                        total_duration_ms: Some(9_600_000),
                        remaining_duration_ms: Some(1_440_000),
                        created_at: Utc::now(),
                        last_listened: Some(Utc::now()),
                    },
                    Audiobook {
                        id: Some(7),
                        directory: "/audiobooks".to_string(),
                        name: "The Catcher in the Rye".to_string(),
                        author: Some("J.D. Salinger".to_string()),
                        full_path: "/audiobooks/The Catcher in the Rye".to_string(),
                        completeness: 0,
                        default_order: 6,
                        selected_file: None,
                        total_duration_ms: Some(11_400_000),
                        remaining_duration_ms: Some(11_400_000),
                        created_at: Utc::now(),
                        last_listened: None,
                    },
                    Audiobook {
                        id: Some(8),
                        directory: "/audiobooks".to_string(),
                        name: "To Kill a Mockingbird".to_string(),
                        author: Some("Harper Lee".to_string()),
                        full_path: "/audiobooks/To Kill a Mockingbird".to_string(),
                        completeness: 45,
                        default_order: 7,
                        selected_file: None,
                        total_duration_ms: Some(16_200_000),
                        remaining_duration_ms: Some(8_910_000),
                        created_at: Utc::now(),
                        last_listened: Some(Utc::now()),
                    },
                    Audiobook {
                        id: Some(9),
                        directory: "/audiobooks".to_string(),
                        name: "The Great Gatsby".to_string(),
                        author: Some("F. Scott Fitzgerald".to_string()),
                        full_path: "/audiobooks/The Great Gatsby".to_string(),
                        completeness: 100,
                        default_order: 8,
                        selected_file: None,
                        total_duration_ms: Some(7_800_000),
                        remaining_duration_ms: Some(0),
                        created_at: Utc::now(),
                        last_listened: Some(Utc::now()),
                    },
                    Audiobook {
                        id: Some(10),
                        directory: "/audiobooks".to_string(),
                        name: "Animal Farm".to_string(),
                        author: Some("George Orwell".to_string()),
                        full_path: "/audiobooks/Animal Farm".to_string(),
                        completeness: 10,
                        default_order: 9,
                        selected_file: None,
                        total_duration_ms: Some(8_400_000),
                        remaining_duration_ms: Some(7_560_000),
                        created_at: Utc::now(),
                        last_listened: Some(Utc::now()),
                    },
                    Audiobook {
                        id: Some(11),
                        directory: "/audiobooks".to_string(),
                        name: "Lord of the Flies".to_string(),
                        author: Some("William Golding".to_string()),
                        full_path: "/audiobooks/Lord of the Flies".to_string(),
                        completeness: 0,
                        default_order: 10,
                        selected_file: None,
                        total_duration_ms: Some(9_000_000),
                        remaining_duration_ms: Some(9_000_000),
                        created_at: Utc::now(),
                        last_listened: None,
                    },
                    Audiobook {
                        id: Some(12),
                        directory: "/audiobooks".to_string(),
                        name: "Pride and Prejudice".to_string(),
                        author: Some("Jane Austen".to_string()),
                        full_path: "/audiobooks/Pride and Prejudice".to_string(),
                        completeness: 55,
                        default_order: 11,
                        selected_file: None,
                        total_duration_ms: Some(19_200_000),
                        remaining_duration_ms: Some(8_640_000),
                        created_at: Utc::now(),
                        last_listened: Some(Utc::now()),
                    },
                ];

                for audiobook in &many_audiobooks {
                    let _result = nodoka::db::queries::insert_audiobook(db.connection(), audiobook);
                }

                state.audiobooks = many_audiobooks;
            }
        }
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
        assert_eq!(filenames.len(), 19); // Updated to 19 scenarios
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

        assert!(state.focus_mode);
        assert_eq!(state.playback, PlaybackStatus::Playing);
        Ok(())
    }
}
