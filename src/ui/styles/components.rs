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

/// Empty state component styling utilities
///
/// Provides consistent styling for empty state components following
/// ui-ux-pro-max visual hierarchy specifications.
pub mod empty_state_styles {
    use super::{border_radius, colors, Border, Color};

    /// Returns container style for empty state background
    ///
    /// Creates subtle background tint with border to distinguish
    /// empty state area from main content.
    #[must_use]
    pub fn container_style(_theme: &iced::Theme) -> iced::widget::container::Style {
        iced::widget::container::Style {
            background: Some(colors::BG_SECONDARY.into()),
            border: Border {
                color: colors::BORDER_DEFAULT,
                width: 2.0,
                radius: border_radius::LG.into(),
            },
            ..Default::default()
        }
    }

    /// Icon color for empty state (secondary text color)
    pub const ICON_COLOR: Color = colors::TEXT_SECONDARY;

    /// Heading color for empty state (primary text color)
    pub const HEADING_COLOR: Color = colors::TEXT_PRIMARY;

    /// Description color for empty state (secondary text color)
    pub const DESCRIPTION_COLOR: Color = colors::TEXT_SECONDARY;
}

/// Notification toast styling utilities
///
/// Provides consistent styling for notification toasts following
/// ui-ux-pro-max feedback patterns with semantic color variants.
///
/// # Notification Variants
///
/// - **Success** (Green): Positive confirmation, auto-dismiss 3s
/// - **Info** (Blue): Neutral information, auto-dismiss 3s
/// - **Warning** (Orange): Caution/attention needed, auto-dismiss 5s
/// - **Error** (Red): Critical issue, manual dismiss only
pub mod notification_styles {
    use super::{border_radius, shadows, Border, Color};
    use crate::ui::state::NotificationType;

    /// Returns background color for notification variant
    ///
    /// Per ui-ux-pro-max: use semantic colors with sufficient contrast
    /// for WCAG AA compliance (4.5:1 minimum).
    #[must_use]
    pub const fn background_color(kind: &NotificationType) -> Color {
        match kind {
            NotificationType::Success => Color::from_rgb(0.11, 0.50, 0.11), // Darker green for WCAG AA (4.5:1)
            NotificationType::Info => Color::from_rgb(0.0, 0.46, 0.78), // Darker blue for WCAG AA (4.5:1)
            NotificationType::Warning => Color::from_rgb(0.70, 0.38, 0.0), // Much darker orange for WCAG AA (4.5:1)
            NotificationType::Error => Color::from_rgb(0.75, 0.0, 0.0), // Darker red for WCAG AA (4.5:1)
        }
    }

    /// Returns text color for notification (always white for contrast)
    pub const TEXT_COLOR: Color = Color::WHITE;

    /// Returns icon color for notification (always white)
    pub const ICON_COLOR: Color = Color::WHITE;

    /// Returns container style for notification toast
    ///
    /// Creates elevated notification with shadow and rounded corners.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// container(content)
    ///     .style(notification_styles::container_style(&NotificationType::Success))
    /// ```
    #[must_use]
    pub fn container_style(kind: &NotificationType) -> iced::widget::container::Style {
        iced::widget::container::Style {
            background: Some(background_color(kind).into()),
            text_color: Some(TEXT_COLOR),
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: border_radius::MD.into(),
            },
            shadow: shadows::shadow_lg(),
            ..Default::default()
        }
    }

    /// Returns progress bar color for countdown indicator
    ///
    /// Uses semi-transparent white overlay for visibility on any background.
    pub const PROGRESS_COLOR: Color = Color::from_rgba(1.0, 1.0, 1.0, 0.3);

    /// Returns progress bar background color
    pub const PROGRESS_BG: Color = Color::from_rgba(0.0, 0.0, 0.0, 0.2);
}

/// Tooltip styling utilities
///
/// Provides consistent styling for tooltips following ui-ux-pro-max
/// discoverability patterns.
///
/// # Tooltip Specifications
///
/// - **Max width**: 250px to prevent overly long tooltips
/// - **Background**: Dark semi-transparent for contrast on any surface
/// - **Text**: White for WCAG AAA contrast ratio
/// - **Font size**: `SIZE_SM` (14px) for readability without obtrusiveness
/// - **Padding**: 8px horizontal, 12px vertical for comfortable spacing
/// - **Border radius**: `RADIUS_SM` (4px) for consistency
/// - **Shadow**: `shadow_md` for elevation
pub mod tooltip_styles {
    use super::{border_radius, shadows, Color};

    /// Maximum tooltip width in pixels
    pub const MAX_WIDTH: f32 = 250.0;

    /// Tooltip background color (dark semi-transparent)
    pub const BACKGROUND_COLOR: Color = Color::from_rgba(0.0, 0.0, 0.0, 0.9);

    /// Tooltip text color (white for high contrast)
    pub const TEXT_COLOR: Color = Color::WHITE;

    /// Tooltip vertical padding in pixels.
    pub const PADDING_VERTICAL: f32 = 8.0;
    /// Tooltip horizontal padding in pixels.
    pub const PADDING_HORIZONTAL: f32 = 12.0;

    /// Returns container style for tooltip
    #[must_use]
    pub fn container_style(_theme: &iced::Theme) -> iced::widget::container::Style {
        use super::Border;

        iced::widget::container::Style {
            background: Some(BACKGROUND_COLOR.into()),
            text_color: Some(TEXT_COLOR),
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: border_radius::SM.into(),
            },
            shadow: shadows::shadow_md(),
            ..Default::default()
        }
    }
}
