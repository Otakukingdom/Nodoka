use crate::db::{self, Database};
use crate::ui::{Message, State};

#[test]
fn test_volume_changed_clamps_and_persists_without_player(
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    use super::super::update;

    let db = Database::new_in_memory()?;
    db::initialize(db.connection())?;

    let mut state = State::default();
    let mut player = None;

    let _ = update(&mut state, Message::VolumeChanged(999), &mut player, &db);

    assert_eq!(state.volume, 200);

    let saved = crate::db::queries::get_metadata(db.connection(), "volume")?
        .ok_or("missing volume metadata")?;
    assert_eq!(saved, "200");
    Ok(())
}

#[test]
fn test_speed_changed_quantizes_and_persists_without_player(
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    use super::super::update;

    let db = Database::new_in_memory()?;
    db::initialize(db.connection())?;

    let mut state = State::default();
    let mut player = None;

    let _ = update(&mut state, Message::SpeedChanged(1.26), &mut player, &db);

    assert!((state.speed - 1.25).abs() < 0.0001);

    let saved = crate::db::queries::get_metadata(db.connection(), "speed")?
        .ok_or("missing speed metadata")?;
    assert_eq!(saved, "1.25");
    Ok(())
}

#[test]
fn test_speed_preset_values_are_preserved_exactly(
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    use super::super::update;

    let db = Database::new_in_memory()?;
    db::initialize(db.connection())?;

    let mut state = State::default();
    let mut player = None;

    let _ = update(&mut state, Message::SpeedChanged(0.75), &mut player, &db);

    assert!((state.speed - 0.75).abs() < 0.0001);

    let saved = crate::db::queries::get_metadata(db.connection(), "speed")?
        .ok_or("missing speed metadata")?;
    assert_eq!(saved, "0.75");
    Ok(())
}
