//! Component style modules for various UI elements.
//!
//! This file contains style modules for empty states, notifications,
//! tooltips, inputs, and dividers.

/// Empty state component styling utilities
pub mod empty_state_styles {
    use crate::ui::styles::{border_radius, colors, Border};
    use iced::widget::container;

    #[must_use]
    pub fn container_style(_theme: &iced::Theme) -> container::Style {
        container::Style {
            background: Some(colors::BG_SECONDARY.into()),
            border: Border {
                color: colors::BORDER_DEFAULT,
                width: 2.0,
                radius: border_radius::LG.into(),
            },
            ..Default::default()
        }
    }

    pub const ICON_COLOR: iced::Color = colors::TEXT_SECONDARY;
    pub const HEADING_COLOR: iced::Color = colors::TEXT_PRIMARY;
    pub const DESCRIPTION_COLOR: iced::Color = colors::TEXT_SECONDARY;
}

/// Notification toast styling utilities
pub mod notification_styles {
    use crate::ui::state::NotificationType;
    use crate::ui::styles::accessibility;
    use crate::ui::styles::{border_radius, colors, shadows, Border};
    use iced::widget::container;

    #[must_use]
    pub fn background_color(kind: &NotificationType) -> iced::Color {
        fn pick_readable_background(a: iced::Color, b: iced::Color) -> iced::Color {
            let ratio_a = accessibility::gradient::contrast_ratio(TEXT_COLOR, a);
            let ratio_b = accessibility::gradient::contrast_ratio(TEXT_COLOR, b);
            let mut bg = if ratio_a >= ratio_b { a } else { b };

            // Ensure WCAG AA contrast for normal text (4.5:1).
            // If the palette colors are too bright, darken towards black until readable.
            let mut t = 0.0_f32;
            while accessibility::gradient::contrast_ratio(TEXT_COLOR, bg)
                < accessibility::CONTRAST_RATIO_AA_NORMAL
                && t < 0.6
            {
                t = (t + 0.05).min(0.6);
                bg = iced::Color::from_rgb(
                    bg.r + (0.0 - bg.r) * t,
                    bg.g + (0.0 - bg.g) * t,
                    bg.b + (0.0 - bg.b) * t,
                );
            }

            bg
        }

        match kind {
            NotificationType::Success => pick_readable_background(
                colors::EMERALD_GRADIENT_START,
                colors::EMERALD_GRADIENT_END,
            ),
            NotificationType::Info => pick_readable_background(
                colors::AURORA_ROSE_GRADIENT_START,
                colors::AURORA_ROSE_GRADIENT_END,
            ),
            NotificationType::Warning => {
                pick_readable_background(colors::GOLD_GRADIENT_START, colors::GOLD_GRADIENT_END)
            }
            NotificationType::Error => {
                pick_readable_background(colors::SUNSET_GRADIENT_START, colors::SUNSET_GRADIENT_END)
            }
        }
    }

    pub const TEXT_COLOR: iced::Color = iced::Color::WHITE;
    pub const ICON_COLOR: iced::Color = iced::Color::WHITE;

    #[must_use]
    pub fn container_style(kind: &NotificationType) -> container::Style {
        container::Style {
            background: Some(background_color(kind).into()),
            text_color: Some(TEXT_COLOR),
            border: Border {
                color: iced::Color::TRANSPARENT,
                width: 0.0,
                radius: border_radius::MD.into(),
            },
            shadow: shadows::shadow_lg(),
            ..Default::default()
        }
    }

    pub const PROGRESS_COLOR: iced::Color = iced::Color::from_rgba(1.0, 1.0, 1.0, 0.3);
    pub const PROGRESS_BG: iced::Color = iced::Color::from_rgba(0.0, 0.0, 0.0, 0.2);
}

/// Tooltip styling utilities
pub mod tooltip_styles {
    use crate::ui::styles::{border_radius, shadows, Border};
    use iced::widget::container;

    pub const MAX_WIDTH: f32 = 250.0;
    pub const BACKGROUND_COLOR: iced::Color = iced::Color::from_rgba(0.0, 0.0, 0.0, 0.9);
    pub const TEXT_COLOR: iced::Color = iced::Color::WHITE;
    pub const PADDING_VERTICAL: f32 = 8.0;
    pub const PADDING_HORIZONTAL: f32 = 12.0;

    #[must_use]
    pub fn container_style(_theme: &iced::Theme) -> container::Style {
        container::Style {
            background: Some(BACKGROUND_COLOR.into()),
            text_color: Some(TEXT_COLOR),
            border: Border {
                color: iced::Color::TRANSPARENT,
                width: 0.0,
                radius: border_radius::SM.into(),
            },
            shadow: shadows::shadow_md(),
            ..Default::default()
        }
    }
}

/// Premium input field styles
pub mod input_styles {
    use crate::ui::styles::{border_radius, colors, Border};

    #[must_use]
    pub fn text_input_style(
        _theme: &iced::Theme,
        status: iced::widget::text_input::Status,
    ) -> iced::widget::text_input::Style {
        match status {
            iced::widget::text_input::Status::Focused { .. } => iced::widget::text_input::Style {
                background: iced::Background::Color(colors::BG_ELEVATED),
                border: Border {
                    color: colors::PRIMARY_GRADIENT_START,
                    width: 2.0,
                    radius: border_radius::MD.into(),
                },
                icon: colors::TEXT_SECONDARY,
                placeholder: colors::TEXT_DISABLED,
                value: colors::TEXT_PRIMARY,
                selection: colors::SELECTION_BG,
            },
            _ => iced::widget::text_input::Style {
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
        }
    }
}

/// Divider/separator styling utilities
pub mod divider_styles {
    use crate::ui::styles::colors;

    #[must_use]
    pub fn horizontal() -> iced::Border {
        iced::Border {
            color: colors::BORDER_SUBTLE,
            width: 1.0,
            radius: 0.0.into(),
        }
    }

    #[must_use]
    pub fn prominent() -> iced::Border {
        iced::Border {
            color: colors::BORDER_PROMINENT,
            width: 1.5,
            radius: 0.0.into(),
        }
    }

    #[must_use]
    pub fn gradient() -> iced::Border {
        iced::Border {
            color: colors::PRIMARY_GRADIENT_START,
            width: 1.0,
            radius: 0.0.into(),
        }
    }

    #[must_use]
    pub fn vertical() -> iced::Border {
        iced::Border {
            color: colors::BORDER_SUBTLE,
            width: 1.0,
            radius: 0.0.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_state_container_style_returns_valid() {
        let style = empty_state_styles::container_style(&iced::Theme::Light);
        let _ = style;
    }

    #[test]
    fn test_notification_background_success_returns_valid_color() {
        let color =
            notification_styles::background_color(&crate::ui::state::NotificationType::Success);
        assert!(color.r >= 0.0 && color.r <= 1.0);
    }

    #[test]
    fn test_tooltip_container_style_returns_valid() {
        let style = tooltip_styles::container_style(&iced::Theme::Light);
        let _ = style;
    }

    #[test]
    fn test_divider_horizontal_returns_valid() {
        let border = divider_styles::horizontal();
        assert!((border.width - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_input_text_input_style_focused_returns_valid() {
        let style = input_styles::text_input_style(
            &iced::Theme::Light,
            iced::widget::text_input::Status::Focused { is_hovered: false },
        );
        let _ = style;
    }
}
