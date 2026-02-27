//! Volume controls with color-coded level indicators.

use crate::conversions::f64_to_ms;
use crate::ui::styles::{button_styles, colors, focus_indicator, spacing, typography};
use crate::ui::styles::slider;
use crate::ui::{Message, State};
use iced::widget::{button, mouse_area, row, container, slider as widget_slider, text};
use iced::{Color, Element};

/// Renders volume controls with expandable slider
pub fn volume_controls(state: &State, is_focused: bool) -> Element<'_, Message> {
    let volume_level = volume_to_level(state.volume);
    let volume_color = volume_level_color(volume_level);
    let gradient_color = slider::volume_slider::level_color_gradient_start(volume_level);

    let toggle = button(text("Vol").size(typography::SIZE_SM))
        .on_press(Message::ToggleVolumeExpanded)
        .padding(iced::Padding::from([spacing::XS, spacing::SM]))
        .style(button_styles::secondary);

    let slider_row: Element<'_, Message> = if state.volume_expanded {
        row![
            widget_slider(0..=200, state.volume, Message::VolumeChanged)
                .width(iced::Length::Fixed(120.0))
                .height(20.0)
                .style(move |_theme: &iced::Theme, _status: iced::widget::slider::Status| {
                    let level_color = slider::volume_slider::level_color(volume_level);
                    iced::widget::slider::Style {
                        rail: iced::widget::slider::Rail {
                            backgrounds: (
                                iced::Background::Color(colors::BG_SECONDARY),
                                iced::Background::Color(level_color),
                            ),
                            width: 4.0,
                            border: iced::Border {
                                color: Color::TRANSPARENT,
                                width: 0.0,
                                radius: 2.0.into(),
                            },
                        },
                        handle: iced::widget::slider::Handle {
                            shape: iced::widget::slider::HandleShape::Circle { radius: 8.0 },
                            background: iced::Background::Color(gradient_color),
                            border_width: 2.0,
                            border_color: colors::BORDER_DEFAULT,
                        },
                    }
                }),
            text(format!("{}%", state.volume))
                .size(typography::SIZE_SM)
                .style(move |_theme: &iced::Theme| iced::widget::text::Style {
                    color: Some(volume_color),
                }),
        ]
        .spacing(spacing::XS)
        .align_y(iced::Alignment::Center)
        .into()
    } else {
        text(format!("{}%", state.volume))
            .size(typography::SIZE_SM)
            .style(move |_theme: &iced::Theme| iced::widget::text::Style {
                color: Some(volume_color),
            })
            .into()
    };

    let content = container(
        row![toggle, slider_row]
            .spacing(spacing::SM)
            .align_y(iced::Alignment::Center),
    )
    .style(focus_indicator(is_focused));

    mouse_area(content)
        .on_enter(Message::TooltipHoverStart(
            crate::ui::tooltip::TooltipTarget::Volume,
        ))
        .on_exit(Message::TooltipHoverEnd)
        .into()
}

/// Converts volume percentage (0-200) to level (0.0-1.0)
pub fn volume_to_level(volume: i32) -> f32 {
    let clamped = volume.clamp(0, 200);
    let as_u16 = u16::try_from(clamped).unwrap_or(0);
    f32::from(as_u16) / 200.0
}

/// Returns color-coded volume level indicator
///
/// - High volume (70%+): Success green
/// - Medium volume (30-70%): Warning yellow
/// - Low volume (1-30%): Error red
/// - Muted (0%): Disabled gray
#[must_use]
pub fn volume_level_color(volume: f32) -> iced::Color {
    if volume >= 0.7 {
        colors::SUCCESS // High volume: green
    } else if volume >= 0.3 {
        colors::WARNING // Medium volume: yellow
    } else if volume > 0.0 {
        colors::ERROR // Low volume: red
    } else {
        colors::TEXT_DISABLED // Muted: gray
    }
}

/// Formats time in milliseconds for display (H:MM:SS or M:SS format)
#[must_use]
pub fn format_time(ms: f64) -> String {
    // Convert f64 milliseconds to i64 for time calculations
    let ms_i64 = f64_to_ms(ms).unwrap_or(0);
    let seconds = ms_i64 / 1000;
    let minutes = seconds / 60;
    let hours = minutes / 60;

    if hours > 0 {
        format!("{}:{:02}:{:02}", hours, minutes % 60, seconds % 60)
    } else {
        format!("{}:{:02}", minutes, seconds % 60)
    }
}

/// Formats remaining time with a "-" prefix to indicate time left
#[must_use]
pub fn format_time_remaining(ms: f64) -> String {
    let ms_i64 = f64_to_ms(ms).unwrap_or(0).max(0);
    let seconds = ms_i64 / 1000;
    let minutes = seconds / 60;
    let hours = minutes / 60;

    if hours > 0 {
        format!("-{}:{:02}:{:02}", hours, minutes % 60, seconds % 60)
    } else {
        format!("-{}:{:02}", minutes, seconds % 60)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_time_with_hours() {
        assert_eq!(format_time(3_661_000.0), "1:01:01");
        assert_eq!(format_time(7_200_000.0), "2:00:00");
        assert_eq!(format_time(3_723_000.0), "1:02:03");
    }

    #[test]
    fn test_format_time_with_minutes_only() {
        assert_eq!(format_time(125_000.0), "2:05");
        assert_eq!(format_time(60000.0), "1:00");
        assert_eq!(format_time(0.0), "0:00");
    }

    #[test]
    fn test_format_time_handles_zero() {
        assert_eq!(format_time(0.0), "0:00");
    }

    #[test]
    fn test_format_time_handles_negative() {
        // Negative values should be handled gracefully
        let result = format_time(-1000.0);
        assert!(result == "0:00" || result.starts_with('-'));
    }

    #[test]
    fn test_volume_to_level() {
        assert_eq!(volume_to_level(0), 0.0);
        assert_eq!(volume_to_level(100), 0.5);
        assert_eq!(volume_to_level(200), 1.0);
    }

    #[test]
    fn test_volume_level_color() {
        // High volume: green
        assert!(matches!(volume_level_color(0.8), Color { g, .. } if g > 0.4));
        // Medium volume: yellow
        assert!(matches!(volume_level_color(0.5), Color { r, g, .. } if r > 0.7 && g > 0.4));
        // Low volume: red
        assert!(matches!(volume_level_color(0.1), Color { r, .. } if r > 0.7));
        // Muted: gray
        assert!(matches!(volume_level_color(0.0), Color { r, g, b, .. } if (r - g).abs() < 0.1));
    }
}
