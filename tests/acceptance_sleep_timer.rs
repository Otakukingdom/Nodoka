//! Acceptance tests for Sleep Timer (Category E)
//!
//! Tests sleep timer countdown, end-of-chapter mode, and expiration logic.

use nodoka::models::{SleepTimer, SleepTimerMode};
use std::error::Error;

#[test]
fn test_create_timer_with_duration() -> Result<(), Box<dyn Error>> {
    let timer = SleepTimer::new(SleepTimerMode::Duration(1800), 5);

    assert_eq!(timer.mode, SleepTimerMode::Duration(1800));
    assert_eq!(timer.fade_duration_secs, 5);

    Ok(())
}

#[test]
fn test_timer_countdown() -> Result<(), Box<dyn Error>> {
    let timer = SleepTimer::new(SleepTimerMode::Duration(2), 5);

    assert!(!timer.is_expired());

    std::thread::sleep(std::time::Duration::from_secs(3));

    assert!(timer.is_expired());

    Ok(())
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
fn test_end_of_chapter_mode() -> Result<(), Box<dyn Error>> {
    let timer = SleepTimer::new(SleepTimerMode::EndOfChapter, 5);

    assert_eq!(timer.mode, SleepTimerMode::EndOfChapter);
    assert_eq!(timer.remaining_seconds(), None); // No duration-based countdown
    assert!(!timer.is_expired()); // Never expires on its own

    Ok(())
}

#[test]
fn test_predefined_durations() -> Result<(), Box<dyn Error>> {
    let durations = vec![15 * 60, 30 * 60, 45 * 60, 60 * 60];

    for duration in durations {
        let timer = SleepTimer::new(SleepTimerMode::Duration(duration), 10);
        assert_eq!(timer.mode, SleepTimerMode::Duration(duration));
    }

    Ok(())
}

#[test]
fn test_custom_fade_duration() -> Result<(), Box<dyn Error>> {
    let timer1 = SleepTimer::new(SleepTimerMode::Duration(300), 5);
    let timer2 = SleepTimer::new(SleepTimerMode::Duration(300), 10);
    let timer3 = SleepTimer::new(SleepTimerMode::Duration(300), 15);

    assert_eq!(timer1.fade_duration_secs, 5);
    assert_eq!(timer2.fade_duration_secs, 10);
    assert_eq!(timer3.fade_duration_secs, 15);

    Ok(())
}

#[test]
fn test_timer_expiration_boundary() -> Result<(), Box<dyn Error>> {
    let timer = SleepTimer::new(SleepTimerMode::Duration(1), 0);

    // Should not be expired immediately
    assert!(!timer.is_expired());

    // Wait slightly less than duration
    std::thread::sleep(std::time::Duration::from_millis(900));
    assert!(!timer.is_expired());

    // Wait past duration
    std::thread::sleep(std::time::Duration::from_millis(200));
    assert!(timer.is_expired());

    Ok(())
}

#[test]
fn test_remaining_time_decreases() -> Result<(), Box<dyn Error>> {
    let timer = SleepTimer::new(SleepTimerMode::Duration(10), 5);

    let first = timer.remaining_seconds().ok_or("No time")?;

    std::thread::sleep(std::time::Duration::from_secs(2));

    let second = timer.remaining_seconds().ok_or("No time")?;

    assert!(second < first);
    assert!(second <= 8);

    Ok(())
}

#[test]
fn test_remaining_time_bottoms_at_zero() -> Result<(), Box<dyn Error>> {
    let timer = SleepTimer::new(SleepTimerMode::Duration(1), 0);

    std::thread::sleep(std::time::Duration::from_secs(2));

    let remaining = timer.remaining_seconds().ok_or("No time")?;
    assert_eq!(remaining, 0);

    Ok(())
}

#[test]
fn test_timer_mode_equality() -> Result<(), Box<dyn Error>> {
    let mode1 = SleepTimerMode::Duration(300);
    let mode2 = SleepTimerMode::Duration(300);
    let mode3 = SleepTimerMode::Duration(600);
    let mode4 = SleepTimerMode::EndOfChapter;

    assert_eq!(mode1, mode2);
    assert_ne!(mode1, mode3);
    assert_ne!(mode1, mode4);

    Ok(())
}

#[test]
fn test_short_duration_timer() -> Result<(), Box<dyn Error>> {
    let timer = SleepTimer::new(SleepTimerMode::Duration(1), 1);

    assert!(!timer.is_expired());

    std::thread::sleep(std::time::Duration::from_millis(1100));

    assert!(timer.is_expired());

    Ok(())
}

#[test]
fn test_timer_started_at_timestamp() -> Result<(), Box<dyn Error>> {
    let before = chrono::Utc::now();
    let timer = SleepTimer::new(SleepTimerMode::Duration(300), 5);
    let after = chrono::Utc::now();

    // started_at should be between before and after
    assert!(timer.started_at >= before);
    assert!(timer.started_at <= after);

    Ok(())
}
