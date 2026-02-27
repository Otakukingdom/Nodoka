//! Player controls component: transport buttons, seek bar, speed, and volume.
//!
//! This module is split into multiple sub-modules for maintainability:
//! - [`playback`]: Play/pause/stop button controls
//! - [`speed`]: Playback speed controls
//! - [`volume`]: Volume slider with color-coded levels
//! - [`sleep`]: Sleep timer controls
//! - [`helpers`]: Time formatting and progress calculation utilities

mod playback;
mod sleep;
mod speed;
mod volume;
pub mod helpers;

use crate::ui::styles::{
    border_radius, colors, focus_indicator, shadows, spacing, typography,
};
use crate::ui::{FocusedElement, Message, PlaybackStatus, State};
use iced::widget::{
    column, container, image, mouse_area, row, slider as widget_slider, text, Space,
};
use iced::{Border, Color, Element, Length};

pub use playback::playback_controls;
pub use sleep::{format_remaining_seconds, sleep_timer_controls};
pub use speed::{format_speed_label, speed_controls, SpeedPreset};
pub use volume::{format_time, format_time_remaining, volume_controls, volume_level_color};
pub use helpers::{calculate_progress_summary, extract_chapter_name};

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
        let is_active = state.playback == PlaybackStatus::Playing;
        widget_slider(0.0..=max_duration, current_time, Message::SeekTo)
            .height(24.0)
            .style(move |_theme: &iced::Theme, _status: iced::widget::slider::Status| {
                let alpha = if is_active { 0.98 } else { 0.88 };
                iced::widget::slider::Style {
                    rail: iced::widget::slider::Rail {
                        backgrounds: (
                            iced::Background::Color(colors::BG_SECONDARY),
                            // Premium sunset gradient for fill with vibrant rose-to-coral transition
                            iced::Background::Color(Color::from_rgba(
                                colors::SUNSET_GRADIENT_START.r,
                                colors::SUNSET_GRADIENT_START.g,
                                colors::SUNSET_GRADIENT_START.b,
                                alpha,
                            )),
                        ),
                        width: 8.0, // Thicker for better visibility
                        border: iced::Border {
                            // Inset shadow for carved/recessed appearance
                            color: Color::from_rgba(0.0, 0.0, 0.0, 0.15),
                            width: 1.5,
                            radius: 4.0.into(),
                        },
                    },
                    handle: iced::widget::slider::Handle {
                        shape: iced::widget::slider::HandleShape::Circle { radius: 11.0 },
                        // Premium sunset gradient for active handle
                        background: if is_active {
                            iced::Background::Color(colors::SUNSET_GRADIENT_START)
                        } else {
                            iced::Background::Color(Color::WHITE)
                        },
                        border_width: 2.5,
                        border_color: if is_active {
                            colors::SUNSET_GRADIENT_END
                        } else {
                            Color::WHITE
                        },
                    },
                }
            })
        // Larger height for better grab affordance with enhanced styling
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
            // Enhanced with border and elevation for premium feel
            let cover_element: Element<'_, Message> = audiobook_id
                .and_then(|id| state.cover_thumbnails.get(&id))
                .map_or_else(
                    || {
                        // No cover available - show placeholder with enhanced gradient pattern
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
                            // Subtle gradient pattern for premium placeholder
                            background: Some(Color::from_rgba(
                                f32::midpoint(colors::BG_ELEVATED.r, colors::BG_RAISED.r),
                                f32::midpoint(colors::BG_ELEVATED.g, colors::BG_RAISED.g),
                                f32::midpoint(colors::BG_ELEVATED.b, colors::BG_RAISED.b),
                                1.0,
                            ).into()),
                            border: Border {
                                color: colors::PRIMARY_GRADIENT_START,
                                width: 1.5,
                                radius: border_radius::MD.into(),
                            },
                            shadow: shadows::shadow_md(),
                            ..Default::default()
                        })
                        .into()
                    },
                    |path| {
                        // Show cover thumbnail with enhanced border
                        container(
                            image(iced::widget::image::Handle::from_path(path.clone()))
                                .width(Length::Fixed(100.0))
                                .height(Length::Fixed(100.0)),
                        )
                        .style(|_theme: &iced::Theme| container::Style {
                            border: Border {
                                color: colors::BORDER_SUBTLE,
                                width: 1.0,
                                radius: border_radius::MD.into(),
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
            .style(|_theme: &iced::Theme| container::Style {
                // Multi-layer gradient effect for depth - using blended color
                background: Some(Color::from_rgba(
                    f32::midpoint(colors::CONTINUE_GRADIENT_START.r, colors::CONTINUE_GRADIENT_END.r),
                    f32::midpoint(colors::CONTINUE_GRADIENT_START.g, colors::CONTINUE_GRADIENT_END.g),
                    f32::midpoint(colors::CONTINUE_GRADIENT_START.b, colors::CONTINUE_GRADIENT_END.b),
                    1.0,
                ).into()),
                border: Border {
                    color: colors::ACCENT_GRADIENT_END,
                    width: 2.5,
                    radius: border_radius::LG.into(),
                },
                // Multi-layer shadow for dramatic elevation
                shadow: iced::Shadow {
                    color: Color::from_rgba(0.882, 0.114, 0.282, 0.20),
                    offset: iced::Vector::new(0.0, 8.0),
                    blur_radius: 24.0,
                },
                text_color: Some(colors::TEXT_ON_PRIMARY),
                ..Default::default()
            })
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
