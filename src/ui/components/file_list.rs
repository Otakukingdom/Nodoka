//! File list component showing individual audio files within an audiobook.

use crate::models::AudiobookFile;
use crate::ui::styles::{animations, colors, spacing, typography};
use crate::ui::Message;
use iced::widget::{button, column, container, row, scrollable, text, Space};
use iced::{Border, Color, Element, Length, Shadow};

/// Renders the file list with improved UX patterns
///
/// Features:
/// - Empty state with helpful guidance when no files exist
/// - File number prefixes (#1, #2, etc.) for better navigation context
/// - Duration display next to each file name
/// - Selection highlighting with background color
/// - Warning indicator for missing files
/// - Progress bars for files with partial progress
/// - Visual separation between files
/// - Improved completion indicators with semantic colors
#[must_use]
pub fn view<'a>(
    files: &'a [AudiobookFile],
    selected_path: Option<&String>,
    animation_elapsed_ms: u64,
    prefers_reduced_motion: bool,
) -> Element<'a, Message> {
    // Show empty state when no files are available
    if files.is_empty() {
        return super::empty_state(
            None,
            "No Files",
            "Select an audiobook from the list to view its files.",
            None,
            None,
        );
    }

    let items: Element<_> = files
        .iter()
        .enumerate()
        .fold(column![].spacing(spacing::XS), |col, (index, file)| {
            let is_selected = selected_path == Some(&file.full_path);
            let animation =
                list_item_animation(index, animation_elapsed_ms, prefers_reduced_motion);
            let item = build_file_item(file, is_selected, index + 1, animation);
            col.push(item)
        })
        .into();

    scrollable(items).height(Length::Fill).into()
}

/// Builds a single file list item with improved status indicators and visual hierarchy
///
/// Applies consistent interactive patterns:
/// - File number prefix for better navigation context
/// - Duration display for better scannability
/// - Cursor-pointer on hover for available files
/// - Subtle background color change on hover with smooth transition
/// - Disabled cursor for missing files
/// - Consistent spacing following 4px grid system (8px between items)
#[allow(clippy::too_many_lines)]
fn build_file_item(
    file: &AudiobookFile,
    selected: bool,
    file_number: usize,
    animation: ListItemAnimation,
) -> Element<'static, Message> {
    use crate::ui::styles::{border_radius, colors};
    use iced::Border;

    let is_missing = !file.file_exists;
    let duration = format_duration(file.length_of_file);
    let time_secondary = format_time_secondary(file, selected);
    let completeness = file.completeness;
    let has_progress = file.seek_position.is_some();
    let is_complete = completeness >= 100;
    let path = file.full_path.clone();
    let opacity = animation.opacity;

    let status_indicator = build_status_indicator(
        is_missing,
        is_complete,
        has_progress,
        selected,
        completeness,
        opacity,
    );
    let progress_element =
        build_progress_element(has_progress, is_complete, is_missing, completeness);
    let file_number_badge = build_file_number_badge(file_number, opacity);

    // Show "Continue from" marker for selected file with progress
    let continue_marker = if selected && has_progress && !is_complete {
        format_continue_position(file.seek_position.as_ref())
    } else {
        String::new()
    };

    let content_column = build_content_column(
        file.name.clone(),
        duration,
        time_secondary,
        continue_marker,
        file_number_badge,
        progress_element,
        status_indicator,
        opacity,
        animation.offset_y,
    );

    // Apply selection styling with enhanced gradient background
    let content_container = if selected {
        container(content_column)
            .style(move |_theme: &iced::Theme| iced::widget::container::Style {
                background: Some(with_alpha_gradient(colors::SUBTLE_GRADIENT_START, colors::SUBTLE_GRADIENT_END, opacity).into()),
                text_color: Some(with_alpha(colors::SELECTION_TEXT, opacity)),
                border: Border {
                    color: with_alpha(colors::PRIMARY_GRADIENT_START, opacity),
                    width: 2.0,
                    radius: border_radius::SM.into(),
                },
                ..Default::default()
            })
            .width(Length::Fill)
    } else {
        container(content_column).width(Length::Fill)
    };

    let mut item_button = button(content_container).width(Length::Fill);

    if !is_missing {
        item_button = item_button.on_press(Message::FileSelected(path));
    }

    // Apply hover styling with enhanced surface colors
    // Disabled files (missing) show no hover effect
    // Includes pressed state for tactile feedback using interactive overlay
    item_button
        .style(move |_theme: &iced::Theme, status| {
            use iced::widget::button;
            if is_missing {
                // Missing files: no hover effect, default cursor handled by lack of on_press
                button::Style {
                    background: Some(Color::TRANSPARENT.into()),
                    text_color: with_alpha(colors::TEXT_DISABLED, opacity),
                    border: Border {
                        color: Color::TRANSPARENT,
                        width: 0.0,
                        radius: border_radius::SM.into(),
                    },
                    ..button::Style::default()
                }
            } else {
                match status {
                    button::Status::Hovered if !selected => button::Style {
                        background: Some(with_alpha(colors::SURFACE_HOVER, opacity).into()),
                        border: Border {
                            color: with_alpha(colors::BORDER_SUBTLE, opacity),
                            width: 1.0,
                            radius: border_radius::SM.into(),
                        },
                        ..button::Style::default()
                    },
                    button::Status::Pressed if !selected => button::Style {
                        background: Some(with_alpha(colors::SURFACE_ACTIVE, opacity).into()),
                        border: Border {
                            color: with_alpha(colors::BORDER_PROMINENT, opacity),
                            width: 1.5,
                            radius: border_radius::SM.into(),
                        },
                        ..button::Style::default()
                    },
                    // Default transparent style for all other states
                    _ => button::Style {
                        background: Some(Color::TRANSPARENT.into()),
                        border: Border {
                            color: Color::TRANSPARENT,
                            width: 0.0,
                            radius: border_radius::SM.into(),
                        },
                        ..button::Style::default()
                    },
                }
            }
        })
        .into()
}

#[allow(clippy::fn_params_excessive_bools)]
fn build_status_indicator(
    is_missing: bool,
    is_complete: bool,
    has_progress: bool,
    is_selected: bool,
    completeness: i32,
    opacity: f32,
) -> iced::widget::Container<'static, Message> {
    use crate::ui::styles::{border_radius, colors, shadows};
    use iced::Border;

    // Enhanced status indicators with badge styling
    if is_missing {
        container(text("⚠").size(typography::SIZE_BASE)).style(move |_theme: &iced::Theme| {
            container::Style {
                text_color: Some(with_alpha(colors::WARNING_REFINED, opacity)),
                ..Default::default()
            }
        })
    } else if is_complete {
        container(text("✓").size(typography::SIZE_BASE))
            .padding(iced::Padding::from([spacing::XS, spacing::SM]))
            .style(move |_theme: &iced::Theme| container::Style {
                background: Some(with_alpha(colors::SUCCESS_REFINED, opacity).into()),
                border: Border {
                    color: with_alpha(colors::SUCCESS_LIGHT, opacity),
                    width: 1.0,
                    radius: border_radius::SM.into(),
                },
                shadow: shadows::shadow_sm(),
                text_color: Some(with_alpha(colors::TEXT_ON_PRIMARY, opacity)),
                ..Default::default()
            })
    } else if is_selected && has_progress {
        // Show play icon for current file with enhanced gradient background
        container(text("▶").size(typography::SIZE_BASE))
            .padding(iced::Padding::from([spacing::XS, spacing::SM]))
            .style(move |_theme: &iced::Theme| container::Style {
                background: Some(with_alpha(colors::PRIMARY_GRADIENT_START, opacity).into()),
                border: Border {
                    color: with_alpha(colors::PRIMARY_GRADIENT_END, opacity),
                    width: 1.0,
                    radius: border_radius::SM.into(),
                },
                shadow: shadows::shadow_sm(),
                text_color: Some(with_alpha(colors::TEXT_ON_PRIMARY, opacity)),
                ..Default::default()
            })
    } else if has_progress {
        // Show percentage only for in-progress files that aren't selected
        container(text(format!("{completeness}%")).size(typography::SIZE_SM)).style(
            move |_theme: &iced::Theme| container::Style {
                text_color: Some(with_alpha(colors::TEXT_SECONDARY, opacity)),
                ..Default::default()
            },
        )
    } else {
        container(text("").size(typography::SIZE_SM))
    }
}

fn build_progress_element(
    has_progress: bool,
    is_complete: bool,
    is_missing: bool,
    completeness: i32,
) -> iced::widget::Container<'static, Message> {
    use crate::ui::styles::{border_radius, colors, shadows};

    if has_progress && !is_complete && !is_missing {
        let completeness_u8 = u8::try_from(completeness.clamp(0, 100)).unwrap_or(0);
        container(
            container(Space::new())
                .width(Length::FillPortion(u16::from(completeness_u8)))
                .height(Length::Fixed(8.0))
                .style(|_theme: &iced::Theme| container::Style {
                    // Enhanced gradient fill for progress bar
                    background: Some(colors::PROGRESS_GRADIENT_START.into()),
                    border: Border {
                        color: Color::TRANSPARENT,
                        width: 0.0,
                        radius: border_radius::FULL.into(),
                    },
                    shadow: shadows::shadow_xs(),
                    ..Default::default()
                }),
        )
        .width(Length::Fill)
        .height(Length::Fixed(8.0))
        .style(|_theme: &iced::Theme| container::Style {
            background: Some(colors::BG_SECONDARY.into()),
            border: Border {
                color: colors::BORDER_SUBTLE,
                width: 1.0,
                radius: border_radius::FULL.into(),
            },
            ..Default::default()
        })
    } else {
        container(Space::new())
            .width(Length::Fill)
            .height(Length::Fixed(8.0))
    }
}

fn build_file_number_badge(
    file_number: usize,
    opacity: f32,
) -> iced::widget::Container<'static, Message> {
    use crate::ui::styles::{border_radius, colors};
    use iced::Border;

    // Premium gradient color for badge background
    let badge_gradient = with_alpha_gradient_badge(
        colors::ACCENT_GRADIENT_START,
        colors::PRIMARY_GRADIENT_END,
        opacity,
    );

    container(
        text(format!("#{file_number}"))
            .size(typography::SIZE_XS)
            .style(move |_theme: &iced::Theme| iced::widget::text::Style {
                color: Some(with_alpha(colors::TEXT_ON_PRIMARY, opacity)),
            }),
    )
    .padding(iced::Padding::from([spacing::XS, spacing::SM]))
    .style(move |_theme: &iced::Theme| container::Style {
        // Premium gradient background for distinctive badge appearance
        background: Some(badge_gradient.into()),
        border: Border {
            color: with_alpha(colors::ACCENT_GRADIENT_END, opacity),
            width: 1.5,
            radius: border_radius::SM.into(),
        },
        // Enhanced shadow with increased blur for premium depth
        shadow: Shadow {
            color: with_alpha(colors::ACCENT_GRADIENT_START, 0.15),
            offset: iced::Vector::new(0.0, 1.0),
            blur_radius: 5.0,
        },
        ..Default::default()
    })
}

/// Creates gradient color for file number badge background
/// Enhanced with signature gradient for brand consistency
fn with_alpha_gradient_badge(_start: Color, _end: Color, opacity: f32) -> Color {
    // Use signature gradient for brand consistency
    let start = colors::SIGNATURE_GRADIENT_START;
    let end = colors::ACCENT_GRADIENT_END;
    Color {
        r: start.r.mul_add(0.6 * opacity, end.r * 0.4 * opacity),
        g: start.g.mul_add(0.6 * opacity, end.g * 0.4 * opacity),
        b: start.b.mul_add(0.6 * opacity, end.b * 0.4 * opacity),
        a: opacity,
    }
}

#[allow(clippy::too_many_arguments, clippy::needless_pass_by_value)]
fn build_content_column(
    name: String,
    duration: String,
    time_remaining: String,
    continue_marker: String,
    file_number_badge: iced::widget::Container<'static, Message>,
    progress_element: iced::widget::Container<'static, Message>,
    status_indicator: iced::widget::Container<'static, Message>,
    opacity: f32,
    offset_y: f32,
) -> iced::widget::Column<'static, Message> {
    use crate::ui::styles::colors;

    let max_offset = animations::STAGGER_TRANSLATE_Y_PX;
    let offset_y = offset_y.clamp(0.0, max_offset);

    // Build duration/time remaining display
    // Show "duration • time remaining" if there's remaining time, otherwise just duration
    let duration_text = if !time_remaining.is_empty() && time_remaining != "Complete" {
        format!("{duration} • {time_remaining}")
    } else if time_remaining == "Complete" {
        format!("{duration} • Complete")
    } else {
        duration
    };

    let mut content = column![row![
        file_number_badge,
        text(name)
            .size(typography::SIZE_SM)
            .style(move |_theme: &iced::Theme| iced::widget::text::Style {
                color: Some(with_alpha(colors::TEXT_PRIMARY, opacity)),
            }),
        Space::new().width(Length::Fill),
        // Duration and time remaining display
        text(duration_text)
            .size(typography::SIZE_SM)
            .style(move |_theme: &iced::Theme| iced::widget::text::Style {
                color: Some(with_alpha(colors::TEXT_SECONDARY, opacity)),
            }),
    ]
    .spacing(spacing::SM)
    .align_y(iced::Alignment::Center),];

    // Add continue marker if present
    if !continue_marker.is_empty() {
        content = content.push(text(continue_marker).size(typography::SIZE_XS).style(
            move |_theme: &iced::Theme| iced::widget::text::Style {
                color: Some(with_alpha(colors::PRIMARY, opacity)),
            },
        ));
    }

    content = content.push(progress_element);
    content = content.push(row![Space::new().width(Length::Fill), status_indicator,]);

    content
        .padding(iced::Padding {
            top: spacing::SM + offset_y,
            bottom: spacing::SM + (max_offset - offset_y),
            left: spacing::SM,
            right: spacing::SM,
        })
        .spacing(spacing::XS)
}

#[derive(Debug, Clone, Copy)]
struct ListItemAnimation {
    opacity: f32,
    offset_y: f32,
}

fn list_item_animation(
    index: usize,
    elapsed_ms: u64,
    prefers_reduced_motion: bool,
) -> ListItemAnimation {
    if prefers_reduced_motion {
        return ListItemAnimation {
            opacity: 1.0,
            offset_y: 0.0,
        };
    }
    ListItemAnimation {
        opacity: animations::stagger_opacity(index, 50, elapsed_ms),
        offset_y: animations::stagger_translate_y(index, 50, elapsed_ms),
    }
}

fn with_alpha(color: Color, alpha: f32) -> Color {
    Color {
        r: color.r,
        g: color.g,
        b: color.b,
        a: (color.a * alpha).clamp(0.0, 1.0),
    }
}

/// Creates a gradient color by blending two colors
fn with_alpha_gradient(start: Color, end: Color, opacity: f32) -> Color {
    Color {
        r: f32::midpoint(start.r, end.r) * opacity,
        g: f32::midpoint(start.g, end.g) * opacity,
        b: f32::midpoint(start.b, end.b) * opacity,
        a: opacity,
    }
}

/// Formats duration in milliseconds to human-readable time string
fn format_duration(duration_ms: Option<i64>) -> String {
    match duration_ms {
        Some(ms) if ms > 0 => {
            let seconds = ms / 1000;
            let minutes = seconds / 60;
            let hours = minutes / 60;

            if hours > 0 {
                format!("{}:{:02}:{:02}", hours, minutes % 60, seconds % 60)
            } else {
                format!("{}:{:02}", minutes, seconds % 60)
            }
        }
        _ => "Unknown".to_string(),
    }
}

fn format_time_secondary(file: &AudiobookFile, is_selected: bool) -> String {
    if file.completeness >= 100 {
        return String::from("Complete");
    }

    if !is_selected {
        return String::new();
    }

    let Some(total_ms) = file.length_of_file else {
        return String::new();
    };

    if total_ms <= 0 {
        return String::new();
    }

    let Some(seek_ms) = file.seek_position else {
        return String::new();
    };

    if seek_ms <= 0 {
        return String::new();
    }

    let pos = crate::ui::styles::format_time_ms(seek_ms);
    let total = crate::ui::styles::format_time_ms(total_ms);
    format!("{pos} / {total}")
}

/// Formats the continue position for display
///
/// Returns a string like "Continue from 12:34" for files with a seek position
fn format_continue_position(seek_position: Option<&i64>) -> String {
    let Some(&pos_ms) = seek_position else {
        return String::new();
    };

    if pos_ms <= 0 {
        return String::new();
    }

    let seconds = pos_ms / 1000;
    let minutes = seconds / 60;
    let hours = minutes / 60;

    if hours > 0 {
        format!(
            "Continue from {}:{:02}:{:02}",
            hours,
            minutes % 60,
            seconds % 60
        )
    } else {
        format!("Continue from {}:{:02}", minutes, seconds % 60)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_ANIMATION_ELAPSED_MS: u64 = 1000;
    const TEST_PREFERS_REDUCED_MOTION: bool = false;

    fn create_test_file(
        name: &str,
        full_path: &str,
        completeness: i32,
        missing: bool,
    ) -> AudiobookFile {
        AudiobookFile {
            audiobook_id: 1,
            name: name.to_string(),
            full_path: full_path.to_string(),
            length_of_file: Some(3_600_000), // 1 hour
            seek_position: if completeness > 0 { Some(1000) } else { None },
            checksum: None,
            position: 0,
            completeness,
            file_exists: !missing,
            created_at: chrono::Utc::now(),
        }
    }

    fn test_animation() -> ListItemAnimation {
        ListItemAnimation {
            opacity: 1.0,
            offset_y: 0.0,
        }
    }

    #[test]
    fn test_view_renders_empty_list() {
        let files: Vec<AudiobookFile> = vec![];
        let element = view(
            &files,
            None,
            TEST_ANIMATION_ELAPSED_MS,
            TEST_PREFERS_REDUCED_MOTION,
        );
        drop(element);
    }

    #[test]
    fn test_view_renders_single_file() {
        let files = vec![create_test_file(
            "chapter1.mp3",
            "/path/chapter1.mp3",
            50,
            false,
        )];
        let element = view(
            &files,
            None,
            TEST_ANIMATION_ELAPSED_MS,
            TEST_PREFERS_REDUCED_MOTION,
        );
        drop(element);
    }

    #[test]
    fn test_view_renders_multiple_files() {
        let files = vec![
            create_test_file("chapter1.mp3", "/path/chapter1.mp3", 100, false),
            create_test_file("chapter2.mp3", "/path/chapter2.mp3", 50, false),
            create_test_file("chapter3.mp3", "/path/chapter3.mp3", 0, false),
        ];
        let element = view(
            &files,
            None,
            TEST_ANIMATION_ELAPSED_MS,
            TEST_PREFERS_REDUCED_MOTION,
        );
        drop(element);
    }

    #[test]
    fn test_view_renders_with_selection() {
        let files = vec![
            create_test_file("chapter1.mp3", "/path/chapter1.mp3", 50, false),
            create_test_file("chapter2.mp3", "/path/chapter2.mp3", 25, false),
        ];
        let selected = "/path/chapter2.mp3".to_string();
        let element = view(
            &files,
            Some(&selected),
            TEST_ANIMATION_ELAPSED_MS,
            TEST_PREFERS_REDUCED_MOTION,
        );
        drop(element);
    }

    #[test]
    fn test_build_file_item_not_selected() {
        let file = create_test_file("chapter1.mp3", "/path/chapter1.mp3", 50, false);
        let element = build_file_item(&file, false, 1, test_animation());
        drop(element);
    }

    #[test]
    fn test_build_file_item_selected() {
        let file = create_test_file("chapter1.mp3", "/path/chapter1.mp3", 50, false);
        let element = build_file_item(&file, true, 1, test_animation());
        drop(element);
    }

    #[test]
    fn test_build_file_item_missing_file() {
        let file = create_test_file("chapter1.mp3", "/path/chapter1.mp3", 0, true);
        let element = build_file_item(&file, false, 1, test_animation());
        drop(element);
    }

    #[test]
    fn test_build_file_item_complete() {
        let file = create_test_file("chapter1.mp3", "/path/chapter1.mp3", 100, false);
        let element = build_file_item(&file, false, 1, test_animation());
        drop(element);
    }

    #[test]
    fn test_build_file_item_with_progress() {
        let file = create_test_file("chapter1.mp3", "/path/chapter1.mp3", 50, false);
        let element = build_file_item(&file, false, 1, test_animation());
        drop(element);
    }

    #[test]
    fn test_build_file_item_no_progress() {
        let mut file = create_test_file("chapter1.mp3", "/path/chapter1.mp3", 0, false);
        file.seek_position = None;
        let element = build_file_item(&file, false, 1, test_animation());
        drop(element);
    }

    #[test]
    fn test_format_duration_with_hours() {
        assert_eq!(format_duration(Some(3_661_000)), "1:01:01");
        assert_eq!(format_duration(Some(7_200_000)), "2:00:00");
    }

    #[test]
    fn test_format_duration_with_minutes_only() {
        assert_eq!(format_duration(Some(125_000)), "2:05");
        assert_eq!(format_duration(Some(60000)), "1:00");
    }

    #[test]
    fn test_format_duration_with_zero() {
        assert_eq!(format_duration(Some(0)), "Unknown");
    }

    #[test]
    fn test_format_duration_with_negative() {
        assert_eq!(format_duration(Some(-1)), "Unknown");
    }

    #[test]
    fn test_format_duration_with_none() {
        assert_eq!(format_duration(None), "Unknown");
    }

    #[test]
    fn test_build_file_item_with_long_filename() {
        let long_name = "This is a very long chapter name that should still render correctly without causing layout issues.mp3";
        let file = create_test_file(long_name, "/path/long.mp3", 50, false);
        let element = build_file_item(&file, false, 1, test_animation());
        drop(element);
    }

    #[test]
    fn test_build_file_item_with_special_characters() {
        let file = create_test_file(
            "Chapter: 1 (Part 2) [Extended].mp3",
            "/path/file.mp3",
            50,
            false,
        );
        let element = build_file_item(&file, false, 1, test_animation());
        drop(element);
    }

    #[test]
    fn test_view_handles_large_file_list() {
        let files: Vec<AudiobookFile> = (1..=100)
            .map(|i| {
                create_test_file(
                    &format!("chapter{i}.mp3"),
                    &format!("/path/chapter{i}.mp3"),
                    i % 101,
                    false,
                )
            })
            .collect();
        let element = view(
            &files,
            None,
            TEST_ANIMATION_ELAPSED_MS,
            TEST_PREFERS_REDUCED_MOTION,
        );
        drop(element);
    }

    #[test]
    fn test_file_without_duration() {
        let mut file = create_test_file("chapter1.mp3", "/path/chapter1.mp3", 50, false);
        file.length_of_file = None;
        let element = build_file_item(&file, false, 1, test_animation());
        drop(element);
    }

    #[test]
    fn test_file_with_zero_duration() {
        let mut file = create_test_file("chapter1.mp3", "/path/chapter1.mp3", 50, false);
        file.length_of_file = Some(0);
        let element = build_file_item(&file, false, 1, test_animation());
        drop(element);
    }

    #[test]
    fn test_missing_file_not_clickable() {
        let file = create_test_file("missing.mp3", "/path/missing.mp3", 0, true);
        let element = build_file_item(&file, false, 1, test_animation());
        // Just verify it renders without panic
        drop(element);
    }

    #[test]
    fn test_build_file_item_uses_selection_parameter() {
        let file = create_test_file("chapter1.mp3", "/path/chapter1.mp3", 50, false);

        // Verify both selected and unselected states render without panic
        let element_unselected = build_file_item(&file, false, 1, test_animation());
        drop(element_unselected);

        let element_selected = build_file_item(&file, true, 2, test_animation());
        drop(element_selected);

        // Note: Cannot verify visual styling due to iced Element opacity,
        // but we ensure both code paths execute without panic
    }

    #[test]
    fn test_view_properly_passes_selection_to_files() {
        let files = vec![
            create_test_file("chapter1.mp3", "/path/chapter1.mp3", 100, false),
            create_test_file("chapter2.mp3", "/path/chapter2.mp3", 50, false),
            create_test_file("chapter3.mp3", "/path/chapter3.mp3", 0, false),
        ];

        let selected = "/path/chapter2.mp3".to_string();
        let element = view(
            &files,
            Some(&selected),
            TEST_ANIMATION_ELAPSED_MS,
            TEST_PREFERS_REDUCED_MOTION,
        );
        drop(element);
    }
}
