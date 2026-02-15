//! UI error handling integration tests.
//!
//! These tests exercise `nodoka::ui::update` to validate user-facing error
//! handling behavior without relying on lint suppressions.

use nodoka::db::{self, Database};
use nodoka::player::Vlc;
use nodoka::ui::update;
use nodoka::ui::{Message, State};

fn new_db() -> std::result::Result<Database, Box<dyn std::error::Error>> {
    let db = Database::new_in_memory()?;
    db::initialize(db.connection())?;
    Ok(db)
}

#[test]
fn test_sleep_timer_custom_minutes_negative_rejected(
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let db = new_db()?;
    let mut state = State::default();
    let mut player: Option<Vlc> = None;

    let _ = update::update(
        &mut state,
        Message::SleepTimerCustomMinutesChanged("-5".to_string()),
        &mut player,
        &db,
    );
    let _ = update::update(
        &mut state,
        Message::SleepTimerCustomSubmit,
        &mut player,
        &db,
    );

    assert!(state.sleep_timer.is_none());
    assert_eq!(
        state.sleep_timer_custom_error.as_deref(),
        Some("Minutes must be greater than zero")
    );
    Ok(())
}

#[test]
fn test_sleep_timer_custom_minutes_zero_rejected(
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let db = new_db()?;
    let mut state = State::default();
    let mut player: Option<Vlc> = None;

    let _ = update::update(
        &mut state,
        Message::SleepTimerCustomMinutesChanged("0".to_string()),
        &mut player,
        &db,
    );
    let _ = update::update(
        &mut state,
        Message::SleepTimerCustomSubmit,
        &mut player,
        &db,
    );

    assert!(state.sleep_timer.is_none());
    assert_eq!(
        state.sleep_timer_custom_error.as_deref(),
        Some("Minutes must be greater than zero")
    );
    Ok(())
}

#[test]
fn test_sleep_timer_custom_minutes_non_numeric_rejected(
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let db = new_db()?;
    let mut state = State::default();
    let mut player: Option<Vlc> = None;

    let _ = update::update(
        &mut state,
        Message::SleepTimerCustomMinutesChanged("abc".to_string()),
        &mut player,
        &db,
    );
    let _ = update::update(
        &mut state,
        Message::SleepTimerCustomSubmit,
        &mut player,
        &db,
    );

    assert!(state.sleep_timer.is_none());
    assert_eq!(
        state.sleep_timer_custom_error.as_deref(),
        Some("Minutes must be a whole number")
    );
    Ok(())
}

#[test]
fn test_sleep_timer_custom_minutes_valid_creates_timer(
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let db = new_db()?;
    let mut state = State::default();
    let mut player: Option<Vlc> = None;

    let _ = update::update(
        &mut state,
        Message::SleepTimerCustomMinutesChanged("15".to_string()),
        &mut player,
        &db,
    );
    let _ = update::update(
        &mut state,
        Message::SleepTimerCustomSubmit,
        &mut player,
        &db,
    );

    let timer = state.sleep_timer.as_ref().ok_or("expected sleep timer")?;
    assert!(matches!(
        timer.mode,
        nodoka::models::SleepTimerMode::Duration(900)
    ));
    assert!(state.sleep_timer_custom_error.is_none());
    Ok(())
}

#[test]
fn test_volume_changed_clamps_and_persists() -> std::result::Result<(), Box<dyn std::error::Error>>
{
    let db = new_db()?;
    let mut state = State::default();
    let mut player: Option<Vlc> = None;

    let _ = update::update(&mut state, Message::VolumeChanged(-10), &mut player, &db);
    assert_eq!(state.volume, 0);

    let saved = nodoka::db::queries::get_metadata(db.connection(), "volume")?
        .ok_or("missing volume metadata")?;
    assert_eq!(saved, "0");
    Ok(())
}

#[test]
fn test_speed_changed_clamps_and_persists() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let db = new_db()?;
    let mut state = State::default();
    let mut player: Option<Vlc> = None;

    let _ = update::update(&mut state, Message::SpeedChanged(0.3), &mut player, &db);
    assert!((state.speed - 0.5).abs() < 0.0001);

    let saved = nodoka::db::queries::get_metadata(db.connection(), "speed")?
        .ok_or("missing speed metadata")?;
    assert_eq!(saved, "0.5");
    Ok(())
}
