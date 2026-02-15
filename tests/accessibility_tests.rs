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
        settings_open: true,
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
    assert!(state.settings_open);

    let _ = update::update(&mut state, Message::CloseModal, &mut player, &db);
    assert!(!state.settings_open);
    Ok(())
}

#[test]
fn test_keyboard_shortcuts_suppressed_while_modal_open(
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let db = new_db()?;
    let mut player: Option<Vlc> = None;

    let mut state = State {
        settings_open: true,
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
