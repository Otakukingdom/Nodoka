//! Interactive feedback patterns for hover, focus, and pressed states.
//!
//! This module provides utilities for applying consistent interactive
//! overlays to colors.

use iced::{Color, Point};

/// Hover overlay alpha for subtle feedback (5% black overlay)
pub const HOVER_OVERLAY_ALPHA: f32 = 0.05;

/// Active/pressed overlay alpha (10% black overlay)
pub const ACTIVE_OVERLAY_ALPHA: f32 = 0.1;

/// Disabled overlay alpha (40% opacity reduction)
pub const DISABLED_OVERLAY_ALPHA: f32 = 0.4;

/// Ripple animation duration in milliseconds.
pub const RIPPLE_DURATION_MS: u64 = 300;

/// Ripple animation maximum opacity.
pub const RIPPLE_MAX_OPACITY: f32 = 0.2;

/// Interactive element state enum
///
/// Represents the combined interaction state of a UI element.
/// Compound states (e.g., `HoverFocus`) should
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
/// Hover states should provide clear visual
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
/// Visual feedback should progress naturally
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
/// Focus indicators should be 3px minimum for WCAG 2.1 compliance.
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
/// Focus rings should only appear for keyboard focus, not mouse clicks.
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

/// Ripple animation state for interactive press feedback.
///
/// Uses a 300ms ease-out expansion with a subtle fade.
#[derive(Debug, Clone)]
pub struct RippleAnimation {
    center: Point,
    elapsed_ms: u64,
    duration_ms: u64,
    max_radius: f32,
}

impl RippleAnimation {
    /// Creates a new ripple animation.
    #[must_use]
    pub const fn new(center: Point, max_radius: f32, reduced_motion: bool) -> Self {
        Self {
            center,
            elapsed_ms: 0,
            duration_ms: if reduced_motion {
                0
            } else {
                RIPPLE_DURATION_MS
            },
            max_radius,
        }
    }

    /// Updates animation progress with elapsed time.
    pub fn update(&mut self, delta_ms: u64) {
        self.elapsed_ms = (self.elapsed_ms + delta_ms).min(self.duration_ms);
    }

    /// Returns animation progress from 0.0 to 1.0.
    #[must_use]
    pub fn progress(&self) -> f32 {
        if self.duration_ms == 0 {
            return 1.0;
        }
        let elapsed =
            f32::from(u16::try_from(self.elapsed_ms.min(self.duration_ms)).unwrap_or(u16::MAX));
        let duration = f32::from(u16::try_from(self.duration_ms).unwrap_or(u16::MAX));
        (elapsed / duration).min(1.0)
    }

    /// Returns eased radius for the ripple.
    #[must_use]
    pub fn radius(&self) -> f32 {
        let t = ease_out_cubic(self.progress());
        self.max_radius * t
    }

    /// Returns current opacity for the ripple.
    #[must_use]
    pub fn opacity(&self) -> f32 {
        let t = self.progress();
        RIPPLE_MAX_OPACITY * (1.0 - t)
    }

    /// Returns ripple center point.
    #[must_use]
    pub const fn center(&self) -> Point {
        self.center
    }

    /// Returns whether the animation is complete.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.progress() >= 1.0
    }
}

fn ease_out_cubic(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    1.0 - (1.0 - t).powi(3)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ripple_progress_reduced_motion() {
        let ripple = RippleAnimation::new(Point::new(0.0, 0.0), 50.0, true);
        assert!((ripple.progress() - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_ripple_opacity_decreases() {
        let mut ripple = RippleAnimation::new(Point::new(0.0, 0.0), 50.0, false);
        let initial_opacity = ripple.opacity();
        ripple.update(150);
        assert!(ripple.opacity() < initial_opacity);
    }
}
