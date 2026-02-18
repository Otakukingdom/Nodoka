//! UI Styles Module - Nodoka Design System
//!
//! This module provides a comprehensive design system for the Nodoka audiobook player.
//! It implements a vibrant rose palette optimized for audiobook player applications with
//! WCAG AA accessibility compliance.
//!
//! # Design System Overview
//!
//! The design system is organized into several modules:
//!
//! - **Colors**: Brand colors, semantic colors, text colors with WCAG AA contrast ratios
//! - **Spacing**: 4px base grid system for consistent layouts
//! - **Typography**: Size scale for text hierarchy
//! - **Border Radius**: Corner rounding constants for visual consistency
//! - **Transitions**: Animation durations with reduced motion support
//! - **Cursor**: Interaction affordances for different UI states
//! - **Interactive**: Hover and active state overlays
//! - **Loading**: Skeleton UI patterns for async operations
//! - **Button Styles**: Pre-configured button variants with focus states
//!
//! # UX Patterns
//!
//! ## Interactive Feedback Patterns
//!
//! All clickable elements must provide clear visual feedback:
//!
//! - **Cursor**: Use `cursor::clickable()` for pointer cursor on interactive elements
//! - **Hover**: Apply `BG_HOVER` background color with 200ms transition
//! - **Pressed**: Apply `interactive::apply_active_overlay()` for darker feedback
//! - **Focus**: Use 3px `FOCUS_RING` border for keyboard navigation visibility
//!
//! ## Empty State Pattern
//!
//! Use `empty_state()` component when lists are empty to guide users:
//!
//! ```ignore
//! if items.is_empty() {
//!     return empty_state(
//!         "No Items Found",
//!         "Helpful description explaining why empty and what to do next.",
//!         Some("Primary Action"),
//!         Some(Message::OpenSettings),
//!     );
//! }
//! ```
//!
//! ## Loading State Pattern
//!
//! Show skeleton UI for operations taking longer than 200ms to provide clear feedback:
//!
//! ```ignore
//! match scan_state {
//!     ScanState::Scanning { .. } => skeleton_list(),
//!     ScanState::Idle => actual_content(),
//! }
//! ```
//!
//! ## Accessibility Requirements
//!
//! - All components support keyboard navigation (Tab/Shift+Tab)
//! - Focus indicators meet WCAG 2.1 2.4.7 (3px minimum, high contrast)
//! - Text contrast ratios meet WCAG AA 4.5:1 minimum
//! - Reduced motion preferences respected for animations
//! - All interactive elements have accessible labels
//!
//! # References
//!
//! - Design System: [`colors`], [`layout`], [`components`], [`interactive`]
//! - WCAG 2.1: <https://www.w3.org/WAI/WCAG21/quickref/>

use iced::theme::Palette;
use iced::widget::container;
use iced::{Border, Color, Theme};

// Module declarations
pub mod accessibility;
pub mod animations;
pub mod colors;
pub mod components;
pub mod effects;
pub mod interactive;
pub mod layout;

// Re-export submodules for backward compatibility
pub use accessibility::*;
pub use colors::*;
pub use components::*;
pub use effects::{cursor, shadows, transitions};
pub use interactive::*;
pub use layout::*;

/// Focus indicator styling for keyboard navigation accessibility
///
/// Provides visual focus indicators compliant with WCAG 2.1 AA 2.4.7.
/// Use with application state tracking since iced 0.14 doesn't expose focus state directly.
///
/// # Usage
///
/// ```ignore
/// let is_focused = state.focused_element == FocusedElement::PlayPauseButton;
/// container(button(text("Play/Pause")).on_press(Message::PlayPause))
///     .style(move |theme| focus_indicator(is_focused)(theme))
/// ```
///
/// # Focus Ring Specifications
///
/// - **Color**: Blue #2563EB (`FOCUS_RING` constant)
/// - **Width**: 3px (WCAG recommends minimum 3px for visibility)
/// - **Radius**: Medium border radius (8px)
/// - **Contrast**: High contrast against all backgrounds
pub fn focus_indicator(is_focused: bool) -> impl Fn(&iced::Theme) -> container::Style {
    move |_theme: &iced::Theme| {
        if is_focused {
            container::Style {
                border: Border {
                    color: colors::FOCUS_RING,
                    width: 3.0,
                    radius: border_radius::MD.into(),
                },
                ..Default::default()
            }
        } else {
            container::Style::default()
        }
    }
}

/// Legacy top-bar color re-export (alias for [`colors::TOP_BAR_COLOR`]).
pub const TOP_BAR_COLOR: Color = colors::TOP_BAR_COLOR;
/// Legacy player background re-export (alias for [`colors::PLAYER_BG_COLOR`]).
pub const PLAYER_BG_COLOR: Color = colors::PLAYER_BG_COLOR;
/// Legacy player text color re-export (alias for [`colors::PLAYER_TEXT_COLOR`]).
pub const PLAYER_TEXT_COLOR: Color = colors::PLAYER_TEXT_COLOR;
/// Legacy audiobook list background re-export (alias for [`colors::AUDIOBOOK_LIST_BG`]).
pub const AUDIOBOOK_LIST_BG: Color = colors::AUDIOBOOK_LIST_BG;
/// Legacy file list background re-export (alias for [`colors::FILE_LIST_BG`]).
pub const FILE_LIST_BG: Color = colors::FILE_LIST_BG;
/// Legacy selected item background re-export (alias for [`colors::SELECTION_BG`]).
pub const SELECTED_ITEM_BG: Color = colors::SELECTION_BG;
/// Legacy text color re-export (alias for [`colors::TEXT_PRIMARY`]).
pub const TEXT_COLOR: Color = colors::TEXT_PRIMARY;

/// Creates the custom Nodoka theme based on the design system
#[must_use]
pub fn nodoka_theme() -> Theme {
    let palette = Palette {
        background: colors::BG_PRIMARY,
        text: colors::TEXT_PRIMARY,
        primary: colors::PRIMARY,
        success: colors::SUCCESS,
        danger: colors::ERROR,
        warning: colors::WARNING,
    };

    Theme::custom("Nodoka".to_string(), palette)
}

/// Formats duration in milliseconds to human-readable time string (H:MM:SS or M:SS)
#[must_use]
pub fn format_duration(ms: Option<i64>) -> String {
    match ms {
        Some(duration) if duration > 0 => {
            let total_seconds = duration / 1000;
            let hours = total_seconds / 3600;
            let minutes = (total_seconds % 3600) / 60;
            let seconds = total_seconds % 60;

            if hours > 0 {
                format!("{hours}:{minutes:02}:{seconds:02}")
            } else {
                format!("{minutes}:{seconds:02}")
            }
        }
        _ => String::from("--:--"),
    }
}

/// Formats time in milliseconds to human-readable time string (H:MM:SS or M:SS)
#[must_use]
pub fn format_time_ms(ms: i64) -> String {
    let ms = ms.max(0);
    let total_seconds = ms / 1000;
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

    if hours > 0 {
        format!("{hours}:{minutes:02}:{seconds:02}")
    } else {
        format!("{minutes}:{seconds:02}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn srgb_to_linear(channel: f64) -> f64 {
        if channel <= 0.04045 {
            channel / 12.92
        } else {
            ((channel + 0.055) / 1.055).powf(2.4)
        }
    }

    fn wcag_relative_luminance(color: iced::Color) -> f64 {
        // WCAG 2.1 relative luminance for sRGB.
        let r = srgb_to_linear(f64::from(color.r));
        let g = srgb_to_linear(f64::from(color.g));
        let b = srgb_to_linear(f64::from(color.b));
        (0.2126_f64).mul_add(r, (0.7152_f64).mul_add(g, 0.0722_f64 * b))
    }

    fn wcag_contrast_ratio(foreground: iced::Color, background: iced::Color) -> f64 {
        let fg = wcag_relative_luminance(foreground);
        let bg = wcag_relative_luminance(background);
        let (lighter, darker) = if fg >= bg { (fg, bg) } else { (bg, fg) };
        (lighter + 0.05) / (darker + 0.05)
    }

    #[test]
    fn test_format_duration_with_hours() {
        assert_eq!(format_duration(Some(3_661_000)), "1:01:01");
        assert_eq!(format_duration(Some(7_200_000)), "2:00:00");
    }

    #[test]
    fn test_format_duration_with_minutes_only() {
        assert_eq!(format_duration(Some(125_000)), "2:05");
        assert_eq!(format_duration(Some(60000)), "1:00");
    }

    #[test]
    fn test_format_duration_with_zero() {
        assert_eq!(format_duration(Some(0)), "--:--");
    }

    #[test]
    fn test_format_duration_with_none() {
        assert_eq!(format_duration(None), "--:--");
    }

    #[test]
    fn test_format_duration_with_negative() {
        assert_eq!(format_duration(Some(-500)), "--:--");
    }

    #[test]
    fn test_format_time_ms_with_hours() {
        assert_eq!(format_time_ms(3_661_000), "1:01:01");
        assert_eq!(format_time_ms(7_200_000), "2:00:00");
    }

    #[test]
    fn test_format_time_ms_with_minutes_only() {
        assert_eq!(format_time_ms(125_000), "2:05");
        assert_eq!(format_time_ms(60000), "1:00");
    }

    #[test]
    fn test_format_time_ms_with_zero() {
        assert_eq!(format_time_ms(0), "0:00");
    }

    #[test]
    fn test_format_time_ms_with_negative_clamped_to_zero() {
        assert_eq!(format_time_ms(-500), "0:00");
    }

    #[test]
    fn test_primary_text_contrast_on_primary_bg() {
        let ratio = wcag_contrast_ratio(colors::TEXT_PRIMARY, colors::BG_PRIMARY);
        assert!(
            ratio >= 4.5,
            "Primary text contrast ratio {ratio:.2} does not meet WCAG AA (4.5:1)"
        );
    }

    #[test]
    fn test_text_on_primary_button_contrast() {
        let ratio = wcag_contrast_ratio(colors::TEXT_ON_PRIMARY, colors::PRIMARY);
        assert!(
            ratio >= 4.5,
            "Button text contrast ratio {ratio:.2} does not meet WCAG AA (4.5:1)"
        );
    }

    #[test]
    fn test_focus_ring_contrast() {
        let ratio = wcag_contrast_ratio(colors::FOCUS_RING, colors::BG_PRIMARY);
        assert!(
            ratio >= 3.0,
            "Focus ring contrast ratio {ratio:.2} does not meet WCAG 2.1 (3:1)"
        );
    }

    #[test]
    fn test_error_button_contrast() {
        let ratio = wcag_contrast_ratio(colors::TEXT_ON_PRIMARY, colors::ERROR);
        assert!(
            ratio >= 4.5,
            "Error button contrast ratio {ratio:.2} does not meet WCAG AA (4.5:1)"
        );
    }

    #[test]
    fn test_transitions_respect_reduced_motion() {
        use animations::transition_duration;

        let normal = transition_duration(false);
        assert_eq!(normal, 200, "Normal transition should be 200ms");

        let reduced = transition_duration(true);
        assert_eq!(reduced, 0, "Reduced motion transition should be 0ms");
    }

    #[test]
    fn test_hover_overlay_darkens_color() {
        use interactive::apply_hover_overlay;

        let base = colors::BG_ELEVATED;
        let hovered = apply_hover_overlay(base);

        // Hover should darken all channels
        assert!(hovered.r < base.r);
        assert!(hovered.g < base.g);
        assert!(hovered.b < base.b);
    }

    #[test]
    fn test_active_overlay_darker_than_hover() {
        use interactive::{apply_active_overlay, apply_hover_overlay};

        let base = colors::BG_ELEVATED;
        let hovered = apply_hover_overlay(base);
        let active = apply_active_overlay(base);

        // Active should be darker than hover
        assert!(active.r < hovered.r);
        assert!(active.g < hovered.g);
        assert!(active.b < hovered.b);
    }

    #[test]
    fn test_interactive_state_overlay_progression() {
        use interactive::{apply_state_overlay, InteractiveState};

        let base = colors::BG_ELEVATED;
        let idle = apply_state_overlay(base, InteractiveState::Idle);
        let hover = apply_state_overlay(base, InteractiveState::Hover);
        let pressed = apply_state_overlay(base, InteractiveState::Pressed);

        // Visual feedback should progress: Idle > Hover > Pressed
        assert!(
            (idle.r - base.r).abs() < f32::EPSILON,
            "Idle should not change color"
        );
        assert!(hover.r < idle.r, "Hover should be darker than idle");
        assert!(pressed.r < hover.r, "Pressed should be darker than hover");
    }

    #[test]
    fn test_animations_enabled_returns_correct_bool() {
        use animations::animations_enabled;

        assert!(animations_enabled(false), "Should enable animations");
        assert!(!animations_enabled(true), "Should disable animations");
    }

    #[test]
    fn test_fade_transition_bounds() {
        use animations::FadeTransition;

        let mut fade = FadeTransition::new(200, 0.0, 1.0);

        // Test initial state
        assert!((fade.opacity() - 0.0).abs() < f32::EPSILON);

        // Test mid-transition
        fade.update(100);
        let mid = fade.opacity();
        assert!(
            (0.0..=1.0).contains(&mid),
            "Mid opacity {mid} out of bounds"
        );

        // Test complete transition
        fade.update(200);
        assert!((fade.opacity() - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_scale_transition_bounds() {
        use animations::ScaleTransition;

        let mut scale = ScaleTransition::new(200, 0.95, 1.0);

        // Test initial state
        assert!((scale.scale() - 0.95).abs() < f32::EPSILON);

        // Test mid-transition
        scale.update(100);
        let mid = scale.scale();
        assert!((0.95..=1.0).contains(&mid), "Mid scale {mid} out of bounds");

        // Test complete transition
        scale.update(200);
        assert!((scale.scale() - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_stagger_delay_progression() {
        use animations::stagger_delay;

        assert_eq!(stagger_delay(0, 50), 0);
        assert_eq!(stagger_delay(1, 50), 50);
        assert_eq!(stagger_delay(2, 50), 100);
        assert_eq!(stagger_delay(5, 50), 250);
    }

    #[test]
    fn test_pulse_animation_opacity_range() {
        use animations::PulseAnimation;

        let mut pulse = PulseAnimation::new(1000);

        // Test multiple cycles
        for _ in 0..100 {
            pulse.update(20);
            let opacity = pulse.opacity();
            assert!(
                (-0.01..=1.01).contains(&opacity),
                "Pulse opacity {opacity} out of range"
            );
        }
    }

    #[test]
    fn test_pulse_animation_with_zero_duration() {
        use animations::PulseAnimation;

        let mut pulse = PulseAnimation::new(0);
        let initial = pulse.opacity();

        pulse.update(100);
        let after_update = pulse.opacity();

        // With zero duration, opacity should not change
        assert!((initial - after_update).abs() < f32::EPSILON);
    }

    #[test]
    fn test_pulse_animation_multiple_updates() {
        use animations::PulseAnimation;

        let mut pulse = PulseAnimation::new(1000);

        // Update multiple times with small deltas (simulate frames)
        for _ in 0..60 {
            pulse.update(16); // 16ms per frame at 60fps
            let opacity = pulse.opacity();
            assert!(
                (-0.01..=1.01).contains(&opacity),
                "Opacity {opacity} out of range"
            );
        }
    }

    #[test]
    fn test_ease_in_out_is_symmetric() {
        use animations::easing;

        let t1 = easing::ease_in_out(0.25);
        let t2 = easing::ease_in_out(0.75);

        // ease_in_out should be symmetric around 0.5
        assert!((t1 + t2 - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_ease_out_starts_faster_than_ease_in() {
        use animations::easing;

        let ease_out_early = easing::ease_out(0.1);
        let ease_in_early = easing::ease_in(0.1);

        // Ease-out should progress faster initially
        assert!(ease_out_early > ease_in_early);
    }
}
