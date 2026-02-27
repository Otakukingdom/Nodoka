//! Playback speed controls with preset options.

use crate::ui::styles::{colors, focus_indicator, spacing, typography};
use crate::ui::{Message, State};
use iced::widget::{container, mouse_area, pick_list, row, text};
use iced::Element;

/// Playback speed preset options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpeedPreset {
    X075,
    X10,
    X125,
    X15,
    X20,
}

impl SpeedPreset {
    pub const ALL: [Self; 5] = [Self::X075, Self::X10, Self::X125, Self::X15, Self::X20];

    #[must_use]
    pub const fn as_f32(self) -> f32 {
        match self {
            Self::X075 => 0.75,
            Self::X10 => 1.0,
            Self::X125 => 1.25,
            Self::X15 => 1.5,
            Self::X20 => 2.0,
        }
    }

    #[must_use]
    pub fn from_speed(speed: f32) -> Option<Self> {
        Self::ALL
            .iter()
            .copied()
            .find(|preset| (speed - preset.as_f32()).abs() < 0.01)
    }
}

impl std::fmt::Display for SpeedPreset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = self.as_f32();
        let fixed = format!("{value:.2}");
        let trimmed = fixed.trim_end_matches('0').trim_end_matches('.');
        write!(f, "{trimmed}x")
    }
}

/// Renders speed controls with preset dropdown
#[must_use]
pub fn speed_controls(state: &State, _speed_step: i32, is_focused: bool) -> Element<'_, Message> {
    let selected = SpeedPreset::from_speed(state.speed);
    let picker = pick_list(SpeedPreset::ALL, selected, |preset| {
        Message::SpeedChanged(preset.as_f32())
    })
    .placeholder("Speed")
    .width(iced::Length::Fixed(140.0));

    // Enhanced gradient color based on speed level
    let speed_gradient_color = if state.speed >= 1.5 {
        // Fast speeds: accent gradient
        iced::Color::from_rgb(
            colors::ACCENT_GRADIENT_START
                .r
                .mul_add(0.4, colors::PRIMARY_GRADIENT_START.r * 0.6),
            colors::ACCENT_GRADIENT_START
                .g
                .mul_add(0.4, colors::PRIMARY_GRADIENT_START.g * 0.6),
            colors::ACCENT_GRADIENT_START
                .b
                .mul_add(0.4, colors::PRIMARY_GRADIENT_START.b * 0.6),
        )
    } else if state.speed >= 1.0 {
        // Normal speeds: primary gradient
        iced::Color::from_rgb(
            colors::PRIMARY_GRADIENT_START
                .r
                .mul_add(0.5, colors::SIGNATURE_GRADIENT_START.r * 0.5),
            colors::PRIMARY_GRADIENT_START
                .g
                .mul_add(0.5, colors::SIGNATURE_GRADIENT_START.g * 0.5),
            colors::PRIMARY_GRADIENT_START
                .b
                .mul_add(0.5, colors::SIGNATURE_GRADIENT_START.b * 0.5),
        )
    } else {
        // Slow speeds: softer aurora gradient
        iced::Color::from_rgb(
            colors::AURORA_ROSE_GRADIENT_START
                .r
                .mul_add(0.4, colors::CELESTIAL_GRADIENT_START.r * 0.6),
            colors::AURORA_ROSE_GRADIENT_START
                .g
                .mul_add(0.4, colors::CELESTIAL_GRADIENT_START.g * 0.6),
            colors::AURORA_ROSE_GRADIENT_START
                .b
                .mul_add(0.4, colors::CELESTIAL_GRADIENT_START.b * 0.6),
        )
    };

    let content = container(
        row![
            text("Speed:").size(typography::SIZE_SM),
            picker,
            text(format_speed_label(state.speed))
                .size(typography::SIZE_SM)
                .style(move |_theme: &iced::Theme| iced::widget::text::Style {
                    color: Some(speed_gradient_color),
                }),
        ]
        .spacing(spacing::SM)
        .align_y(iced::Alignment::Center),
    )
    .style(focus_indicator(is_focused));

    mouse_area(content)
        .on_enter(Message::TooltipHoverStart(
            crate::ui::tooltip::TooltipTarget::Speed,
        ))
        .on_exit(Message::TooltipHoverEnd)
        .into()
}

/// Formats speed value as "1.5x" string
#[must_use]
pub fn format_speed_label(speed: f32) -> String {
    let fixed = format!("{speed:.2}");
    let trimmed = fixed.trim_end_matches('0').trim_end_matches('.');
    format!("{trimmed}x")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_speed_preset_values() {
        const EPSILON: f32 = 0.001;
        assert!(
            (SpeedPreset::X075.as_f32() - 0.75).abs() < EPSILON,
            "Speed preset 0.75x mismatch"
        );
        assert!(
            (SpeedPreset::X10.as_f32() - 1.0).abs() < EPSILON,
            "Speed preset 1.0x mismatch"
        );
        assert!(
            (SpeedPreset::X125.as_f32() - 1.25).abs() < EPSILON,
            "Speed preset 1.25x mismatch"
        );
        assert!(
            (SpeedPreset::X15.as_f32() - 1.5).abs() < EPSILON,
            "Speed preset 1.5x mismatch"
        );
        assert!(
            (SpeedPreset::X20.as_f32() - 2.0).abs() < EPSILON,
            "Speed preset 2.0x mismatch"
        );
    }

    #[test]
    fn test_speed_preset_from_speed() {
        assert_eq!(SpeedPreset::from_speed(0.75), Some(SpeedPreset::X075));
        assert_eq!(SpeedPreset::from_speed(1.0), Some(SpeedPreset::X10));
        assert_eq!(SpeedPreset::from_speed(1.25), Some(SpeedPreset::X125));
        assert_eq!(SpeedPreset::from_speed(1.5), Some(SpeedPreset::X15));
        assert_eq!(SpeedPreset::from_speed(2.0), Some(SpeedPreset::X20));
        assert_eq!(SpeedPreset::from_speed(0.8), None);
    }

    #[test]
    fn test_format_speed_label() {
        assert_eq!(format_speed_label(1.0), "1x");
        assert_eq!(format_speed_label(1.5), "1.5x");
        assert_eq!(format_speed_label(2.0), "2x");
        assert_eq!(format_speed_label(0.75), "0.75x");
    }
}
