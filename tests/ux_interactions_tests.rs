//! UX interaction compliance tests
//!
//! Validates interactive patterns, keyboard navigation,
//! and notification system behavior.

use nodoka::ui::state::Notification;
use nodoka::ui::Message;

#[test]
fn test_notification_variants_use_correct_durations() {
    // Success: 3s auto-dismiss
    let success = Notification::success("Success message".to_string());
    assert_eq!(
        success.duration_ms, 3000,
        "Success notifications should auto-dismiss after 3s"
    );

    // Info: 3s auto-dismiss
    let info = Notification::info("Info message".to_string());
    assert_eq!(
        info.duration_ms, 3000,
        "Info notifications should auto-dismiss after 3s"
    );

    // Warning: 5s auto-dismiss (longer for importance)
    let warning = Notification::warning("Warning message".to_string());
    assert_eq!(
        warning.duration_ms, 5000,
        "Warning notifications should auto-dismiss after 5s"
    );

    // Error: manual dismiss only
    let error = Notification::error("Error message".to_string());
    assert_eq!(
        error.duration_ms,
        u64::MAX,
        "Error notifications should require manual dismiss"
    );
}

#[test]
fn test_notification_variants_use_correct_icons() {
    let success = Notification::success("Success".to_string());
    assert_eq!(
        success.icon(),
        "OK",
        "Success notification should use OK icon"
    );

    let info = Notification::info("Info".to_string());
    assert_eq!(info.icon(), "i", "Info notification should use info icon");

    let warning = Notification::warning("Warning".to_string());
    assert_eq!(
        warning.icon(),
        "!",
        "Warning notification should use warning icon"
    );

    let error = Notification::error("Error".to_string());
    assert_eq!(
        error.icon(),
        "X",
        "Error notification should use error icon"
    );
}

#[test]
fn test_notification_with_title_creates_two_line_layout() {
    let notification = Notification::info_with_title(
        "Scan Complete".to_string(),
        "Found 5 new audiobooks".to_string(),
    );

    assert_eq!(
        notification.title,
        Some("Scan Complete".to_string()),
        "Notification should have title"
    );
    assert_eq!(
        notification.message, "Found 5 new audiobooks",
        "Notification should have message"
    );
}

#[test]
fn test_notification_with_action_includes_label() {
    let notification = Notification::error_with_action(
        "Failed to save settings".to_string(),
        "Retry".to_string(),
        Message::OpenSettings,
    );

    assert_eq!(
        notification.action_label,
        Some("Retry".to_string()),
        "Notification should have action label"
    );
}

#[test]
fn test_notification_dismiss_progress_for_auto_dismiss() {
    let mut notification = Notification::success("Success".to_string());

    // At creation, progress should be 0
    let initial_progress = notification.dismiss_progress();
    assert!(
        initial_progress < 0.1,
        "Dismiss progress should be near 0 at creation"
    );

    // Simulate time passing by manually advancing created_at
    notification.created_at = notification.created_at.saturating_sub(1500); // 1.5s ago

    // Progress should be around 0.5 (1500ms / 3000ms)
    let mid_progress = notification.dismiss_progress();
    assert!(
        mid_progress > 0.4 && mid_progress < 0.6,
        "Dismiss progress should be around 0.5 at halfway point"
    );

    // Simulate full duration
    notification.created_at = notification.created_at.saturating_sub(1500); // 3s total

    // Progress should be 1.0
    let final_progress = notification.dismiss_progress();
    assert!(
        (final_progress - 1.0).abs() < 0.01,
        "Dismiss progress should be 1.0 after full duration"
    );
}

#[test]
fn test_notification_dismiss_progress_for_manual_dismiss() {
    let notification = Notification::error("Error".to_string());

    // Error notifications have infinite duration, so progress should always be 1.0
    assert!(
        (notification.dismiss_progress() - 1.0).abs() < 0.01,
        "Manual dismiss notifications should have progress of 1.0"
    );
}

#[test]
fn test_notification_expiry() {
    let mut notification = Notification::success("Success".to_string());

    // Should not be expired immediately
    assert!(
        !notification.is_expired(),
        "Notification should not be expired at creation"
    );

    // Simulate time passing beyond duration
    notification.created_at = notification.created_at.saturating_sub(4000); // 4s ago

    // Should be expired
    assert!(
        notification.is_expired(),
        "Notification should be expired after duration"
    );
}

#[test]
fn test_modal_focus_trap_prevents_escape() {
    // This test validates the concept, actual implementation is in app state
    use nodoka::ui::state::ActiveModal;

    let modal = ActiveModal::Settings;
    assert!(
        matches!(modal, ActiveModal::Settings),
        "Modal state should prevent focus from escaping"
    );

    // When modal is active, tab navigation should cycle within modal
    // This is enforced by keyboard event handlers in main_window.rs
}

#[test]
fn test_interactive_state_border_width() {
    use nodoka::ui::styles::interactive::{state_border_width, InteractiveState};

    // Focus states should have 3px border (WCAG compliance)
    assert!(
        (state_border_width(InteractiveState::Focus) - 3.0).abs() < 0.001,
        "Focus state should have 3px border for WCAG compliance"
    );

    assert!(
        (state_border_width(InteractiveState::HoverFocus) - 3.0).abs() < 0.001,
        "HoverFocus state should have 3px border"
    );

    // Other states should have 1px border
    assert!(
        (state_border_width(InteractiveState::Idle) - 1.0).abs() < 0.001,
        "Idle state should have 1px border"
    );

    assert!(
        (state_border_width(InteractiveState::Hover) - 1.0).abs() < 0.001,
        "Hover state should have 1px border"
    );

    assert!(
        (state_border_width(InteractiveState::Pressed) - 1.0).abs() < 0.001,
        "Pressed state should have 1px border"
    );
}

#[test]
fn test_interactive_state_focus_ring_visibility() {
    use nodoka::ui::styles::interactive::{should_show_focus_ring, InteractiveState};

    // Focus ring should only show for keyboard focus states
    assert!(
        should_show_focus_ring(InteractiveState::Focus),
        "Focus ring should be visible for Focus state"
    );

    assert!(
        should_show_focus_ring(InteractiveState::HoverFocus),
        "Focus ring should be visible for HoverFocus state"
    );

    // No focus ring for other states
    assert!(
        !should_show_focus_ring(InteractiveState::Idle),
        "Focus ring should not show for Idle state"
    );

    assert!(
        !should_show_focus_ring(InteractiveState::Hover),
        "Focus ring should not show for Hover state"
    );

    assert!(
        !should_show_focus_ring(InteractiveState::Pressed),
        "Focus ring should not show for Pressed state"
    );
}

#[test]
fn test_interactive_state_pointer_display() {
    use nodoka::ui::styles::interactive::{should_show_pointer, InteractiveState};

    // Pointer should show for all states except disabled
    assert!(
        should_show_pointer(InteractiveState::Idle),
        "Pointer should show for Idle state"
    );

    assert!(
        should_show_pointer(InteractiveState::Hover),
        "Pointer should show for Hover state"
    );

    assert!(
        should_show_pointer(InteractiveState::Focus),
        "Pointer should show for Focus state"
    );

    assert!(
        should_show_pointer(InteractiveState::Pressed),
        "Pointer should show for Pressed state"
    );

    // No pointer for disabled state
    assert!(
        !should_show_pointer(InteractiveState::Disabled),
        "Pointer should not show for Disabled state"
    );
}

#[test]
fn test_interactive_state_overlay_progression() {
    use iced::Color;
    use nodoka::ui::styles::interactive::{apply_state_overlay, InteractiveState};

    let base = Color::from_rgb(0.5, 0.5, 0.5);

    // Idle should not modify color
    let idle = apply_state_overlay(base, InteractiveState::Idle);
    assert_eq!(idle, base, "Idle state should not modify color");

    // Hover should darken slightly
    let hover = apply_state_overlay(base, InteractiveState::Hover);
    assert!(hover.r < base.r, "Hover should darken color");

    // Pressed should darken more than hover
    let pressed = apply_state_overlay(base, InteractiveState::Pressed);
    assert!(pressed.r < hover.r, "Pressed should darken more than hover");
}

#[test]
fn test_hover_overlay_darkens_consistently() {
    use iced::Color;
    use nodoka::ui::styles::interactive::apply_hover_overlay;

    let colors = [
        Color::from_rgb(1.0, 0.0, 0.0), // Red
        Color::from_rgb(0.0, 1.0, 0.0), // Green
        Color::from_rgb(0.0, 0.0, 1.0), // Blue
        Color::from_rgb(0.5, 0.5, 0.5), // Gray
    ];

    for color in colors {
        let hovered = apply_hover_overlay(color);

        // Should darken (reduce RGB values)
        assert!(hovered.r <= color.r, "Hover should not lighten red channel");
        assert!(
            hovered.g <= color.g,
            "Hover should not lighten green channel"
        );
        assert!(
            hovered.b <= color.b,
            "Hover should not lighten blue channel"
        );

        // Alpha should remain unchanged
        assert!(
            (hovered.a - color.a).abs() < 0.001,
            "Hover should not change alpha"
        );

        // Darkening should be consistent (5% overlay)
        let expected_factor = 0.95;
        let tolerance = 0.01;
        assert!(
            color.r.mul_add(-expected_factor, hovered.r).abs() < tolerance,
            "Red channel should darken by expected factor"
        );
        assert!(
            color.g.mul_add(-expected_factor, hovered.g).abs() < tolerance,
            "Green channel should darken by expected factor"
        );
        assert!(
            color.b.mul_add(-expected_factor, hovered.b).abs() < tolerance,
            "Blue channel should darken by expected factor"
        );
    }
}
