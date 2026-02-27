//! Premium card style system for consistent elevation and depth
//!
//! Provides multiple card variants for different use cases with hover states.

use crate::ui::styles::{border_radius, colors, shadows, Border, Color};

/// Premium card style system for consistent elevation and depth
///
/// Provides multiple card variants for different use cases:
/// - `card_default()` - Standard card with subtle elevation
/// - `card_default_hover()` - Hover state for default cards
/// - `card_elevated()` - Prominent card with stronger shadow
/// - `card_elevated_hover()` - Hover state for elevated cards
/// - `card_interactive()` - Card with hover state support
/// - `card_interactive_hover()` - Hover state for interactive cards
/// - `card_selected()` - Card with active/selected styling
/// - `card_selected_hover()` - Hover state for selected cards
/// - `card_premium()` - Card with rose-tinted shadow for brand presence
/// - `card_premium_hover()` - Hover state for premium cards
/// - `card_featured()` - Card with maximum elevation and accent border
///
/// # Card Style Hierarchy
///
/// Cards use elevation through shadows to establish visual hierarchy:
/// - **Default**: Small shadow (4px offset, 6px blur)
/// - **Elevated**: Medium shadow (8px offset, 12px blur) with gradient border
/// - **Interactive**: Medium shadow with hover transition and gradient highlight
/// - **Selected**: Large shadow (16px offset, 24px blur) with accent border
/// - **Premium**: Multi-layer rose-tinted shadow with gradient background
/// - **Featured**: Maximum elevation with multi-layer shadow and premium gradient
///
///   Standard card style with subtle elevation
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

/// Default card hover state with subtle gradient border
///
/// Apply to button wrapper when default card is in hovered state.
/// Provides subtle gradient border on hover for liveliness.
///
/// # Examples
///
/// ```ignore
/// button(container(content).style(card_styles::card_default()))
///     .style(|_theme, status| {
///         match status {
///             button::Status::Hovered => card_styles::card_default_hover()(_theme),
///             _ => button::Style::default(),
///         }
///     })
/// ```
#[must_use = "card style must be applied to a container"]
pub fn card_default_hover() -> impl Fn(&iced::Theme) -> iced::widget::container::Style {
    move |_theme: &iced::Theme| iced::widget::container::Style {
        background: Some(colors::BG_ELEVATED.into()),
        border: Border {
            // Subtle gradient border for hover effect
            color: colors::PRIMARY_GRADIENT_START,
            width: 1.5,
            radius: border_radius::MD.into(),
        },
        shadow: shadows::shadow_md(),
        ..Default::default()
    }
}

/// Elevated card style for prominent elements
///
/// Use for featured content or important cards that need stronger elevation.
/// Provides enhanced shadow with gradient border for visual prominence.
/// Uses rose-tinted shadow for brand presence.
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
        background: Some(
            {
                // Subtle gradient tint for elevated appearance
                let primary = colors::PRIMARY_GRADIENT_START;
                let bg = colors::BG_RAISED;
                Color::from_rgb(
                    primary.r.mul_add(0.15, bg.r * 0.85),
                    primary.g.mul_add(0.15, bg.g * 0.85),
                    primary.b.mul_add(0.15, bg.b * 0.85),
                )
            }
            .into(),
        ),
        border: Border {
            color: colors::PRIMARY_GRADIENT_START,
            width: 1.5,
            radius: border_radius::LG.into(),
        },
        // Rose-tinted shadow for brand presence
        shadow: shadows::shadow_lg_brand(),
        ..Default::default()
    }
}

/// Elevated card hover state with enhanced elevation
///
/// Apply to button wrapper when elevated card is in hovered state.
/// Provides enhanced shadow and brighter gradient border for premium feedback.
///
/// # Examples
///
/// ```ignore
/// button(container(featured_content).style(card_styles::card_elevated()))
///     .style(|_theme, status| {
///         match status {
///             button::Status::Hovered => card_styles::card_elevated_hover()(_theme),
///             _ => button::Style::default(),
///         }
///     })
/// ```
#[must_use = "card style must be applied to a container"]
pub fn card_elevated_hover() -> impl Fn(&iced::Theme) -> iced::widget::container::Style {
    move |_theme: &iced::Theme| iced::widget::container::Style {
        background: Some(
            {
                // Brighter gradient tint for hover effect
                let primary = colors::PRIMARY_GRADIENT_START;
                let bg = colors::BG_RAISED;
                Color::from_rgb(
                    primary.r.mul_add(0.25, bg.r * 0.75),
                    primary.g.mul_add(0.25, bg.g * 0.75),
                    primary.b.mul_add(0.25, bg.b * 0.75),
                )
            }
            .into(),
        ),
        border: Border {
            color: colors::PRIMARY_GRADIENT_END,
            width: 2.0,
            radius: border_radius::LG.into(),
        },
        // Use shadow_xl for elevation (rose-tinted via glow_premium)
        shadow: shadows::shadow_xl(),
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
/// Provides gradient background and enhanced shadow for premium feedback.
#[must_use = "card style must be applied to a container"]
pub fn card_interactive_hover() -> impl Fn(&iced::Theme) -> iced::widget::container::Style {
    move |_theme: &iced::Theme| iced::widget::container::Style {
        background: Some(
            {
                // Enhanced gradient tint for interactive hover
                let primary = colors::PRIMARY_GRADIENT_START;
                let hover = colors::SURFACE_HOVER;
                Color::from_rgb(
                    primary.r.mul_add(0.25, hover.r * 0.75),
                    primary.g.mul_add(0.25, hover.g * 0.75),
                    primary.b.mul_add(0.25, hover.b * 0.75),
                )
            }
            .into(),
        ),
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
/// elevation with large shadow and distinctive accent border with gradient tint.
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
        background: Some(
            {
                // Subtle accent gradient tint for selected state
                let accent = colors::ACCENT_GRADIENT_START;
                let bg = colors::BG_RAISED;
                Color::from_rgb(
                    accent.r.mul_add(0.20, bg.r * 0.80),
                    accent.g.mul_add(0.20, bg.g * 0.80),
                    accent.b.mul_add(0.20, bg.b * 0.80),
                )
            }
            .into(),
        ),
        border: Border {
            color: colors::ACCENT_GRADIENT_END,
            width: 2.5,
            radius: border_radius::MD.into(),
        },
        shadow: shadows::shadow_lg(),
        ..Default::default()
    }
}

/// Selected card hover state with enhanced gradient treatment
///
/// Apply to button wrapper when selected card is in hovered state.
/// Provides enhanced elevation and gradient glow effect.
///
/// # Examples
///
/// ```ignore
/// button(container(content).style(card_styles::card_selected()))
///     .style(|_theme, status| {
///         match status {
///             button::Status::Hovered => card_styles::card_selected_hover()(_theme),
///             _ => button::Style::default(),
///         }
///     })
/// ```
#[must_use = "card style must be applied to a container"]
pub fn card_selected_hover() -> impl Fn(&iced::Theme) -> iced::widget::container::Style {
    move |_theme: &iced::Theme| iced::widget::container::Style {
        background: Some(
            {
                // Enhanced accent gradient tint for hover state
                let accent = colors::ACCENT_GRADIENT_START;
                let primary = colors::PRIMARY_GRADIENT_START;
                let bg = colors::BG_RAISED;
                Color::from_rgb(
                    accent.r.mul_add(0.3, primary.r.mul_add(0.2, bg.r * 0.5)),
                    accent.g.mul_add(0.3, primary.g.mul_add(0.2, bg.g * 0.5)),
                    accent.b.mul_add(0.3, primary.b.mul_add(0.2, bg.b * 0.5)),
                )
            }
            .into(),
        ),
        border: Border {
            color: colors::PRIMARY_GRADIENT_START,
            width: 3.0,
            radius: border_radius::MD.into(),
        },
        // Enhanced multi-layer shadow with rose glow
        shadow: shadows::glow_premium(0.7),
        ..Default::default()
    }
}

/// Premium card with rose-tinted shadow and gradient background
///
/// Use for branded content that needs subtle brand color integration.
/// Provides rose-tinted shadow with gradient background for distinctive character.
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
        // Multi-layer gradient background for premium appearance
        background: Some(
            {
                // Blend sunset and primary gradients for sophisticated rose tone
                let sunset = colors::SUNSET_GRADIENT_START;
                let primary = colors::PRIMARY_GRADIENT_START;
                Color::from_rgb(
                    primary.r.mul_add(0.5, sunset.r * 0.5),
                    primary.g.mul_add(0.5, sunset.g * 0.5),
                    primary.b.mul_add(0.5, sunset.b * 0.5),
                )
            }
            .into(),
        ),
        border: Border {
            color: colors::PRIMARY_GRADIENT_END,
            width: 2.0,
            radius: border_radius::MD.into(),
        },
        // Use shadow_lg with rose-tinted glow for premium brand presence
        shadow: shadows::shadow_lg_brand(),
        ..Default::default()
    }
}

/// Premium card hover state with enhanced elevation and gradient border
///
/// Apply to button wrapper when premium card is in hovered state.
/// Provides enhanced shadow with sunset gradient border for premium feedback.
///
/// # Examples
///
/// ```ignore
/// button(container(branded_content).style(card_styles::card_premium()))
///     .style(|_theme, status| {
///         match status {
///             button::Status::Hovered => card_styles::card_premium_hover()(_theme),
///             _ => button::Style::default(),
///         }
///     })
/// ```
#[must_use = "card style must be applied to a container"]
pub fn card_premium_hover() -> impl Fn(&iced::Theme) -> iced::widget::container::Style {
    move |_theme: &iced::Theme| iced::widget::container::Style {
        // Enhanced multi-layer gradient background for hover state
        background: Some(
            {
                // Brighter blend for hover effect with increased saturation
                let sunset = colors::SUNSET_GRADIENT_START;
                let primary = colors::PRIMARY_GRADIENT_START;
                let accent = colors::ACCENT_GRADIENT_START;
                Color::from_rgb(
                    accent
                        .r
                        .mul_add(0.2, sunset.r.mul_add(0.4, primary.r * 0.4)),
                    accent
                        .g
                        .mul_add(0.2, sunset.g.mul_add(0.4, primary.g * 0.4)),
                    accent
                        .b
                        .mul_add(0.2, sunset.b.mul_add(0.4, primary.b * 0.4)),
                )
            }
            .into(),
        ),
        border: Border {
            // Sunset gradient border for distinctive hover appearance
            color: colors::SUNSET_GRADIENT_END,
            width: 2.5,
            radius: border_radius::MD.into(),
        },
        // Enhanced multi-layer shadow with rose glow for dramatic elevation
        shadow: shadows::glow_premium(0.7),
        ..Default::default()
    }
}

/// Featured card with maximum elevation and premium gradient background
///
/// Use for hero content, featured items, or elements requiring maximum prominence.
/// Provides strongest elevation with extra-large shadow and premium gradient border.
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
        // Multi-layer gradient background for dramatic featured appearance
        background: Some(
            {
                // Blend sunset, primary, and accent gradients for maximum impact
                let sunset = colors::SUNSET_GRADIENT_START;
                let primary = colors::PRIMARY_GRADIENT_START;
                let accent = colors::ACCENT_GRADIENT_START;
                Color::from_rgb(
                    accent
                        .r
                        .mul_add(0.25, sunset.r.mul_add(0.4, primary.r * 0.35)),
                    accent
                        .g
                        .mul_add(0.25, sunset.g.mul_add(0.4, primary.g * 0.35)),
                    accent
                        .b
                        .mul_add(0.25, sunset.b.mul_add(0.4, primary.b * 0.35)),
                )
            }
            .into(),
        ),
        border: Border {
            color: colors::ACCENT_GRADIENT_END,
            width: 2.5,
            radius: border_radius::LG.into(),
        },
        shadow: shadows::shadow_xl(),
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_default_returns_valid_style() {
        let style = card_default()(&iced::Theme::Light);
        // Just verify it doesn't panic
        let _ = style;
    }

    #[test]
    fn test_card_default_hover_returns_valid_style() {
        let style = card_default_hover()(&iced::Theme::Light);
        let _ = style;
    }

    #[test]
    fn test_card_elevated_returns_valid_style() {
        let style = card_elevated()(&iced::Theme::Light);
        let _ = style;
    }

    #[test]
    fn test_card_elevated_hover_returns_valid_style() {
        let style = card_elevated_hover()(&iced::Theme::Light);
        let _ = style;
    }

    #[test]
    fn test_card_interactive_returns_valid_style() {
        let style = card_interactive()(&iced::Theme::Light);
        let _ = style;
    }

    #[test]
    fn test_card_interactive_hover_returns_valid_style() {
        let style = card_interactive_hover()(&iced::Theme::Light);
        let _ = style;
    }

    #[test]
    fn test_card_selected_returns_valid_style() {
        let style = card_selected()(&iced::Theme::Light);
        let _ = style;
    }

    #[test]
    fn test_card_selected_hover_returns_valid_style() {
        let style = card_selected_hover()(&iced::Theme::Light);
        let _ = style;
    }

    #[test]
    fn test_card_premium_returns_valid_style() {
        let style = card_premium()(&iced::Theme::Light);
        let _ = style;
    }

    #[test]
    fn test_card_premium_hover_returns_valid_style() {
        let style = card_premium_hover()(&iced::Theme::Light);
        let _ = style;
    }

    #[test]
    fn test_card_featured_returns_valid_style() {
        let style = card_featured()(&iced::Theme::Light);
        let _ = style;
    }
}
