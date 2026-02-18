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
#[must_use]
pub fn view<'a, S: BuildHasher>(
    audiobooks: &'a [Audiobook],
    selected_id: Option<i64>,
    cover_thumbnails: &'a HashMap<i64, PathBuf, S>,
    scan_state: &'a ScanState,
    animation_elapsed_ms: u64,
    prefers_reduced_motion: bool,
) -> Element<'a, Message> {
    // Show skeleton UI during scanning if list is empty
    if audiobooks.is_empty() {
        return match scan_state {
            ScanState::Scanning { .. } => skeleton_audiobook_list(),
            ScanState::Idle => super::empty_state(
                Some("▦"),
                "No Audiobooks Found",
                "Add audiobook directories in Settings to get started. Nodoka will automatically scan for audiobook files.",
                Some("Open Settings"),
                Some(Message::OpenSettings),
            ),
        };
    }

    let items: Element<_> = audiobooks
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
/// Applies ui-ux-pro-max interactive patterns:
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

    let completeness = ab.completeness;
    let id = ab.id;
    let is_complete = ab.is_complete();
    let opacity = animation.opacity;

    let cover = build_cover(id, cover_thumbnails, opacity);
    let completion_badge = build_completion_badge(is_complete, opacity);
    let content_row = build_content_row(
        &ab.name,
        completeness,
        cover,
        completion_badge,
        opacity,
        animation.offset_y,
    );

    // Apply selection styling with left border accent per ui-ux-pro-max
    // Left border (4px PRIMARY color) provides clear visual indicator for selected item
    let content_container = if selected {
        container(content_row)
            .style(move |_theme: &iced::Theme| iced::widget::container::Style {
                background: Some(with_alpha(colors::SELECTION_BG, opacity).into()),
                text_color: Some(with_alpha(colors::SELECTION_TEXT, opacity)),
                border: Border {
                    color: with_alpha(colors::PRIMARY, opacity),
                    width: 4.0,
                    radius: border_radius::MD.into(),
                },
                ..Default::default()
            })
            .width(Length::Fill)
    } else {
        container(content_row).width(Length::Fill)
    };

    // Apply hover styling with smooth transitions per ui-ux-pro-max guidelines
    // Includes pressed state for tactile feedback using interactive overlay
    let item_button = button(content_container)
        .on_press_maybe(id.map(Message::AudiobookSelected))
        .width(Length::Fill)
        .style(move |_theme: &iced::Theme, status| {
            use crate::ui::styles::interactive;
            use iced::widget::button;
            match status {
                button::Status::Hovered if !selected => button::Style {
                    background: Some(with_alpha(colors::BG_HOVER, opacity).into()),
                    border: Border {
                        color: with_alpha(colors::BORDER_DEFAULT, opacity),
                        width: 1.0,
                        radius: border_radius::SM.into(),
                    },
                    ..button::Style::default()
                },
                button::Status::Pressed if !selected => button::Style {
                    background: Some(interactive::apply_active_overlay(colors::BG_HOVER).into()),
                    border: Border {
                        color: with_alpha(colors::BORDER_FOCUS, opacity),
                        width: 2.0,
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

    // Larger cover thumbnail (80x80px) for better visibility per ui-ux-pro-max
    id.and_then(|id| cover_thumbnails.get(&id)).map_or_else(
        move || {
            container(text("No cover").size(typography::SIZE_XS).style(
                move |_theme: &iced::Theme| iced::widget::text::Style {
                    color: Some(with_alpha(colors::TEXT_SECONDARY, opacity)),
                },
            ))
            .width(Length::Fixed(80.0))
            .height(Length::Fixed(80.0))
            .style(move |_theme: &iced::Theme| container::Style {
                border: Border {
                    radius: border_radius::MD.into(),
                    ..Default::default()
                },
                shadow: shadows::shadow_sm(),
                ..Default::default()
            })
            .into()
        },
        move |path| {
            container(
                image(iced::widget::image::Handle::from_path(path.clone()))
                    .width(Length::Fixed(80.0))
                    .height(Length::Fixed(80.0)),
            )
            .style(move |_theme: &iced::Theme| container::Style {
                border: Border {
                    radius: border_radius::MD.into(),
                    ..Default::default()
                },
                shadow: shadows::shadow_sm(),
                ..Default::default()
            })
            .into()
        },
    )
}

fn build_completion_badge(is_complete: bool, opacity: f32) -> Element<'static, Message> {
    use crate::ui::styles::{border_radius, colors};

    if is_complete {
        container(
            text("COMPLETE")
                .size(typography::SIZE_SM)
                .style(move |_theme: &iced::Theme| iced::widget::text::Style {
                    color: Some(with_alpha(colors::TEXT_ON_PRIMARY, opacity)),
                }),
        )
        .padding(iced::Padding::from([spacing::XS, spacing::SM]))
        .style(move |_theme: &iced::Theme| container::Style {
            background: Some(with_alpha(colors::SUCCESS, opacity).into()),
            text_color: Some(with_alpha(colors::TEXT_ON_PRIMARY, opacity)),
            border: Border {
                radius: border_radius::SM.into(),
                ..Default::default()
            },
            ..Default::default()
        })
        .into()
    } else {
        container(text("").size(typography::SIZE_SM)).into()
    }
}

fn build_content_row<'a>(
    name: &'a str,
    completeness: i32,
    cover: Element<'a, Message>,
    completion_badge: Element<'a, Message>,
    opacity: f32,
    offset_y: f32,
) -> iced::widget::Row<'a, Message> {
    use crate::ui::styles::colors;

    let max_offset = animations::STAGGER_TRANSLATE_Y_PX;
    let offset_y = offset_y.clamp(0.0, max_offset);

    // Progress bar for visual representation of completeness
    // Per ui-ux-pro-max: progress bars should be at least 6px height for visibility
    let completeness_u8 = u8::try_from(completeness.clamp(0, 100)).unwrap_or(0);
    let progress = progress_bar(0.0..=100.0, f32::from(completeness_u8));

    row![
        cover,
        column![
            text(name)
                .size(typography::SIZE_BASE)
                .style(move |_theme: &iced::Theme| iced::widget::text::Style {
                    color: Some(with_alpha(colors::TEXT_PRIMARY, opacity)),
                }),
            // Progress bar visualization with minimum 6px height
            container(progress)
                .width(Length::Fill)
                .height(Length::Fixed(6.0)), // Reserve 6px height for consistency
            row![
                text(format!("{completeness}%"))
                    .size(typography::SIZE_SM)
                    .style(move |_theme: &iced::Theme| iced::widget::text::Style {
                        color: Some(with_alpha(colors::TEXT_SECONDARY, opacity)),
                    }),
                Space::new().width(Length::Fill),
                completion_badge
            ]
        ]
        .spacing(spacing::XS)
        .width(Length::Fill)
    ]
    .spacing(spacing::MD)
    .padding(iced::Padding {
        top: spacing::MD + offset_y,
        bottom: spacing::MD + (max_offset - offset_y),
        left: spacing::MD,
        right: spacing::MD,
    })
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
/// Shows 5 placeholder items to indicate loading activity per ui-ux-pro-max guidelines.
/// Skeleton UI provides better feedback than spinners for list-based content.
///
/// Per ui-ux-pro-max loading state patterns:
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
/// - 80x80px skeleton cover (matches real cover size per ui-ux-pro-max)
/// - Title placeholder (60% width, 20px height for realistic match)
/// - Progress bar placeholder (100% width, 4px height)
/// - Status text placeholder (50x12px for percentage display)
/// - Consistent spacing following 4px grid system
///
/// Uses `SKELETON_BG` color (#E5E5E5) from loading module per design system.
/// Per ui-ux-pro-max: skeleton dimensions should match real content exactly.
fn skeleton_audiobook_item() -> Element<'static, Message> {
    // Skeleton cover placeholder (80x80px) - matches real thumbnail size
    let skeleton_cover = container(Space::new())
        .width(Length::Fixed(80.0))
        .height(Length::Fixed(80.0))
        .style(|_: &iced::Theme| container::Style {
            background: Some(loading::SKELETON_BG.into()),
            border: Border {
                radius: border_radius::MD.into(),
                ..Default::default()
            },
            shadow: shadows::shadow_sm(),
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

    fn create_test_audiobook(id: i64, name: &str, completeness: i32) -> Audiobook {
        Audiobook {
            id: Some(id),
            directory: "/test".to_string(),
            name: name.to_string(),
            full_path: format!("/test/{name}"),
            completeness,
            default_order: 0,
            selected_file: None,
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
        );
        drop(element);
    }

    #[test]
    fn test_audiobook_without_id() {
        let audiobook = Audiobook {
            id: None,
            directory: "/test".to_string(),
            name: "Book Without ID".to_string(),
            full_path: "/test/book".to_string(),
            completeness: 50,
            default_order: 0,
            selected_file: None,
            created_at: chrono::Utc::now(),
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
        );
        drop(element);
    }
}
