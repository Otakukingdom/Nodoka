//! Pre-configured component styles for common UI elements.
//!
//! This module provides button styles and loading indicators
//! with consistent visual design.

use iced::widget::button;
use iced::{Border, Color};

use super::{border_radius, colors, shadows};

/// Loading states and skeleton UI patterns
///
/// Provides color constants and utilities for loading indicators and skeleton UI
/// patterns. Loading indicators should be shown for operations taking longer than
/// 200ms to provide clear feedback.
pub mod loading {
    use super::{colors, Color};

    /// Skeleton background color for loading placeholders
    pub const SKELETON_BG: Color = Color::from_rgb(0.9, 0.9, 0.9);

    /// Skeleton shimmer color for animation
    pub const SKELETON_SHIMMER: Color = Color::from_rgb(0.95, 0.95, 0.95);

    /// Spinner color uses primary color for brand consistency
    pub const SPINNER_COLOR: Color = colors::PRIMARY;
}

/// Native iced 0.14 button styling using the new Style API
///
/// iced 0.14 provides proper button theming through the `button::Style` type
/// and closures that receive both `theme` and `status` parameters.
///
/// # Focus Indicators
///
/// iced 0.14's `button::Status` enum does not include a Focused state, so focus
/// indicators must be implemented through the shadow property when focus state
/// is tracked separately in application state.
///
/// # Usage
/// ```rust,ignore
/// button(text("Save"))
///     .on_press(Message::Save)
///     .style(button_styles::primary)
/// ```
pub mod button_styles {
    use super::{border_radius, button, colors, shadows, Border, Color};

    /// Primary action button style (e.g., Play, Save, Add)
    /// Uses primary brand color with premium gradient treatment
    /// Enhanced with refined hover states and focus indicators
    #[must_use]
    pub fn primary(_theme: &iced::Theme, status: button::Status) -> button::Style {
        match status {
            button::Status::Active => button::Style {
                // Multi-layer primary gradient for depth
                background: Some({
                    let start = colors::PRIMARY_GRADIENT_START;
                    let end = colors::PRIMARY_GRADIENT_END;
                    Color::from_rgb(
                        f32::midpoint(start.r, end.r),
                        f32::midpoint(start.g, end.g),
                        f32::midpoint(start.b, end.b),
                    )
                }.into()),
                text_color: colors::TEXT_ON_PRIMARY,
                border: Border {
                    color: colors::PRIMARY_GRADIENT_END,
                    width: 1.0,
                    radius: border_radius::MD.into(),
                },
                shadow: shadows::shadow_sm(),
                ..Default::default()
            },
            button::Status::Hovered => button::Style {
                // Brightened gradient for hover effect
                background: Some({
                    let start = colors::PRIMARY_GRADIENT_START;
                    let end = colors::PRIMARY_GRADIENT_END;
                    Color::from_rgb(
                        f32::midpoint(start.r, end.r).mul_add(0.05, 0.05),
                        f32::midpoint(start.g, end.g).mul_add(0.05, 0.05),
                        f32::midpoint(start.b, end.b).mul_add(0.05, 0.05),
                    )
                }.into()),
                text_color: colors::TEXT_ON_PRIMARY,
                border: Border {
                    color: colors::PRIMARY_GRADIENT_END,
                    width: 1.5,
                    radius: border_radius::MD.into(),
                },
                shadow: shadows::shadow_md(),
                ..Default::default()
            },
            button::Status::Pressed => button::Style {
                // Darkened gradient for pressed feedback
                background: Some({
                    let base = colors::PRIMARY;
                    Color::from_rgb(base.r * 0.85, base.g * 0.85, base.b * 0.85)
                }.into()),
                text_color: colors::TEXT_ON_PRIMARY,
                border: Border {
                    color: colors::PRIMARY_GRADIENT_END,
                    width: 2.0,
                    radius: border_radius::MD.into(),
                },
                shadow: shadows::shadow_xs(),
                ..Default::default()
            },
            button::Status::Disabled => button::Style {
                background: Some(Color {
                    r: colors::PRIMARY_GRADIENT_START.r * 0.5,
                    g: colors::PRIMARY_GRADIENT_START.g * 0.5,
                    b: colors::PRIMARY_GRADIENT_START.b * 0.5,
                    a: 0.5,
                }.into()),
                text_color: Color {
                    r: colors::TEXT_ON_PRIMARY.r,
                    g: colors::TEXT_ON_PRIMARY.g,
                    b: colors::TEXT_ON_PRIMARY.b,
                    a: 0.5,
                },
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: border_radius::MD.into(),
                },
                ..Default::default()
            },
        }
    }

    /// Secondary action button style (e.g., Cancel, Close, Back)
    /// Uses outlined style with premium border and hover effects
    #[must_use]
    pub fn secondary(_theme: &iced::Theme, status: button::Status) -> button::Style {
        match status {
            button::Status::Active => button::Style {
                background: Some(Color::TRANSPARENT.into()),
                text_color: colors::TEXT_PRIMARY,
                border: Border {
                    color: colors::BORDER_PROMINENT,
                    width: 1.5,
                    radius: border_radius::MD.into(),
                },
                ..Default::default()
            },
            button::Status::Hovered => button::Style {
                background: Some(colors::SURFACE_HOVER.into()),
                text_color: colors::PRIMARY,
                border: Border {
                    color: colors::PRIMARY,
                    width: 1.5,
                    radius: border_radius::MD.into(),
                },
                shadow: shadows::shadow_sm(),
                ..Default::default()
            },
            button::Status::Pressed => button::Style {
                background: Some(colors::SURFACE_ACTIVE.into()),
                text_color: colors::PRIMARY_ACTIVE,
                border: Border {
                    color: colors::PRIMARY_ACTIVE,
                    width: 2.0,
                    radius: border_radius::MD.into(),
                },
                ..Default::default()
            },
            button::Status::Disabled => button::Style {
                background: Some(Color::TRANSPARENT.into()),
                text_color: colors::TEXT_DISABLED,
                border: Border {
                    color: colors::BORDER_SUBTLE,
                    width: 1.0,
                    radius: border_radius::MD.into(),
                },
                ..Default::default()
            },
        }
    }

    /// Elevated action button style (e.g., Primary CTAs, prominent actions)
    /// Uses multi-layer gradient background for dramatic prominence
    /// Enhanced with premium micro-interactions and strong shadows
    #[must_use]
    pub fn elevated(_theme: &iced::Theme, status: button::Status) -> button::Style {
        match status {
            button::Status::Active => button::Style {
                // Multi-layer gradient combining sunset, primary, and accent
                background: Some({
                    let sunset = colors::SUNSET_GRADIENT_START;
                    let primary = colors::PRIMARY_GRADIENT_START;
                    let accent = colors::ACCENT_GRADIENT_START;
                    Color::from_rgb(
                        accent.r.mul_add(0.25, sunset.r.mul_add(0.4, primary.r * 0.35)),
                        accent.g.mul_add(0.25, sunset.g.mul_add(0.4, primary.g * 0.35)),
                        accent.b.mul_add(0.25, sunset.b.mul_add(0.4, primary.b * 0.35)),
                    )
                }.into()),
                text_color: colors::TEXT_ON_PRIMARY,
                border: Border {
                    color: colors::ACCENT_GRADIENT_END,
                    width: 1.5,
                    radius: border_radius::MD.into(),
                },
                shadow: shadows::shadow_md(),
                ..Default::default()
            },
            button::Status::Hovered => button::Style {
                // Enhanced gradient with stronger accent presence
                background: Some({
                    let sunset = colors::SUNSET_GRADIENT_START;
                    let accent = colors::ACCENT_GRADIENT_START;
                    Color::from_rgb(
                        accent.r.mul_add(0.55, sunset.r * 0.45),
                        accent.g.mul_add(0.55, sunset.g * 0.45),
                        accent.b.mul_add(0.55, sunset.b * 0.45),
                    )
                }.into()),
                text_color: colors::TEXT_ON_PRIMARY,
                border: Border {
                    color: colors::ACCENT_GRADIENT_END,
                    width: 2.0,
                    radius: border_radius::MD.into(),
                },
                shadow: shadows::shadow_lg(),
                ..Default::default()
            },
            button::Status::Pressed => button::Style {
                background: Some(colors::PRIMARY.into()),
                text_color: colors::TEXT_ON_PRIMARY,
                border: Border {
                    color: colors::PRIMARY_GRADIENT_END,
                    width: 2.0,
                    radius: border_radius::MD.into(),
                },
                shadow: shadows::shadow_sm(),
                ..Default::default()
            },
            button::Status::Disabled => button::Style {
                background: Some(Color {
                    r: colors::PRIMARY_GRADIENT_START.r * 0.5,
                    g: colors::PRIMARY_GRADIENT_START.g * 0.5,
                    b: colors::PRIMARY_GRADIENT_START.b * 0.5,
                    a: 0.5,
                }.into()),
                text_color: Color {
                    r: colors::TEXT_ON_PRIMARY.r,
                    g: colors::TEXT_ON_PRIMARY.g,
                    b: colors::TEXT_ON_PRIMARY.b,
                    a: 0.5,
                },
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: border_radius::MD.into(),
                },
                ..Default::default()
            },
        }
    }

    /// Danger action button style (e.g., Delete, Remove)
    /// Uses sunset gradient for premium error treatment
    /// Enhanced with multi-layer red gradient for better visual feedback
    #[must_use]
    pub fn danger(_theme: &iced::Theme, status: button::Status) -> button::Style {
        match status {
            button::Status::Active => button::Style {
                // Enhanced sunset gradient for danger state
                background: Some({
                    let sunset = colors::SUNSET_GRADIENT_START;
                    let error = colors::ERROR;
                    Color::from_rgb(
                        error.r.mul_add(0.5, sunset.r * 0.5),
                        error.g.mul_add(0.5, sunset.g * 0.5),
                        error.b.mul_add(0.5, sunset.b * 0.5),
                    )
                }.into()),
                text_color: colors::TEXT_ON_PRIMARY,
                border: Border {
                    color: colors::SUNSET_GRADIENT_END,
                    width: 1.0,
                    radius: border_radius::MD.into(),
                },
                shadow: shadows::shadow_sm(),
                ..Default::default()
            },
            button::Status::Hovered => button::Style {
                // Stronger gradient for hover state
                background: Some({
                    let sunset_end = colors::SUNSET_GRADIENT_END;
                    let error = colors::ERROR;
                    Color::from_rgb(
                        error.r.mul_add(0.4, sunset_end.r * 0.6),
                        error.g.mul_add(0.4, sunset_end.g * 0.6),
                        error.b.mul_add(0.4, sunset_end.b * 0.6),
                    )
                }.into()),
                text_color: colors::TEXT_ON_PRIMARY,
                border: Border {
                    color: colors::SUNSET_GRADIENT_END,
                    width: 1.5,
                    radius: border_radius::MD.into(),
                },
                shadow: shadows::shadow_md(),
                ..Default::default()
            },
            button::Status::Pressed => button::Style {
                background: Some(colors::ERROR_ACTIVE.into()),
                text_color: colors::TEXT_ON_PRIMARY,
                border: Border {
                    color: colors::SUNSET_GRADIENT_END,
                    width: 2.0,
                    radius: border_radius::MD.into(),
                },
                shadow: shadows::shadow_xs(),
                ..Default::default()
            },
            button::Status::Disabled => button::Style {
                background: Some(Color {
                    r: colors::ERROR.r * 0.5,
                    g: colors::ERROR.g * 0.5,
                    b: colors::ERROR.b * 0.5,
                    a: 0.5,
                }.into()),
                text_color: Color {
                    r: colors::TEXT_ON_PRIMARY.r,
                    g: colors::TEXT_ON_PRIMARY.g,
                    b: colors::TEXT_ON_PRIMARY.b,
                    a: 0.5,
                },
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: border_radius::MD.into(),
                },
                ..Default::default()
            },
        }
    }

    /// Primary button style with focus indicator
    /// Adds a blue shadow/ring for WCAG 2.1 AA compliance (3px minimum)
    /// Enhanced with glow effect for better visibility
    #[must_use]
    pub fn primary_focused(theme: &iced::Theme, status: button::Status) -> button::Style {
        let mut style = primary(theme, status);
        style.shadow = shadows::glow_primary(0.6);
        style.border = Border {
            color: colors::FOCUS_RING,
            width: 3.0,
            radius: border_radius::MD.into(),
        };
        style
    }

    /// Secondary button style with focus indicator
    /// Enhanced with glow effect for visibility
    #[must_use]
    pub fn secondary_focused(theme: &iced::Theme, status: button::Status) -> button::Style {
        let mut style = secondary(theme, status);
        style.shadow = shadows::glow_primary(0.3);
        style.border = Border {
            color: colors::FOCUS_RING,
            width: 3.0,
            radius: border_radius::MD.into(),
        };
        style
    }

    /// Elevated button style with focus indicator
    /// Enhanced with strong glow effect for prominence
    #[must_use]
    pub fn elevated_focused(theme: &iced::Theme, status: button::Status) -> button::Style {
        let mut style = elevated(theme, status);
        style.shadow = shadows::glow_primary(0.8);
        style.border = Border {
            color: colors::FOCUS_RING,
            width: 3.0,
            radius: border_radius::MD.into(),
        };
        style
    }

    /// Danger button style with focus indicator
    /// Enhanced with red glow for danger indication
    #[must_use]
    pub fn danger_focused(theme: &iced::Theme, status: button::Status) -> button::Style {
        let mut style = danger(theme, status);
        style.shadow = shadows::glow_error(0.5);
        style.border = Border {
            color: colors::FOCUS_RING,
            width: 3.0,
            radius: border_radius::MD.into(),
        };
        style
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loading_spinner_color_is_valid() {
        let color = loading::SPINNER_COLOR;
        assert!(color.r >= 0.0 && color.r <= 1.0);
        assert!(color.g >= 0.0 && color.g <= 1.0);
        assert!(color.b >= 0.0 && color.b <= 1.0);
    }

    #[test]
    fn test_loading_skeleton_bg_is_valid() {
        let color = loading::SKELETON_BG;
        assert!(color.r >= 0.0 && color.r <= 1.0);
        assert!(color.g >= 0.0 && color.g <= 1.0);
        assert!(color.b >= 0.0 && color.b <= 1.0);
    }

    #[test]
    fn test_loading_skeleton_shimmer_is_valid() {
        let color = loading::SKELETON_SHIMMER;
        assert!(color.r >= 0.0 && color.r <= 1.0);
        assert!(color.g >= 0.0 && color.g <= 1.0);
        assert!(color.b >= 0.0 && color.b <= 1.0);
    }

    #[test]
    fn test_button_primary_active_returns_valid() {
        let style = button_styles::primary(&iced::Theme::Light, button::Status::Active);
        drop(style);
    }

    #[test]
    fn test_button_primary_hovered_returns_valid() {
        let style = button_styles::primary(&iced::Theme::Light, button::Status::Hovered);
        drop(style);
    }

    #[test]
    fn test_button_primary_pressed_returns_valid() {
        let style = button_styles::primary(&iced::Theme::Light, button::Status::Pressed);
        drop(style);
    }

    #[test]
    fn test_button_primary_disabled_returns_valid() {
        let style = button_styles::primary(&iced::Theme::Light, button::Status::Disabled);
        drop(style);
    }

    #[test]
    fn test_button_secondary_active_returns_valid() {
        let style = button_styles::secondary(&iced::Theme::Light, button::Status::Active);
        drop(style);
    }

    #[test]
    fn test_button_elevated_active_returns_valid() {
        let style = button_styles::elevated(&iced::Theme::Light, button::Status::Active);
        drop(style);
    }

    #[test]
    fn test_button_danger_active_returns_valid() {
        let style = button_styles::danger(&iced::Theme::Light, button::Status::Active);
        drop(style);
    }

    #[test]
    fn test_button_primary_focused_returns_valid() {
        let style = button_styles::primary_focused(&iced::Theme::Light, button::Status::Active);
        drop(style);
    }

    #[test]
    fn test_button_secondary_focused_returns_valid() {
        let style = button_styles::secondary_focused(&iced::Theme::Light, button::Status::Active);
        drop(style);
    }

    #[test]
    fn test_button_elevated_focused_returns_valid() {
        let style = button_styles::elevated_focused(&iced::Theme::Light, button::Status::Active);
        drop(style);
    }

    #[test]
    fn test_button_danger_focused_returns_valid() {
        let style = button_styles::danger_focused(&iced::Theme::Light, button::Status::Active);
        drop(style);
    }
}
