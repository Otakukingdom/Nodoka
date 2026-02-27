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
    /// Uses primary brand color with white text
    /// Provides hover/pressed/focus states with enhanced micro-interactions
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
                shadow: shadows::shadow_sm(),
                ..Default::default()
            },
            button::Status::Hovered => button::Style {
                background: Some(colors::PRIMARY_GRADIENT_START.into()),
                text_color: colors::TEXT_ON_PRIMARY,
                border: Border {
                    color: colors::ACCENT_GRADIENT_END,
                    width: 1.0,
                    radius: border_radius::MD.into(),
                },
                shadow: shadows::glow_sunset(0.4), // Enhanced glow
                ..Default::default()
            },
            button::Status::Pressed => button::Style {
                background: Some(colors::PRIMARY_ACTIVE.into()),
                text_color: colors::TEXT_ON_PRIMARY,
                border: Border {
                    color: colors::PRIMARY_GRADIENT_END,
                    width: 1.5,
                    radius: border_radius::MD.into(),
                },
                shadow: shadows::shadow_xs(),
                ..Default::default()
            },
            button::Status::Disabled => button::Style {
                background: Some(Color {
                    r: colors::PRIMARY.r * 0.6,
                    g: colors::PRIMARY.g * 0.6,
                    b: colors::PRIMARY.b * 0.6,
                    a: 0.6,
                }.into()),
                text_color: Color {
                    r: colors::TEXT_ON_PRIMARY.r,
                    g: colors::TEXT_ON_PRIMARY.g,
                    b: colors::TEXT_ON_PRIMARY.b,
                    a: 0.6,
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

    /// Secondary action button style (e.g., Cancel, Close, Stop)
    /// Uses elevated background with primary text color and border
    /// Enhanced with refined micro-interactions for premium feel
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
                shadow: shadows::shadow_xs(),
                ..Default::default()
            },
            button::Status::Hovered => button::Style {
                background: Some(colors::BG_TIER.into()),
                text_color: colors::TEXT_PRIMARY,
                border: Border {
                    color: colors::PRIMARY_GRADIENT_START,
                    width: 1.0,
                    radius: border_radius::MD.into(),
                },
                shadow: shadows::shadow_sm(),
                ..Default::default()
            },
            button::Status::Pressed => button::Style {
                background: Some(colors::SURFACE_ACTIVE.into()),
                text_color: colors::TEXT_PRIMARY,
                border: Border {
                    color: colors::BORDER_FOCUS,
                    width: 2.0,
                    radius: border_radius::MD.into(),
                },
                shadow: shadows::shadow_xs(),
                ..Default::default()
            },
            button::Status::Disabled => button::Style {
                background: Some(Color {
                    r: colors::BG_ELEVATED.r,
                    g: colors::BG_ELEVATED.g,
                    b: colors::BG_ELEVATED.b,
                    a: 0.7,
                }.into()),
                text_color: colors::TEXT_DISABLED,
                border: Border {
                    color: Color {
                        r: colors::BORDER_DEFAULT.r,
                        g: colors::BORDER_DEFAULT.g,
                        b: colors::BORDER_DEFAULT.b,
                        a: 0.5,
                    },
                    width: 1.0,
                    radius: border_radius::MD.into(),
                },
                ..Default::default()
            },
        }
    }

    /// Elevated action button style (e.g., Primary CTAs, prominent actions)
    /// Uses shadow elevation with gradient background for prominence
    /// Enhanced with premium micro-interactions
    #[must_use]
    pub fn elevated(_theme: &iced::Theme, status: button::Status) -> button::Style {
        match status {
            button::Status::Active => button::Style {
                background: Some(colors::PRIMARY_GRADIENT_START.into()),
                text_color: colors::TEXT_ON_PRIMARY,
                border: Border {
                    color: colors::PRIMARY_GRADIENT_END,
                    width: 1.0,
                    radius: border_radius::MD.into(),
                },
                shadow: shadows::shadow_md(),
                ..Default::default()
            },
            button::Status::Hovered => button::Style {
                background: Some(colors::ACCENT_GRADIENT_START.into()),
                text_color: colors::TEXT_ON_PRIMARY,
                border: Border {
                    color: colors::ACCENT_GRADIENT_END,
                    width: 1.0,
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
                    width: 1.5,
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
    /// Uses error/danger color with white text
    /// Enhanced with red gradient for better visual feedback
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
                shadow: shadows::shadow_sm(),
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
                shadow: shadows::shadow_md(),
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

/// Empty state component styling utilities
///
/// Provides consistent styling for empty state components following
/// a clear visual hierarchy.
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
/// feedback patterns with semantic color variants.
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
    /// Enhanced with darker, premium colors for WCAG AA compliance (4.5:1 minimum)
    /// while maintaining distinctive character.
    #[must_use]
    pub const fn background_color(kind: &NotificationType) -> Color {
        match kind {
            // Success with darker emerald for WCAG AA compliance (4.5:1)
            NotificationType::Success => Color::from_rgb(0.02, 0.48, 0.26),
            // Info with deeper cool blue for WCAG AA compliance
            NotificationType::Info => Color::from_rgb(0.0, 0.35, 0.62),
            // Warning with deeper amber for WCAG AA compliance
            NotificationType::Warning => Color::from_rgb(0.58, 0.28, 0.0),
            // Error with deeper sunset red for WCAG AA compliance
            NotificationType::Error => Color::from_rgb(0.80, 0.10, 0.25),
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
/// Provides consistent styling for tooltips.
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

/// Premium input field styles for text inputs and search
///
/// Provides consistent styling for text input components with:
/// - Premium border colors with subtle rose tint
/// - Enhanced focus state with thicker border
/// - Background color that works in both themes
/// - Proper placeholder text color
/// - Consistent padding and border radius
pub mod input_styles {
    use super::{border_radius, colors, Border};

    /// Returns premium text input style with consistent visual treatment
    ///
    /// # Input Style Specifications
    ///
    /// - **Border width**: 2px for focused state, 1px for default
    /// - **Border radius**: Medium (8px) for soft appearance
    /// - **Background**: Elevated surface color
    /// - **Placeholder text**: Secondary color with reduced opacity
    ///
    /// # Examples
    ///
    /// ```ignore
    /// text_input("Search...", search_query)
    ///     .style(input_styles::text_input)
    /// ```
    #[must_use]
    pub fn text_input(_theme: &iced::Theme, status: iced::widget::text_input::Status) -> iced::widget::text_input::Style {
        match status {
            iced::widget::text_input::Status::Active
            | iced::widget::text_input::Status::Hovered => iced::widget::text_input::Style {
                background: iced::Background::Color(colors::BG_ELEVATED),
                border: Border {
                    color: colors::BORDER_SUBTLE,
                    width: 1.0,
                    radius: border_radius::MD.into(),
                },
                icon: colors::TEXT_SECONDARY,
                placeholder: colors::TEXT_DISABLED,
                value: colors::TEXT_PRIMARY,
                selection: colors::SELECTION_BG,
            },
            iced::widget::text_input::Status::Focused { .. } => iced::widget::text_input::Style {
                background: iced::Background::Color(colors::BG_ELEVATED),
                border: Border {
                    color: colors::PRIMARY_GRADIENT_START, // Premium gradient border
                    width: 2.5, // Slightly thicker for emphasis
                    radius: border_radius::MD.into(),
                },
                icon: colors::TEXT_SECONDARY,
                placeholder: colors::TEXT_DISABLED,
                value: colors::TEXT_PRIMARY,
                selection: colors::SELECTION_BG,
            },
            iced::widget::text_input::Status::Disabled => iced::widget::text_input::Style {
                background: iced::Background::Color(iced::Color {
                    r: colors::BG_ELEVATED.r,
                    g: colors::BG_ELEVATED.g,
                    b: colors::BG_ELEVATED.b,
                    a: 0.6,
                }),
                border: Border {
                    color: colors::BORDER_SUBTLE,
                    width: 1.0,
                    radius: border_radius::MD.into(),
                },
                icon: colors::TEXT_DISABLED,
                placeholder: colors::TEXT_DISABLED,
                value: colors::TEXT_DISABLED,
                selection: colors::SELECTION_BG,
            },
        }
    }
}

/// Premium card style system for consistent elevation and depth
///
/// Provides multiple card variants for different use cases:
/// - `card_default()` - Standard card with subtle elevation
/// - `card_elevated()` - Prominent card with stronger shadow
/// - `card_interactive()` - Card with hover state support
/// - `card_selected()` - Card with active/selected styling
/// - `card_premium()` - Card with rose-tinted shadow for brand presence
/// - `card_featured()` - Card with maximum elevation and accent border
///
/// # Card Style Hierarchy
///
/// Cards use elevation through shadows to establish visual hierarchy:
/// - **Default**: Small shadow (1px offset, 2px blur)
/// - **Elevated**: Medium shadow (2px offset, 4px blur)
/// - **Interactive**: Medium shadow with hover transition
/// - **Selected**: Large shadow (4px offset, 8px blur) with accent border
/// - **Premium**: Rose-tinted shadow for distinctive brand presence
/// - **Featured**: Maximum elevation with gold-tinted shadow
pub mod card_styles {
    use super::{border_radius, colors, shadows, Border};

    /// Standard card style with subtle elevation
    ///
    /// Use for standard content cards that need mild elevation from the background.
    /// Provides subtle background tint with border for clear separation.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// container(content)
    ///     .style(move |_theme| card_styles::card_default()(_theme))
    /// ```
    #[must_use = "card style must be applied to a container"]
    pub fn card_default() -> impl Fn(&iced::Theme) -> iced::widget::container::Style {
        move |_theme: &iced::Theme| iced::widget::container::Style {
            background: Some(colors::BG_ELEVATED.into()),
            border: Border {
                color: colors::BORDER_SUBTLE,
                width: 1.0,
                radius: border_radius::MD.into(),
            },
            shadow: shadows::shadow_sm(),
            ..Default::default()
        }
    }

    /// Elevated card style for prominent elements
    ///
    /// Use for featured content or important cards that need stronger elevation.
    /// Provides enhanced shadow and accent border for visual prominence.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// container(featured_content)
    ///     .style(move |_theme| card_styles::card_elevated()(_theme))
    /// ```
    #[must_use = "card style must be applied to a container"]
    pub fn card_elevated() -> impl Fn(&iced::Theme) -> iced::widget::container::Style {
        move |_theme: &iced::Theme| iced::widget::container::Style {
            background: Some(colors::BG_RAISED.into()),
            border: Border {
                color: colors::PRIMARY_GRADIENT_START,
                width: 1.5,
                radius: border_radius::LG.into(),
            },
            shadow: shadows::shadow_lg(),
            ..Default::default()
        }
    }

    /// Interactive card style with hover state support
    ///
    /// Use for cards that respond to user interaction. Apply this style
    /// to a button wrapper for hover effects.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// button(container(content).style(card_styles::card_default()))
    ///     .style(|_theme, status| {
    ///         match status {
    ///             button::Status::Hovered => card_styles::card_interactive_hover()(_theme),
    ///             _ => button::Style::default(),
    ///         }
    ///     })
    /// ```
    #[must_use = "card style must be applied to a container"]
    pub fn card_interactive() -> impl Fn(&iced::Theme) -> iced::widget::container::Style {
        move |_theme: &iced::Theme| iced::widget::container::Style {
            background: Some(colors::BG_ELEVATED.into()),
            border: Border {
                color: colors::BORDER_PROMINENT,
                width: 1.0,
                radius: border_radius::MD.into(),
            },
            shadow: shadows::shadow_md(),
            ..Default::default()
        }
    }

    /// Interactive card hover state with enhanced styling
    ///
    /// Apply to button wrapper when card is in hovered state.
    /// Provides gradient background and enhanced shadow for feedback.
    #[must_use = "card style must be applied to a container"]
    pub fn card_interactive_hover() -> impl Fn(&iced::Theme) -> iced::widget::container::Style {
        move |_theme: &iced::Theme| iced::widget::container::Style {
            background: Some(colors::SURFACE_HOVER.into()),
            border: Border {
                color: colors::PRIMARY_GRADIENT_START,
                width: 1.5,
                radius: border_radius::MD.into(),
            },
            shadow: shadows::shadow_md(),
            ..Default::default()
        }
    }

    /// Selected card style with accent border and elevation
    ///
    /// Use for currently selected items or active cards. Provides maximum
    /// elevation with large shadow and distinctive accent border.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// container(content)
    ///     .style(move |_theme| card_styles::card_selected()(_theme))
    /// ```
    #[must_use = "card style must be applied to a container"]
    pub fn card_selected() -> impl Fn(&iced::Theme) -> iced::widget::container::Style {
        move |_theme: &iced::Theme| iced::widget::container::Style {
            background: Some(colors::BG_RAISED.into()),
            border: Border {
                color: colors::ACCENT_GRADIENT_END,
                width: 2.5,
                radius: border_radius::MD.into(),
            },
            shadow: shadows::shadow_lg(),
            ..Default::default()
        }
    }

    /// Premium card with rose-tinted shadow for distinctive brand presence
    ///
    /// Use for branded content that needs subtle brand color integration.
    /// Provides rose-tinted shadow for distinctive character while maintaining elegance.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// container(branded_content)
    ///     .style(move |_theme| card_styles::card_premium()(_theme))
    /// ```
    #[must_use = "card style must be applied to a container"]
    pub fn card_premium() -> impl Fn(&iced::Theme) -> iced::widget::container::Style {
        move |_theme: &iced::Theme| iced::widget::container::Style {
            background: Some(colors::BG_RAISED.into()),
            border: Border {
                color: colors::PRIMARY_GRADIENT_START,
                width: 1.5,
                radius: border_radius::MD.into(),
            },
            shadow: shadows::shadow_md_brand(),
            ..Default::default()
        }
    }

    /// Featured card with maximum elevation and accent border
    ///
    /// Use for hero content, featured items, or elements requiring maximum prominence.
    /// Provides strongest elevation with extra-large shadow and accent color border.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// container(featured_hero_content)
    ///     .style(move |_theme| card_styles::card_featured()(_theme))
    /// ```
    #[must_use = "card style must be applied to a container"]
    pub fn card_featured() -> impl Fn(&iced::Theme) -> iced::widget::container::Style {
        move |_theme: &iced::Theme| iced::widget::container::Style {
            background: Some(colors::BG_ELEVATED.into()),
            border: Border {
                color: colors::ACCENT_GRADIENT_END,
                width: 2.0,
                radius: border_radius::LG.into(),
            },
            shadow: shadows::shadow_xl(),
            ..Default::default()
        }
    }
}

/// Premium badge styling system for distinctive visual accents
///
/// Provides badge variants with gradient backgrounds for status indicators,
/// metadata tags, and featured content labels.
///
/// # Badge Variants
///
/// - **Primary**: Brand rose gradient for primary badges
/// - **Accent**: Vibrant rose-to-magenta for CTAs
/// - **Success**: Emerald green for positive indicators
/// - **Warning**: Gold amber for caution states
/// - **Error**: Sunset red for negative feedback
///
/// # Badge Sizes
///
/// - **Compact**: Smaller padding for tight spaces
/// - **Default**: Standard padding for general use
/// - **Prominent**: Larger padding for emphasis
pub mod badge_styles {
    use super::{border_radius, colors, shadows, Border, Color};

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
}

/// Divider/separator styling utilities
///
/// Provides consistent styling for visual separators between content sections.
///
/// # Divider Types
///
/// - **Subtle**: Minimal divider with default border color
/// - **Prominent**: More visible divider with thicker border
/// - **Gradient**: Distinctive divider with brand color gradient
pub mod divider_styles {
    use super::colors;

    /// Returns border style for subtle horizontal divider
    ///
    /// Use for standard content separation where visual hierarchy should be minimal.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// container(Space::new())
    ///     .height(1.0)
    ///     .style(|_theme| container::Style {
    ///         border: divider_styles::horizontal(),
    ///         ..Default::default()
    ///     })
    /// ```
    #[must_use]
    pub fn horizontal() -> iced::Border {
        iced::Border {
            color: colors::BORDER_SUBTLE,
            width: 1.0,
            radius: 0.0.into(),
        }
    }

    /// Returns border style for prominent divider
    ///
    /// Use for major section separation where stronger visual hierarchy is needed.
    #[must_use]
    pub fn prominent() -> iced::Border {
        iced::Border {
            color: colors::BORDER_PROMINENT,
            width: 1.5,
            radius: 0.0.into(),
        }
    }

    /// Returns gradient border for distinctive divider
    ///
    /// Use for brand-accented dividers that need distinctive character.
    #[must_use]
    pub fn gradient() -> iced::Border {
        iced::Border {
            color: colors::PRIMARY_GRADIENT_START,
            width: 1.0,
            radius: 0.0.into(),
        }
    }

    /// Returns vertical divider border
    ///
    /// Use for horizontal content separation in side-by-side layouts.
    #[must_use]
    pub fn vertical() -> iced::Border {
        iced::Border {
            color: colors::BORDER_SUBTLE,
            width: 1.0,
            radius: 0.0.into(),
        }
    }
}
