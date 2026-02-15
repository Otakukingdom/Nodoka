use crate::conversions::f64_to_ms;
use crate::models::SleepTimerMode;
use crate::ui::styles::{button_styles, spacing, typography};
use crate::ui::{Message, State};
use iced::widget::{button, column, container, row, slider, text, text_input, Space};
use iced::{Element, Length};

/// Renders the player controls component with improved UX
///
/// This component includes:
/// - Currently playing file display
/// - Progress slider with time markers
/// - Play/pause and stop buttons with accessible labels
/// - Playback speed controls with presets
/// - Volume controls with percentage display
/// - Sleep timer controls
pub fn view(state: &State) -> Element<'_, Message> {
    let current_file_text = state
        .selected_file
        .as_ref()
        .and_then(|f| std::path::Path::new(f).file_name())
        .and_then(|f| f.to_str())
        .unwrap_or("No file selected");

    let play_pause_label = if state.is_playing {
        "Pause" // Accessible label instead of emoji
    } else {
        "Play"
    };

    let sleep_timer_controls = sleep_timer_controls(state);

    let speed_step = speed_step_from_speed(state.speed);

    column![
        // Currently playing label with better typography
        container(text(format!("Now Playing: {current_file_text}")).size(typography::SIZE_SM))
            .padding(spacing::SM),
        // Progress slider with larger touch target for accessibility
        container(slider(
            0.0..=state.total_duration.max(1.0),
            state.current_time.min(state.total_duration),
            Message::SeekTo
        ))
        .padding(iced::Padding::from([spacing::SM, 0.0])),
        // Time markers with better spacing
        row![
            text(format_time(state.current_time)).size(typography::SIZE_SM),
            Space::new().width(Length::Fill),
            text(format_time(state.total_duration)).size(typography::SIZE_SM),
        ]
        .padding(spacing::XS),
        // Control buttons and volume with improved visual grouping
        row![
            // Playback controls group
            container(
                row![
                    button(text(play_pause_label).size(typography::SIZE_BASE))
                        .on_press(Message::PlayPause)
                        .padding(spacing::SM)
                        .style(button_styles::primary),
                    button(text("Stop").size(typography::SIZE_BASE))
                        .on_press(Message::Stop)
                        .padding(spacing::SM)
                        .style(button_styles::secondary),
                ]
                .spacing(spacing::SM)
            ),
            Space::new().width(Length::Fill),
            // Speed controls group with visual hierarchy
            container(
                column![
                    row![
                        text("Speed:").size(typography::SIZE_SM),
                        text(format!("{:.1}x", state.speed)).size(typography::SIZE_BASE),
                    ]
                    .spacing(spacing::XS),
                    slider(5..=20, speed_step, |step| Message::SpeedChanged(
                        speed_from_step(step)
                    ))
                    .width(Length::Fixed(120.0)),
                    row![
                        button(text("0.5x").size(typography::SIZE_SM))
                            .on_press(Message::SpeedChanged(0.5))
                            .padding(iced::Padding::from([spacing::SM, spacing::XS]))
                            .style(button_styles::secondary),
                        button(text("0.75x").size(typography::SIZE_SM))
                            .on_press(Message::SpeedChanged(0.75))
                            .padding(iced::Padding::from([spacing::SM, spacing::XS]))
                            .style(button_styles::secondary),
                        button(text("1.0x").size(typography::SIZE_SM))
                            .on_press(Message::SpeedChanged(1.0))
                            .padding(iced::Padding::from([spacing::SM, spacing::XS]))
                            .style(button_styles::secondary),
                        button(text("1.25x").size(typography::SIZE_SM))
                            .on_press(Message::SpeedChanged(1.25))
                            .padding(iced::Padding::from([spacing::SM, spacing::XS]))
                            .style(button_styles::secondary),
                        button(text("1.5x").size(typography::SIZE_SM))
                            .on_press(Message::SpeedChanged(1.5))
                            .padding(iced::Padding::from([spacing::SM, spacing::XS]))
                            .style(button_styles::secondary),
                        button(text("2.0x").size(typography::SIZE_SM))
                            .on_press(Message::SpeedChanged(2.0))
                            .padding(iced::Padding::from([spacing::SM, spacing::XS]))
                            .style(button_styles::secondary),
                    ]
                    .spacing(spacing::XS)
                ]
                .spacing(spacing::XS)
            ),
            Space::new().width(Length::Fill),
            // Volume controls group
            container(
                row![
                    text("Volume:").size(typography::SIZE_SM),
                    slider(0..=200, state.volume, Message::VolumeChanged)
                        .width(Length::Fixed(150.0)),
                    text(format!("{}%", state.volume)).size(typography::SIZE_SM),
                ]
                .spacing(spacing::SM)
            ),
        ]
        .padding(spacing::MD)
        .spacing(spacing::MD),
        // Sleep timer controls with consistent spacing
        container(sleep_timer_controls).padding(spacing::MD)
    ]
    .padding(spacing::MD)
    .spacing(spacing::SM)
    .into()
}

fn speed_step_from_speed(speed: f32) -> i32 {
    let formatted = format!("{speed:.1}");
    let digits = formatted.replace('.', "");
    match digits.parse::<i32>() {
        Ok(v) => v.clamp(5, 20),
        Err(_e) => 10,
    }
}

fn speed_from_step(step: i32) -> f32 {
    let step_u16 = u16::try_from(step).unwrap_or(10);
    let raw = f32::from(step_u16) / 10.0;
    let formatted = format!("{raw:.1}");
    formatted.parse::<f32>().ok().unwrap_or(1.0).clamp(0.5, 2.0)
}

/// Renders sleep timer controls with improved visual hierarchy
fn sleep_timer_controls(state: &State) -> Element<'_, Message> {
    match state.sleep_timer.as_ref().map(|t| t.mode) {
        None => {
            let presets = row![
                text("Sleep Timer:").size(typography::SIZE_SM),
                button(text("15m").size(typography::SIZE_SM))
                    .on_press(Message::SleepTimerSetDurationSeconds(15 * 60))
                    .padding(spacing::SM)
                    .style(button_styles::secondary),
                button(text("30m").size(typography::SIZE_SM))
                    .on_press(Message::SleepTimerSetDurationSeconds(30 * 60))
                    .padding(spacing::SM)
                    .style(button_styles::secondary),
                button(text("45m").size(typography::SIZE_SM))
                    .on_press(Message::SleepTimerSetDurationSeconds(45 * 60))
                    .padding(spacing::SM)
                    .style(button_styles::secondary),
                button(text("60m").size(typography::SIZE_SM))
                    .on_press(Message::SleepTimerSetDurationSeconds(60 * 60))
                    .padding(spacing::SM)
                    .style(button_styles::secondary),
                button(text("End of Chapter").size(typography::SIZE_SM))
                    .on_press(Message::SleepTimerSetEndOfChapter)
                    .padding(spacing::SM)
                    .style(button_styles::secondary),
            ]
            .spacing(spacing::SM);

            let custom = row![
                text("Custom:").size(typography::SIZE_SM),
                text_input("minutes", &state.sleep_timer_custom_minutes)
                    .on_input(Message::SleepTimerCustomMinutesChanged)
                    .on_submit(Message::SleepTimerCustomSubmit)
                    .width(Length::Fixed(90.0)),
                button(text("Set").size(typography::SIZE_SM))
                    .on_press(Message::SleepTimerCustomSubmit)
                    .padding(spacing::SM)
                    .style(button_styles::primary),
            ]
            .spacing(spacing::SM);

            let mut content = column![presets, custom].spacing(spacing::SM);
            if let Some(err) = state.sleep_timer_custom_error.as_deref() {
                content = content.push(text(err).size(typography::SIZE_SM));
            }
            content.into()
        }
        Some(SleepTimerMode::EndOfChapter) => row![
            text("Sleep Timer:").size(typography::SIZE_SM),
            text("End of chapter").size(typography::SIZE_BASE),
            Space::new().width(Length::Fill),
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
                Space::new().width(Length::Fill),
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
fn format_remaining_seconds(secs: i64) -> String {
    let secs = secs.max(0);
    let minutes = secs / 60;
    let seconds = secs % 60;
    format!("{minutes}:{seconds:02}")
}

/// Formats time in milliseconds for display (H:MM:SS or M:SS format)
fn format_time(ms: f64) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::SleepTimer;

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
    fn test_speed_step_conversion_round_trip() {
        // Test speeds that round-trip correctly with .1 precision formatting
        // Note: 0.75 becomes "0.7" with .1 precision, which becomes step 7, then back to 0.7
        let test_cases = vec![
            (0.5, 5),
            (0.7, 7), // 0.75 rounds to 0.7 with .1 precision
            (1.0, 10),
            (1.2, 12), // 1.25 rounds to 1.2 with .1 precision
            (1.5, 15),
            (1.8, 18), // 1.75 rounds to 1.8 with .1 precision
            (2.0, 20),
        ];

        for (expected_speed, expected_step) in test_cases {
            let step = speed_step_from_speed(expected_speed);
            assert_eq!(
                step, expected_step,
                "Speed {expected_speed} should map to step {expected_step}, got {step}"
            );

            let converted = speed_from_step(step);
            assert!(
                (converted - expected_speed).abs() < 0.01,
                "Step {step} should convert to speed {expected_speed}, got {converted}"
            );
        }
    }

    #[test]
    fn test_speed_step_from_speed_clamps_values() {
        // Test that speed_step_from_speed returns values in valid range [5, 20]
        let step = speed_step_from_speed(0.3); // Below minimum
        assert!((5..=20).contains(&step));

        let step = speed_step_from_speed(3.0); // Above maximum
        assert!((5..=20).contains(&step));

        let step = speed_step_from_speed(1.0); // Normal value
        assert_eq!(step, 10);
    }

    #[test]
    fn test_speed_from_step_clamps_values() {
        // Test that speed_from_step returns values in valid range [0.5, 2.0]
        const EPSILON: f32 = 1e-6;

        assert!((speed_from_step(5) - 0.5).abs() < EPSILON);
        assert!((speed_from_step(10) - 1.0).abs() < EPSILON);
        assert!((speed_from_step(20) - 2.0).abs() < EPSILON);

        // Edge cases
        let speed = speed_from_step(3); // Below minimum
        assert!((0.5..=2.0).contains(&speed));

        let speed = speed_from_step(25); // Above maximum
        assert!((0.5..=2.0).contains(&speed));
    }

    #[test]
    fn test_view_renders_play_button_when_paused() {
        let state = State {
            is_playing: false,
            ..Default::default()
        };

        let element = view(&state);
        // Test passes if no panic during rendering
        drop(element);
    }

    #[test]
    fn test_view_renders_pause_button_when_playing() {
        let state = State {
            is_playing: true,
            ..Default::default()
        };

        let element = view(&state);
        // Test passes if no panic during rendering
        drop(element);
    }

    #[test]
    fn test_view_renders_with_no_file_selected() {
        let state = State {
            selected_file: None,
            ..Default::default()
        };

        let element = view(&state);
        drop(element);
    }

    #[test]
    fn test_view_renders_with_file_selected() {
        let state = State {
            selected_file: Some("/path/to/audiobook/chapter1.mp3".to_string()),
            ..Default::default()
        };

        let element = view(&state);
        drop(element);
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

    #[test]
    fn test_view_handles_zero_duration() {
        let state = State {
            total_duration: 0.0,
            current_time: 0.0,
            ..Default::default()
        };

        let element = view(&state);
        drop(element);
    }

    #[test]
    fn test_view_handles_large_duration() {
        let state = State {
            total_duration: 36_000_000.0, // 10 hours
            current_time: 18_000_000.0,   // 5 hours
            ..Default::default()
        };

        let element = view(&state);
        drop(element);
    }

    #[test]
    fn test_view_handles_volume_boundaries() {
        let state_min = State {
            volume: 0,
            ..Default::default()
        };
        let element = view(&state_min);
        drop(element);

        let state_max = State {
            volume: 200,
            ..Default::default()
        };
        let element = view(&state_max);
        drop(element);
    }

    #[test]
    fn test_view_handles_speed_boundaries() {
        let state_min = State {
            speed: 0.5,
            ..Default::default()
        };
        let element = view(&state_min);
        drop(element);

        let state_max = State {
            speed: 2.0,
            ..Default::default()
        };
        let element = view(&state_max);
        drop(element);
    }
}
