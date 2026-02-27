//! Player controls component: transport buttons, seek bar, speed, and volume.
//!
//! This module is split into multiple sub-modules for maintainability:
//! - [`playback`]: Play/pause/stop button controls
//! - [`speed`]: Playback speed controls
//! - [`volume`]: Volume slider with color-coded levels
//! - [`sleep`]: Sleep timer controls
//! - [`helpers`]: Time formatting and progress calculation utilities

pub mod helpers;
mod playback;
mod sleep;
mod speed;
mod volume;

use crate::ui::styles::components::border_width;
use crate::ui::styles::{border_radius, colors, focus_indicator, shadows, spacing, typography};
use crate::ui::{FocusedElement, Message, PlaybackStatus, State};
use iced::widget::{
    column, container, image, mouse_area, row, slider as widget_slider, text, Space,
};
use iced::{Border, Color, Element, Length};

pub use helpers::{calculate_progress_summary, extract_chapter_name};
pub use playback::playback_controls;
pub use sleep::{format_remaining_seconds, sleep_timer_controls};
pub use speed::{format_speed_label, speed_controls, SpeedPreset};
pub use volume::{format_time, format_time_remaining, volume_controls, volume_level_color};

/// Renders the player controls component with improved UX
///
/// This component includes:
/// - Currently playing file display with audiobook name and chapter
/// - Progress slider with time markers
/// - Play/pause and stop buttons with accessible labels
/// - Playback speed controls with presets
/// - Volume controls with percentage display
/// - Sleep timer controls
#[must_use]
pub fn view(state: &State) -> Element<'_, Message> {
    let now_playing = now_playing_info(state);
    let play_pause_label = if state.playback == PlaybackStatus::Playing {
        "Pause" // Accessible label instead of emoji
    } else {
        "Play"
    };
    column![
        now_playing_card(state, &now_playing),
        progress_slider(state),
        time_markers(state),
        controls_row(state, play_pause_label),
        sleep_timer_section(state),
    ]
    .padding(spacing::MD)
    .spacing(spacing::SM)
    .into()
}

struct NowPlayingInfo {
    current_file_text: String,
    audiobook_name: String,
    audiobook_id: Option<i64>,
    progress_summary: String,
}

fn now_playing_info(state: &State) -> NowPlayingInfo {
    let current_file_text = state
        .selected_file
        .as_ref()
        .and_then(|f| std::path::Path::new(f).file_name())
        .and_then(|f| f.to_str())
        .map_or("No file selected", extract_chapter_name)
        .to_string();

    let mut audiobook_name = String::new();
    let mut audiobook_id = None;
    let mut progress_summary = String::new();

    if let Some(id) = state.selected_audiobook {
        if let Some(ab) = state.audiobooks.iter().find(|ab| ab.id == Some(id)) {
            audiobook_name.clone_from(&ab.name);
            audiobook_id = ab.id;
            progress_summary = calculate_progress_summary(
                &state.current_files,
                state.selected_file.as_ref(),
                ab.total_duration_ms,
            );
        }
    }

    NowPlayingInfo {
        current_file_text,
        audiobook_name,
        audiobook_id,
        progress_summary,
    }
}

fn now_playing_card(state: &State, info: &NowPlayingInfo) -> Element<'static, Message> {
    if info.audiobook_name.is_empty() {
        return container(
            text(format!("Now Playing: {}", info.current_file_text)).size(typography::SIZE_SM),
        )
        .padding(spacing::SM)
        .into();
    }

    // Build cover art thumbnail (100x100px for better visibility)
    let cover_element: Element<'_, Message> = info
        .audiobook_id
        .and_then(|id| state.cover_thumbnails.get(&id))
        .map_or_else(|| cover_placeholder(), |path| cover_thumbnail(path.clone()));

    let text_col = now_playing_text(info);

    container(
        row![cover_element, text_col]
            .spacing(spacing::LG)
            .align_y(iced::Alignment::Center),
    )
    .padding(spacing::MD)
    .style(|_theme: &iced::Theme| container::Style {
        background: Some({
            let start = colors::CONTINUE_GRADIENT_START;
            let end = colors::CONTINUE_GRADIENT_END;
            Color::from_rgb(
                f32::midpoint(start.r, end.r),
                f32::midpoint(start.g, end.g),
                f32::midpoint(start.b, end.b),
            )
            .into()
        }),
        border: Border {
            color: colors::CONTINUE_GRADIENT_END,
            width: border_width::REGULAR,
            radius: border_radius::LG.into(),
        },
        shadow: iced::Shadow {
            color: Color::from_rgba(0.96, 0.81, 1.0, 0.18),
            offset: iced::Vector::new(0.0, 6.0),
            blur_radius: 20.0,
        },
        text_color: Some(colors::TEXT_ON_PRIMARY),
        ..Default::default()
    })
    .into()
}

fn cover_placeholder() -> Element<'static, Message> {
    container(
        text("♪")
            .size(typography::SIZE_XXL)
            .style(|_theme: &iced::Theme| iced::widget::text::Style {
                color: Some(colors::TEXT_SECONDARY),
            }),
    )
    .width(Length::Fixed(100.0))
    .height(Length::Fixed(100.0))
    .center_x(Length::Fill)
    .center_y(Length::Fill)
    .style(|_theme: &iced::Theme| container::Style {
        background: Some({
            let base = colors::BG_ELEVATED;
            let accent = colors::PRIMARY_GRADIENT_START;
            Color::from_rgb(
                f32::midpoint(base.r, accent.r),
                f32::midpoint(base.g, accent.g),
                f32::midpoint(base.b, accent.b),
            )
            .into()
        }),
        border: Border {
            color: colors::PRIMARY_GRADIENT_END,
            width: border_width::REGULAR,
            radius: border_radius::MD.into(),
        },
        shadow: shadows::shadow_sm(),
        ..Default::default()
    })
    .into()
}

fn cover_thumbnail(path: std::path::PathBuf) -> Element<'static, Message> {
    container(
        image(iced::widget::image::Handle::from_path(path))
            .width(Length::Fixed(100.0))
            .height(Length::Fixed(100.0)),
    )
    .style(|_theme: &iced::Theme| container::Style {
        border: Border {
            color: colors::PRIMARY_GRADIENT_START,
            width: border_width::THIN,
            radius: border_radius::MD.into(),
        },
        shadow: shadows::shadow_sm(),
        ..Default::default()
    })
    .into()
}

fn now_playing_text(info: &NowPlayingInfo) -> iced::widget::Column<'static, Message> {
    let mut text_col = column![
        text(info.audiobook_name.clone())
            .size(typography::SIZE_XL)
            .style(|_theme: &iced::Theme| iced::widget::text::Style {
                color: Some(colors::TEXT_PRIMARY),
            }),
        text(info.current_file_text.clone())
            .size(typography::SIZE_BASE)
            .style(|_theme: &iced::Theme| iced::widget::text::Style {
                color: Some(colors::TEXT_SECONDARY),
            }),
    ]
    .spacing(spacing::XS);

    if !info.progress_summary.is_empty() {
        text_col = text_col.push(
            text(info.progress_summary.clone())
                .size(typography::SIZE_SM)
                .style(|_theme: &iced::Theme| iced::widget::text::Style {
                    color: Some(colors::PRIMARY),
                }),
        );
    }

    text_col
}

fn progress_slider(state: &State) -> Element<'_, Message> {
    let progress_container = container({
        let max_duration = state.total_duration.max(1.0);
        let current_time = state.current_time.clamp(0.0, max_duration);
        let is_active = state.playback == PlaybackStatus::Playing;

        widget_slider(0.0..=max_duration, current_time, Message::SeekTo)
            .height(28.0)
            .style(
                move |_theme: &iced::Theme, _status: iced::widget::slider::Status| {
                    let alpha = if is_active { 1.0 } else { 0.92 };
                    iced::widget::slider::Style {
                        rail: iced::widget::slider::Rail {
                            backgrounds: (
                                iced::Background::Color(colors::BG_SECONDARY),
                                iced::Background::Color(Color::from_rgba(
                                    f32::midpoint(
                                        colors::PRIMARY_GRADIENT_START.r,
                                        colors::PROGRESS_GRADIENT_END.r,
                                    ) * alpha,
                                    f32::midpoint(
                                        colors::PRIMARY_GRADIENT_START.g,
                                        colors::PROGRESS_GRADIENT_END.g,
                                    ) * alpha,
                                    f32::midpoint(
                                        colors::PRIMARY_GRADIENT_START.b,
                                        colors::PROGRESS_GRADIENT_END.b,
                                    ) * alpha,
                                    alpha,
                                )),
                            ),
                            width: 10.0,
                            border: iced::Border {
                                color: Color::from_rgba(0.0, 0.0, 0.0, 0.20),
                                width: border_width::THIN,
                                radius: 5.0.into(),
                            },
                        },
                        handle: iced::widget::slider::Handle {
                            shape: iced::widget::slider::HandleShape::Circle { radius: 12.0 },
                            background: if is_active {
                                iced::Background::Color(Color::from_rgb(
                                    f32::midpoint(
                                        colors::PRIMARY_GRADIENT_START.r,
                                        colors::SUNSET_GRADIENT_START.r,
                                    ),
                                    f32::midpoint(
                                        colors::PRIMARY_GRADIENT_START.g,
                                        colors::SUNSET_GRADIENT_START.g,
                                    ),
                                    f32::midpoint(
                                        colors::PRIMARY_GRADIENT_START.b,
                                        colors::SUNSET_GRADIENT_START.b,
                                    ),
                                ))
                            } else {
                                iced::Background::Color(colors::BG_ELEVATED)
                            },
                            border_width: border_width::THICK,
                            border_color: if is_active {
                                colors::ACCENT_GRADIENT_END
                            } else {
                                colors::BORDER_SUBTLE
                            },
                        },
                    }
                },
            )
    })
    .style(focus_indicator(
        state.focused_element == FocusedElement::ProgressSlider,
    ))
    .padding(iced::Padding::from([spacing::MD, 0.0]));

    mouse_area(progress_container)
        .on_enter(Message::TooltipHoverStart(
            crate::ui::tooltip::TooltipTarget::Progress,
        ))
        .on_exit(Message::TooltipHoverEnd)
        .into()
}

fn time_markers(state: &State) -> Element<'_, Message> {
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
    .padding(spacing::XS)
    .into()
}

fn controls_row<'a>(state: &'a State, play_pause_label: &'static str) -> Element<'a, Message> {
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
    .spacing(spacing::LG)
    .into()
}

fn sleep_timer_section(state: &State) -> Element<'_, Message> {
    container(sleep_timer_controls(state))
        .padding(spacing::MD)
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

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
