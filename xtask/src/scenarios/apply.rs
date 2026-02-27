use chrono::Utc;
use nodoka::db::Database;
use nodoka::models::{Audiobook, AudiobookFile, Bookmark, SleepTimer, SleepTimerMode};
use nodoka::ui::animations::modal::ModalState;
use nodoka::ui::{ActiveModal, BookmarkEditor, FocusedElement, LoadState, PlaybackStatus, State};

use super::Scenario;

pub(super) fn apply_to_state(scenario: Scenario, state: &mut State, db: &Database) {
    *state = State::default();
    state.load_state = LoadState::Ready;

    match scenario {
        Scenario::EmptyLibrary => {}
        Scenario::LibraryWithAudiobooks => apply_library_with_audiobooks(state, db),
        Scenario::AudiobookSelected => apply_audiobook_selected(state, db),
        Scenario::PlayingState => apply_playing_state(state, db),
        Scenario::PausedState => apply_paused_state(state, db),
        Scenario::FocusModeEnabled => apply_focus_mode_enabled(state, db),
        Scenario::SettingsModalOpen => apply_settings_modal_open(state, db),
        Scenario::BookmarkEditorOpen => apply_bookmark_editor_open(state, db),
        Scenario::KeyboardHelpOverlay => apply_keyboard_help_overlay(state, db),
        Scenario::NotificationVisible => apply_notification_visible(state, db),
        Scenario::VolumeExpanded => apply_volume_expanded(state, db),
        Scenario::SearchActive => apply_search_active(state, db),
        Scenario::HoverStates => apply_hover_states(state, db),
        Scenario::ErrorState => apply_error_state(state, db),
        Scenario::MultipleBookmarks => apply_multiple_bookmarks(state, db),
        Scenario::LongTitle => apply_long_title(state, db),
        Scenario::LoadingState => apply_loading_state(state),
        Scenario::ScrollbarVisible => apply_scrollbar_visible(state, db),
        Scenario::ManyItems => apply_many_items(state, db),
        Scenario::BookmarkSelected => apply_bookmark_selected(state, db),
        Scenario::FileSelected => apply_file_selected(state, db),
        Scenario::SpeedChanged => apply_speed_changed(state, db),
        Scenario::VolumeMuted => apply_volume_muted(state, db),
        Scenario::SleepTimerActive => apply_sleep_timer_active(state, db),
    }
}

fn apply_library_with_audiobooks(state: &mut State, db: &Database) {
    state.audiobooks = super::create_fixture_audiobooks(db);
}

fn apply_audiobook_selected(state: &mut State, db: &Database) {
    let _id = super::setup_with_selected_audiobook(state, db);
}

fn apply_playing_state(state: &mut State, db: &Database) {
    if super::setup_with_selected_audiobook(state, db).is_some() {
        state.playback = PlaybackStatus::Playing;
        state.current_time = 45_000.0;
        state.total_duration = 300_000.0;
        state.volume = 75;
        state.speed = 1.0;
    }
}

fn apply_paused_state(state: &mut State, db: &Database) {
    if super::setup_with_selected_audiobook(state, db).is_some() {
        state.playback = PlaybackStatus::Paused;
        state.current_time = 120_000.0;
        state.total_duration = 300_000.0;
    }
}

fn apply_focus_mode_enabled(state: &mut State, db: &Database) {
    if super::setup_with_selected_audiobook(state, db).is_some() {
        state.playback = PlaybackStatus::Playing;
        state.focus_mode = true.into();
    }
}

fn apply_settings_modal_open(state: &mut State, db: &Database) {
    state.audiobooks = super::create_fixture_audiobooks(db);
    state.directories = super::create_fixture_directories();
    state.active_modal = ActiveModal::Settings;
    state.modal_state = ModalState::Open;
}

fn apply_bookmark_editor_open(state: &mut State, db: &Database) {
    if let Some(id) = super::setup_with_selected_audiobook(state, db) {
        state.bookmarks = super::create_fixture_bookmarks(id);
        state.active_modal = ActiveModal::BookmarkEditor;
        state.modal_state = ModalState::Open;
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

fn apply_keyboard_help_overlay(state: &mut State, db: &Database) {
    state.audiobooks = super::create_fixture_audiobooks(db);
    state.active_modal = ActiveModal::KeyboardHelp;
    state.modal_state = ModalState::Open;
}

fn apply_notification_visible(state: &mut State, db: &Database) {
    state.audiobooks = super::create_fixture_audiobooks(db);
    state
        .notification_queue
        .push(nodoka::ui::Notification::success(
            "Bookmark saved successfully".to_string(),
        ));
}

fn apply_volume_expanded(state: &mut State, db: &Database) {
    if super::setup_with_selected_audiobook(state, db).is_some() {
        state.volume_expanded = true.into();
        state.volume = 80;
    }
}

fn apply_search_active(state: &mut State, db: &Database) {
    state.audiobooks = super::create_fixture_audiobooks(db);
    state.search_query = "foundation".to_string();
}

fn apply_hover_states(state: &mut State, db: &Database) {
    if super::setup_with_selected_audiobook(state, db).is_some() {
        state.playback = PlaybackStatus::Playing;
        state.current_time = 60_000.0;
        state.total_duration = 300_000.0;
        state.focused_element = FocusedElement::FileList;
    }
}

fn apply_error_state(state: &mut State, db: &Database) {
    if let Some(id) = super::setup_with_selected_audiobook(state, db) {
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
                file_exists: false,
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
        state
            .notification_queue
            .push(nodoka::ui::Notification::error(
                "Some files could not be found".to_string(),
            ));
    }
}

fn apply_multiple_bookmarks(state: &mut State, db: &Database) {
    if let Some(id) = super::setup_with_selected_audiobook(state, db) {
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

fn apply_long_title(state: &mut State, db: &Database) {
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
        state.current_files = vec![AudiobookFile {
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
        }];
    }
}

fn apply_loading_state(state: &mut State) {
    state.load_state = LoadState::Loading;
    state.audiobooks.clear();
}

fn apply_scrollbar_visible(state: &mut State, db: &Database) {
    if let Some(id) = super::setup_with_selected_audiobook(state, db) {
        const CHAPTER_TITLES: [&str; 5] = [
            "The Beginning",
            "The Journey",
            "The Discovery",
            "The Challenge",
            "The Resolution",
        ];

        let mut many_files = Vec::new();
        for i in 0..25 {
            let chapter_title = CHAPTER_TITLES
                .get(i % CHAPTER_TITLES.len())
                .copied()
                .unwrap_or("The Beginning");
            let i_i64 = i64::try_from(i).unwrap_or(i64::MAX);
            let length_of_file = 300_000_i64.saturating_add(i_i64.saturating_mul(10_000));

            many_files.push(AudiobookFile {
                audiobook_id: id,
                name: format!("Chapter {} - {chapter_title}", i + 1),
                full_path: format!("/audiobooks/Foundation/chapter{}.mp3", i + 1),
                length_of_file: Some(length_of_file),
                seek_position: if i == 0 { Some(45_000) } else { Some(0) },
                checksum: Some(format!("checksum{i}")),
                position: i32::try_from(i).unwrap_or(i32::MAX),
                completeness: if i == 0 { 15 } else { 0 },
                file_exists: true,
                created_at: Utc::now(),
            });
        }

        state.current_files = many_files;
    }
}

fn apply_many_items(state: &mut State, db: &Database) {
    state.audiobooks = create_fixture_many_audiobooks(db);
}

fn apply_bookmark_selected(state: &mut State, db: &Database) {
    if let Some(id) = super::setup_with_selected_audiobook(state, db) {
        state.bookmarks = super::create_fixture_bookmarks(id);
        state.focused_element = FocusedElement::BookmarkList;
    }
}

fn apply_file_selected(state: &mut State, db: &Database) {
    if super::setup_with_selected_audiobook(state, db).is_some() {
        state.selected_file = Some("/audiobooks/Foundation/chapter2.mp3".to_string());
    }
}

fn apply_speed_changed(state: &mut State, db: &Database) {
    if super::setup_with_selected_audiobook(state, db).is_some() {
        state.playback = PlaybackStatus::Playing;
        state.current_time = 60_000.0;
        state.total_duration = 300_000.0;
        state.speed = 1.5;
    }
}

fn apply_volume_muted(state: &mut State, db: &Database) {
    if super::setup_with_selected_audiobook(state, db).is_some() {
        state.volume = 0;
        state.volume_expanded = true.into();
    }
}

fn apply_sleep_timer_active(state: &mut State, db: &Database) {
    if super::setup_with_selected_audiobook(state, db).is_some() {
        state.playback = PlaybackStatus::Playing;
        state.sleep_timer = Some(SleepTimer::new(SleepTimerMode::Duration(1800), 10));
    }
}

struct ManyAudiobookFixture {
    name: &'static str,
    author: Option<&'static str>,
    full_path: &'static str,
    completeness: i32,
    selected_file: Option<&'static str>,
    total_duration_ms: i64,
    remaining_duration_ms: i64,
    last_listened: bool,
}

const MANY_ITEMS_FIXTURES: [ManyAudiobookFixture; 12] = [
    ManyAudiobookFixture {
        name: "Foundation",
        author: Some("Isaac Asimov"),
        full_path: "/audiobooks/Foundation",
        completeness: 35,
        selected_file: Some("/audiobooks/Foundation/chapter1.mp3"),
        total_duration_ms: 18_000_000,
        remaining_duration_ms: 11_700_000,
        last_listened: true,
    },
    ManyAudiobookFixture {
        name: "Dune",
        author: Some("Frank Herbert"),
        full_path: "/audiobooks/Dune",
        completeness: 0,
        selected_file: None,
        total_duration_ms: 21_600_000,
        remaining_duration_ms: 21_600_000,
        last_listened: false,
    },
    ManyAudiobookFixture {
        name: "The Hobbit",
        author: Some("J.R.R. Tolkien"),
        full_path: "/audiobooks/The Hobbit",
        completeness: 100,
        selected_file: None,
        total_duration_ms: 10_800_000,
        remaining_duration_ms: 0,
        last_listened: true,
    },
    ManyAudiobookFixture {
        name: "1984",
        author: Some("George Orwell"),
        full_path: "/audiobooks/1984",
        completeness: 62,
        selected_file: None,
        total_duration_ms: 12_600_000,
        remaining_duration_ms: 4_788_000,
        last_listened: true,
    },
    ManyAudiobookFixture {
        name: "Brave New World",
        author: Some("Aldous Huxley"),
        full_path: "/audiobooks/Brave New World",
        completeness: 20,
        selected_file: None,
        total_duration_ms: 14_400_000,
        remaining_duration_ms: 11_520_000,
        last_listened: true,
    },
    ManyAudiobookFixture {
        name: "Fahrenheit 451",
        author: Some("Ray Bradbury"),
        full_path: "/audiobooks/Fahrenheit 451",
        completeness: 85,
        selected_file: None,
        total_duration_ms: 9_600_000,
        remaining_duration_ms: 1_440_000,
        last_listened: true,
    },
    ManyAudiobookFixture {
        name: "The Catcher in the Rye",
        author: Some("J.D. Salinger"),
        full_path: "/audiobooks/The Catcher in the Rye",
        completeness: 0,
        selected_file: None,
        total_duration_ms: 11_400_000,
        remaining_duration_ms: 11_400_000,
        last_listened: false,
    },
    ManyAudiobookFixture {
        name: "To Kill a Mockingbird",
        author: Some("Harper Lee"),
        full_path: "/audiobooks/To Kill a Mockingbird",
        completeness: 45,
        selected_file: None,
        total_duration_ms: 16_200_000,
        remaining_duration_ms: 8_910_000,
        last_listened: true,
    },
    ManyAudiobookFixture {
        name: "The Great Gatsby",
        author: Some("F. Scott Fitzgerald"),
        full_path: "/audiobooks/The Great Gatsby",
        completeness: 100,
        selected_file: None,
        total_duration_ms: 7_800_000,
        remaining_duration_ms: 0,
        last_listened: true,
    },
    ManyAudiobookFixture {
        name: "Animal Farm",
        author: Some("George Orwell"),
        full_path: "/audiobooks/Animal Farm",
        completeness: 10,
        selected_file: None,
        total_duration_ms: 8_400_000,
        remaining_duration_ms: 7_560_000,
        last_listened: true,
    },
    ManyAudiobookFixture {
        name: "Lord of the Flies",
        author: Some("William Golding"),
        full_path: "/audiobooks/Lord of the Flies",
        completeness: 0,
        selected_file: None,
        total_duration_ms: 9_000_000,
        remaining_duration_ms: 9_000_000,
        last_listened: false,
    },
    ManyAudiobookFixture {
        name: "Pride and Prejudice",
        author: Some("Jane Austen"),
        full_path: "/audiobooks/Pride and Prejudice",
        completeness: 55,
        selected_file: None,
        total_duration_ms: 19_200_000,
        remaining_duration_ms: 8_640_000,
        last_listened: true,
    },
];

fn create_fixture_many_audiobooks(db: &Database) -> Vec<Audiobook> {
    let now = Utc::now();
    let mut audiobooks = Vec::new();

    for (index, fixture) in MANY_ITEMS_FIXTURES.iter().enumerate() {
        let id_i64 = i64::try_from(index + 1).unwrap_or(i64::MAX);
        let audiobook = Audiobook {
            id: Some(id_i64),
            directory: "/audiobooks".to_string(),
            name: fixture.name.to_string(),
            author: fixture.author.map(std::string::ToString::to_string),
            full_path: fixture.full_path.to_string(),
            completeness: fixture.completeness,
            default_order: i32::try_from(index).unwrap_or(i32::MAX),
            selected_file: fixture.selected_file.map(std::string::ToString::to_string),
            total_duration_ms: Some(fixture.total_duration_ms),
            remaining_duration_ms: Some(fixture.remaining_duration_ms),
            created_at: now,
            last_listened: fixture.last_listened.then_some(now),
        };

        let _result = nodoka::db::queries::insert_audiobook(db.connection(), &audiobook);
        audiobooks.push(audiobook);
    }

    audiobooks
}
