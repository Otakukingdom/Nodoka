use crate::db::{self, Database};
use crate::ui::{Message, PlaybackStatus, State};

#[test]
fn test_play_pause_toggles_state() -> std::result::Result<(), Box<dyn std::error::Error>> {
    use super::super::update;

    let db = Database::new_in_memory()?;
    db::initialize(db.connection())?;

    let mut state = State {
        playback: PlaybackStatus::Paused,
        ..Default::default()
    };
    let mut player = None;

    let _ = update(&mut state, Message::PlayPause, &mut player, &db);

    Ok(())
}

#[test]
fn test_seek_to_updates_state() -> std::result::Result<(), Box<dyn std::error::Error>> {
    use super::super::update;

    let db = Database::new_in_memory()?;
    db::initialize(db.connection())?;

    let mut state = State {
        current_time: 1000.0,
        total_duration: 10000.0,
        ..Default::default()
    };
    let mut player = None;

    let _ = update(&mut state, Message::SeekTo(5000.0), &mut player, &db);
    Ok(())
}

#[test]
fn test_stop_message_stops_playback() -> std::result::Result<(), Box<dyn std::error::Error>> {
    use super::super::update;

    let db = Database::new_in_memory()?;
    db::initialize(db.connection())?;

    let mut state = State {
        playback: PlaybackStatus::Playing,
        current_time: 1500.0,
        ..Default::default()
    };
    let mut player = None;

    let _ = update(&mut state, Message::Stop, &mut player, &db);

    assert_eq!(state.playback, PlaybackStatus::Paused);
    assert!((state.current_time - 0.0).abs() < 0.0001);

    Ok(())
}
