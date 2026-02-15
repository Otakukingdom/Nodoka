//! Acceptance tests for Sleep Timer (Category E)
//!
//! Tests sleep timer countdown, end-of-chapter mode, and expiration logic.

use nodoka::models::{SleepTimer, SleepTimerMode};
use nodoka::ui::{Message, PlaybackStatus, State};
use std::error::Error;

#[test]
fn test_create_timer_with_duration() {
    let timer = SleepTimer::new(SleepTimerMode::Duration(1800), 5);

    assert_eq!(timer.mode, SleepTimerMode::Duration(1800));
    assert_eq!(timer.fade_duration_secs, 5);
}

#[test]
fn test_timer_countdown() {
    let mut timer = SleepTimer::new(SleepTimerMode::Duration(2), 5);

    assert!(!timer.is_expired());

    timer.started_at = chrono::Utc::now() - chrono::Duration::seconds(3);

    assert!(timer.is_expired());
}

#[test]
fn test_remaining_time_calculation() -> Result<(), Box<dyn Error>> {
    let timer = SleepTimer::new(SleepTimerMode::Duration(60), 5);

    let remaining = timer.remaining_seconds();
    assert!(remaining.is_some());
    let secs = remaining.ok_or("No remaining time")?;

    // Should be close to 60 (allow small variance for execution time)
    assert!(secs <= 60);
    assert!(secs > 55);

    Ok(())
}

#[test]
fn test_end_of_chapter_mode() {
    let timer = SleepTimer::new(SleepTimerMode::EndOfChapter, 5);

    assert_eq!(timer.mode, SleepTimerMode::EndOfChapter);
    assert_eq!(timer.remaining_seconds(), None); // No duration-based countdown
    assert!(!timer.is_expired()); // Never expires on its own
}

#[test]
fn test_predefined_durations() {
    let durations = vec![15 * 60, 30 * 60, 45 * 60, 60 * 60];

    for duration in durations {
        let timer = SleepTimer::new(SleepTimerMode::Duration(duration), 10);
        assert_eq!(timer.mode, SleepTimerMode::Duration(duration));
    }
}

#[test]
fn test_custom_fade_duration() {
    let timer1 = SleepTimer::new(SleepTimerMode::Duration(300), 5);
    let timer2 = SleepTimer::new(SleepTimerMode::Duration(300), 10);
    let timer3 = SleepTimer::new(SleepTimerMode::Duration(300), 15);

    assert_eq!(timer1.fade_duration_secs, 5);
    assert_eq!(timer2.fade_duration_secs, 10);
    assert_eq!(timer3.fade_duration_secs, 15);
}

#[test]
fn test_timer_expiration_boundary() {
    let mut timer = SleepTimer::new(SleepTimerMode::Duration(1), 0);

    // Should not be expired immediately
    assert!(!timer.is_expired());

    // Simulate slightly less than duration
    timer.started_at = chrono::Utc::now() - chrono::Duration::milliseconds(900);
    assert!(!timer.is_expired());

    // Simulate past duration
    timer.started_at = chrono::Utc::now() - chrono::Duration::milliseconds(1100);
    assert!(timer.is_expired());
}

#[test]
fn test_remaining_time_decreases() -> Result<(), Box<dyn Error>> {
    let mut timer = SleepTimer::new(SleepTimerMode::Duration(10), 5);

    let first = timer.remaining_seconds().ok_or("No time")?;

    timer.started_at = chrono::Utc::now() - chrono::Duration::seconds(2);

    let second = timer.remaining_seconds().ok_or("No time")?;

    assert!(second < first);
    assert!(second <= 8);

    Ok(())
}

#[test]
fn test_remaining_time_bottoms_at_zero() -> Result<(), Box<dyn Error>> {
    let mut timer = SleepTimer::new(SleepTimerMode::Duration(1), 0);
    timer.started_at = chrono::Utc::now() - chrono::Duration::seconds(2);

    let remaining = timer.remaining_seconds().ok_or("No time")?;
    assert_eq!(remaining, 0);

    Ok(())
}

#[test]
fn test_timer_mode_equality() {
    let mode1 = SleepTimerMode::Duration(300);
    let mode2 = SleepTimerMode::Duration(300);
    let mode3 = SleepTimerMode::Duration(600);
    let mode4 = SleepTimerMode::EndOfChapter;

    assert_eq!(mode1, mode2);
    assert_ne!(mode1, mode3);
    assert_ne!(mode1, mode4);
}

#[test]
fn test_short_duration_timer() {
    let mut timer = SleepTimer::new(SleepTimerMode::Duration(1), 1);

    assert!(!timer.is_expired());

    timer.started_at = chrono::Utc::now() - chrono::Duration::milliseconds(1100);

    assert!(timer.is_expired());
}

#[test]
fn test_timer_started_at_timestamp() {
    let before = chrono::Utc::now();
    let timer = SleepTimer::new(SleepTimerMode::Duration(300), 5);
    let after = chrono::Utc::now();

    // started_at should be between before and after
    assert!(timer.started_at >= before);
    assert!(timer.started_at <= after);
}

#[test]
fn test_end_of_chapter_mode_single_file() {
    // End-of-chapter with single-file audiobook should pause at end
    let timer = SleepTimer::new(SleepTimerMode::EndOfChapter, 0);

    // Timer should indicate it waits for chapter boundary
    assert!(matches!(timer.mode, SleepTimerMode::EndOfChapter));
}

#[test]
fn test_timer_zero_duration() {
    // Timer with 0 duration should expire immediately or be invalid
    let timer = SleepTimer::new(SleepTimerMode::Duration(0), 0);

    // Should be expired or handled specially
    assert!(timer.is_expired() || timer.remaining_seconds() == Some(0));
}

#[test]
fn test_timer_very_long_duration() {
    // Timer with very long duration (e.g., 24 hours)
    let timer = SleepTimer::new(SleepTimerMode::Duration(86400), 0); // 24 hours in seconds

    assert!(!timer.is_expired());
    if let Some(remaining) = timer.remaining_seconds() {
        assert!(remaining > 86390);
    }
}

#[test]
fn test_multiple_timer_instances() {
    // Multiple timers should be independent
    let mut timer1 = SleepTimer::new(SleepTimerMode::Duration(60), 5);
    let timer2 = SleepTimer::new(SleepTimerMode::Duration(60), 5);

    timer1.started_at = timer2.started_at - chrono::Duration::milliseconds(500);

    // timer1 started earlier, so should have less time remaining
    if let (Some(remaining1), Some(remaining2)) =
        (timer1.remaining_seconds(), timer2.remaining_seconds())
    {
        // Allow some tolerance for timing variations
        assert!(
            remaining1 <= remaining2,
            "Timer1 should have equal or less time remaining"
        );
    }
}

#[test]
fn test_timer_fade_duration_zero() {
    // Timer with zero fade duration
    let timer = SleepTimer::new(SleepTimerMode::Duration(5), 0);

    // Should work with immediate cut-off
    assert!(!timer.is_expired());
}

#[test]
fn test_timer_fade_longer_than_duration() {
    // Fade duration longer than timer duration (edge case)
    let timer = SleepTimer::new(SleepTimerMode::Duration(5), 10);

    // Should handle gracefully
    assert!(!timer.is_expired());
}

#[test]
fn test_sleep_timer_expires_and_pauses_playback_state() -> Result<(), Box<dyn Error>> {
    let db = nodoka::db::Database::new_in_memory()?;
    nodoka::db::initialize(db.connection())?;

    let mut state = State {
        selected_file: Some("/tmp/book/ch1.mp3".to_string()),
        playback: PlaybackStatus::Playing,
        ..State::default()
    };

    let mut player: Option<nodoka::player::Vlc> = None;

    let _ = nodoka::ui::update::update(
        &mut state,
        Message::SleepTimerSetDurationSeconds(1),
        &mut player,
        &db,
    );

    let Some(ref mut timer) = state.sleep_timer else {
        return Err("expected active sleep timer".into());
    };
    timer.started_at = chrono::Utc::now() - chrono::Duration::seconds(2);

    let _ = nodoka::ui::update::update(&mut state, Message::PlayerTick, &mut player, &db);

    assert!(state.sleep_timer.is_none());
    assert_eq!(state.playback, PlaybackStatus::Paused);
    Ok(())
}
