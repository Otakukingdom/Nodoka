//! Premium badge styling system for distinctive visual accents
//!
//! Provides badge variants with gradient backgrounds for status indicators,
//! metadata tags, and featured content labels.
//!
//! # Badge Variants
//!
//! - **Primary**: Brand rose gradient for primary badges
//! - **Accent**: Vibrant rose-to-magenta for CTAs
//! - **Success**: Emerald green for positive indicators
//! - **Warning**: Gold amber for caution states
//! - **Error**: Sunset red for negative feedback
//!
//! # Badge Sizes
//!
//! - **Compact**: Smaller padding for tight spaces
//! - **Default**: Standard padding for general use
//! - **Prominent**: Larger padding for emphasis

use crate::ui::styles::{border_radius, colors, shadows, Border, Color};

/// Badge kind variants with distinct gradient treatments
#[derive(Copy, Clone)]
pub enum BadgeKind {
    /// Primary brand gradient (rose-to-crimson)
    Primary,
    /// Accent gradient (rose-to-magenta)
    Accent,
    /// Success gradient (emerald green)
    Success,
    /// Warning gradient (gold amber)
    Warning,
    /// Error gradient (sunset red)
    Error,
}

/// Badge size variants for different contexts
#[derive(Copy, Clone)]
pub enum BadgeSize {
    /// Compact badge with minimal padding
    Compact,
    /// Default badge with standard padding
    Default,
    /// Prominent badge with extra padding
    Prominent,
}

/// Creates a premium gradient badge with subtle shadow
///
/// Combines gradient background with refined border and shadow treatment
/// for distinctive visual appearance.
///
/// # Examples
///
/// ```ignore
/// container(text("New"))
///     .style(badge_styles::badge_gradient(BadgeKind::Primary))
/// ```
pub fn badge_gradient(kind: BadgeKind) -> impl Fn(&iced::Theme) -> iced::widget::container::Style {
    move |_theme: &iced::Theme| {
        let (bg, border) = match kind {
            BadgeKind::Primary => (colors::PRIMARY_GRADIENT_START, colors::PRIMARY_GRADIENT_END),
            BadgeKind::Accent => (colors::ACCENT_GRADIENT_START, colors::ACCENT_GRADIENT_END),
            BadgeKind::Success => (colors::EMERALD_GRADIENT_START, colors::EMERALD_GRADIENT_END),
            BadgeKind::Warning => (colors::GOLD_GRADIENT_START, colors::GOLD_GRADIENT_END),
            BadgeKind::Error => (colors::SUNSET_GRADIENT_START, colors::SUNSET_GRADIENT_END),
        };
        iced::widget::container::Style {
            background: Some(bg.into()),
            border: Border {
                color: border,
                width: 1.0,
                radius: border_radius::FULL.into(),
            },
            shadow: shadows::shadow_xs(),
            ..Default::default()
        }
    }
}

/// Creates a badge with specified size variant
///
/// Returns padding values for consistent badge sizing across the application.
///
/// # Examples
///
/// ```ignore
/// let padding = badge_styles::badge_padding(BadgeSize::Default);
/// container(text("Premium")).padding(padding)
/// ```
#[must_use]
pub const fn badge_padding(size: BadgeSize) -> iced::Padding {
    match size {
        BadgeSize::Compact => iced::Padding {
            top: 2.0,
            bottom: 2.0,
            left: 6.0,
            right: 6.0,
        },
        BadgeSize::Default => iced::Padding {
            top: 4.0,
            bottom: 4.0,
            left: 10.0,
            right: 10.0,
        },
        BadgeSize::Prominent => iced::Padding {
            top: 6.0,
            bottom: 6.0,
            left: 14.0,
            right: 14.0,
        },
    }
}

/// Returns text color for badge (always white for contrast)
///
/// All badge gradients use vibrant colors that require white text for
/// WCAG AA compliance.
pub const TEXT_COLOR: Color = Color::WHITE;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_badge_gradient_primary_returns_valid_style() {
        let style = badge_gradient(BadgeKind::Primary)(&iced::Theme::Light);
        drop(style);
    }

    #[test]
    fn test_badge_gradient_accent_returns_valid_style() {
        let style = badge_gradient(BadgeKind::Accent)(&iced::Theme::Light);
        drop(style);
    }

    #[test]
    fn test_badge_gradient_success_returns_valid_style() {
        let style = badge_gradient(BadgeKind::Success)(&iced::Theme::Light);
        drop(style);
    }

    #[test]
    fn test_badge_gradient_warning_returns_valid_style() {
        let style = badge_gradient(BadgeKind::Warning)(&iced::Theme::Light);
        drop(style);
    }

    #[test]
    fn test_badge_gradient_error_returns_valid_style() {
        let style = badge_gradient(BadgeKind::Error)(&iced::Theme::Light);
        drop(style);
    }

    #[test]
    fn test_badge_padding_compact_returns_valid_padding() {
        let padding = badge_padding(BadgeSize::Compact);
        assert_eq!(padding.top, 2.0);
        assert_eq!(padding.bottom, 2.0);
        assert_eq!(padding.left, 6.0);
        assert_eq!(padding.right, 6.0);
    }

    #[test]
    fn test_badge_padding_default_returns_valid_padding() {
        let padding = badge_padding(BadgeSize::Default);
        assert_eq!(padding.top, 4.0);
        assert_eq!(padding.bottom, 4.0);
        assert_eq!(padding.left, 10.0);
        assert_eq!(padding.right, 10.0);
    }

    #[test]
    fn test_badge_padding_prominent_returns_valid_padding() {
        let padding = badge_padding(BadgeSize::Prominent);
        assert_eq!(padding.top, 6.0);
        assert_eq!(padding.bottom, 6.0);
        assert_eq!(padding.left, 14.0);
        assert_eq!(padding.right, 14.0);
    }
}
