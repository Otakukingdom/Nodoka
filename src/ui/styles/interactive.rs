//! Interactive feedback patterns for hover, focus, and pressed states.
//!
//! This module provides utilities for applying consistent interactive
//! overlays to colors following ui-ux-pro-max guidelines.

use iced::Color;

/// Hover overlay alpha for subtle feedback (5% black overlay)
pub const HOVER_OVERLAY_ALPHA: f32 = 0.05;

/// Active/pressed overlay alpha (10% black overlay)
pub const ACTIVE_OVERLAY_ALPHA: f32 = 0.1;

/// Disabled overlay alpha (40% opacity reduction)
pub const DISABLED_OVERLAY_ALPHA: f32 = 0.4;

/// Interactive element state enum
///
/// Represents the combined interaction state of a UI element.
/// Per ui-ux-pro-max: compound states (e.g., `HoverFocus`) should
/// provide cumulative visual feedback.
///
/// # Examples
///
/// ```ignore
/// use nodoka::ui::styles::interactive::InteractiveState;
///
/// let state = if disabled {
///     InteractiveState::Disabled
/// } else if is_hovered && is_focused {
///     InteractiveState::HoverFocus
/// } else if is_hovered {
///     InteractiveState::Hover
/// } else if is_focused {
///     InteractiveState::Focus
/// } else {
///     InteractiveState::Idle
/// };
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InteractiveState {
    /// Default state with no interaction
    Idle,
    /// Mouse cursor is over the element
    Hover,
    /// Element has keyboard focus
    Focus,
    /// Element is hovered and focused
    HoverFocus,
    /// Element is being pressed/clicked
    Pressed,
    /// Element is disabled and non-interactive
    Disabled,
}

/// Applies hover overlay to a color for subtle interactive feedback
///
/// Per ui-ux-pro-max guidelines, hover states should provide clear visual
/// feedback without causing layout shift. This function applies a 5% black
/// overlay to darken the base color slightly.
///
/// # Examples
///
/// ```ignore
/// use nodoka::ui::styles::{colors, interactive};
/// let hover_color = interactive::apply_hover_overlay(colors::BG_ELEVATED);
/// ```
#[must_use]
pub fn apply_hover_overlay(base: Color) -> Color {
    Color {
        r: base.r * (1.0 - HOVER_OVERLAY_ALPHA),
        g: base.g * (1.0 - HOVER_OVERLAY_ALPHA),
        b: base.b * (1.0 - HOVER_OVERLAY_ALPHA),
        a: base.a,
    }
}

/// Applies active/pressed overlay to a color
///
/// Provides stronger visual feedback for pressed/active states with a
/// 10% black overlay.
///
/// # Examples
///
/// ```ignore
/// use nodoka::ui::styles::{colors, interactive};
/// let active_color = interactive::apply_active_overlay(colors::BG_HOVER);
/// ```
#[must_use]
pub fn apply_active_overlay(base: Color) -> Color {
    Color {
        r: base.r * (1.0 - ACTIVE_OVERLAY_ALPHA),
        g: base.g * (1.0 - ACTIVE_OVERLAY_ALPHA),
        b: base.b * (1.0 - ACTIVE_OVERLAY_ALPHA),
        a: base.a,
    }
}

/// Applies appropriate overlay based on interactive state
///
/// Per ui-ux-pro-max: visual feedback should progress naturally
/// from Idle → Hover → Focus → Pressed with increasing emphasis.
///
/// # Arguments
///
/// * `base` - Base color to apply overlay to
/// * `state` - Current interactive state
///
/// # Examples
///
/// ```ignore
/// use nodoka::ui::styles::{colors, interactive};
/// use nodoka::ui::styles::interactive::InteractiveState;
///
/// let color = interactive::apply_state_overlay(
///     colors::BG_ELEVATED,
///     InteractiveState::HoverFocus
/// );
/// ```
#[must_use]
pub fn apply_state_overlay(base: Color, state: InteractiveState) -> Color {
    match state {
        InteractiveState::Idle => base,
        InteractiveState::Hover | InteractiveState::Focus | InteractiveState::HoverFocus => {
            apply_hover_overlay(base)
        }
        InteractiveState::Pressed => apply_active_overlay(base),
        InteractiveState::Disabled => Color {
            r: base.r * (1.0 - DISABLED_OVERLAY_ALPHA),
            g: base.g * (1.0 - DISABLED_OVERLAY_ALPHA),
            b: base.b * (1.0 - DISABLED_OVERLAY_ALPHA),
            a: base.a * 0.5, // Reduce overall opacity
        },
    }
}

/// Returns whether cursor should be pointer for given state
///
/// Per WCAG 2.1: cursor affordances help users understand interactivity.
///
/// # Examples
///
/// ```ignore
/// if interactive::should_show_pointer(state) {
///     // Apply pointer cursor
/// }
/// ```
#[must_use]
pub const fn should_show_pointer(state: InteractiveState) -> bool {
    !matches!(state, InteractiveState::Disabled)
}

/// Returns border width based on interactive state
///
/// Per ui-ux-pro-max: focus indicators should be 3px minimum for WCAG 2.1 compliance.
/// Regular borders are 1px for subtle separation.
///
/// # Examples
///
/// ```ignore
/// let width = interactive::state_border_width(InteractiveState::Focus); // 3.0
/// ```
#[must_use]
pub const fn state_border_width(state: InteractiveState) -> f32 {
    match state {
        InteractiveState::Focus | InteractiveState::HoverFocus => 3.0,
        _ => 1.0,
    }
}

/// Returns whether state should show focus ring
///
/// Focus rings should only appear for keyboard focus, not mouse clicks,
/// per ui-ux-pro-max guidelines.
///
/// # Examples
///
/// ```ignore
/// if interactive::should_show_focus_ring(state) {
///     // Apply focus ring style
/// }
/// ```
#[must_use]
pub const fn should_show_focus_ring(state: InteractiveState) -> bool {
    matches!(
        state,
        InteractiveState::Focus | InteractiveState::HoverFocus
    )
}
