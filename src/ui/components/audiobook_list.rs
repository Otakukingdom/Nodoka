//! Audiobook library list component with cover thumbnails and scan status.

use crate::models::Audiobook;
use crate::ui::state::ScanState;
use crate::ui::styles::components::border_width;
use crate::ui::styles::{animations, border_radius, loading, shadows, spacing, typography};
use crate::ui::Message;
use iced::widget::{button, column, container, image, mouse_area, row, scrollable, text, Space};
use iced::{Border, Color, Element, Length};
use std::collections::HashMap;
use std::hash::BuildHasher;
use std::path::PathBuf;

/// Parameter bundle for [`view`].
pub struct ViewParams<'a, S: BuildHasher> {
    pub audiobooks: &'a [Audiobook],
    pub selected_id: Option<i64>,
    pub cover_thumbnails: &'a HashMap<i64, PathBuf, S>,
    pub scan_state: &'a ScanState,
    pub animation_elapsed_ms: u64,
    pub prefers_reduced_motion: bool,
    pub search_query: &'a str,
    pub sort_order: crate::ui::state::AudiobookSortOrder,
    pub filter_status: crate::ui::state::AudiobookFilterStatus,
}

fn filter_and_sort_audiobooks<'a>(
    audiobooks: &'a [Audiobook],
    search_query: &str,
    sort_order: crate::ui::state::AudiobookSortOrder,
    filter_status: crate::ui::state::AudiobookFilterStatus,
) -> Vec<&'a Audiobook> {
    use crate::ui::state::{AudiobookFilterStatus, AudiobookSortOrder};

    let query_lower = search_query.to_lowercase();

    let mut filtered: Vec<&Audiobook> = audiobooks
        .iter()
        .filter(|ab| {
            if !query_lower.is_empty() {
                let title_match = ab.name.to_lowercase().contains(&query_lower);
                let author_match = ab
                    .author
                    .as_ref()
                    .is_some_and(|a| a.to_lowercase().contains(&query_lower));
                if !title_match && !author_match {
                    return false;
                }
            }

            match filter_status {
                AudiobookFilterStatus::All => true,
                AudiobookFilterStatus::InProgress => ab.completeness > 0 && ab.completeness < 100,
                AudiobookFilterStatus::Complete => ab.completeness >= 100,
                AudiobookFilterStatus::NotStarted => ab.completeness == 0,
            }
        })
        .collect();

    match sort_order {
        AudiobookSortOrder::RecentlyPlayed => {
            filtered.sort_by(|a, b| b.last_listened.cmp(&a.last_listened));
        }
        AudiobookSortOrder::Title => {
            filtered.sort_by(|a, b| a.name.cmp(&b.name));
        }
        AudiobookSortOrder::Progress => {
            filtered.sort_by(|a, b| b.completeness.cmp(&a.completeness));
        }
        AudiobookSortOrder::DateAdded => {
            filtered.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        }
    }

    filtered
}

/// Renders the audiobook list with improved visual design.
///
/// Features:
/// - Empty state with helpful guidance when no audiobooks exist
/// - Skeleton UI loading states during directory scanning
/// - Larger cover thumbnails (80x80px) for better visibility
/// - Selection highlighting with left border accent (4px PRIMARY color)
/// - Progress bar visualization for each audiobook
/// - Improved spacing (8px between items) for better touch targets
/// - Success indicator for completed audiobooks
/// - Search/filter/sort support with efficient filtering
#[must_use]
pub fn view<'a, S: BuildHasher>(params: &ViewParams<'a, S>) -> Element<'a, Message> {
    use crate::ui::state::{AudiobookFilterStatus, AudiobookSortOrder};

    let filtered = filter_and_sort_audiobooks(
        params.audiobooks,
        params.search_query,
        params.sort_order,
        params.filter_status,
    );

    // Check if we should show empty state
    let has_active_filters = !params.search_query.is_empty()
        || params.sort_order != AudiobookSortOrder::default()
        || params.filter_status != AudiobookFilterStatus::default();

    if filtered.is_empty() {
        return match params.scan_state {
            ScanState::Scanning { .. } => skeleton_audiobook_list(),
            ScanState::Idle => {
                if has_active_filters {
                    super::empty_state(
                        None,
                        "No Results Found",
                        "No audiobooks match your current search or filters. Try adjusting your search query or clearing filters.",
                        Some("Clear Filters"),
                        Some(Message::ClearFilters),
                    )
                } else {
                    super::empty_state(
                        None,
                        "No Audiobooks Found",
                        "Add audiobook directories in Settings to get started. Nodoka will automatically scan for audiobook files.",
                        Some("Open Settings"),
                        Some(Message::OpenSettings),
                    )
                }
            }
        };
    }

    let items: Element<_> = filtered
        .iter()
        .enumerate()
        .fold(column![].spacing(spacing::SM), |col, (index, ab)| {
            let is_selected = params.selected_id == ab.id;
            let animation = list_item_animation(
                index,
                params.animation_elapsed_ms,
                params.prefers_reduced_motion,
            );
            let item = build_audiobook_item(ab, is_selected, params.cover_thumbnails, animation);
            col.push(item)
        })
        .into();

    scrollable(items).height(Length::Fill).into()
}

/// Builds a single audiobook list item with improved visual feedback
///
/// Applies consistent interactive patterns:
/// - Cursor-pointer on hover for clear affordance
/// - Subtle background color change on hover (200ms transition)
/// - Selection highlighting with primary color
/// - Proper focus indicators for keyboard navigation
fn build_audiobook_item<'a, S: BuildHasher>(
    ab: &'a Audiobook,
    selected: bool,
    cover_thumbnails: &'a HashMap<i64, PathBuf, S>,
    animation: ListItemAnimation,
) -> Element<'a, Message> {
    use crate::ui::styles::{border_radius, colors};

    let opacity = animation.opacity;
    let id = ab.id;

    let cover = build_cover(ab.id, cover_thumbnails, opacity);
    let content_row = build_content_row(ab, cover, opacity, animation.offset_y);

    // Apply selection styling with simplified 2-color gradient
    let content_container = if selected {
        container(content_row)
            .style(move |_theme: &iced::Theme| iced::widget::container::Style {
                // Simplified 2-color gradient for cleaner selection appearance
                background: Some(
                    Color {
                        r: f32::midpoint(
                            colors::PRIMARY_GRADIENT_START.r,
                            colors::ACCENT_GRADIENT_START.r,
                        ) * opacity,
                        g: f32::midpoint(
                            colors::PRIMARY_GRADIENT_START.g,
                            colors::ACCENT_GRADIENT_START.g,
                        ) * opacity,
                        b: f32::midpoint(
                            colors::PRIMARY_GRADIENT_START.b,
                            colors::ACCENT_GRADIENT_START.b,
                        ) * opacity,
                        a: 0.98 * opacity,
                    }
                    .into(),
                ),
                text_color: Some(with_alpha(colors::TEXT_ON_PRIMARY, opacity)),
                border: Border {
                    color: with_alpha(colors::ACCENT_GRADIENT_END, opacity),
                    width: 3.0, // Consistent selection border width
                    radius: border_radius::MD.into(),
                },
                shadow: shadows::shadow_md(),
                ..Default::default()
            })
            .width(Length::Fill)
    } else {
        container(content_row).width(Length::Fill)
    };

    // Apply hover styling with simplified gradient
    let item_button = button(content_container)
        .on_press_maybe(id.map(Message::AudiobookSelected))
        .width(Length::Fill)
        .style(move |_theme: &iced::Theme, status| {
            use iced::widget::button;
            match status {
                button::Status::Hovered if !selected => {
                    // Simplified 2-color sunset gradient for hover
                    button::Style {
                        background: Some(
                            {
                                let start = colors::SUNSET_GRADIENT_START;
                                let end = colors::SUNSET_GRADIENT_END;
                                Color {
                                    r: f32::midpoint(start.r, end.r) * 0.30 * opacity,
                                    g: f32::midpoint(start.g, end.g) * 0.30 * opacity,
                                    b: f32::midpoint(start.b, end.b) * 0.30 * opacity,
                                    a: 0.30 * opacity,
                                }
                            }
                            .into(),
                        ),
                        border: Border {
                            color: with_alpha(colors::SUNSET_GRADIENT_END, opacity),
                            width: border_width::THIN,
                            radius: border_radius::SM.into(),
                        },
                        shadow: shadows::shadow_sm(),
                        ..button::Style::default()
                    }
                }
                button::Status::Pressed if !selected => button::Style {
                    background: Some(with_alpha(colors::SURFACE_ACTIVE, opacity).into()),
                    border: Border {
                        color: with_alpha(colors::BORDER_PROMINENT, opacity),
                        width: border_width::THIN,
                        radius: border_radius::SM.into(),
                    },
                    shadow: shadows::shadow_xs(),
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
        });

    mouse_area(item_button)
        .on_enter(Message::TooltipHoverStart(
            crate::ui::tooltip::TooltipTarget::AudiobookItem,
        ))
        .on_exit(Message::TooltipHoverEnd)
        .into()
}

fn build_cover<S: BuildHasher>(
    id: Option<i64>,
    cover_thumbnails: &HashMap<i64, PathBuf, S>,
    opacity: f32,
) -> Element<'_, Message> {
    use crate::ui::styles::{border_radius, colors};

    // Premium cover thumbnail (120x120px) with refined shadow and gradient treatments
    id.and_then(|id| cover_thumbnails.get(&id)).map_or_else(
        move || {
            // Simplified 2-color gradient for cover placeholder
            let placeholder_bg = with_alpha_gradient_cover(
                colors::BG_ELEVATED,
                colors::PRIMARY_GRADIENT_START,
                opacity,
            );
            container(
                text("♪")
                    .size(typography::SIZE_XL)
                    .style(move |_theme: &iced::Theme| iced::widget::text::Style {
                        color: Some(with_alpha(colors::TEXT_SECONDARY, opacity)),
                    }),
            )
            .width(Length::Fixed(120.0))
            .height(Length::Fixed(120.0))
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .style(move |_theme: &iced::Theme| container::Style {
                // Simplified gradient for premium placeholder appearance
                background: Some(placeholder_bg.into()),
                border: Border {
                    color: with_alpha(colors::PRIMARY_GRADIENT_END, opacity),
                    width: border_width::REGULAR,
                    radius: border_radius::MD.into(),
                },
                shadow: shadows::shadow_sm(),
                ..Default::default()
            })
            .into()
        },
        move |path| {
            container(
                image(iced::widget::image::Handle::from_path(path.clone()))
                    .width(Length::Fixed(120.0))
                    .height(Length::Fixed(120.0)),
            )
            .style(move |_theme: &iced::Theme| container::Style {
                border: Border {
                    color: with_alpha(colors::PRIMARY_GRADIENT_START, opacity),
                    width: border_width::THIN,
                    radius: border_radius::MD.into(),
                },
                shadow: shadows::shadow_sm(),
                ..Default::default()
            })
            .into()
        },
    )
}

/// Creates simplified 2-color gradient for cover placeholder background
fn with_alpha_gradient_cover(base: Color, primary: Color, opacity: f32) -> Color {
    Color {
        r: f32::midpoint(base.r, primary.r) * opacity,
        g: f32::midpoint(base.g, primary.g) * opacity,
        b: f32::midpoint(base.b, primary.b) * opacity,
        a: opacity,
    }
}

fn build_content_row<'a>(
    ab: &'a Audiobook,
    cover: Element<'a, Message>,
    opacity: f32,
    offset_y: f32,
) -> iced::widget::Row<'a, Message> {
    let max_offset = animations::STAGGER_TRANSLATE_Y_PX;
    let offset_y = offset_y.clamp(0.0, max_offset);
    let completeness_u8 = u8::try_from(ab.completeness.clamp(0, 100)).unwrap_or(0);

    let remaining_text = format_remaining_time(
        ab.remaining_duration_ms,
        ab.total_duration_ms,
        ab.completeness,
    );
    let last_listened_text = format_relative_time(ab.last_listened);

    let metadata_items = build_metadata_items(ab, remaining_text, last_listened_text, opacity);
    let metadata_row = iced::widget::Row::with_children(metadata_items);

    let mut content_column = build_content_column(ab, opacity);
    content_column.push(build_progress_bar_element(completeness_u8, opacity));
    content_column.push(metadata_row.into());

    row![
        cover,
        iced::widget::Column::with_children(content_column)
            .spacing(spacing::XS)
            .width(Length::Fill)
    ]
    .spacing(spacing::MD)
    .padding(iced::Padding {
        top: spacing::SM + offset_y,
        bottom: spacing::SM + (max_offset - offset_y),
        left: spacing::MD,
        right: spacing::MD,
    })
}

/// Builds metadata items for the audiobook row
fn build_metadata_items<'a>(
    ab: &'a Audiobook,
    remaining_text: String,
    last_listened_text: String,
    opacity: f32,
) -> Vec<Element<'a, Message>> {
    use crate::ui::styles::{border_radius, colors};

    let mut metadata_items: Vec<Element<'a, Message>> = Vec::new();

    if ab.completeness >= 100 {
        metadata_items.push(
            container(text("✓ Complete").size(typography::SIZE_SM).style(
                move |_theme: &iced::Theme| iced::widget::text::Style {
                    color: Some(with_alpha(colors::TEXT_ON_PRIMARY, opacity)),
                },
            ))
            .padding(iced::Padding::from([spacing::XS, spacing::SM]))
            .style(move |_theme: &iced::Theme| container::Style {
                background: Some(with_alpha(colors::SUCCESS_REFINED, opacity).into()),
                border: Border {
                    radius: border_radius::SM.into(),
                    ..Default::default()
                },
                ..Default::default()
            })
            .into(),
        );
    } else if !remaining_text.is_empty() {
        metadata_items.push(
            text(remaining_text)
                .size(typography::SIZE_SM)
                .style(move |_theme: &iced::Theme| iced::widget::text::Style {
                    color: Some(with_alpha(colors::TEXT_SECONDARY, opacity)),
                })
                .into(),
        );
    }

    if !last_listened_text.is_empty() {
        metadata_items.push(Space::new().width(Length::Fill).into());
        metadata_items.push(
            text(last_listened_text)
                .size(typography::SIZE_XS)
                .style(move |_theme: &iced::Theme| iced::widget::text::Style {
                    color: Some(with_alpha(colors::TEXT_SECONDARY, opacity)),
                })
                .into(),
        );
    }

    metadata_items
}

/// Builds the content column with title and author
fn build_content_column(ab: &Audiobook, opacity: f32) -> Vec<Element<'_, Message>> {
    use crate::ui::styles::colors;

    let mut content_column = vec![text(&ab.name)
        .size(typography::SIZE_LG)
        .style(move |_theme: &iced::Theme| iced::widget::text::Style {
            color: Some(with_alpha(colors::TEXT_PRIMARY, opacity)),
        })
        .into()];

    if let Some(author) = &ab.author {
        content_column.push(
            text(author)
                .size(typography::SIZE_SM)
                .style(move |_theme: &iced::Theme| iced::widget::text::Style {
                    color: Some(with_alpha(colors::TEXT_SECONDARY, opacity)),
                })
                .into(),
        );
    }

    content_column
}

/// Builds the progress bar element
fn build_progress_bar_element(completeness_u8: u8, opacity: f32) -> Element<'static, Message> {
    use crate::ui::styles::{border_radius, colors};

    let p = u16::from(completeness_u8.min(100));
    let inner: Element<'static, Message> = if p == 0 {
        container(Space::new())
            .width(Length::Fill)
            .height(Length::Fixed(8.0))
            .into()
    } else if p >= 100 {
        container(Space::new())
            .width(Length::Fill)
            .height(Length::Fixed(8.0))
            .style(move |_theme: &iced::Theme| container::Style {
                background: Some(with_alpha(colors::PROGRESS_GRADIENT_START, opacity).into()),
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: border_radius::FULL.into(),
                },
                shadow: shadows::shadow_xs(),
                ..Default::default()
            })
            .into()
    } else {
        let filled = container(Space::new())
            .width(Length::FillPortion(p))
            .height(Length::Fixed(8.0))
            .style(move |_theme: &iced::Theme| container::Style {
                background: Some(with_alpha(colors::PROGRESS_GRADIENT_START, opacity).into()),
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: border_radius::FULL.into(),
                },
                shadow: shadows::shadow_xs(),
                ..Default::default()
            });
        let empty = container(Space::new())
            .width(Length::FillPortion(100 - p))
            .height(Length::Fixed(8.0));

        row![filled, empty]
            .width(Length::Fill)
            .height(Length::Fixed(8.0))
            .into()
    };

    container(inner)
        .width(Length::Fill)
        .height(Length::Fixed(8.0))
        .style(move |_theme: &iced::Theme| container::Style {
            background: Some(with_alpha(colors::BG_SECONDARY, opacity).into()),
            border: Border {
                color: with_alpha(colors::BORDER_SUBTLE, opacity),
                width: 1.0,
                radius: border_radius::FULL.into(),
            },
            ..Default::default()
        })
        .into()
}

/// Formats remaining time for display
fn format_remaining_time(
    remaining_duration_ms: Option<i64>,
    total_duration_ms: Option<i64>,
    completeness: i32,
) -> String {
    let remaining = remaining_duration_ms.filter(|ms| *ms > 0).or_else(|| {
        total_duration_ms.and_then(|total| {
            if total <= 0 {
                return None;
            }
            let remaining = total * (100 - i64::from(completeness.clamp(0, 100))) / 100;
            (remaining > 0).then_some(remaining)
        })
    });

    let Some(remaining) = remaining else {
        return String::new();
    };

    let hours = remaining / (1000 * 60 * 60);
    let minutes = (remaining / (1000 * 60)) % 60;
    if hours > 0 {
        format!("{hours}h {minutes}m remaining")
    } else {
        format!("{minutes}m remaining")
    }
}

/// Formats relative time from optional timestamp
fn format_relative_time(timestamp: Option<chrono::DateTime<chrono::Utc>>) -> String {
    let Some(ts) = timestamp else {
        return String::new();
    };
    let now = chrono::Utc::now();
    let duration = now.signed_duration_since(ts);

    if duration.num_minutes() < 1 {
        "Just now".to_string()
    } else if duration.num_hours() < 1 {
        format!("{} min ago", duration.num_minutes())
    } else if duration.num_days() < 1 {
        format!("{} hours ago", duration.num_hours())
    } else if duration.num_days() == 1 {
        "Yesterday".to_string()
    } else if duration.num_days() < 7 {
        format!("{} days ago", duration.num_days())
    } else if duration.num_days() < 30 {
        format!("{} weeks ago", duration.num_days() / 7)
    } else {
        ts.format("%b %d, %Y").to_string()
    }
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

/// Skeleton UI for audiobook list during scanning
///
/// Shows 5 placeholder items to indicate loading activity.
/// Skeleton UI provides better feedback than spinners for list-based content.
///
/// Loading state patterns:
/// - Show skeleton UI for operations taking longer than 200ms
/// - Use grayscale colors (`SKELETON_BG`, `SKELETON_SHIMMER`) from loading module
/// - Match skeleton structure to real content layout (cover + text + progress)
/// - Respect prefers-reduced-motion (no pulsing animation)
fn skeleton_audiobook_list() -> Element<'static, Message> {
    let items: Element<_> = (0..5)
        .fold(column![].spacing(spacing::XS), |col, _| {
            col.push(skeleton_audiobook_item())
        })
        .into();

    scrollable(items).height(Length::Fill).into()
}

/// Single skeleton audiobook item
///
/// Creates a placeholder with complete structure matching real audiobook items:
/// - 120x120px skeleton cover (matches real cover size)
/// - Title placeholder (60% width, 20px height for realistic match)
/// - Progress bar placeholder (100% width, 4px height)
/// - Status text placeholder (50x12px for percentage display)
/// - Consistent spacing following 4px grid system
///
/// Uses `SKELETON_BG` color (#E5E5E5) from loading module per design system.
/// Skeleton dimensions should match real content exactly.
fn skeleton_audiobook_item() -> Element<'static, Message> {
    // Skeleton cover placeholder (120x120px) - matches real thumbnail size
    // Enhanced shadow (md) to match real cover elevation
    let skeleton_cover = container(Space::new())
        .width(Length::Fixed(120.0))
        .height(Length::Fixed(120.0))
        .style(|_: &iced::Theme| container::Style {
            background: Some(loading::SKELETON_BG.into()),
            border: Border {
                radius: border_radius::MD.into(),
                ..Default::default()
            },
            shadow: shadows::shadow_md(),
            ..Default::default()
        });

    // Skeleton title placeholder (60% width for realistic variation)
    let skeleton_title = container(Space::new())
        .width(Length::FillPortion(60))
        .height(Length::Fixed(20.0))
        .style(|_: &iced::Theme| container::Style {
            background: Some(loading::SKELETON_BG.into()),
            border: Border {
                radius: border_radius::SM.into(),
                ..Default::default()
            },
            ..Default::default()
        });

    // Skeleton progress bar placeholder (4px height to match real progress bars)
    let skeleton_progress = container(Space::new())
        .width(Length::Fill)
        .height(Length::Fixed(4.0))
        .style(|_: &iced::Theme| container::Style {
            background: Some(loading::SKELETON_BG.into()),
            border: Border {
                radius: border_radius::SM.into(),
                ..Default::default()
            },
            ..Default::default()
        });

    // Skeleton status text placeholder (percentage display area)
    let skeleton_status = container(Space::new())
        .width(Length::Fixed(50.0))
        .height(Length::Fixed(12.0))
        .style(|_: &iced::Theme| container::Style {
            background: Some(loading::SKELETON_BG.into()),
            border: Border {
                radius: border_radius::SM.into(),
                ..Default::default()
            },
            ..Default::default()
        });

    // Match the structure of real audiobook items exactly
    let content = row![
        skeleton_cover,
        column![
            skeleton_title,
            Space::new().height(Length::Fixed(spacing::XS)),
            container(skeleton_progress).width(Length::Fill),
            Space::new().height(Length::Fixed(spacing::XS)),
            row![skeleton_status, Space::new().width(Length::Fill),]
        ]
        .spacing(spacing::XS)
        .width(Length::Fill)
    ]
    .spacing(spacing::MD)
    .padding(spacing::MD);

    container(content).width(Length::Fill).into()
}

// Include tests from separate file to keep main module under 1000 lines
#[cfg(test)]
include!("audiobook_list_tests.rs");
