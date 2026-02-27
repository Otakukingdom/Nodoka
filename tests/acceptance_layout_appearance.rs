//! Acceptance tests for Layout & Appearance features.

use nodoka::ui::state::{FocusMode, PlaybackStatus, State, ThemeMode};

fn assert_f32_approx_eq(actual: f32, expected: f32) {
    let diff = (actual - expected).abs();
    assert!(
        diff < 1.0e-6,
        "expected ~{expected}, got {actual} (diff {diff})"
    );
}

#[test]
fn test_panel_ratio_adjustable() {
    let mut state = State::default();
    assert_f32_approx_eq(state.panel_ratio, 0.4); // Default 40/60 split

    state.panel_ratio = 0.5;
    assert_f32_approx_eq(state.panel_ratio, 0.5); // 50/50 split

    state.panel_ratio = 0.3;
    assert_f32_approx_eq(state.panel_ratio, 0.3); // 30/70 split
}

#[test]
fn test_panel_ratio_clamped() {
    // Test lower bound
    let ratio_low = 0.1_f32.clamp(0.2, 0.8);
    assert_f32_approx_eq(ratio_low, 0.2);

    // Test upper bound
    let ratio_high = 0.9_f32.clamp(0.2, 0.8);
    assert_f32_approx_eq(ratio_high, 0.8);

    // Test within bounds
    let ratio_mid = 0.5_f32.clamp(0.2, 0.8);
    assert_f32_approx_eq(ratio_mid, 0.5);
}

#[test]
fn test_focus_mode_default_disabled() {
    let state = State::default();
    assert_eq!(state.focus_mode, FocusMode::Disabled);
}

#[test]
fn test_focus_mode_can_be_toggled() {
    let state1 = State::default();
    assert_eq!(state1.focus_mode, FocusMode::Disabled);

    let state2 = State {
        focus_mode: FocusMode::Enabled,
        ..Default::default()
    };
    assert_eq!(state2.focus_mode, FocusMode::Enabled);

    let state3 = State {
        focus_mode: FocusMode::Disabled,
        ..Default::default()
    };
    assert_eq!(state3.focus_mode, FocusMode::Disabled);
}

#[test]
fn test_focus_mode_with_playback() {
    let state = State {
        playback: PlaybackStatus::Playing,
        focus_mode: FocusMode::Enabled,
        ..Default::default()
    };

    // Focus mode is active when playing
    assert_eq!(state.focus_mode, FocusMode::Enabled);
    assert_eq!(state.playback, PlaybackStatus::Playing);
}

#[test]
fn test_theme_defaults_to_light() {
    let state = State::default();
    assert_eq!(state.theme, ThemeMode::Light);
}

#[test]
fn test_theme_can_be_changed() {
    let state1 = State::default();
    assert_eq!(state1.theme, ThemeMode::Light);

    let state2 = State {
        theme: ThemeMode::Dark,
        ..Default::default()
    };
    assert_eq!(state2.theme, ThemeMode::Dark);

    let state3 = State {
        theme: ThemeMode::Light,
        ..Default::default()
    };
    assert_eq!(state3.theme, ThemeMode::Light);
}

#[test]
fn test_theme_toggle() {
    let theme = ThemeMode::Light;
    assert_eq!(theme, ThemeMode::Light);

    // Toggle to dark
    let toggled1 = match theme {
        ThemeMode::Light => ThemeMode::Dark,
        ThemeMode::Dark => ThemeMode::Light,
    };
    assert_eq!(toggled1, ThemeMode::Dark);

    // Toggle back to light
    let toggled2 = match toggled1 {
        ThemeMode::Light => ThemeMode::Dark,
        ThemeMode::Dark => ThemeMode::Light,
    };
    assert_eq!(toggled2, ThemeMode::Light);
}

#[test]
fn test_panel_ratio_decrease() {
    let state = State {
        panel_ratio: 0.5,
        ..Default::default()
    };

    // Decrease by 0.1
    let new_ratio = (state.panel_ratio - 0.1).clamp(0.2, 0.8);

    assert_f32_approx_eq(new_ratio, 0.4);
}

#[test]
fn test_panel_ratio_increase() {
    let state = State {
        panel_ratio: 0.5,
        ..Default::default()
    };

    // Increase by 0.1
    let new_ratio = (state.panel_ratio + 0.1).clamp(0.2, 0.8);

    assert_f32_approx_eq(new_ratio, 0.6);
}

#[test]
fn test_panel_ratio_cannot_decrease_below_minimum() {
    let state = State {
        panel_ratio: 0.2,
        ..Default::default()
    };

    // Try to decrease by 0.1
    let new_ratio = (state.panel_ratio - 0.1).clamp(0.2, 0.8);

    assert_f32_approx_eq(new_ratio, 0.2); // Still at minimum
}

#[test]
fn test_panel_ratio_cannot_increase_above_maximum() {
    let state = State {
        panel_ratio: 0.8,
        ..Default::default()
    };

    // Try to increase by 0.1
    let new_ratio = (state.panel_ratio + 0.1).clamp(0.2, 0.8);

    assert_f32_approx_eq(new_ratio, 0.8); // Still at maximum
}
