use crate::db::{self, Database};
use crate::models::{Audiobook, AudiobookFile};
use crate::ui::{Message, State};

#[test]
fn test_handle_time_updated_clamps_negative_to_zero(
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    use super::super::update;

    let db = Database::new_in_memory()?;
    db::initialize(db.connection())?;

    let audiobook = Audiobook::new(
        "/dir".to_string(),
        "Test".to_string(),
        "/dir/book".to_string(),
        0,
    );
    let audiobook_id = crate::db::queries::insert_audiobook(db.connection(), &audiobook)?;

    let file_path = "/dir/book/ch1.mp3";
    let file = AudiobookFile::new(audiobook_id, "ch1".to_string(), file_path.to_string(), 0);
    crate::db::queries::insert_audiobook_file(db.connection(), &file)?;

    let mut state = State {
        selected_audiobook: Some(audiobook_id),
        selected_file: Some(file_path.to_string()),
        total_duration: 10_000.0,
        current_time: 5_000.0,
        ..Default::default()
    };
    let mut player = None;

    let _ = update(
        &mut state,
        Message::PlayerTimeUpdated(-123.0),
        &mut player,
        &db,
    );

    assert!(
        (state.current_time - 0.0).abs() < 0.0001,
        "negative time updates should clamp to zero"
    );

    let saved = crate::db::queries::get_audiobook_file_by_path(db.connection(), file_path)?
        .ok_or("missing audiobook file")?;
    assert_eq!(
        saved.seek_position,
        Some(0),
        "negative time update should persist as 0ms"
    );
    Ok(())
}

#[test]
fn test_handle_time_updated_ignores_non_finite_values(
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    use super::super::update;

    let db = Database::new_in_memory()?;
    db::initialize(db.connection())?;

    let audiobook = Audiobook::new(
        "/dir".to_string(),
        "Test".to_string(),
        "/dir/book".to_string(),
        0,
    );
    let audiobook_id = crate::db::queries::insert_audiobook(db.connection(), &audiobook)?;

    let file_path = "/dir/book/ch1.mp3";
    let file = AudiobookFile::new(audiobook_id, "ch1".to_string(), file_path.to_string(), 0);
    crate::db::queries::insert_audiobook_file(db.connection(), &file)?;
    crate::db::queries::update_file_progress(db.connection(), file_path, 5_000.0, 50)?;

    let mut state = State {
        selected_audiobook: Some(audiobook_id),
        selected_file: Some(file_path.to_string()),
        total_duration: 10_000.0,
        current_time: 5_000.0,
        ..Default::default()
    };
    let mut player = None;

    let _ = update(
        &mut state,
        Message::PlayerTimeUpdated(f64::NAN),
        &mut player,
        &db,
    );

    assert!(
        (state.current_time - 5_000.0).abs() < 0.0001,
        "non-finite time updates should be ignored"
    );

    let saved = crate::db::queries::get_audiobook_file_by_path(db.connection(), file_path)?
        .ok_or("missing audiobook file")?;
    assert_eq!(
        saved.seek_position,
        Some(5000),
        "non-finite time updates should not persist progress"
    );
    Ok(())
}
