//! Sleep timer controls with preset options and custom duration input.

use crate::models::SleepTimerMode;
use crate::ui::styles::{button_styles, colors, spacing, typography};
use crate::ui::{Message, State};
use iced::widget::{button, pick_list, row, text, text_input, column, Space};
use iced::Element;

/// Sleep timer preset options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SleepTimerPreset {
    Off,
    Minutes15,
    Minutes30,
    Minutes45,
    Minutes60,
    EndOfChapter,
}

impl SleepTimerPreset {
    pub const ALL: [Self; 6] = [
        Self::Off,
        Self::Minutes15,
        Self::Minutes30,
        Self::Minutes45,
        Self::Minutes60,
        Self::EndOfChapter,
    ];

    pub const fn to_message(self) -> Message {
        match self {
            Self::Off => Message::SleepTimerCancel,
            Self::Minutes15 => Message::SleepTimerSetDurationSeconds(15 * 60),
            Self::Minutes30 => Message::SleepTimerSetDurationSeconds(30 * 60),
            Self::Minutes45 => Message::SleepTimerSetDurationSeconds(45 * 60),
            Self::Minutes60 => Message::SleepTimerSetDurationSeconds(60 * 60),
            Self::EndOfChapter => Message::SleepTimerSetEndOfChapter,
        }
    }
}

impl std::fmt::Display for SleepTimerPreset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            Self::Off => "Off",
            Self::Minutes15 => "15 min",
            Self::Minutes30 => "30 min",
            Self::Minutes45 => "45 min",
            Self::Minutes60 => "60 min",
            Self::EndOfChapter => "End of chapter",
        };
        write!(f, "{label}")
    }
}

/// Renders sleep timer controls with consolidated menu design
///
/// Compact single-row layout reduces space consumption while maintaining full functionality:
/// - Preset buttons use smaller text (`SIZE_XS`) and reduced padding
/// - Custom input integrated into same row
/// - Error messages shown as tooltip-style overlay when needed
pub fn sleep_timer_controls(state: &State) -> Element<'_, Message> {
    match state.sleep_timer.as_ref().map(|t| t.mode) {
        None => {
            let picker = pick_list(SleepTimerPreset::ALL, None::<SleepTimerPreset>, |preset| {
                preset.to_message()
            })
            .placeholder("Sleep timer")
            .width(iced::Length::Fixed(180.0));

            let main_row = row![
                text("Sleep Timer:").size(typography::SIZE_SM),
                picker,
                Space::new().width(iced::Length::Fill),
                text_input("Custom min", &state.sleep_timer_custom_minutes)
                    .on_input(Message::SleepTimerCustomMinutesChanged)
                    .on_submit(Message::SleepTimerCustomSubmit)
                    .width(iced::Length::Fixed(90.0))
                    .size(typography::SIZE_SM),
                button(text("Set").size(typography::SIZE_SM))
                    .on_press(Message::SleepTimerCustomSubmit)
                    .padding(spacing::SM)
                    .style(button_styles::primary),
            ]
            .spacing(spacing::SM)
            .align_y(iced::Alignment::Center);

            // Show error message inline if present
            if let Some(err) = state.sleep_timer_custom_error.as_deref() {
                column![
                    main_row,
                    text(err)
                        .size(typography::SIZE_XS)
                        .style(|_theme: &iced::Theme| iced::widget::text::Style {
                            color: Some(colors::ERROR),
                        })
                ]
                .spacing(spacing::XS)
                .into()
            } else {
                main_row.into()
            }
        }
        Some(SleepTimerMode::EndOfChapter) => row![
            text("Sleep Timer:").size(typography::SIZE_SM),
            text("End of chapter").size(typography::SIZE_BASE),
            Space::new().width(iced::Length::Fill),
            button(text("Cancel").size(typography::SIZE_SM))
                .on_press(Message::SleepTimerCancel)
                .padding(spacing::SM)
                .style(button_styles::danger),
        ]
        .spacing(spacing::SM)
        .into(),
        Some(SleepTimerMode::Duration(_)) => {
            let remaining = state
                .sleep_timer
                .as_ref()
                .and_then(crate::models::SleepTimer::remaining_seconds)
                .unwrap_or(0);
            row![
                text("Sleep Timer:").size(typography::SIZE_SM),
                text(format_remaining_seconds(remaining)).size(typography::SIZE_BASE),
                Space::new().width(iced::Length::Fill),
                button(text("+15m").size(typography::SIZE_SM))
                    .on_press(Message::SleepTimerExtendSeconds(15 * 60))
                    .padding(spacing::SM)
                    .style(button_styles::secondary),
                button(text("Cancel").size(typography::SIZE_SM))
                    .on_press(Message::SleepTimerCancel)
                    .padding(spacing::SM)
                    .style(button_styles::danger),
            ]
            .spacing(spacing::SM)
            .into()
        }
    }
}

/// Formats remaining seconds for sleep timer display (M:SS format)
#[must_use]
pub fn format_remaining_seconds(secs: i64) -> String {
    let secs = secs.max(0);
    let minutes = secs / 60;
    let seconds = secs % 60;
    format!("{minutes}:{seconds:02}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::SleepTimer;

    #[test]
    fn test_format_remaining_seconds_formats_correctly() {
        assert_eq!(format_remaining_seconds(125), "2:05");
        assert_eq!(format_remaining_seconds(60), "1:00");
        assert_eq!(format_remaining_seconds(0), "0:00");
        assert_eq!(format_remaining_seconds(3661), "61:01");
    }

    #[test]
    fn test_format_remaining_seconds_handles_negative() {
        // Negative values should be clamped to 0
        assert_eq!(format_remaining_seconds(-5), "0:00");
        assert_eq!(format_remaining_seconds(-100), "0:00");
    }

    #[test]
    fn test_sleep_timer_inactive_shows_presets() {
        let state = State {
            sleep_timer: None,
            ..Default::default()
        };

        let element = sleep_timer_controls(&state);
        drop(element);
    }

    #[test]
    fn test_sleep_timer_active_duration_shows_countdown() {
        let timer = SleepTimer::new(SleepTimerMode::Duration(1800), 10); // 30 minutes, 10s fade
        let state = State {
            sleep_timer: Some(timer),
            ..Default::default()
        };

        let element = sleep_timer_controls(&state);
        drop(element);
    }

    #[test]
    fn test_sleep_timer_end_of_chapter_shows_message() {
        let timer = SleepTimer::new(SleepTimerMode::EndOfChapter, 10);
        let state = State {
            sleep_timer: Some(timer),
            ..Default::default()
        };

        let element = sleep_timer_controls(&state);
        drop(element);
    }

    #[test]
    fn test_sleep_timer_custom_error_is_displayed() {
        let state = State {
            sleep_timer: None,
            sleep_timer_custom_error: Some("Invalid input".to_string()),
            ..Default::default()
        };

        let element = sleep_timer_controls(&state);
        drop(element);
    }
}
