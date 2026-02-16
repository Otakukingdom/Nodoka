//! Pre-configured component styles for common UI elements.
//!
//! This module provides button styles and loading indicators
//! with consistent visual design.

use iced::widget::button;
use iced::{Border, Color, Shadow};

use super::{border_radius, colors, shadows};

/// Loading states and skeleton UI patterns
///
/// Provides color constants and utilities for loading indicators and skeleton UI
/// patterns. Per ui-ux-pro-max guidelines, loading indicators should be shown for
/// operations taking longer than 200ms to provide clear feedback.
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
    use super::{border_radius, button, colors, shadows, Border, Color, Shadow};

    /// Primary action button style (e.g., Play, Save, Add)
    /// Uses primary brand color with white text
    /// Provides hover/pressed/focus states
    #[must_use]
    pub fn primary(_theme: &iced::Theme, status: button::Status) -> button::Style {
        match status {
            button::Status::Active => button::Style {
                background: Some(colors::PRIMARY.into()),
                text_color: colors::TEXT_ON_PRIMARY,
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: border_radius::MD.into(),
                },
                ..Default::default()
            },
            button::Status::Hovered => button::Style {
                background: Some(colors::PRIMARY_HOVER.into()),
                text_color: colors::TEXT_ON_PRIMARY,
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: border_radius::MD.into(),
                },
                shadow: shadows::shadow_sm(),
                ..Default::default()
            },
            button::Status::Pressed => button::Style {
                background: Some(colors::PRIMARY_ACTIVE.into()),
                text_color: colors::TEXT_ON_PRIMARY,
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: border_radius::MD.into(),
                },
                ..Default::default()
            },
            button::Status::Disabled => button::Style {
                background: Some(colors::PRIMARY.into()),
                text_color: colors::TEXT_DISABLED,
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: border_radius::MD.into(),
                },
                ..Default::default()
            },
        }
    }

    /// Secondary action button style (e.g., Cancel, Close, Stop)
    /// Uses elevated background with primary text color and border
    #[must_use]
    pub fn secondary(_theme: &iced::Theme, status: button::Status) -> button::Style {
        match status {
            button::Status::Active => button::Style {
                background: Some(colors::BG_ELEVATED.into()),
                text_color: colors::TEXT_PRIMARY,
                border: Border {
                    color: colors::BORDER_DEFAULT,
                    width: 1.0,
                    radius: border_radius::MD.into(),
                },
                ..Default::default()
            },
            button::Status::Hovered => button::Style {
                background: Some(colors::BG_HOVER.into()),
                text_color: colors::TEXT_PRIMARY,
                border: Border {
                    color: colors::BORDER_DEFAULT,
                    width: 1.0,
                    radius: border_radius::MD.into(),
                },
                shadow: shadows::shadow_sm(),
                ..Default::default()
            },
            button::Status::Pressed => button::Style {
                background: Some(colors::BG_ELEVATED.into()),
                text_color: colors::TEXT_PRIMARY,
                border: Border {
                    color: colors::BORDER_FOCUS,
                    width: 2.0,
                    radius: border_radius::MD.into(),
                },
                ..Default::default()
            },
            button::Status::Disabled => button::Style {
                background: Some(colors::BG_ELEVATED.into()),
                text_color: colors::TEXT_DISABLED,
                border: Border {
                    color: colors::BORDER_DEFAULT,
                    width: 1.0,
                    radius: border_radius::MD.into(),
                },
                ..Default::default()
            },
        }
    }

    /// Danger action button style (e.g., Delete, Remove)
    /// Uses error/danger color with white text
    #[must_use]
    pub fn danger(_theme: &iced::Theme, status: button::Status) -> button::Style {
        match status {
            button::Status::Active => button::Style {
                background: Some(colors::ERROR.into()),
                text_color: colors::TEXT_ON_PRIMARY,
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: border_radius::MD.into(),
                },
                ..Default::default()
            },
            button::Status::Hovered => button::Style {
                background: Some(colors::ERROR_HOVER.into()),
                text_color: colors::TEXT_ON_PRIMARY,
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: border_radius::MD.into(),
                },
                shadow: shadows::shadow_sm(),
                ..Default::default()
            },
            button::Status::Pressed => button::Style {
                background: Some(colors::ERROR_ACTIVE.into()),
                text_color: colors::TEXT_ON_PRIMARY,
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: border_radius::MD.into(),
                },
                ..Default::default()
            },
            button::Status::Disabled => button::Style {
                background: Some(colors::ERROR.into()),
                text_color: colors::TEXT_DISABLED,
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
    #[must_use]
    pub fn primary_focused(theme: &iced::Theme, status: button::Status) -> button::Style {
        let mut style = primary(theme, status);
        style.shadow = Shadow {
            color: colors::FOCUS_RING,
            offset: iced::Vector::ZERO,
            blur_radius: 0.0,
        };
        style.border = Border {
            color: colors::FOCUS_RING,
            width: 3.0,
            radius: border_radius::MD.into(),
        };
        style
    }

    /// Secondary button style with focus indicator
    #[must_use]
    pub fn secondary_focused(theme: &iced::Theme, status: button::Status) -> button::Style {
        let mut style = secondary(theme, status);
        style.border = Border {
            color: colors::FOCUS_RING,
            width: 3.0,
            radius: border_radius::MD.into(),
        };
        style
    }

    /// Danger button style with focus indicator
    #[must_use]
    pub fn danger_focused(theme: &iced::Theme, status: button::Status) -> button::Style {
        let mut style = danger(theme, status);
        style.shadow = Shadow {
            color: colors::FOCUS_RING,
            offset: iced::Vector::ZERO,
            blur_radius: 0.0,
        };
        style.border = Border {
            color: colors::FOCUS_RING,
            width: 3.0,
            radius: border_radius::MD.into(),
        };
        style
    }
}
