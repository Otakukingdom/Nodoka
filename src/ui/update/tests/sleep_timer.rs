use crate::db::{self, Database};
use crate::models::{SleepTimer, SleepTimerMode};
use crate::ui::{Message, State};

#[test]
fn test_end_of_chapter_sleep_timer_intercepts_auto_advance() {
    let state = State {
        sleep_timer: Some(SleepTimer::new(SleepTimerMode::EndOfChapter, 0)),
        ..State::default()
    };
    assert!(super::super::sleep_timer::should_pause_for_end_of_chapter(
        &state, true
    ));
    assert!(!super::super::sleep_timer::should_pause_for_end_of_chapter(
        &state, false
    ));
}

#[test]
fn test_sleep_timer_custom_minutes_submit_sets_duration(
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    use super::super::update;

    let db = Database::new_in_memory()?;
    db::initialize(db.connection())?;

    let mut state = State {
        sleep_timer_custom_minutes: "17".to_string(),
        ..State::default()
    };

    let mut player = None;
    let _ = update(
        &mut state,
        Message::SleepTimerCustomSubmit,
        &mut player,
        &db,
    );

    let Some(timer) = state.sleep_timer.as_ref() else {
        return Err("expected active timer".into());
    };
    assert!(matches!(
        timer.mode,
        SleepTimerMode::Duration(secs) if secs == 17 * 60
    ));
    Ok(())
}

#[test]
fn test_sleep_timer_custom_input_validation() -> std::result::Result<(), Box<dyn std::error::Error>>
{
    use super::super::update;

    let db = Database::new_in_memory()?;
    db::initialize(db.connection())?;

    let mut state = State::default();
    let mut player = None;

    let _ = update(
        &mut state,
        Message::SleepTimerCustomMinutesChanged("abc".to_string()),
        &mut player,
        &db,
    );
    assert_eq!(state.sleep_timer_custom_minutes, "abc");

    let _ = update(
        &mut state,
        Message::SleepTimerCustomSubmit,
        &mut player,
        &db,
    );
    assert!(state.sleep_timer_custom_error.is_some());
    assert!(state.sleep_timer.is_none());

    Ok(())
}

#[test]
fn test_sleep_timer_cancel_stops_timer() -> std::result::Result<(), Box<dyn std::error::Error>> {
    use super::super::update;

    let db = Database::new_in_memory()?;
    db::initialize(db.connection())?;

    let mut state = State {
        sleep_timer: Some(SleepTimer::new(SleepTimerMode::Duration(1800), 10)),
        ..Default::default()
    };
    let mut player = None;

    let _ = update(&mut state, Message::SleepTimerCancel, &mut player, &db);

    assert!(state.sleep_timer.is_none());

    Ok(())
}

#[test]
fn test_sleep_timer_extend_adds_time() -> std::result::Result<(), Box<dyn std::error::Error>> {
    use super::super::update;

    let db = Database::new_in_memory()?;
    db::initialize(db.connection())?;

    let mut state = State {
        sleep_timer: Some(SleepTimer::new(SleepTimerMode::Duration(1800), 10)),
        ..Default::default()
    };
    let mut player = None;

    let _ = update(
        &mut state,
        Message::SleepTimerExtendSeconds(900),
        &mut player,
        &db,
    );

    assert!(state.sleep_timer.is_some());

    Ok(())
}

#[test]
fn test_sleep_timer_set_duration_creates_timer(
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    use super::super::update;

    let db = Database::new_in_memory()?;
    db::initialize(db.connection())?;

    let mut state = State::default();
    let mut player = None;

    let _ = update(
        &mut state,
        Message::SleepTimerSetDurationSeconds(1800),
        &mut player,
        &db,
    );

    assert!(state.sleep_timer.is_some());
    if let Some(ref timer) = state.sleep_timer {
        assert!(matches!(timer.mode, SleepTimerMode::Duration(1800)));
    }

    Ok(())
}

#[test]
fn test_sleep_timer_set_end_of_chapter_creates_timer(
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    use super::super::update;

    let db = Database::new_in_memory()?;
    db::initialize(db.connection())?;

    let mut state = State::default();
    let mut player = None;

    let _ = update(
        &mut state,
        Message::SleepTimerSetEndOfChapter,
        &mut player,
        &db,
    );

    assert!(state.sleep_timer.is_some());
    if let Some(ref timer) = state.sleep_timer {
        assert!(matches!(timer.mode, SleepTimerMode::EndOfChapter));
    }

    Ok(())
}
