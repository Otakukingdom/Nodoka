use crate::db::{self, Database};
use crate::models::{Audiobook, AudiobookFile};
use crate::ui::{FocusedElement, Message, State};

#[test]
fn test_focus_updates_on_play_pause_and_open_settings(
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let db = Database::new_in_memory()?;
    db::initialize(db.connection())?;

    let mut state = State::default();
    let mut player = None;

    let _ = super::super::update(&mut state, Message::PlayPause, &mut player, &db);
    assert_eq!(state.focused_element, FocusedElement::PlayPauseButton);

    let _ = super::super::update(&mut state, Message::OpenSettings, &mut player, &db);
    assert!(state.settings_open);
    assert_eq!(state.focused_element, FocusedElement::SettingsButton);

    Ok(())
}

#[test]
fn test_focus_updates_on_audiobook_and_file_selection(
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let db = Database::new_in_memory()?;
    db::initialize(db.connection())?;

    let audiobook = Audiobook::new(
        "/dir".to_string(),
        "Test".to_string(),
        "/dir/book".to_string(),
        0,
    );
    let id = crate::db::queries::insert_audiobook(db.connection(), &audiobook)?;

    let file_path = "/dir/book/ch1.mp3";
    let file = AudiobookFile::new(id, "ch1".to_string(), file_path.to_string(), 0);
    crate::db::queries::insert_audiobook_file(db.connection(), &file)?;

    let mut state = State {
        audiobooks: vec![Audiobook {
            id: Some(id),
            ..audiobook
        }],
        ..Default::default()
    };
    let mut player = None;

    let _ = super::super::update(&mut state, Message::AudiobookSelected(id), &mut player, &db);
    assert_eq!(state.focused_element, FocusedElement::AudiobookList);

    let _ = super::super::update(
        &mut state,
        Message::FileSelected(file_path.to_string()),
        &mut player,
        &db,
    );
    assert_eq!(state.focused_element, FocusedElement::FileList);

    Ok(())
}

#[test]
fn test_focus_updates_on_volume_speed_and_progress_changes(
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let db = Database::new_in_memory()?;
    db::initialize(db.connection())?;

    let mut state = State::default();
    let mut player = None;

    let _ = super::super::update(&mut state, Message::VolumeChanged(120), &mut player, &db);
    assert_eq!(state.focused_element, FocusedElement::VolumeSlider);

    let _ = super::super::update(&mut state, Message::SpeedChanged(1.25), &mut player, &db);
    assert_eq!(state.focused_element, FocusedElement::SpeedSlider);

    let _ = super::super::update(&mut state, Message::SeekTo(123.0), &mut player, &db);
    assert_eq!(state.focused_element, FocusedElement::ProgressSlider);

    Ok(())
}
