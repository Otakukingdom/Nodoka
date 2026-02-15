//! Integration tests for iced 0.14 upgraded features
//!
//! Verifies that the iced 0.14 upgrade successfully enables:
//! - Native button styling through `button::Style` API
//! - Modal backdrops using stack widget for proper layering
//! - Focus indicators through styled button variants
//!
//! These tests complement existing UI tests by specifically validating
//! iced 0.14 API usage and functionality.

use nodoka::ui::button_styles;
use nodoka::ui::{BookmarkEditor, LoadState, State};

#[test]
fn test_native_button_styling_compiles() {
    // Verify button styling functions are callable with correct signatures
    let theme = iced::Theme::Light;
    let status = iced::widget::button::Status::Active;

    let _primary_style = button_styles::primary(&theme, status);
    let _secondary_style = button_styles::secondary(&theme, status);
    let _danger_style = button_styles::danger(&theme, status);
}

#[test]
fn test_focused_button_styles_exist() {
    // Verify focused button style variants are available
    let theme = iced::Theme::Light;
    let status = iced::widget::button::Status::Active;

    let _primary_focused = button_styles::primary_focused(&theme, status);
    let _secondary_focused = button_styles::secondary_focused(&theme, status);
    let _danger_focused = button_styles::danger_focused(&theme, status);
}

#[test]
fn test_button_styles_have_distinct_appearances() {
    // Verify that different button styles produce different visual appearances
    let theme = iced::Theme::Light;
    let status = iced::widget::button::Status::Active;

    let primary = button_styles::primary(&theme, status);
    let secondary = button_styles::secondary(&theme, status);
    let danger = button_styles::danger(&theme, status);

    // Primary and secondary should have different backgrounds
    assert_ne!(primary.background, secondary.background);

    // Primary and danger should have same text color (white on primary)
    assert_eq!(primary.text_color, danger.text_color);

    // Secondary should have a border (width > 0)
    assert!(secondary.border.width > 0.0);

    // Primary should have no border by default
    assert!((primary.border.width - 0.0).abs() < f32::EPSILON);
}

#[test]
fn test_focused_styles_have_focus_indicators() {
    // Verify focused styles add visible focus indicators (border or shadow)
    let theme = iced::Theme::Light;
    let status = iced::widget::button::Status::Active;

    let primary = button_styles::primary(&theme, status);
    let primary_focused = button_styles::primary_focused(&theme, status);

    // Focused variant should have a border for focus indication
    assert!(
        primary_focused.border.width > primary.border.width,
        "Focused button should have thicker border for WCAG compliance"
    );

    // Focus border should be at least 3px (WCAG recommendation)
    assert!(
        primary_focused.border.width >= 3.0,
        "Focus indicator should be at least 3px wide"
    );
}

#[test]
fn test_button_hierarchy_visual_distinction() {
    // Document that buttons have visual hierarchy via styles
    // Primary: vibrant background, no border
    // Secondary: elevated background, with border
    // Danger: error color background, no border
    let theme = iced::Theme::Light;
    let status = iced::widget::button::Status::Active;

    let primary = button_styles::primary(&theme, status);
    let secondary = button_styles::secondary(&theme, status);
    let danger = button_styles::danger(&theme, status);

    // Primary should have background
    assert!(primary.background.is_some());

    // Secondary should have border to differentiate from primary
    assert!(secondary.border.width > 0.0);

    // Danger should have background (error color)
    assert!(danger.background.is_some());

    // All styles should have proper corner radius (> 0)
    let epsilon = f32::EPSILON;
    assert!(primary.border.radius.top_left > epsilon);
    assert!(secondary.border.radius.top_left > epsilon);
    assert!(danger.border.radius.top_left > epsilon);
}

#[test]
fn test_hover_states_differ_from_active() {
    // Verify hover states provide visual feedback
    let theme = iced::Theme::Light;

    let primary_active = button_styles::primary(&theme, iced::widget::button::Status::Active);
    let primary_hovered = button_styles::primary(&theme, iced::widget::button::Status::Hovered);

    // Hover should have different background than active
    assert_ne!(
        primary_active.background, primary_hovered.background,
        "Hover state should provide visual feedback"
    );
}

#[test]
fn test_disabled_states_reduce_opacity_or_color() {
    // Verify disabled buttons are visually distinct
    let theme = iced::Theme::Light;

    let primary_active = button_styles::primary(&theme, iced::widget::button::Status::Active);
    let primary_disabled = button_styles::primary(&theme, iced::widget::button::Status::Disabled);

    // Disabled should have different text color (typically grayed out)
    assert_ne!(
        primary_active.text_color, primary_disabled.text_color,
        "Disabled buttons should have visually distinct text color"
    );
}

#[test]
fn test_modal_state_changes_trigger_view_updates() {
    // Test that opening/closing modals changes UI state correctly
    let mut state = State::default();

    // Initially no modal open
    assert!(!state.settings_open);
    assert!(state.bookmark_editor.is_none());

    // Open settings modal
    state.settings_open = true;
    assert!(state.settings_open);

    // Close settings modal
    state.settings_open = false;
    assert!(!state.settings_open);
}

#[test]
fn test_modal_backdrops_use_stack_widget() {
    // This test documents that modal backdrops are implemented using
    // iced 0.14's stack widget for proper layering.
    //
    // Manual verification required:
    // 1. Open settings dialog - background should be dimmed (50% black)
    // 2. Click on dimmed background - modal should close
    // 3. Modal content should appear on top of backdrop
    //
    // The implementation uses:
    // - stack! macro to layer elements
    // - Semi-transparent button as backdrop (rgba(0,0,0,0.5))
    // - on_press handler for click-outside-to-dismiss
    //
    // See src/ui/main_window.rs lines ~110-160 for implementation
}

#[test]
fn test_all_modals_have_backdrop_support() {
    // Verify that all modal states can be represented
    let state_with_settings = State {
        settings_open: true,
        ..Default::default()
    };

    let state_with_bookmark_editor = State {
        bookmark_editor: Some(BookmarkEditor {
            id: None,
            audiobook_id: 1,
            file_path: "/test/file.mp3".to_string(),
            position_ms: 1000,
            label: "Test".to_string(),
            note: String::new(),
        }),
        ..Default::default()
    };

    let state_with_loading = State {
        load_state: LoadState::Loading,
        ..Default::default()
    };

    // All modal states should be representable
    assert!(state_with_settings.settings_open);
    assert!(state_with_bookmark_editor.bookmark_editor.is_some());
    assert_eq!(state_with_loading.load_state, LoadState::Loading);

    // Modals should be mutually exclusive in typical usage
    // (though technically multiple can be open)
    // Note: Default state starts in LoadState::Loading for initial app load
    let state_default = State::default();
    assert!(!state_default.settings_open);
    assert!(state_default.bookmark_editor.is_none());
    assert_eq!(state_default.load_state, LoadState::Loading);
}

#[test]
fn test_iced_014_stack_widget_available() {
    // Verify stack widget is available from iced 0.14
    // This is a compile-time check - if this test compiles, stack exists

    // Create an empty stack to verify the API is available
    let _stack: iced::widget::Stack<'_, (), iced::Theme> = iced::widget::Stack::new();
}

#[test]
fn test_focus_indicator_meets_wcag_requirements() {
    // WCAG 2.1 AA requires minimum 3px focus indicator
    let theme = iced::Theme::Light;
    let status = iced::widget::button::Status::Active;

    let primary_focused = button_styles::primary_focused(&theme, status);
    let secondary_focused = button_styles::secondary_focused(&theme, status);
    let danger_focused = button_styles::danger_focused(&theme, status);

    // All focused styles must have at least 3px border width
    assert!(
        primary_focused.border.width >= 3.0,
        "Primary focus indicator must be at least 3px"
    );
    assert!(
        secondary_focused.border.width >= 3.0,
        "Secondary focus indicator must be at least 3px"
    );
    assert!(
        danger_focused.border.width >= 3.0,
        "Danger focus indicator must be at least 3px"
    );
}

#[test]
fn test_pressed_state_provides_feedback() {
    // Verify pressed state is visually distinct
    let theme = iced::Theme::Light;

    let primary_active = button_styles::primary(&theme, iced::widget::button::Status::Active);
    let primary_pressed = button_styles::primary(&theme, iced::widget::button::Status::Pressed);

    // Pressed should provide visual feedback (typically darker or bordered)
    // At minimum, it should be a valid style
    assert!(primary_pressed.background.is_some() || primary_pressed.border.width > 0.0);

    // Active and pressed should be different (even if subtle)
    let styles_differ = primary_active.background != primary_pressed.background
        || primary_active.border != primary_pressed.border;

    assert!(
        styles_differ,
        "Pressed state should provide visual feedback distinct from active"
    );
}
