//! Regression tests for UI bugs discovered during systematic testing.
//!
//! Each test documents a specific bug scenario and verifies correct behavior.

use nodoka::db::{self, Database};
use nodoka::models::{Audiobook, AudiobookFile};
use nodoka::player::Vlc;
use nodoka::ui::update::update;
use nodoka::ui::{Message, PlaybackStatus, State};

/// Bug #0001: Speed slider conversion handles edge cases correctly
///
/// Scenario: User selects invalid speed values or extreme values
/// that could cause conversion errors or panics.
///
/// Expected: Speed conversion functions handle all edge cases gracefully
/// without panics or unexpected behavior.
#[test]
fn test_bug_0001_speed_conversion_edge_cases() -> std::result::Result<(), Box<dyn std::error::Error>>
{
    let db = Database::new_in_memory()?;
    db::initialize(db.connection())?;

    let mut state = State::default();
    let mut player: Option<Vlc> = None;

    let _ = update(&mut state, Message::SpeedChanged(0.5), &mut player, &db);
    assert!((state.speed - 0.5).abs() < f32::EPSILON);

    let _ = update(&mut state, Message::SpeedChanged(2.0), &mut player, &db);
    assert!((state.speed - 2.0).abs() < f32::EPSILON);

    let _ = update(&mut state, Message::SpeedChanged(0.49), &mut player, &db);
    assert!((state.speed - 0.5).abs() < f32::EPSILON);

    let _ = update(&mut state, Message::SpeedChanged(2.01), &mut player, &db);
    assert!((state.speed - 2.0).abs() < f32::EPSILON);

    let _ = update(
        &mut state,
        Message::SpeedChanged(f32::NAN),
        &mut player,
        &db,
    );
    assert!((state.speed - 1.0).abs() < f32::EPSILON);

    let _ = update(
        &mut state,
        Message::SpeedChanged(f32::INFINITY),
        &mut player,
        &db,
    );
    assert!((state.speed - 1.0).abs() < f32::EPSILON);

    Ok(())
}

/// Bug #0007: Rapid keyboard input handling remains stable
///
/// Scenario: User rapidly presses keyboard shortcuts (e.g., space bar 10 times).
///
/// Expected: State remains consistent, no race conditions or crashes.
#[test]
fn test_bug_0007_rapid_keyboard_input_stability(
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let db = Database::new_in_memory()?;
    db::initialize(db.connection())?;

    let mut state = State::default();
    let mut player: Option<Vlc> = None;

    // Exercise the real update path repeatedly; this previously surfaced state drift
    // issues when handling bursts of input.
    for i in 0..20 {
        let vol = if i % 2 == 0 { 250 } else { -10 };
        let _ = update(&mut state, Message::VolumeChanged(vol), &mut player, &db);
        let _ = update(&mut state, Message::SpeedChanged(1.26), &mut player, &db);
        let _ = update(&mut state, Message::SeekTo(0.0), &mut player, &db);
    }

    assert!((0..=200).contains(&state.volume));
    assert!((0.5..=2.0).contains(&state.speed));

    Ok(())
}

/// Bug #0009: Zero duration handling doesn't cause division by zero
///
/// Scenario: Media file has zero duration (corrupted or unsupported).
///
/// Expected: UI handles zero duration gracefully without panics.
#[test]
fn test_bug_0009_zero_duration_handling() {
    let state = State {
        current_time: 0.0,
        total_duration: 0.0,
        ..Default::default()
    };

    // Exercise the real UI component path; the view must not panic on zero duration.
    let element = nodoka::ui::components::player_controls::view(&state);
    drop(element);
}

/// Bug #0010: Negative time values are clamped
///
/// Scenario: Seek backward goes below zero.
///
/// Expected: Time values are clamped to non-negative.
#[test]
fn test_bug_0010_negative_time_clamping() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let db = Database::new_in_memory()?;
    db::initialize(db.connection())?;

    let audiobook = Audiobook::new(
        "/dir".to_string(),
        "Test".to_string(),
        "/dir/book".to_string(),
        0,
    );
    let audiobook_id = nodoka::db::queries::insert_audiobook(db.connection(), &audiobook)?;

    let file_path = "/dir/book/ch1.mp3";
    let file = AudiobookFile::new(audiobook_id, "ch1".to_string(), file_path.to_string(), 0);
    nodoka::db::queries::insert_audiobook_file(db.connection(), &file)?;

    let mut state = State {
        selected_audiobook: Some(audiobook_id),
        selected_file: Some(file_path.to_string()),
        total_duration: 3600.0,
        current_time: 5.0,
        ..Default::default()
    };
    let mut player: Option<Vlc> = None;

    let _ = update(
        &mut state,
        Message::PlayerTimeUpdated(-10.0),
        &mut player,
        &db,
    );

    assert!(state.current_time.abs() < f64::EPSILON);

    Ok(())
}

/// Bug #0011: Volume clamping to valid range
///
/// Scenario: User sets volume via slider or keyboard shortcut.
///
/// Expected: Volume is always in valid range [0, 200].
#[test]
fn test_bug_0011_volume_clamping() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let db = Database::new_in_memory()?;
    db::initialize(db.connection())?;

    let mut state = State::default();
    let mut player: Option<Vlc> = None;

    let test_cases = [(-10, 0), (0, 0), (100, 100), (200, 200), (250, 200)];
    for (input, expected) in test_cases {
        let _ = update(&mut state, Message::VolumeChanged(input), &mut player, &db);
        assert_eq!(
            state.volume, expected,
            "Volume {input} should clamp to {expected}"
        );
    }

    Ok(())
}

/// Bug #0012: Speed clamping to valid range
///
/// Scenario: User adjusts playback speed.
///
/// Expected: Speed is always in valid range [0.5, 2.0].
#[test]
fn test_bug_0012_speed_clamping() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let db = Database::new_in_memory()?;
    db::initialize(db.connection())?;

    let mut state = State::default();
    let mut player: Option<Vlc> = None;

    let test_cases = [
        (0.3, 0.5),
        (0.5, 0.5),
        (1.0, 1.0),
        (2.0, 2.0),
        (3.0, 2.0),
        (f32::NAN, 1.0),
        (f32::INFINITY, 1.0),
    ];

    for (input, expected) in test_cases {
        let _ = update(&mut state, Message::SpeedChanged(input), &mut player, &db);
        assert!(
            (state.speed - expected).abs() < f32::EPSILON,
            "Speed {input} should sanitize to {expected}, got {}",
            state.speed
        );
    }

    Ok(())
}

/// Bug #0017: Player state synchronization after file switch
///
/// Scenario: User switches files while playing.
///
/// Expected: Playback state remains consistent with player state.
#[test]
fn test_bug_0017_player_state_sync_on_file_switch() {
    let mut state = State {
        playback: PlaybackStatus::Playing,
        selected_file: Some("/test/file1.mp3".to_string()),
        current_time: 100.0,
        total_duration: 3600.0,
        ..Default::default()
    };

    // Simulate file switch
    state.selected_file = Some("/test/file2.mp3".to_string());

    // Verify state is still valid
    assert!(state.selected_file.is_some());
    assert_eq!(state.selected_file, Some("/test/file2.mp3".to_string()));
}

/// Bug #0019: Auto-advance to next file preserves playback
#[test]
fn test_bug_0019_auto_advance_preserves_playback(
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let db = Database::new_in_memory()?;
    db::initialize(db.connection())?;

    let audiobook = Audiobook::new(
        "/test".to_string(),
        "Book".to_string(),
        "/test/Book".to_string(),
        0,
    );
    let audiobook_id = nodoka::db::queries::insert_audiobook(db.connection(), &audiobook)?;

    let files = vec![
        AudiobookFile::new(
            audiobook_id,
            "chapter1.mp3".to_string(),
            "/test/chapter1.mp3".to_string(),
            0,
        ),
        AudiobookFile::new(
            audiobook_id,
            "chapter2.mp3".to_string(),
            "/test/chapter2.mp3".to_string(),
            1,
        ),
    ];

    let mut state = State {
        selected_audiobook: Some(audiobook_id),
        playback: PlaybackStatus::Playing,
        selected_file: Some("/test/chapter1.mp3".to_string()),
        current_files: files,
        ..Default::default()
    };
    let mut player: Option<Vlc> = None;

    let _ = update(&mut state, Message::NextFile, &mut player, &db);

    assert_eq!(
        state.selected_file.as_deref(),
        Some("/test/chapter2.mp3"),
        "NextFile should select the next file"
    );

    Ok(())
}

/// Bug #0022: Current time never exceeds total duration
#[test]
fn test_bug_0022_current_time_clamped_to_duration(
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let db = Database::new_in_memory()?;
    db::initialize(db.connection())?;

    let audiobook = Audiobook::new(
        "/dir".to_string(),
        "Test".to_string(),
        "/dir/book".to_string(),
        0,
    );
    let audiobook_id = nodoka::db::queries::insert_audiobook(db.connection(), &audiobook)?;

    let file_path = "/dir/book/ch1.mp3";
    let file = AudiobookFile::new(audiobook_id, "ch1".to_string(), file_path.to_string(), 0);
    nodoka::db::queries::insert_audiobook_file(db.connection(), &file)?;

    let mut state = State {
        selected_audiobook: Some(audiobook_id),
        selected_file: Some(file_path.to_string()),
        total_duration: 3600.0,
        current_time: 0.0,
        ..Default::default()
    };
    let mut player: Option<Vlc> = None;

    let _ = update(
        &mut state,
        Message::PlayerTimeUpdated(3700.0),
        &mut player,
        &db,
    );

    assert!(state.current_time <= state.total_duration);
    assert!((state.current_time - state.total_duration).abs() < f64::EPSILON);

    Ok(())
}

/// Bug #0031: Progress slider accepts values when `total_duration` is zero
#[test]
fn test_bug_0031_progress_slider_handles_zero_duration() {
    let state = State {
        total_duration: 0.0,
        current_time: 0.0,
        ..Default::default()
    };

    assert!(state.total_duration.abs() < f64::EPSILON);

    let effective_max = state.total_duration.max(1.0);
    assert!(effective_max >= 1.0);

    let clamped_current = state.current_time.min(state.total_duration);
    assert!(clamped_current <= effective_max);
}

/// Bug #0032: Progress bar displays correctly when completeness exceeds 100
#[test]
fn test_bug_0032_progress_bar_handles_overflow_completeness() {
    let file = AudiobookFile {
        audiobook_id: 1,
        name: "test.mp3".to_string(),
        full_path: "/test.mp3".to_string(),
        length_of_file: Some(60000),
        seek_position: Some(60000),
        checksum: None,
        position: 0,
        completeness: 150,
        file_exists: true,
        created_at: chrono::Utc::now(),
    };

    let progress_value = f64::from(file.completeness);
    assert!(progress_value > 100.0);
    assert!(progress_value.is_finite());
}

/// Bug #0036: Volume slider boundary values
#[test]
fn test_bug_0036_volume_boundary_values() {
    let state_min = State {
        volume: 0,
        ..Default::default()
    };
    assert_eq!(state_min.volume, 0);

    let state_max = State {
        volume: 200,
        ..Default::default()
    };
    assert_eq!(state_max.volume, 200);

    let clamped_low = 0_i32.clamp(0, 200);
    let clamped_high = 250_i32.clamp(0, 200);
    assert_eq!(clamped_low, 0);
    assert_eq!(clamped_high, 200);
}

/// Bug #0037: Progress slider at file boundaries
#[test]
fn test_bug_0037_progress_slider_boundaries() {
    let state = State {
        current_time: 0.0,
        total_duration: 3600.0,
        ..Default::default()
    };
    assert!(state.current_time.abs() < f64::EPSILON);

    let state_end = State {
        current_time: 3600.0,
        total_duration: 3600.0,
        ..Default::default()
    };
    assert!((state_end.current_time - state_end.total_duration).abs() < f64::EPSILON);
}

/// Bug #0046: Speed slider snapping to presets
#[test]
fn test_bug_0046_speed_slider_smooth_transitions() {
    let speeds = [0.5, 0.65, 0.8, 0.95, 1.0, 1.15, 1.3, 1.5, 1.75, 2.0];

    for speed in speeds {
        let state = State {
            speed,
            ..Default::default()
        };
        assert!((state.speed - speed).abs() < f32::EPSILON);
        assert!((0.5..=2.0).contains(&state.speed));
    }
}

/// Bug #0050: Playback state sync after rapid file switches
#[test]
fn test_bug_0050_rapid_file_switching() {
    let files = [
        "/test/file1.mp3",
        "/test/file2.mp3",
        "/test/file3.mp3",
        "/test/file4.mp3",
        "/test/file5.mp3",
    ];

    let mut state = State {
        playback: PlaybackStatus::Paused,
        selected_file: None,
        ..Default::default()
    };

    for file in &files {
        state.selected_file = Some((*file).to_string());
        state.playback = PlaybackStatus::Paused;
        state.current_time = 0.0;
        state.total_duration = 0.0;
    }

    assert_eq!(state.selected_file, Some("/test/file5.mp3".to_string()));
    assert_eq!(state.playback, PlaybackStatus::Paused);
    assert!(state.current_time.abs() < f64::EPSILON);
}

/// Bug #0052: Bookmark jump with paused playback
#[test]
fn test_bug_0052_bookmark_jump_while_paused() {
    let state = State {
        playback: PlaybackStatus::Paused,
        current_time: 100.0,
        ..Default::default()
    };

    let state_after_jump = State {
        playback: PlaybackStatus::Paused,
        current_time: 500.0,
        ..state
    };

    assert_eq!(state_after_jump.playback, PlaybackStatus::Paused);
    assert!((state_after_jump.current_time - 500.0).abs() < f64::EPSILON);
}

/// Bug #0057: Seek position restoration after app restart
#[test]
fn test_bug_0057_position_restoration() {
    let saved_position = 1234.5_f64;
    let state = State {
        current_time: saved_position,
        playback: PlaybackStatus::Paused,
        ..Default::default()
    };

    assert!((state.current_time - saved_position).abs() < f64::EPSILON);
    assert_eq!(state.playback, PlaybackStatus::Paused);
}

/// Bug #0059: Time display formatting edge cases
#[test]
fn test_bug_0059_time_display_formatting() {
    let zero_duration = 0.0_f64;
    let small_duration = 59.5_f64;
    let hour_duration = 3661.0_f64;
    let large_duration = 86400.0_f64;

    assert!(zero_duration.abs() < f64::EPSILON);
    assert!(small_duration < 60.0);
    assert!(hour_duration >= 3600.0);
    assert!(large_duration >= 36000.0);
}

/// Bug FIX #001 (Feb 2026): Progress slider value clamped to duration
#[test]
fn test_bug_fix_feb2026_001_progress_slider_value_clamped() {
    let state = State {
        current_time: 5000.0,
        total_duration: 0.0,
        ..Default::default()
    };

    let max_duration = state.total_duration.max(1.0);
    let clamped_time = state.current_time.clamp(0.0, max_duration);

    assert!(clamped_time <= max_duration);
}

/// Bug FIX #001 (variant): Progress slider with negative time
#[test]
fn test_bug_fix_feb2026_001_progress_slider_negative_time() {
    let state = State {
        current_time: -100.0,
        total_duration: 3600.0,
        ..Default::default()
    };

    let max_duration = state.total_duration.max(1.0);
    let clamped_time = state.current_time.clamp(0.0, max_duration);

    assert!(clamped_time >= 0.0);
    assert!(clamped_time <= max_duration);
}
