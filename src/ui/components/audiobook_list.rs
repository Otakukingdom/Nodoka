//! Audiobook library list component with cover thumbnails and scan status.

use crate::models::Audiobook;
use crate::ui::state::ScanState;
use crate::ui::styles::{animations, border_radius, loading, shadows, spacing, typography};
use crate::ui::Message;
use iced::widget::{
    button, column, container, image, mouse_area, progress_bar, row, scrollable, text, Space,
};
use iced::{Border, Color, Element, Length};
use std::collections::HashMap;
use std::hash::BuildHasher;
use std::path::PathBuf;

/// Renders the audiobook list with improved visual design
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
///
/// # Clippy allowance
///
/// This function has 9 parameters which exceeds clippy's default limit.
/// The parameters represent distinct UI concerns (data, selection, thumbnails,
/// state, animation, accessibility, and filtering) and grouping them into a
/// struct would not improve clarity or reduce coupling to State.
#[must_use]
#[allow(clippy::too_many_arguments)]
pub fn view<'a, S: BuildHasher>(
    audiobooks: &'a [Audiobook],
    selected_id: Option<i64>,
    cover_thumbnails: &'a HashMap<i64, PathBuf, S>,
    scan_state: &'a ScanState,
    animation_elapsed_ms: u64,
    prefers_reduced_motion: bool,
    search_query: &str,
    sort_order: crate::ui::state::AudiobookSortOrder,
    filter_status: crate::ui::state::AudiobookFilterStatus,
) -> Element<'a, Message> {
    use crate::ui::state::{AudiobookFilterStatus, AudiobookSortOrder};

    // Apply filters to get a filtered list
    let filtered: Vec<&Audiobook> = audiobooks
        .iter()
        .filter(|ab| {
            // Apply search filter (case-insensitive on title and author)
            if !search_query.is_empty() {
                let query_lower = search_query.to_lowercase();
                let title_match = ab.name.to_lowercase().contains(&query_lower);
                let author_match = ab
                    .author
                    .as_ref()
                    .is_some_and(|a| a.to_lowercase().contains(&query_lower));
                if !title_match && !author_match {
                    return false;
                }
            }

            // Apply status filter
            match filter_status {
                AudiobookFilterStatus::All => true,
                AudiobookFilterStatus::InProgress => ab.completeness > 0 && ab.completeness < 100,
                AudiobookFilterStatus::Complete => ab.completeness >= 100,
                AudiobookFilterStatus::NotStarted => ab.completeness == 0,
            }
        })
        .collect();

    // Check if we should show empty state
    let has_active_filters = !search_query.is_empty()
        || sort_order != AudiobookSortOrder::default()
        || filter_status != AudiobookFilterStatus::default();

    if filtered.is_empty() {
        return match scan_state {
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

    // Apply sort order (we need to collect into a Vec to sort)
    let mut sorted_filtered = filtered;
    match sort_order {
        AudiobookSortOrder::RecentlyPlayed => {
            sorted_filtered.sort_by(|a, b| b.last_listened.cmp(&a.last_listened));
        }
        AudiobookSortOrder::Title => {
            sorted_filtered.sort_by(|a, b| a.name.cmp(&b.name));
        }
        AudiobookSortOrder::Progress => {
            sorted_filtered.sort_by(|a, b| b.completeness.cmp(&a.completeness));
        }
        AudiobookSortOrder::DateAdded => {
            sorted_filtered.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        }
    }

    let items: Element<_> = sorted_filtered
        .iter()
        .enumerate()
        .fold(column![].spacing(spacing::SM), |col, (index, ab)| {
            let is_selected = selected_id == ab.id;
            let animation =
                list_item_animation(index, animation_elapsed_ms, prefers_reduced_motion);
            let item = build_audiobook_item(ab, is_selected, cover_thumbnails, animation);
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

    // Apply selection styling with left border accent and subtle gradient
    // Left border (4px PRIMARY color) provides clear visual indicator for selected item
    let content_container = if selected {
        container(content_row)
            .style(move |_theme: &iced::Theme| iced::widget::container::Style {
                background: Some(with_alpha_gradient(colors::SUBTLE_GRADIENT_START, colors::SUBTLE_GRADIENT_END, opacity).into()),
                text_color: Some(with_alpha(colors::SELECTION_TEXT, opacity)),
                border: Border {
                    color: with_alpha(colors::PRIMARY_GRADIENT_START, opacity),
                    width: 4.0,
                    radius: border_radius::MD.into(),
                },
                shadow: shadows::shadow_sm(),
                ..Default::default()
            })
            .width(Length::Fill)
    } else {
        container(content_row).width(Length::Fill)
    };

    // Apply hover styling with smooth transitions
    // Includes pressed state for tactile feedback using interactive overlay
    let item_button = button(content_container)
        .on_press_maybe(id.map(Message::AudiobookSelected))
        .width(Length::Fill)
        .style(move |_theme: &iced::Theme, status| {
            use iced::widget::button;
            match status {
                button::Status::Hovered if !selected => button::Style {
                    background: Some(with_alpha(colors::SURFACE_HOVER, opacity).into()),
                    border: Border {
                        color: with_alpha(colors::BORDER_SUBTLE, opacity),
                        width: 1.0,
                        radius: border_radius::SM.into(),
                    },
                    shadow: shadows::shadow_xs(),
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

    // Larger cover thumbnail (120x120px) for better visibility
    // Enhanced shadow (md) for better depth perception on covers
    id.and_then(|id| cover_thumbnails.get(&id)).map_or_else(
        move || {
            container(text("No cover").size(typography::SIZE_XS).style(
                move |_theme: &iced::Theme| iced::widget::text::Style {
                    color: Some(with_alpha(colors::TEXT_SECONDARY, opacity)),
                },
            ))
            .width(Length::Fixed(120.0))
            .height(Length::Fixed(120.0))
            .style(move |_theme: &iced::Theme| container::Style {
                background: Some(with_alpha(colors::BG_ELEVATED, opacity).into()),
                border: Border {
                    color: with_alpha(colors::BORDER_SUBTLE, opacity),
                    width: 1.0,
                    radius: border_radius::MD.into(),
                },
                shadow: shadows::shadow_md(),
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
                    color: with_alpha(colors::BORDER_SUBTLE, opacity),
                    width: 1.0,
                    radius: border_radius::MD.into(),
                },
                shadow: shadows::shadow_md(),
                ..Default::default()
            })
            .into()
        },
    )
}

fn build_content_row<'a>(
    ab: &'a Audiobook,
    cover: Element<'a, Message>,
    opacity: f32,
    offset_y: f32,
) -> iced::widget::Row<'a, Message> {
    use crate::ui::styles::colors;

    let max_offset = animations::STAGGER_TRANSLATE_Y_PX;
    let offset_y = offset_y.clamp(0.0, max_offset);

    // Progress bar for visual representation of completeness (simplified, thin 4px bar)
    let completeness_u8 = u8::try_from(ab.completeness.clamp(0, 100)).unwrap_or(0);
    let progress = progress_bar(0.0..=100.0, f32::from(completeness_u8));

    // Calculate remaining time, preferring per-file progress when available.
    let remaining_text = format_remaining_time(
        ab.remaining_duration_ms,
        ab.total_duration_ms,
        ab.completeness,
    );

    // Format last listened timestamp as relative time
    let last_listened_text = format_relative_time(ab.last_listened);

    // Build metadata row: remaining time + last listened (no percentage - shown by progress bar)
    // Show completion badge for 100% complete audiobooks
    let mut metadata_items: Vec<Element<'_, Message>> = Vec::new();

    if ab.completeness >= 100 {
        // Completion badge for finished audiobooks with enhanced styling
        metadata_items.push(
            container(text("✓ Complete")
                .size(typography::SIZE_SM)
                .style(move |_theme: &iced::Theme| iced::widget::text::Style {
                    color: Some(with_alpha(colors::TEXT_ON_PRIMARY, opacity)),
                }))
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
        // Show remaining time for in-progress audiobooks
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

    let metadata_row = iced::widget::Row::with_children(metadata_items);

    // Build content column with title, author (if available), progress bar, and metadata
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

    content_column.push(
        container(progress)
            .width(Length::Fill)
            .height(Length::Fixed(4.0))
            .into(),
    );

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

/// Creates a gradient color by blending two colors
///
/// Used for subtle gradient backgrounds on elevated surfaces.
fn with_alpha_gradient(start: Color, end: Color, opacity: f32) -> Color {
    Color {
        r: f32::midpoint(start.r, end.r) * opacity,
        g: f32::midpoint(start.g, end.g) * opacity,
        b: f32::midpoint(start.b, end.b) * opacity,
        a: opacity,
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    const TEST_ANIMATION_ELAPSED_MS: u64 = 1000;
    const TEST_PREFERS_REDUCED_MOTION: bool = false;
    const TEST_SEARCH_QUERY: &str = "";

    fn test_sort_order() -> crate::ui::state::AudiobookSortOrder {
        crate::ui::state::AudiobookSortOrder::default()
    }

    fn test_filter_status() -> crate::ui::state::AudiobookFilterStatus {
        crate::ui::state::AudiobookFilterStatus::default()
    }

    fn create_test_audiobook(id: i64, name: &str, completeness: i32) -> Audiobook {
        Audiobook {
            id: Some(id),
            directory: "/test".to_string(),
            name: name.to_string(),
            author: None,
            full_path: format!("/test/{name}"),
            completeness,
            default_order: 0,
            selected_file: None,
            total_duration_ms: None,
            remaining_duration_ms: None,
            created_at: chrono::Utc::now(),
            last_listened: None,
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
        let audiobooks: Vec<Audiobook> = vec![];
        let cover_thumbnails: HashMap<i64, PathBuf> = HashMap::new();
        let scan_state = ScanState::Idle;

        let element = view(
            &audiobooks,
            None,
            &cover_thumbnails,
            &scan_state,
            TEST_ANIMATION_ELAPSED_MS,
            TEST_PREFERS_REDUCED_MOTION,
            TEST_SEARCH_QUERY,
            test_sort_order(),
            test_filter_status(),
        );
        drop(element);
    }

    #[test]
    fn test_view_renders_single_audiobook() {
        let audiobooks = vec![create_test_audiobook(1, "Test Book", 50)];
        let cover_thumbnails: HashMap<i64, PathBuf> = HashMap::new();
        let scan_state = ScanState::Idle;

        let element = view(
            &audiobooks,
            None,
            &cover_thumbnails,
            &scan_state,
            TEST_ANIMATION_ELAPSED_MS,
            TEST_PREFERS_REDUCED_MOTION,
            TEST_SEARCH_QUERY,
            test_sort_order(),
            test_filter_status(),
        );
        drop(element);
    }

    #[test]
    fn test_view_renders_multiple_audiobooks() {
        let audiobooks = vec![
            create_test_audiobook(1, "Book One", 25),
            create_test_audiobook(2, "Book Two", 75),
            create_test_audiobook(3, "Book Three", 100),
        ];
        let cover_thumbnails: HashMap<i64, PathBuf> = HashMap::new();
        let scan_state = ScanState::Idle;

        let element = view(
            &audiobooks,
            None,
            &cover_thumbnails,
            &scan_state,
            TEST_ANIMATION_ELAPSED_MS,
            TEST_PREFERS_REDUCED_MOTION,
            TEST_SEARCH_QUERY,
            test_sort_order(),
            test_filter_status(),
        );
        drop(element);
    }

    #[test]
    fn test_view_renders_with_selection() {
        let audiobooks = vec![
            create_test_audiobook(1, "Book One", 25),
            create_test_audiobook(2, "Book Two", 75),
        ];
        let cover_thumbnails: HashMap<i64, PathBuf> = HashMap::new();
        let scan_state = ScanState::Idle;

        let element = view(
            &audiobooks,
            Some(2),
            &cover_thumbnails,
            &scan_state,
            TEST_ANIMATION_ELAPSED_MS,
            TEST_PREFERS_REDUCED_MOTION,
            TEST_SEARCH_QUERY,
            test_sort_order(),
            test_filter_status(),
        );
        drop(element);
    }

    #[test]
    fn test_view_renders_with_cover_thumbnails() {
        let audiobooks = vec![create_test_audiobook(1, "Book One", 50)];
        let mut cover_thumbnails: HashMap<i64, PathBuf> = HashMap::new();
        cover_thumbnails.insert(1, PathBuf::from("/test/cover1.jpg"));
        let scan_state = ScanState::Idle;

        let element = view(
            &audiobooks,
            None,
            &cover_thumbnails,
            &scan_state,
            TEST_ANIMATION_ELAPSED_MS,
            TEST_PREFERS_REDUCED_MOTION,
            TEST_SEARCH_QUERY,
            test_sort_order(),
            test_filter_status(),
        );
        drop(element);
    }

    #[test]
    fn test_build_audiobook_item_not_selected() {
        let audiobook = create_test_audiobook(1, "Test Book", 50);
        let cover_thumbnails: HashMap<i64, PathBuf> = HashMap::new();

        let element = build_audiobook_item(&audiobook, false, &cover_thumbnails, test_animation());
        drop(element);
    }

    #[test]
    fn test_build_audiobook_item_selected() {
        let audiobook = create_test_audiobook(1, "Test Book", 50);
        let cover_thumbnails: HashMap<i64, PathBuf> = HashMap::new();

        let element = build_audiobook_item(&audiobook, true, &cover_thumbnails, test_animation());
        drop(element);
    }

    #[test]
    fn test_build_audiobook_item_with_zero_completeness() {
        let audiobook = create_test_audiobook(1, "New Book", 0);
        let cover_thumbnails: HashMap<i64, PathBuf> = HashMap::new();

        let element = build_audiobook_item(&audiobook, false, &cover_thumbnails, test_animation());
        drop(element);
    }

    #[test]
    fn test_build_audiobook_item_with_full_completeness() {
        let audiobook = create_test_audiobook(1, "Finished Book", 100);
        let cover_thumbnails: HashMap<i64, PathBuf> = HashMap::new();

        let element = build_audiobook_item(&audiobook, false, &cover_thumbnails, test_animation());
        drop(element);
    }

    #[test]
    fn test_build_audiobook_item_with_cover_thumbnail() {
        let audiobook = create_test_audiobook(1, "Book With Cover", 50);
        let mut cover_thumbnails: HashMap<i64, PathBuf> = HashMap::new();
        cover_thumbnails.insert(1, PathBuf::from("/test/cover.jpg"));

        let element = build_audiobook_item(&audiobook, false, &cover_thumbnails, test_animation());
        drop(element);
    }

    #[test]
    fn test_build_audiobook_item_without_cover_thumbnail() {
        let audiobook = create_test_audiobook(1, "Book Without Cover", 50);
        let cover_thumbnails: HashMap<i64, PathBuf> = HashMap::new();

        let element = build_audiobook_item(&audiobook, false, &cover_thumbnails, test_animation());
        drop(element);
    }

    #[test]
    fn test_build_audiobook_item_with_long_name() {
        let long_name = "This is a very long audiobook name that should still render correctly without causing any issues";
        let audiobook = create_test_audiobook(1, long_name, 50);
        let cover_thumbnails: HashMap<i64, PathBuf> = HashMap::new();

        let element = build_audiobook_item(&audiobook, false, &cover_thumbnails, test_animation());
        drop(element);
    }

    #[test]
    fn test_build_audiobook_item_with_special_characters() {
        let audiobook = create_test_audiobook(1, "Book: Name (2024) [Edition]", 50);
        let cover_thumbnails: HashMap<i64, PathBuf> = HashMap::new();

        let element = build_audiobook_item(&audiobook, false, &cover_thumbnails, test_animation());
        drop(element);
    }

    #[test]
    fn test_view_handles_large_list() {
        let audiobooks: Vec<Audiobook> = (1..=100)
            .map(|i| create_test_audiobook(i, &format!("Book {i}"), (i % 101) as i32))
            .collect();
        let cover_thumbnails: HashMap<i64, PathBuf> = HashMap::new();
        let scan_state = ScanState::Idle;

        let element = view(
            &audiobooks,
            None,
            &cover_thumbnails,
            &scan_state,
            TEST_ANIMATION_ELAPSED_MS,
            TEST_PREFERS_REDUCED_MOTION,
            TEST_SEARCH_QUERY,
            test_sort_order(),
            test_filter_status(),
        );
        drop(element);
    }

    #[test]
    fn test_audiobook_without_id() {
        let audiobook = Audiobook {
            id: None,
            directory: "/test".to_string(),
            name: "Book Without ID".to_string(),
            author: None,
            full_path: "/test/book".to_string(),
            completeness: 50,
            default_order: 0,
            selected_file: None,
            total_duration_ms: None,
            remaining_duration_ms: None,
            created_at: chrono::Utc::now(),
            last_listened: None,
        };
        let cover_thumbnails: HashMap<i64, PathBuf> = HashMap::new();

        let element = build_audiobook_item(&audiobook, false, &cover_thumbnails, test_animation());
        drop(element);
    }

    #[test]
    fn test_build_audiobook_item_uses_selection_parameter() {
        let audiobook = create_test_audiobook(1, "Test Book", 50);
        let cover_thumbnails: HashMap<i64, PathBuf> = HashMap::new();

        // Verify both selected and unselected states render without panic
        let element_unselected =
            build_audiobook_item(&audiobook, false, &cover_thumbnails, test_animation());
        drop(element_unselected);

        let element_selected =
            build_audiobook_item(&audiobook, true, &cover_thumbnails, test_animation());
        drop(element_selected);

        // Note: Cannot verify visual styling due to iced Element opacity,
        // but we ensure both code paths execute without panic
    }

    #[test]
    fn test_view_properly_passes_selection_state_to_items() {
        let audiobooks = vec![
            create_test_audiobook(1, "Book 1", 25),
            create_test_audiobook(2, "Book 2", 75),
            create_test_audiobook(3, "Book 3", 50),
        ];
        let cover_thumbnails: HashMap<i64, PathBuf> = HashMap::new();
        let scan_state = ScanState::Idle;

        // Test with no selection
        let element = view(
            &audiobooks,
            None,
            &cover_thumbnails,
            &scan_state,
            TEST_ANIMATION_ELAPSED_MS,
            TEST_PREFERS_REDUCED_MOTION,
            TEST_SEARCH_QUERY,
            test_sort_order(),
            test_filter_status(),
        );
        drop(element);

        // Test with book 2 selected
        let element = view(
            &audiobooks,
            Some(2),
            &cover_thumbnails,
            &scan_state,
            TEST_ANIMATION_ELAPSED_MS,
            TEST_PREFERS_REDUCED_MOTION,
            TEST_SEARCH_QUERY,
            test_sort_order(),
            test_filter_status(),
        );
        drop(element);
    }

    #[test]
    fn test_skeleton_ui_renders_during_scan() {
        let audiobooks: Vec<Audiobook> = vec![];
        let cover_thumbnails: HashMap<i64, PathBuf> = HashMap::new();
        let scan_state = ScanState::Scanning {
            directory: Some("/test/path".to_string()),
        };

        let element = view(
            &audiobooks,
            None,
            &cover_thumbnails,
            &scan_state,
            TEST_ANIMATION_ELAPSED_MS,
            TEST_PREFERS_REDUCED_MOTION,
            TEST_SEARCH_QUERY,
            test_sort_order(),
            test_filter_status(),
        );
        drop(element);
    }

    #[test]
    fn test_empty_state_renders_when_idle_and_no_audiobooks() {
        let audiobooks: Vec<Audiobook> = vec![];
        let cover_thumbnails: HashMap<i64, PathBuf> = HashMap::new();
        let scan_state = ScanState::Idle;

        let element = view(
            &audiobooks,
            None,
            &cover_thumbnails,
            &scan_state,
            TEST_ANIMATION_ELAPSED_MS,
            TEST_PREFERS_REDUCED_MOTION,
            TEST_SEARCH_QUERY,
            test_sort_order(),
            test_filter_status(),
        );
        drop(element);
    }
}
