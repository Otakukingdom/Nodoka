//! Accessibility-related integration tests.
//!
//! These focus on keyboard navigation behaviors that are critical for WCAG 2.1 AA
//! compliance (e.g., Escape-to-close and shortcut suppression while a modal is open).

use nodoka::db::{self, Database};
use nodoka::player::Vlc;
use nodoka::ui::update;
use nodoka::ui::{BookmarkEditor, Message, State};

fn new_db() -> std::result::Result<Database, Box<dyn std::error::Error>> {
    let db = Database::new_in_memory()?;
    db::initialize(db.connection())?;
    Ok(db)
}

#[test]
fn test_escape_close_modal_closes_topmost_first(
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let db = new_db()?;
    let mut player: Option<Vlc> = None;

    let mut state = State {
        active_modal: nodoka::ui::ActiveModal::Settings,
        bookmark_editor: Some(BookmarkEditor {
            id: None,
            audiobook_id: 1,
            file_path: "/tmp/file.mp3".to_string(),
            position_ms: 0,
            label: "Bookmark".to_string(),
            note: String::new(),
        }),
        ..Default::default()
    };

    let _ = update::update(&mut state, Message::CloseModal, &mut player, &db);
    assert!(state.bookmark_editor.is_none());
    assert!(state.active_modal == nodoka::ui::ActiveModal::Settings);

    let _ = update::update(&mut state, Message::CloseModal, &mut player, &db);
    assert!(state.active_modal != nodoka::ui::ActiveModal::Settings);
    Ok(())
}

#[test]
fn test_keyboard_shortcuts_suppressed_while_modal_open(
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let db = new_db()?;
    let mut player: Option<Vlc> = None;

    let mut state = State {
        active_modal: nodoka::ui::ActiveModal::Settings,
        selected_file: Some("/tmp/file.mp3".to_string()),
        total_duration: 10_000.0,
        current_time: 1_000.0,
        ..Default::default()
    };

    let _ = update::update(&mut state, Message::SeekForward(5), &mut player, &db);
    assert!((state.current_time - 1_000.0).abs() < 0.0001);

    let _ = update::update(&mut state, Message::NextFile, &mut player, &db);
    assert_eq!(state.selected_file.as_deref(), Some("/tmp/file.mp3"));
    Ok(())
}

#[test]
fn test_focus_visible_indicators_present() {
    // Verify focus indicators are implemented for all interactive elements
    // Per WCAG 2.1 2.4.7: Focus Visible
    use iced::widget::button;
    use nodoka::ui::{button_styles, nodoka_theme};

    let theme = nodoka_theme();

    // Focused button styles should differ from unfocused
    let primary_focused = button_styles::primary_focused(&theme, button::Status::Active);
    let primary_unfocused = button_styles::primary(&theme, button::Status::Active);

    // Check border width difference (avoid strict float equality)
    let width_diff = (primary_focused.border.width - primary_unfocused.border.width).abs();
    assert!(
        width_diff > 0.1,
        "Focused buttons should have visible focus indicator (different border width)"
    );

    // Focus indicator should be at least 3px per WCAG recommendations
    assert!(
        primary_focused.border.width >= 3.0,
        "Focus indicator should be at least 3px wide"
    );
}

#[test]
fn test_reduced_motion_preference_in_state() {
    // Verify State struct has prefers_reduced_motion field
    let state = State::default();

    // Should default to false (animations enabled by default)
    assert!(
        !state.prefers_reduced_motion,
        "Reduced motion should default to false"
    );
}

#[test]
fn test_cursor_styles_available() {
    // Verify cursor utilities are available for accessibility
    use nodoka::ui::cursor;

    // Should provide different cursor types for different interaction states
    let _clickable = cursor::clickable();
    let _disabled = cursor::disabled();
    let _text_cursor = cursor::text();

    // Just verify they return values (iced::mouse::Interaction is opaque)
    // Values are consumed automatically at end of scope
}

#[test]
fn test_cursor_affordances_on_interactive_elements() {
    // Verify cursor module provides correct interaction types
    use iced::mouse::Interaction;
    use nodoka::ui::cursor;

    assert_eq!(
        cursor::clickable(),
        Interaction::Pointer,
        "Clickable elements should use pointer cursor"
    );

    assert_eq!(
        cursor::disabled(),
        Interaction::NotAllowed,
        "Disabled elements should use not-allowed cursor"
    );

    assert_eq!(
        cursor::text(),
        Interaction::Text,
        "Text input areas should use text cursor"
    );
}

#[test]
fn test_focus_management_navigation_elements() -> std::result::Result<(), Box<dyn std::error::Error>>
{
    // Verify focus moves correctly through navigation elements (settings, lists)
    // Per WCAG 2.1 2.4.3: Focus Order must be logical and sequential
    use nodoka::db::{self, Database};
    use nodoka::models::{Audiobook, AudiobookFile};
    use nodoka::ui::{FocusedElement, Message, State};

    let db = Database::new_in_memory()?;
    db::initialize(db.connection())?;

    let audiobook = Audiobook::new(
        "/test".to_string(),
        "Test Book".to_string(),
        "/test/book".to_string(),
        0,
    );
    let audiobook_id = nodoka::db::queries::insert_audiobook(db.connection(), &audiobook)?;

    let file = AudiobookFile::new(
        audiobook_id,
        "Chapter 1".to_string(),
        "/test/book/ch1.mp3".to_string(),
        0,
    );
    nodoka::db::queries::insert_audiobook_file(db.connection(), &file)?;

    let mut state = State {
        audiobooks: vec![Audiobook {
            id: Some(audiobook_id),
            ..audiobook
        }],
        current_files: vec![file],
        ..Default::default()
    };
    let mut player = None;

    // Verify focus starts at None
    assert_eq!(state.focused_element, FocusedElement::None);

    // Settings button should receive focus when opened
    let _ = nodoka::ui::update::update(&mut state, Message::OpenSettings, &mut player, &db);
    assert_eq!(state.focused_element, FocusedElement::SettingsButton);

    let _ = nodoka::ui::update::update(&mut state, Message::CloseSettings, &mut player, &db);

    // Audiobook list should receive focus when item selected
    let _ = nodoka::ui::update::update(
        &mut state,
        Message::AudiobookSelected(audiobook_id),
        &mut player,
        &db,
    );
    assert_eq!(state.focused_element, FocusedElement::AudiobookList);

    // File list should receive focus when file selected
    let _ = nodoka::ui::update::update(
        &mut state,
        Message::FileSelected("/test/book/ch1.mp3".to_string()),
        &mut player,
        &db,
    );
    assert_eq!(state.focused_element, FocusedElement::FileList);

    Ok(())
}

#[test]
fn test_focus_management_player_controls() -> std::result::Result<(), Box<dyn std::error::Error>> {
    // Verify focus moves correctly through player control buttons
    // Per WCAG 2.1 2.4.3: Focus Order must be logical and sequential
    use nodoka::db::{self, Database};
    use nodoka::ui::{FocusedElement, Message, State};

    let db = Database::new_in_memory()?;
    db::initialize(db.connection())?;

    let mut state = State::default();
    let mut player = None;

    // Play/Pause button should receive focus when activated
    let _ = nodoka::ui::update::update(&mut state, Message::PlayPause, &mut player, &db);
    assert_eq!(state.focused_element, FocusedElement::PlayPauseButton);

    // Stop button should receive focus when activated
    let _ = nodoka::ui::update::update(&mut state, Message::Stop, &mut player, &db);
    assert_eq!(state.focused_element, FocusedElement::StopButton);

    Ok(())
}

#[test]
fn test_focus_management_sliders() -> std::result::Result<(), Box<dyn std::error::Error>> {
    // Verify focus moves correctly through slider controls
    // Per WCAG 2.1 2.4.3: Focus Order must be logical and sequential
    use nodoka::db::{self, Database};
    use nodoka::ui::{FocusedElement, Message, State};

    let db = Database::new_in_memory()?;
    db::initialize(db.connection())?;

    let mut state = State {
        total_duration: 100.0,
        current_time: 50.0,
        ..Default::default()
    };
    let mut player = None;

    // Progress slider should receive focus when seeking
    let _ = nodoka::ui::update::update(&mut state, Message::SeekTo(75.0), &mut player, &db);
    assert_eq!(state.focused_element, FocusedElement::ProgressSlider);

    // Volume slider should receive focus when adjusted
    let _ = nodoka::ui::update::update(&mut state, Message::VolumeChanged(80), &mut player, &db);
    assert_eq!(state.focused_element, FocusedElement::VolumeSlider);

    // Speed slider should receive focus when adjusted
    let _ = nodoka::ui::update::update(&mut state, Message::SpeedChanged(1.5), &mut player, &db);
    assert_eq!(state.focused_element, FocusedElement::SpeedSlider);

    Ok(())
}
