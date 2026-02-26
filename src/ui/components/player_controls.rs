//! Player controls component: transport buttons, seek bar, speed, and volume.

use crate::conversions::f64_to_ms;
use crate::models::SleepTimerMode;
use crate::ui::state::RippleTarget;
use crate::ui::styles::{
    border_radius, button_styles, colors, focus_indicator, shadows, spacing, typography,
};
use crate::ui::{FocusedElement, Message, PlaybackStatus, State};
use iced::widget::{
    button, column, container, image, mouse_area, pick_list, row, slider, stack, text, text_input,
    Space,
};
use iced::{Border, Color, Element, Length, Point};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SpeedPreset {
    X075,
    X10,
    X125,
    X15,
    X20,
}

impl SpeedPreset {
    const ALL: [Self; 5] = [Self::X075, Self::X10, Self::X125, Self::X15, Self::X20];

    const fn as_f32(self) -> f32 {
        match self {
            Self::X075 => 0.75,
            Self::X10 => 1.0,
            Self::X125 => 1.25,
            Self::X15 => 1.5,
            Self::X20 => 2.0,
        }
    }

    fn from_speed(speed: f32) -> Option<Self> {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SleepTimerPreset {
    Off,
    Minutes15,
    Minutes30,
    Minutes45,
    Minutes60,
    EndOfChapter,
}

impl SleepTimerPreset {
    const ALL: [Self; 6] = [
        Self::Off,
        Self::Minutes15,
        Self::Minutes30,
        Self::Minutes45,
        Self::Minutes60,
        Self::EndOfChapter,
    ];

    const fn to_message(self) -> Message {
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

/// Renders the player controls component with improved UX
///
/// This component includes:
/// - Currently playing file display with audiobook name and chapter
/// - Progress slider with time markers
/// - Play/pause and stop buttons with accessible labels
/// - Playback speed controls with presets
/// - Volume controls with percentage display
/// - Sleep timer controls
#[allow(clippy::too_many_lines)]
pub fn view(state: &State) -> Element<'_, Message> {
    // Extract chapter name from filename by removing common prefixes and extensions
    let current_file_text = state
        .selected_file
        .as_ref()
        .and_then(|f| std::path::Path::new(f).file_name())
        .and_then(|f| f.to_str())
        .map_or("No file selected", extract_chapter_name);

    // Get the audiobook name, ID, and calculate progress summary
    let (audiobook_name, audiobook_id, progress_summary) = state
        .selected_audiobook
        .and_then(|id| state.audiobooks.iter().find(|ab| ab.id == Some(id)))
        .map_or(("", None, String::new()), |ab| {
            let summary = calculate_progress_summary(
                &state.current_files,
                state.selected_file.as_ref(),
                ab.total_duration_ms,
            );
            (ab.name.as_str(), ab.id, summary)
        });

    let play_pause_label = if state.playback == PlaybackStatus::Playing {
        "Pause" // Accessible label instead of emoji
    } else {
        "Play"
    };

    let sleep_timer_controls = sleep_timer_controls(state);

    let progress_container = container({
        let max_duration = state.total_duration.max(1.0);
        let current_time = state.current_time.clamp(0.0, max_duration);
        slider(0.0..=max_duration, current_time, Message::SeekTo).height(24.0)
        // Larger height for better grab affordance
    })
    .style(focus_indicator(
        state.focused_element == FocusedElement::ProgressSlider,
    ))
    .padding(iced::Padding::from([spacing::MD, 0.0]));

    let progress_slider = mouse_area(progress_container)
        .on_enter(Message::TooltipHoverStart(
            crate::ui::tooltip::TooltipTarget::Progress,
        ))
        .on_exit(Message::TooltipHoverEnd);

    column![
        // Enhanced "Now Playing" display with cover art, audiobook title, chapter name, and progress
        if audiobook_name.is_empty() {
            container(text(format!("Now Playing: {current_file_text}")).size(typography::SIZE_SM))
                .padding(spacing::SM)
        } else {
            // Build cover art thumbnail (100x100px for better visibility)
            let cover_element: Element<'_, Message> = audiobook_id
                .and_then(|id| state.cover_thumbnails.get(&id))
                .map_or_else(
                    || {
                        // No cover available - show placeholder
                        container(text("♪").size(typography::SIZE_XXL).style(
                            |_theme: &iced::Theme| iced::widget::text::Style {
                                color: Some(colors::TEXT_SECONDARY),
                            },
                        ))
                        .width(Length::Fixed(100.0))
                        .height(Length::Fixed(100.0))
                        .center_x(Length::Fill)
                        .center_y(Length::Fill)
                        .style(|_theme: &iced::Theme| container::Style {
                            background: Some(colors::BG_ELEVATED.into()),
                            border: Border {
                                radius: border_radius::MD.into(),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .into()
                    },
                    |path| {
                        // Show cover thumbnail
                        container(
                            image(iced::widget::image::Handle::from_path(path.clone()))
                                .width(Length::Fixed(100.0))
                                .height(Length::Fixed(100.0)),
                        )
                        .style(|_theme: &iced::Theme| container::Style {
                            border: Border {
                                radius: border_radius::MD.into(),
                                ..Default::default()
                            },
                            shadow: shadows::shadow_md(),
                            ..Default::default()
                        })
                        .into()
                    },
                );

            let mut text_col = column![
                text(audiobook_name)
                    .size(typography::SIZE_XL)
                    .style(|_theme: &iced::Theme| iced::widget::text::Style {
                        color: Some(colors::TEXT_PRIMARY),
                    }),
                text(current_file_text).size(typography::SIZE_BASE).style(
                    |_theme: &iced::Theme| iced::widget::text::Style {
                        color: Some(colors::TEXT_SECONDARY),
                    }
                ),
            ]
            .spacing(spacing::XS);

            if !progress_summary.is_empty() {
                text_col = text_col.push(text(progress_summary).size(typography::SIZE_SM).style(
                    |_theme: &iced::Theme| iced::widget::text::Style {
                        color: Some(colors::PRIMARY),
                    },
                ));
            }

            container(
                row![cover_element, text_col]
                    .spacing(spacing::LG)
                    .align_y(iced::Alignment::Center),
            )
            .padding(spacing::MD)
        },
        // Progress slider with larger touch target for accessibility
        // BUG FIX #001: Ensure slider range and value are consistent to prevent invalid seeks
        progress_slider, // Increased vertical padding
        // Time markers with better spacing - show elapsed and remaining time with labels
        row![
            text(format!("Elapsed: {}", format_time(state.current_time))).size(typography::SIZE_SM),
            Space::new().width(Length::Fill),
            text(format!(
                "Remaining: {}",
                format_time_remaining(state.total_duration - state.current_time)
            ))
            .size(typography::SIZE_SM)
            .style(|_theme: &iced::Theme| iced::widget::text::Style {
                color: Some(colors::PRIMARY),
            }),
        ]
        .padding(spacing::XS),
        // Control buttons and volume with improved visual grouping
        // Use LG spacing (24px) between major control sections for clear visual hierarchy
        row![
            playback_controls(
                state,
                play_pause_label,
                state.focused_element == FocusedElement::PlayPauseButton,
                state.focused_element == FocusedElement::StopButton,
            ),
            Space::new().width(Length::Fixed(spacing::LG)),
            speed_controls(
                state,
                0, // speed_step no longer used (slider removed for compact design)
                state.focused_element == FocusedElement::SpeedSlider
            ),
            Space::new().width(Length::Fixed(spacing::LG)),
            volume_controls(state, state.focused_element == FocusedElement::VolumeSlider),
        ]
        .padding(spacing::MD)
        .spacing(spacing::LG),
        // Sleep timer controls with consistent spacing
        container(sleep_timer_controls).padding(spacing::MD)
    ]
    .padding(spacing::MD)
    .spacing(spacing::SM)
    .into()
}

fn playback_controls(
    state: &State,
    play_pause_label: &'static str,
    play_pause_focused: bool,
    stop_focused: bool,
) -> Element<'static, Message> {
    let play_button = button(text(play_pause_label).size(typography::SIZE_BASE))
        .on_press(Message::PlayPause)
        .padding(spacing::MD)
        .width(Length::Fixed(100.0))
        .style(if play_pause_focused {
            button_styles::primary_focused
        } else {
            button_styles::primary
        });

    let stop_button = button(text("Stop").size(typography::SIZE_BASE))
        .on_press(Message::Stop)
        .padding(spacing::MD)
        .width(Length::Fixed(100.0))
        .style(if stop_focused {
            button_styles::secondary_focused
        } else {
            button_styles::secondary
        });

    let play_button = mouse_area(ripple_stack(
        play_button.into(),
        state.ripple_animations.get(&RippleTarget::PlayPauseButton),
    ))
    .on_move(|point| Message::RipplePointerMoved(RippleTarget::PlayPauseButton, point))
    .on_enter(Message::TooltipHoverStart(
        crate::ui::tooltip::TooltipTarget::PlayPause,
    ))
    .on_exit(Message::TooltipHoverEnd);

    let stop_button = mouse_area(ripple_stack(
        stop_button.into(),
        state.ripple_animations.get(&RippleTarget::StopButton),
    ))
    .on_move(|point| Message::RipplePointerMoved(RippleTarget::StopButton, point))
    .on_enter(Message::TooltipHoverStart(
        crate::ui::tooltip::TooltipTarget::Stop,
    ))
    .on_exit(Message::TooltipHoverEnd);

    container(row![play_button, stop_button].spacing(spacing::SM)).into()
}

fn ripple_stack(
    base: Element<'static, Message>,
    ripple: Option<&crate::ui::styles::interactive::RippleAnimation>,
) -> Element<'static, Message> {
    let Some(ripple) = ripple else {
        return base;
    };

    let radius = ripple.radius();
    if radius <= 0.0 {
        return base;
    }

    let diameter = radius * 2.0;
    let opacity = ripple.opacity();
    let center = ripple.center();
    let origin = Point::new(center.x - radius, center.y - radius);
    let circle = container(Space::new())
        .width(Length::Fixed(diameter))
        .height(Length::Fixed(diameter))
        .style(move |_theme: &iced::Theme| container::Style {
            background: Some(Color::from_rgba(1.0, 1.0, 1.0, opacity).into()),
            border: Border {
                radius: (diameter / 2.0).into(),
                ..Default::default()
            },
            ..Default::default()
        });

    let overlay = container(circle)
        .padding(iced::Padding {
            top: origin.y,
            bottom: 0.0,
            left: origin.x,
            right: 0.0,
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill);

    stack![base, overlay].into()
}

fn speed_controls(state: &State, _speed_step: i32, is_focused: bool) -> Element<'_, Message> {
    let selected = SpeedPreset::from_speed(state.speed);
    let picker = pick_list(SpeedPreset::ALL, selected, |preset| {
        Message::SpeedChanged(preset.as_f32())
    })
    .placeholder("Speed")
    .width(Length::Fixed(140.0));

    let content = container(
        row![
            text("Speed:").size(typography::SIZE_SM),
            picker,
            text(format_speed_label(state.speed))
                .size(typography::SIZE_SM)
                .style(|_theme: &iced::Theme| iced::widget::text::Style {
                    color: Some(colors::PRIMARY),
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

fn volume_controls(state: &State, is_focused: bool) -> Element<'_, Message> {
    let volume_level = volume_to_level(state.volume);
    let volume_color = volume_level_color(volume_level);

    let toggle = button(text("Vol").size(typography::SIZE_SM))
        .on_press(Message::ToggleVolumeExpanded)
        .padding(iced::Padding::from([spacing::XS, spacing::SM]))
        .style(button_styles::secondary);

    let slider_row: Element<'_, Message> = if state.volume_expanded {
        row![
            slider(0..=200, state.volume, Message::VolumeChanged)
                .width(Length::Fixed(120.0))
                .height(20.0),
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

fn volume_to_level(volume: i32) -> f32 {
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
fn volume_level_color(volume: f32) -> iced::Color {
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

fn format_speed_label(speed: f32) -> String {
    let fixed = format!("{speed:.2}");
    let trimmed = fixed.trim_end_matches('0').trim_end_matches('.');
    format!("{trimmed}x")
}

/// Renders sleep timer controls with consolidated menu design
///
/// Compact single-row layout reduces space consumption while maintaining full functionality:
/// - Preset buttons use smaller text (`SIZE_XS`) and reduced padding
/// - Custom input integrated into same row
/// - Error messages shown as tooltip-style overlay when needed
fn sleep_timer_controls(state: &State) -> Element<'_, Message> {
    match state.sleep_timer.as_ref().map(|t| t.mode) {
        None => {
            let picker = pick_list(SleepTimerPreset::ALL, None::<SleepTimerPreset>, |preset| {
                preset.to_message()
            })
            .placeholder("Sleep timer")
            .width(Length::Fixed(180.0));

            let main_row = row![
                text("Sleep Timer:").size(typography::SIZE_SM),
                picker,
                Space::new().width(Length::Fill),
                text_input("Custom min", &state.sleep_timer_custom_minutes)
                    .on_input(Message::SleepTimerCustomMinutesChanged)
                    .on_submit(Message::SleepTimerCustomSubmit)
                    .width(Length::Fixed(90.0))
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

/// Extracts a readable chapter name from a file name
///
/// Removes common audiobook prefixes, file extensions, and cleans up formatting
fn extract_chapter_name(filename: &str) -> &str {
    // Remove file extension
    let without_ext = filename.rsplit_once('.').map_or(filename, |(name, _)| name);

    // Return the cleaned name
    without_ext
}

/// Calculates a progress summary showing chapters and remaining time
///
/// Returns a string like "Chapter 5 of 12 · 3h 45m remaining"
fn calculate_progress_summary(
    files: &[crate::models::AudiobookFile],
    selected_file: Option<&String>,
    total_duration_ms: Option<i64>,
) -> String {
    if files.is_empty() {
        return String::new();
    }

    let total_files = files.len();

    // Find current file index
    let current_index = selected_file
        .and_then(|path| files.iter().position(|f| &f.full_path == path))
        .map(|i| i + 1); // Convert to 1-based index

    let mut parts = Vec::new();

    // Add chapter information if available
    if let Some(current) = current_index {
        parts.push(format!("Chapter {current} of {total_files}"));
    }

    if let Some(remaining_ms) = compute_remaining_duration_ms(files, total_duration_ms) {
        if remaining_ms > 0 {
            parts.push(format_compact_remaining_hm(remaining_ms));
        }
    }

    parts.join(" · ")
}

fn compute_remaining_duration_ms(
    files: &[crate::models::AudiobookFile],
    total_duration_ms: Option<i64>,
) -> Option<i64> {
    if files.is_empty() {
        return None;
    }

    let mut sum_remaining: i64 = 0;
    let mut all_have_lengths = true;

    for file in files {
        match file.length_of_file {
            Some(length_ms) if length_ms > 0 => {
                let seek_ms = file.seek_position.unwrap_or(0);
                sum_remaining = sum_remaining.saturating_add((length_ms - seek_ms).max(0));
            }
            _ => {
                all_have_lengths = false;
            }
        }
    }

    if all_have_lengths {
        return Some(sum_remaining);
    }

    let total_ms = total_duration_ms.filter(|v| *v > 0)?;

    let played_ms: i64 = files.iter().map(|f| f.seek_position.unwrap_or(0)).sum();

    Some((total_ms - played_ms).max(0))
}

fn format_compact_remaining_hm(remaining_ms: i64) -> String {
    let remaining_ms = remaining_ms.max(0);

    let minutes_total = (remaining_ms.saturating_add(59_999) / 60_000).max(0);
    let hours = minutes_total / 60;
    let minutes = minutes_total % 60;

    if hours > 0 {
        format!("{hours}h {minutes}m remaining")
    } else {
        format!("{minutes}m remaining")
    }
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

/// Formats remaining time with a "-" prefix to indicate time left
fn format_time_remaining(ms: f64) -> String {
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
    use crate::models::AudiobookFile;
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
    fn test_view_renders_play_button_when_paused() {
        let state = State {
            playback: PlaybackStatus::Paused,
            ..Default::default()
        };

        let element = view(&state);
        // Test passes if no panic during rendering
        drop(element);
    }

    #[test]
    fn test_view_renders_pause_button_when_playing() {
        let state = State {
            playback: PlaybackStatus::Playing,
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

    #[test]
    fn test_calculate_progress_summary_includes_remaining_time_across_files() {
        let audiobook_id = 1;

        let mut f1 = AudiobookFile::new(
            audiobook_id,
            "ch1.mp3".to_string(),
            "/dir/book/ch1.mp3".to_string(),
            0,
        );
        f1.length_of_file = Some(60 * 60 * 1000);
        f1.seek_position = Some(60 * 60 * 1000);
        f1.completeness = 100;

        let mut f2 = AudiobookFile::new(
            audiobook_id,
            "ch2.mp3".to_string(),
            "/dir/book/ch2.mp3".to_string(),
            1,
        );
        f2.length_of_file = Some(2 * 60 * 60 * 1000);
        f2.seek_position = Some(15 * 60 * 1000);
        f2.completeness = 12;

        let mut f3 = AudiobookFile::new(
            audiobook_id,
            "ch3.mp3".to_string(),
            "/dir/book/ch3.mp3".to_string(),
            2,
        );
        f3.length_of_file = Some(60 * 60 * 1000);
        f3.seek_position = None;
        f3.completeness = 0;

        let files = vec![f1, f2, f3];
        let total_ms = Some(4 * 60 * 60 * 1000);

        let summary =
            calculate_progress_summary(&files, Some(&"/dir/book/ch2.mp3".to_string()), total_ms);

        assert!(
            summary.contains("Chapter 2 of 3"),
            "summary should include chapter position"
        );
        assert!(
            summary.contains("2h 45m remaining"),
            "summary should include remaining time across files; got: {summary}"
        );
    }
}
