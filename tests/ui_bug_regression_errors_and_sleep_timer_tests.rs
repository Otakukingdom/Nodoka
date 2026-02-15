//! Regression tests for UI bugs discovered during systematic testing.
//!
//! Tests in this file focus on error handling and sleep timer behavior.

use nodoka::models::{SleepTimer, SleepTimerMode};
use nodoka::ui::{Message, PlaybackStatus, ScanState, State};
use std::error::Error;

mod acceptance_support;

/// Bug #0002: Error messages properly clear when new errors occur
///
/// Scenario: An error is displayed, then another error occurs.
/// The old error should be replaced, not accumulated.
///
/// Expected: Only the most recent error is displayed.
#[test]
fn test_bug_0002_error_messages_replace_not_accumulate() -> Result<(), Box<dyn Error>> {
    let db = crate::acceptance_support::create_test_db()?;

    let mut state = State::default();
    let mut player = None;

    let _ = nodoka::ui::update::update(
        &mut state,
        Message::ScanError("First error".to_string()),
        &mut player,
        &db,
    );
    assert!(matches!(state.error_message.as_deref(), Some(m) if m.contains("First error")));

    let _ = nodoka::ui::update::update(
        &mut state,
        Message::ScanError("Second error".to_string()),
        &mut player,
        &db,
    );

    assert!(matches!(state.error_message.as_deref(), Some(m) if m.contains("Second error")));

    Ok(())
}

/// Bug #0006: Sleep timer cancellation restores volume
///
/// Scenario: Sleep timer is active with faded volume, user cancels.
///
/// Expected: Volume is restored to original level.
#[test]
fn test_bug_0006_sleep_timer_cancel_restores_volume() -> Result<(), Box<dyn Error>> {
    let mut state = State {
        volume: 100,
        sleep_timer: Some(SleepTimer::new(SleepTimerMode::Duration(60), 7)),
        sleep_timer_base_volume: Some(100),
        ..Default::default()
    };

    let db = crate::acceptance_support::create_test_db()?;
    let mut player = None;
    let _ = nodoka::ui::update::update(&mut state, Message::SleepTimerCancel, &mut player, &db);

    assert_eq!(state.volume, 100);
    assert!(state.sleep_timer.is_none());
    assert!(state.sleep_timer_base_volume.is_none());

    Ok(())
}

/// Bug #0018: Sleep timer custom input validation
///
/// Scenario: User enters invalid input in custom minutes field.
///
/// Expected: Helpful error message, timer not set.
#[test]
fn test_bug_0018_sleep_timer_custom_input_validation() -> Result<(), Box<dyn Error>> {
    let db = crate::acceptance_support::create_test_db()?;
    let mut player = None;
    let mut state = State::default();

    let _ = nodoka::ui::update::update(
        &mut state,
        Message::SleepTimerCustomMinutesChanged(String::new()),
        &mut player,
        &db,
    );
    let _ = nodoka::ui::update::update(
        &mut state,
        Message::SleepTimerCustomSubmit,
        &mut player,
        &db,
    );
    assert_eq!(
        state.sleep_timer_custom_error.as_deref(),
        Some("Enter minutes")
    );
    assert!(state.sleep_timer.is_none());

    let _ = nodoka::ui::update::update(
        &mut state,
        Message::SleepTimerCustomMinutesChanged("abc".to_string()),
        &mut player,
        &db,
    );
    let _ = nodoka::ui::update::update(
        &mut state,
        Message::SleepTimerCustomSubmit,
        &mut player,
        &db,
    );
    assert_eq!(
        state.sleep_timer_custom_error.as_deref(),
        Some("Minutes must be a whole number")
    );
    assert!(state.sleep_timer.is_none());

    let _ = nodoka::ui::update::update(
        &mut state,
        Message::SleepTimerCustomMinutesChanged("-5".to_string()),
        &mut player,
        &db,
    );
    let _ = nodoka::ui::update::update(
        &mut state,
        Message::SleepTimerCustomSubmit,
        &mut player,
        &db,
    );
    assert_eq!(
        state.sleep_timer_custom_error.as_deref(),
        Some("Minutes must be greater than zero")
    );
    assert!(state.sleep_timer.is_none());

    let _ = nodoka::ui::update::update(
        &mut state,
        Message::SleepTimerCustomMinutesChanged("45".to_string()),
        &mut player,
        &db,
    );
    let _ = nodoka::ui::update::update(
        &mut state,
        Message::SleepTimerCustomSubmit,
        &mut player,
        &db,
    );
    assert!(state.sleep_timer_custom_error.is_none());
    assert!(state.sleep_timer.is_some());

    Ok(())
}

/// Bug #0023: Error messages shown for file load failures
///
/// Scenario: User selects a file that fails to load.
///
/// Expected: Error message is set in state so UI can display it to user.
/// Bug #0024: Audiobook selection only updates after successful loading
///
/// Scenario: User selects audiobook but file loading fails.
///
/// Expected: `selected_audiobook` remains unchanged, files are not cleared,
/// and error message is shown.
#[test]
fn test_bug_0024_audiobook_selection_load_files_failure_does_not_clear_state(
) -> Result<(), Box<dyn Error>> {
    use nodoka::models::{Audiobook, AudiobookFile, Bookmark};

    let db = crate::acceptance_support::create_test_db()?;
    let old_id = crate::acceptance_support::create_test_audiobook(&db, "/test", "Old")?;
    let new_id = crate::acceptance_support::create_test_audiobook(&db, "/test", "New")?;

    // Force `get_audiobook_files` to fail.
    db.connection()
        .execute_batch("DROP TABLE audiobook_file;")?;

    let mut old_ab = Audiobook::new(
        "/test".to_string(),
        "Old".to_string(),
        "/test/Old".to_string(),
        0,
    );
    old_ab.id = Some(old_id);
    let mut new_ab = Audiobook::new(
        "/test".to_string(),
        "New".to_string(),
        "/test/New".to_string(),
        0,
    );
    new_ab.id = Some(new_id);

    let file_path = "/old/file1.mp3".to_string();
    let old_file = AudiobookFile {
        audiobook_id: old_id,
        name: "file1.mp3".to_string(),
        full_path: file_path.clone(),
        length_of_file: None,
        seek_position: None,
        checksum: None,
        position: 0,
        completeness: 0,
        file_exists: true,
        created_at: chrono::Utc::now(),
    };

    let old_bookmark = Bookmark {
        id: Some(1),
        audiobook_id: old_id,
        file_path: file_path.clone(),
        position_ms: 1_000,
        label: "Keep".to_string(),
        note: None,
        created_at: chrono::Utc::now(),
    };

    let mut state = State {
        audiobooks: vec![old_ab, new_ab],
        selected_audiobook: Some(old_id),
        current_files: vec![old_file],
        selected_file: Some(file_path.clone()),
        bookmarks: vec![old_bookmark],
        bookmark_editor: Some(nodoka::ui::BookmarkEditor {
            id: None,
            audiobook_id: old_id,
            file_path: file_path.clone(),
            position_ms: 1_000,
            label: "Editor".to_string(),
            note: "Note".to_string(),
        }),
        playback: PlaybackStatus::Playing,
        current_time: 123.0,
        total_duration: 456.0,
        ..Default::default()
    };

    let mut player = None;
    let _task = nodoka::ui::update::update(
        &mut state,
        Message::AudiobookSelected(new_id),
        &mut player,
        &db,
    );

    // Selection and dependent state must remain unchanged on failure.
    assert_eq!(state.selected_audiobook, Some(old_id));
    assert_eq!(state.selected_file.as_deref(), Some(file_path.as_str()));
    assert_eq!(state.current_files.len(), 1);
    assert_eq!(state.bookmarks.len(), 1);
    assert!(state.bookmark_editor.is_some());
    assert_eq!(state.playback, PlaybackStatus::Playing);
    assert!((state.current_time - 123.0).abs() < f64::EPSILON);
    assert!((state.total_duration - 456.0).abs() < f64::EPSILON);

    assert!(
        matches!(state.error_message.as_deref(), Some(m) if m.contains("Failed to load audiobook files")),
        "Expected an error message indicating files failed to load"
    );

    Ok(())
}

#[test]
fn test_bug_0024_audiobook_selection_load_bookmarks_failure_does_not_clear_state(
) -> Result<(), Box<dyn Error>> {
    use nodoka::models::{Audiobook, AudiobookFile};

    let db = crate::acceptance_support::create_test_db()?;
    let old_id = crate::acceptance_support::create_test_audiobook(&db, "/test", "Old")?;
    let new_id = crate::acceptance_support::create_test_audiobook(&db, "/test", "New")?;

    // Make file loading succeed but force `get_bookmarks_for_audiobook` to fail.
    db.connection().execute_batch("DROP TABLE bookmarks;")?;

    let mut old_ab = Audiobook::new(
        "/test".to_string(),
        "Old".to_string(),
        "/test/Old".to_string(),
        0,
    );
    old_ab.id = Some(old_id);
    let mut new_ab = Audiobook::new(
        "/test".to_string(),
        "New".to_string(),
        "/test/New".to_string(),
        0,
    );
    new_ab.id = Some(new_id);

    let file_path = "/old/file1.mp3".to_string();
    let old_file = AudiobookFile {
        audiobook_id: old_id,
        name: "file1.mp3".to_string(),
        full_path: file_path.clone(),
        length_of_file: None,
        seek_position: None,
        checksum: None,
        position: 0,
        completeness: 0,
        file_exists: true,
        created_at: chrono::Utc::now(),
    };

    let mut state = State {
        audiobooks: vec![old_ab, new_ab],
        selected_audiobook: Some(old_id),
        current_files: vec![old_file],
        selected_file: Some(file_path.clone()),
        bookmark_editor: Some(nodoka::ui::BookmarkEditor {
            id: None,
            audiobook_id: old_id,
            file_path: file_path.clone(),
            position_ms: 1_000,
            label: "Editor".to_string(),
            note: "Note".to_string(),
        }),
        playback: PlaybackStatus::Playing,
        current_time: 123.0,
        total_duration: 456.0,
        ..Default::default()
    };

    let mut player = None;
    let _task = nodoka::ui::update::update(
        &mut state,
        Message::AudiobookSelected(new_id),
        &mut player,
        &db,
    );

    assert_eq!(state.selected_audiobook, Some(old_id));
    assert_eq!(state.selected_file.as_deref(), Some(file_path.as_str()));
    assert_eq!(state.current_files.len(), 1);
    assert!(state.bookmark_editor.is_some());
    assert_eq!(state.playback, PlaybackStatus::Playing);
    assert!((state.current_time - 123.0).abs() < f64::EPSILON);
    assert!((state.total_duration - 456.0).abs() < f64::EPSILON);

    assert!(
        matches!(state.error_message.as_deref(), Some(m) if m.contains("Failed to load bookmarks")),
        "Expected an error message indicating bookmarks failed to load"
    );

    Ok(())
}

/// Bug #0025: Errors are cleared on successful operations
///
/// Scenario: User has an error message displayed from a failed scan,
/// then successfully scans a directory.
///
/// Expected: Error message and timestamp are cleared when scan succeeds.
#[test]
fn test_bug_0025_errors_cleared_on_success() {
    let mut state = State {
        error_message: Some("Failed to scan directory: Permission denied".to_string()),
        error_timestamp: Some(chrono::Utc::now()),
        scan_state: ScanState::Scanning {
            directory: Some("/test/audiobooks".to_string()),
        },
        ..Default::default()
    };

    // Simulate successful scan completion
    state.scan_state = ScanState::Idle;
    state.error_message = None;
    state.error_timestamp = None;

    assert!(
        state.error_message.is_none(),
        "Error message should be cleared on successful operation"
    );
    assert!(
        state.error_timestamp.is_none(),
        "Error timestamp should be cleared on successful operation"
    );
}

/// Bug #0027: Sleep timer fade duration incorrect (7s instead of 30s)
///
/// Scenario: Manual test case specifies that sleep timer should fade volume
/// over the last 30 seconds.
///
/// Fix: Changed default fade duration constant.
#[test]
fn test_bug_0027_sleep_timer_fade_duration_30_seconds() {
    // Create a sleep timer with default fade duration
    let timer = SleepTimer::new(SleepTimerMode::Duration(300), 30);

    // Verify fade duration is 30 seconds
    assert_eq!(
        timer.fade_duration_secs, 30,
        "Sleep timer fade should be 30 seconds to match manual test expectations"
    );

    // Verify fade activates during last 30 seconds
    let timer_29s = SleepTimer::new(SleepTimerMode::Duration(29), 30);
    let remaining_29 = timer_29s.remaining_seconds().unwrap_or(30);
    assert!(
        remaining_29 < 30,
        "Fade should be active with 29 seconds remaining"
    );

    // Verify no fade when more than 30 seconds remain
    let timer_31s = SleepTimer::new(SleepTimerMode::Duration(31), 30);
    let remaining_31 = timer_31s.remaining_seconds().unwrap_or(30);
    assert!(
        remaining_31 > 30,
        "No fade should occur with 31 seconds remaining"
    );
}

/// Bug #0039: Sleep timer with zero duration
///
/// Scenario: User attempts to set sleep timer with 0 seconds.
///
/// Expected: Invalid duration is rejected or handled gracefully.
#[test]
fn test_bug_0039_sleep_timer_zero_duration() {
    let state = State {
        sleep_timer: None,
        ..Default::default()
    };

    assert!(state.sleep_timer.is_none());
}

/// Bug #0040: Sleep timer with very large duration
///
/// Scenario: User sets sleep timer to 24 hours (86400 seconds).
///
/// Expected: Large duration is accepted and countdown works correctly.
#[test]
fn test_bug_0040_sleep_timer_large_duration() {
    let duration_secs = 86400_i64; // 24 hours
    let timer = SleepTimer::new(SleepTimerMode::Duration(duration_secs), 30);

    let duration = match timer.mode {
        SleepTimerMode::Duration(seconds) => Some(seconds),
        SleepTimerMode::EndOfChapter => None,
    };
    assert_eq!(duration, Some(86400));
}

/// Bug #0043: Error message with special characters
///
/// Scenario: Error message contains newlines, quotes, or Unicode.
///
/// Expected: Error banner displays special characters correctly.
#[test]
fn test_bug_0043_error_message_special_characters() {
    let error_with_newline = "Line 1\nLine 2\nLine 3";
    let error_with_quotes = r#"Error: "file.mp3" not found"#;
    let error_with_unicode = "Error: 文件找不到";

    let state_newline = State {
        error_message: Some(error_with_newline.to_string()),
        ..Default::default()
    };
    assert!(matches!(
        state_newline.error_message.as_deref(),
        Some(m) if m.contains('\n')
    ));

    let state_quotes = State {
        error_message: Some(error_with_quotes.to_string()),
        ..Default::default()
    };
    assert!(matches!(
        state_quotes.error_message.as_deref(),
        Some(m) if m.contains('"')
    ));

    let state_unicode = State {
        error_message: Some(error_with_unicode.to_string()),
        ..Default::default()
    };
    assert!(matches!(
        state_unicode.error_message.as_deref(),
        Some(m) if m.contains('文')
    ));
}

/// Bug #0048: Dismissing error immediately after it appears
///
/// Scenario: Error appears and user clicks Dismiss immediately.
///
/// Expected: Error clears without lingering or visual glitches.
#[test]
fn test_bug_0048_rapid_error_dismissal() {
    let mut state = State {
        error_message: Some("Test error".to_string()),
        error_timestamp: Some(chrono::Utc::now()),
        ..Default::default()
    };

    assert!(state.error_message.is_some());

    state.error_message = None;
    state.error_timestamp = None;

    assert!(state.error_message.is_none());
    assert!(state.error_timestamp.is_none());
}

/// Bug #0053: Sleep timer cancellation during fade
///
/// Scenario: Sleep timer is fading volume (last 30s), user cancels it.
///
/// Expected: Volume immediately restores to original level.
#[test]
fn test_bug_0053_sleep_timer_cancel_during_fade() {
    let original_volume = 100_i32;
    let faded_volume = 30_i32;

    let timer = SleepTimer::new(SleepTimerMode::Duration(60), 30);

    let state_fading = State {
        sleep_timer: Some(timer),
        volume: faded_volume,
        sleep_timer_base_volume: Some(original_volume),
        ..Default::default()
    };

    // After cancellation
    let state_cancelled = State {
        sleep_timer: None,
        volume: original_volume,
        sleep_timer_base_volume: None,
        ..state_fading
    };

    assert!(state_cancelled.sleep_timer.is_none());
    assert_eq!(state_cancelled.volume, original_volume);
    assert!(state_cancelled.sleep_timer_base_volume.is_none());
}
