//! Integration tests for player controls UI
//!
//! These tests verify that the player controls component behaves correctly
//! with various state configurations, focusing on the speed and sleep timer
//! controls that are part of the UI usability improvements.

use nodoka::models::{SleepTimer, SleepTimerMode};
use nodoka::ui::components::player_controls;
use nodoka::ui::state::State;

#[test]
fn test_player_controls_renders_with_default_state() {
    let state = State::default();
    let element = player_controls::view(&state);
    drop(element); // Verify it doesn't panic
}

#[test]
fn test_player_controls_renders_with_playing_status() {
    let state = State {
        playback: nodoka::ui::PlaybackStatus::Playing,
        current_time: 30_000.0,    // 30 seconds
        total_duration: 120_000.0, // 2 minutes
        ..Default::default()
    };
    let element = player_controls::view(&state);
    drop(element);
}

#[test]
fn test_player_controls_renders_with_paused_status() {
    let state = State {
        playback: nodoka::ui::PlaybackStatus::Paused,
        current_time: 60_000.0,    // 1 minute
        total_duration: 180_000.0, // 3 minutes
        ..Default::default()
    };
    let element = player_controls::view(&state);
    drop(element);
}

#[test]
fn test_speed_controls_available_with_all_presets() {
    // The player controls should support speed presets: 0.5x, 0.75x, 1.0x, 1.25x, 1.5x, 2.0x
    // We test with each preset to ensure rendering works

    for speed in [0.5, 0.75, 1.0, 1.25, 1.5, 2.0] {
        let state = State {
            speed,
            ..Default::default()
        };
        let element = player_controls::view(&state);
        drop(element);
    }
}

#[test]
fn test_speed_controls_with_custom_speed() {
    // Test with a speed that's not a preset
    let state = State {
        speed: 1.35,
        ..Default::default()
    };
    let element = player_controls::view(&state);
    drop(element);
}

#[test]
fn test_speed_controls_with_extreme_values() {
    // Test boundary values
    let state1 = State {
        speed: 0.5, // Minimum
        ..Default::default()
    };
    let element = player_controls::view(&state1);
    drop(element);

    let state2 = State {
        speed: 2.0, // Maximum
        ..Default::default()
    };
    let element = player_controls::view(&state2);
    drop(element);
}

#[test]
fn test_sleep_timer_controls_when_not_set() {
    let state = State {
        sleep_timer: None,
        ..Default::default()
    };
    let element = player_controls::view(&state);
    drop(element);
}

#[test]
fn test_sleep_timer_controls_with_duration_mode() {
    let timer = SleepTimer::new(SleepTimerMode::Duration(1800), 5); // 30 minutes, 5 sec fade
    let state = State {
        sleep_timer: Some(timer),
        ..Default::default()
    };
    let element = player_controls::view(&state);
    drop(element);
}

#[test]
fn test_sleep_timer_controls_with_end_of_chapter_mode() {
    let timer = SleepTimer::new(SleepTimerMode::EndOfChapter, 5); // 5 sec fade
    let state = State {
        sleep_timer: Some(timer),
        ..Default::default()
    };
    let element = player_controls::view(&state);
    drop(element);
}

#[test]
fn test_sleep_timer_controls_with_custom_input() {
    let state = State {
        sleep_timer: None,
        sleep_timer_custom_minutes: "45".to_string(),
        ..Default::default()
    };
    let element = player_controls::view(&state);
    drop(element);
}

#[test]
fn test_sleep_timer_controls_with_custom_error() {
    let state = State {
        sleep_timer: None,
        sleep_timer_custom_minutes: "invalid".to_string(),
        sleep_timer_custom_error: Some("Invalid input".to_string()),
        ..Default::default()
    };
    let element = player_controls::view(&state);
    drop(element);
}

#[test]
fn test_volume_controls_at_various_levels() {
    // Test different volume levels: muted, low, medium, high, max
    for volume in [0, 30, 100, 150, 200] {
        let state = State {
            volume,
            ..Default::default()
        };
        let element = player_controls::view(&state);
        drop(element);
    }
}

#[test]
fn test_progress_slider_with_zero_duration() {
    let state = State {
        current_time: 0.0,
        total_duration: 0.0,
        ..Default::default()
    };
    let element = player_controls::view(&state);
    drop(element);
}

#[test]
fn test_progress_slider_at_end() {
    let state = State {
        current_time: 120_000.0,
        total_duration: 120_000.0,
        ..Default::default()
    };
    let element = player_controls::view(&state);
    drop(element);
}

#[test]
fn test_progress_slider_exceeding_duration() {
    // Edge case: current time exceeds total duration (shouldn't happen but test robustness)
    let state = State {
        current_time: 150_000.0,
        total_duration: 120_000.0,
        ..Default::default()
    };
    let element = player_controls::view(&state);
    drop(element);
}

#[test]
fn test_player_controls_with_no_file_selected() {
    let state = State {
        selected_file: None,
        ..Default::default()
    };
    let element = player_controls::view(&state);
    drop(element);
}

#[test]
fn test_player_controls_with_file_selected() {
    let state = State {
        selected_file: Some("/path/to/audiobook/chapter1.mp3".to_string()),
        ..Default::default()
    };
    let element = player_controls::view(&state);
    drop(element);
}

#[test]
fn test_player_controls_with_focused_elements() {
    use nodoka::ui::FocusedElement;

    // Test with different focused elements
    for focused in [
        FocusedElement::PlayPauseButton,
        FocusedElement::StopButton,
        FocusedElement::SpeedSlider,
        FocusedElement::VolumeSlider,
        FocusedElement::ProgressSlider,
    ] {
        let state = State {
            focused_element: focused,
            ..Default::default()
        };
        let element = player_controls::view(&state);
        drop(element);
    }
}

#[test]
fn test_player_controls_with_reduced_motion() {
    let state = State {
        prefers_reduced_motion: true,
        ..Default::default()
    };
    let element = player_controls::view(&state);
    drop(element);
}
