//! Premium slider and progress bar styling utilities.
//!
//! This module provides enhanced styling for sliders and progress bars
//! with gradient fills, track decorations, and refined visual treatment.

use crate::ui::styles::colors;
use iced::widget::slider;
use iced::{Border, Color};

/// Premium progress slider style with enhanced gradient fill
#[must_use]
pub fn progress_premium(_theme: &iced::Theme, _status: slider::Status) -> slider::Style {
    let fill_color = Color::from_rgb(
        colors::ACCENT_GRADIENT_START.r.mul_add(
            0.3,
            colors::PRIMARY_GRADIENT_START
                .r
                .mul_add(0.35, colors::SUNSET_GRADIENT_START.r * 0.35),
        ),
        colors::ACCENT_GRADIENT_START.g.mul_add(
            0.3,
            colors::PRIMARY_GRADIENT_START
                .g
                .mul_add(0.35, colors::SUNSET_GRADIENT_START.g * 0.35),
        ),
        colors::ACCENT_GRADIENT_START.b.mul_add(
            0.3,
            colors::PRIMARY_GRADIENT_START
                .b
                .mul_add(0.35, colors::SUNSET_GRADIENT_START.b * 0.35),
        ),
    );

    slider::Style {
        rail: slider::Rail {
            backgrounds: (
                iced::Background::Color(colors::BG_SECONDARY),
                iced::Background::Color(fill_color),
            ),
            width: 10.0,
            border: Border {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.20),
                width: 1.5,
                radius: 5.0.into(),
            },
        },
        handle: slider::Handle {
            shape: slider::HandleShape::Circle { radius: 12.0 },
            background: iced::Background::Color(Color::from_rgb(
                colors::PRIMARY_GRADIENT_START
                    .r
                    .mul_add(0.4, colors::SUNSET_GRADIENT_START.r * 0.6),
                colors::PRIMARY_GRADIENT_START
                    .g
                    .mul_add(0.4, colors::SUNSET_GRADIENT_START.g * 0.6),
                colors::PRIMARY_GRADIENT_START
                    .b
                    .mul_add(0.4, colors::SUNSET_GRADIENT_START.b * 0.6),
            )),
            border_width: 2.5,
            border_color: colors::ACCENT_GRADIENT_END,
        },
    }
}

/// Premium progress bar styling with gradient effects
pub mod progress_bar {
    use crate::ui::styles::colors;
    use iced::widget;
    use iced::Color;

    /// Creates a progress bar style with gradient fill
    ///
    /// # Arguments
    /// * `value` - Current progress value (0.0 to 1.0)
    /// * `has_glow` - Whether to add a subtle glow effect
    pub fn style(
        _value: f32,
        has_glow: bool,
    ) -> impl Fn(&iced::Theme) -> widget::progress_bar::Style {
        let alpha = if has_glow { 0.95 } else { 0.8 };
        move |_theme: &iced::Theme| {
            widget::progress_bar::Style {
                // Set the bar fill with gradient effect
                bar: iced::Background::Color(Color::from_rgba(
                    colors::PROGRESS_GRADIENT_END.r,
                    colors::PROGRESS_GRADIENT_END.g,
                    colors::PROGRESS_GRADIENT_END.b,
                    alpha,
                )),
                // Background track
                background: iced::Background::Color(colors::BG_SECONDARY),
                // Border for the progress bar
                border: iced::Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: 2.0.into(),
                },
            }
        }
    }

    /// Creates a custom slider style with enhanced visual treatment
    pub fn slider_style(is_active: bool) -> impl Fn(&iced::Theme) -> widget::slider::Style {
        move |_theme: &iced::Theme| widget::slider::Style {
            rail: widget::slider::Rail {
                backgrounds: (
                    iced::Background::Color(Color::from_rgb(0.9, 0.9, 0.9)),
                    iced::Background::Color(Color::from_rgb(0.9, 0.9, 0.9)),
                ),
                width: 1.0,
                border: iced::Border {
                    color: Color::from_rgb(0.8, 0.8, 0.8),
                    width: 1.0,
                    radius: 2.0.into(),
                },
            },
            handle: widget::slider::Handle {
                shape: widget::slider::HandleShape::Circle { radius: 8.0 },
                background: if is_active {
                    iced::Background::Color(colors::PRIMARY)
                } else {
                    iced::Background::Color(Color::WHITE)
                },
                border_width: if is_active { 0.0 } else { 2.0 },
                border_color: if is_active {
                    Color::TRANSPARENT
                } else {
                    colors::BORDER_DEFAULT
                },
            },
        }
    }
}

/// Volume slider styling with color-coded gradients
pub mod volume_slider {
    use crate::ui::styles::colors;
    use iced::Color;

    /// Returns the color for a given volume level
    ///
    /// - High (70%+): Success green gradient
    /// - Medium (30-70%): Warning yellow gradient
    /// - Low (1-30%): Error red gradient
    /// - Muted (0%): Disabled gray
    #[must_use]
    pub fn level_color(level: f32) -> Color {
        if level >= 0.7 {
            colors::SUCCESS_REFINED
        } else if level >= 0.3 {
            colors::WARNING_REFINED
        } else if level > 0.0 {
            colors::ERROR
        } else {
            colors::TEXT_DISABLED
        }
    }

    /// Returns gradient start color for volume level
    #[must_use]
    pub fn level_color_gradient_start(level: f32) -> Color {
        if level >= 0.7 {
            colors::SUCCESS_LIGHT
        } else if level >= 0.3 {
            colors::WARNING_LIGHT
        } else if level > 0.0 {
            colors::ERROR_HOVER
        } else {
            Color::from_rgb(0.7, 0.7, 0.7)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_volume_level_color_returns_valid_colors() {
        let high = volume_slider::level_color(0.8);
        assert!(high.r >= 0.0 && high.r <= 1.0);

        let mid = volume_slider::level_color(0.5);
        assert!(mid.r >= 0.0 && mid.r <= 1.0);

        let low = volume_slider::level_color(0.2);
        assert!(low.r >= 0.0 && low.r <= 1.0);

        let muted = volume_slider::level_color(0.0);
        assert!(muted.r >= 0.0 && muted.r <= 1.0);
    }

    #[test]
    fn test_volume_level_gradient_returns_valid_colors() {
        let start = volume_slider::level_color_gradient_start(0.8);
        assert!(start.r >= 0.0 && start.r <= 1.0);

        let start = volume_slider::level_color_gradient_start(0.0);
        assert!(start.r >= 0.0 && start.r <= 1.0);
    }
}
