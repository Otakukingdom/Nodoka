//! UX animation compliance tests
//!
//! Validates animation behavior, timing, and reduced motion support
//! per ui-ux-pro-max guidelines.

use nodoka::ui::styles::animations::*;
fn assert_approx_eq(actual: f32, expected: f32, tolerance: f32, context: &str) {
    assert!(
        (actual - expected).abs() <= tolerance,
        "{context}: expected {expected}, got {actual}"
    );
}

#[test]
fn test_stagger_animations_apply_correct_delays() {
    let delay_ms = 50;

    // First item should have no delay
    assert_eq!(
        stagger_delay(0, delay_ms),
        0,
        "First item should have 0ms delay"
    );

    // Second item should have 50ms delay
    assert_eq!(
        stagger_delay(1, delay_ms),
        50,
        "Second item should have 50ms delay"
    );

    // Third item should have 100ms delay
    assert_eq!(
        stagger_delay(2, delay_ms),
        100,
        "Third item should have 100ms delay"
    );

    // 15th item should have 750ms delay
    assert_eq!(
        stagger_delay(15, delay_ms),
        750,
        "15th item should have 750ms delay"
    );
}

#[test]
fn test_stagger_opacity_before_delay() {
    // When elapsed time is less than item delay
    let opacity = stagger_opacity(2, 50, 50); // Item 2 needs 100ms, only 50ms elapsed

    // Then opacity should be 0
    assert_approx_eq(
        opacity,
        0.0,
        0.001,
        "Stagger opacity should be 0 before item delay",
    );
}

#[test]
fn test_stagger_opacity_after_delay() {
    // When elapsed time exceeds item delay + animation duration
    let opacity = stagger_opacity(2, 50, 350); // Item 2 delay 100ms + 200ms animation = 300ms

    // Then opacity should be 1.0 (fully visible)
    assert_approx_eq(
        opacity,
        1.0,
        0.001,
        "Stagger opacity should be 1.0 after animation completes",
    );
}

#[test]
fn test_stagger_opacity_caps_at_15_items() {
    // Items beyond 15 should use the same delay as item 15
    let opacity_15 = stagger_opacity(15, 50, 1000);
    let opacity_20 = stagger_opacity(20, 50, 1000);

    assert_approx_eq(
        opacity_15,
        opacity_20,
        0.001,
        "Stagger should cap at 15 items to prevent excessive delays",
    );
}

#[test]
fn test_stagger_translate_y_starts_below() {
    // When animation hasn't started
    let offset = stagger_translate_y(2, 50, 50); // Item 2 needs 100ms, only 50ms elapsed

    // Then offset should be 20px below
    assert_approx_eq(
        offset,
        20.0,
        0.001,
        "Stagger translate should start 20px below final position",
    );
}

#[test]
fn test_stagger_translate_y_ends_at_zero() {
    // When animation completes
    let offset = stagger_translate_y(2, 50, 350); // Item 2 delay 100ms + 200ms animation

    // Then offset should be 0 (final position)
    assert_approx_eq(
        offset,
        0.0,
        0.001,
        "Stagger translate should end at 0 offset",
    );
}

#[test]
fn test_modal_animations_complete_within_budget() {
    use nodoka::ui::styles::animations::modal::ModalState;

    let mut modal = ModalState::Opening { elapsed_ms: 0 };

    // Opening animation should complete in 300ms
    modal.update(300);
    assert_eq!(
        modal,
        ModalState::Open,
        "Modal should be fully open after 300ms"
    );

    // Closing animation should complete in 200ms
    modal = ModalState::Closing { elapsed_ms: 0 };
    modal.update(200);
    assert_eq!(
        modal,
        ModalState::Closed,
        "Modal should be fully closed after 200ms"
    );
}

#[test]
fn test_modal_backdrop_opacity_animation() {
    use nodoka::ui::styles::animations::modal::ModalState;

    // Closed state
    let modal = ModalState::Closed;
    assert_approx_eq(
        modal.backdrop_opacity(),
        0.0,
        0.001,
        "Backdrop opacity should be 0 when closed",
    );

    // Opening state at 50% progress
    let modal = ModalState::Opening { elapsed_ms: 150 };
    let opacity = modal.backdrop_opacity();
    assert!(
        opacity > 0.0 && opacity < 0.5,
        "Backdrop opacity should be between 0 and 0.5 during opening"
    );

    // Fully open
    let modal = ModalState::Open;
    assert_approx_eq(
        modal.backdrop_opacity(),
        0.5,
        0.001,
        "Backdrop opacity should be 0.5 when fully open",
    );
}

#[test]
fn test_modal_content_scale_animation() {
    use nodoka::ui::styles::animations::modal::ModalState;

    // Closed/Closing state
    let modal = ModalState::Closed;
    assert_approx_eq(
        modal.content_scale(),
        0.9,
        0.001,
        "Content scale should be 0.9 when closed",
    );

    // Opening state at 50% progress
    let modal = ModalState::Opening { elapsed_ms: 150 };
    let scale = modal.content_scale();
    assert!(
        scale > 0.9 && scale < 1.0,
        "Content scale should be between 0.9 and 1.0 during opening"
    );

    // Fully open
    let modal = ModalState::Open;
    assert_approx_eq(
        modal.content_scale(),
        1.0,
        0.001,
        "Content scale should be 1.0 when fully open",
    );
}

#[test]
fn test_scale_transition_maintains_bounds() {
    let mut scale = ScaleTransition::new(200, 0.95, 1.0);

    // Initial state
    assert_approx_eq(
        scale.scale(),
        0.95,
        0.001,
        "Scale should start at from value",
    );

    // Mid-transition
    scale.update(100);
    let mid_scale = scale.scale();
    assert!(
        (0.95..=1.0).contains(&mid_scale),
        "Mid-transition scale {mid_scale} out of bounds"
    );

    // Completed
    scale.update(100);
    assert_approx_eq(scale.scale(), 1.0, 0.001, "Scale should end at to value");
}

#[test]
fn test_fade_transition_bounds() {
    let mut fade = FadeTransition::new(200, 0.0, 1.0);

    // Initial state
    assert_approx_eq(
        fade.opacity(),
        0.0,
        0.001,
        "Opacity should start at from value",
    );

    // Throughout transition, opacity should stay in bounds
    for _ in 0..20 {
        fade.update(10);
        let opacity = fade.opacity();
        assert!(
            (0.0..=1.0).contains(&opacity),
            "Pulse opacity {opacity} out of bounds"
        );
    }
}

#[test]
fn test_pulse_animation_oscillation() {
    let mut pulse = PulseAnimation::new(1000);

    // Collect opacity values over one full cycle
    let mut opacities = Vec::new();
    for _ in 0..100 {
        pulse.update(10); // Total 1000ms
        opacities.push(pulse.opacity());
    }

    // Should oscillate - have values both above and below 0.5
    let has_low = opacities.iter().any(|&o| o < 0.4);
    let has_high = opacities.iter().any(|&o| o > 0.6);

    assert!(
        has_low && has_high,
        "Pulse animation should oscillate between low and high values"
    );
}

#[test]
fn test_easing_functions_mathematical_correctness() {
    use nodoka::ui::styles::animations::easing;

    // Linear easing
    assert_approx_eq(easing::linear(0.0), 0.0, 0.001, "Linear at 0");
    assert_approx_eq(easing::linear(0.5), 0.5, 0.001, "Linear at 0.5");
    assert_approx_eq(easing::linear(1.0), 1.0, 0.001, "Linear at 1.0");

    // Ease-in at t=0 and t=1
    assert_approx_eq(easing::ease_in(0.0), 0.0, 0.001, "Ease-in at 0");
    assert_approx_eq(easing::ease_in(1.0), 1.0, 0.001, "Ease-in at 1.0");

    // Ease-out at t=0 and t=1
    assert_approx_eq(easing::ease_out(0.0), 0.0, 0.001, "Ease-out at 0");
    assert_approx_eq(easing::ease_out(1.0), 1.0, 0.001, "Ease-out at 1.0");

    // Ease-in-out at t=0, t=0.5, t=1
    assert_approx_eq(easing::ease_in_out(0.0), 0.0, 0.001, "Ease-in-out at 0");
    assert_approx_eq(easing::ease_in_out(1.0), 1.0, 0.001, "Ease-in-out at 1.0");
    let mid = easing::ease_in_out(0.5);
    assert!(
        mid > 0.4 && mid < 0.6,
        "Ease-in-out at 0.5 should be near 0.5"
    );

    // Bounce should overshoot then settle
    let bounce = easing::ease_out_bounce(0.95);
    assert!(bounce >= 0.95, "Bounce should reach target");

    // Elastic should oscillate then settle
    let elastic = easing::ease_out_elastic(1.0);
    assert_approx_eq(elastic, 1.0, 0.01, "Elastic should settle at target");
}

#[test]
fn test_all_animations_respect_reduced_motion() {
    // Fade with 0 duration (reduced motion equivalent)
    let fade = FadeTransition::new(0, 0.0, 1.0);
    assert_approx_eq(
        fade.opacity(),
        1.0,
        0.001,
        "Fade with 0 duration should be instant",
    );

    // Scale with 0 duration
    let scale = ScaleTransition::new(0, 0.9, 1.0);
    assert_approx_eq(
        scale.scale(),
        1.0,
        0.001,
        "Scale with 0 duration should be instant",
    );

    // Pulse with 0 duration
    let mut pulse = PulseAnimation::new(0);
    pulse.update(100);
    // Should not crash and remain stable
    let _ = pulse.opacity();
}
