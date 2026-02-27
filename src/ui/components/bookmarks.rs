//! Bookmark list component for displaying and managing position bookmarks.

use crate::models::{AudiobookFile, Bookmark};
use crate::ui::state::BookmarkEditor;
use crate::ui::styles::colors;
use crate::ui::styles::components::border_width;
use crate::ui::styles::components::text_input_styles;
use crate::ui::styles::{animations, button_styles, shadows, spacing, typography};
use crate::ui::{FocusedElement, Message};
use iced::widget::{button, column, container, row, scrollable, text, text_input, Space};
use iced::{Color, Element, Length};

/// Renders the bookmark list view with improved visual design
///
/// Features:
/// - Clear header with add button
/// - Empty state with helpful guidance when no bookmarks exist
/// - Scrollable list of bookmarks with timestamp badges
/// - Visual indicators for missing files (WARNING color)
/// - Note indicator badges for bookmarks with notes
/// - Keyboard shortcuts (N for new, E for edit)
/// - Improved spacing and typography
#[must_use]
pub fn view<'a>(
    bookmarks: &'a [Bookmark],
    files: &'a [AudiobookFile],
    animation_elapsed_ms: u64,
    prefers_reduced_motion: bool,
) -> Element<'a, Message> {
    let header = row![
        text("Bookmarks")
            .size(typography::SIZE_XL)
            .style(move |_theme: &iced::Theme| iced::widget::text::Style {
                color: Some(colors::TEXT_PRIMARY),
            }),
        Space::new().width(Length::Fill),
        button(text("Add Bookmark").size(typography::SIZE_SM))
            .on_press(Message::CreateBookmark)
            .padding([spacing::XS, spacing::SM])
            .style(button_styles::primary)
    ]
    .padding(iced::Padding {
        top: spacing::MD,
        bottom: spacing::SM,
        left: spacing::MD,
        right: spacing::MD,
    });

    let items: Element<_> = if bookmarks.is_empty() {
        super::empty_state(
            None,
            "No Bookmarks",
            "Create bookmarks to mark important positions in your audiobooks.",
            Some("Add Bookmark"),
            Some(Message::CreateBookmark),
        )
    } else {
        bookmarks
            .iter()
            .enumerate()
            .fold(column![].spacing(spacing::XS), |col, (index, bm)| {
                let animation =
                    list_item_animation(index, animation_elapsed_ms, prefers_reduced_motion);
                col.push(bookmark_row(bm, files, animation))
            })
            .into()
    };

    column![header, scrollable(items).height(Length::FillPortion(1))]
        .height(Length::Fill)
        .into()
}

/// Renders the bookmark editor modal with improved layout and visual feedback
///
/// Features:
/// - Clear modal title
/// - Position display
/// - Form inputs with proper spacing
/// - Action buttons with visual grouping
#[must_use]
pub fn editor(
    editor: &BookmarkEditor,
    focused_element: FocusedElement,
    scale: f32,
) -> Element<'static, Message> {
    let label_input = text_input("Label", &editor.label)
        .on_input(Message::BookmarkEditorLabelChanged)
        .padding(spacing::SM)
        .width(Length::Fill)
        .style(text_input_styles::premium);

    let note_input = text_input("Note (optional)", &editor.note)
        .on_input(Message::BookmarkEditorNoteChanged)
        .padding(spacing::SM)
        .width(Length::Fill)
        .style(text_input_styles::premium);

    let pos = format_position(editor.position_ms);

    container(
        column![
            text("Edit Bookmark").size(typography::SIZE_XL),
            text(format!("Position: {pos}")).size(typography::SIZE_SM),
            container(label_input).padding(iced::Padding::from([0.0, spacing::SM])),
            container(note_input).padding(iced::Padding::from([0.0, spacing::SM])),
            row![
                button(text("Cancel").size(typography::SIZE_SM))
                    .on_press(Message::BookmarkEditorCancel)
                    .padding(spacing::SM)
                    .style(
                        if focused_element == FocusedElement::BookmarkEditorCancelButton {
                            button_styles::secondary_focused
                        } else {
                            button_styles::secondary
                        }
                    ),
                Space::new().width(Length::Fill),
                button(text("Save").size(typography::SIZE_SM))
                    .on_press(Message::BookmarkEditorSave)
                    .padding(spacing::SM)
                    .style(
                        if focused_element == FocusedElement::BookmarkEditorSaveButton {
                            button_styles::primary_focused
                        } else {
                            button_styles::primary
                        }
                    )
            ]
            .spacing(spacing::MD)
            .padding(iced::Padding::from([spacing::MD, 0.0]))
        ]
        .spacing(spacing::SM)
        .padding(spacing::MD),
    )
    .width(Length::Fixed(600.0 * scale))
    .into()
}

/// Builds a single bookmark row with improved action button layout and timestamp badge
///
/// Applies consistent interaction and hierarchy patterns:
/// - Timestamp badge with colored background (ACCENT color) for better visibility
/// - Visual indicators for missing files with WARNING color
/// - Note indicator badge for bookmarks with notes
/// - Improved spacing and visual hierarchy
fn bookmark_row(
    bm: &Bookmark,
    files: &[AudiobookFile],
    animation: ListItemAnimation,
) -> Element<'static, Message> {
    let is_missing = files
        .iter()
        .find(|f| f.full_path == bm.file_path)
        .is_some_and(|f| !f.file_exists);

    let max_offset = animations::STAGGER_TRANSLATE_Y_PX;
    let offset_y = animation.offset_y.clamp(0.0, max_offset);
    let opacity = animation.opacity;

    let label_text = if is_missing {
        format!("[MISSING] {}", bm.label)
    } else {
        bm.label.clone()
    };

    let pos = format_position(bm.position_ms);
    let id = bm.id;

    let timestamp_badge = build_timestamp_badge(pos, opacity);
    let note_badge = build_note_badge(bm, opacity);

    row![
        timestamp_badge,
        note_badge,
        build_jump_button(label_text, id, opacity),
        Space::new().width(Length::Fill),
        build_edit_button(id, opacity),
        build_delete_button(id, opacity),
    ]
    .spacing(spacing::SM)
    .padding(iced::Padding {
        top: spacing::SM + offset_y,
        bottom: spacing::SM + (max_offset - offset_y),
        left: spacing::SM,
        right: spacing::SM,
    })
    .align_y(iced::Alignment::Center)
    .into()
}

/// Builds the timestamp badge for a bookmark row
fn build_timestamp_badge(pos: String, opacity: f32) -> Element<'static, Message> {
    use crate::ui::styles::{border_radius, colors};
    use iced::Border;

    let timestamp_badge_color = Color::from_rgb(
        f32::midpoint(
            colors::PRIMARY_GRADIENT_START.r,
            colors::ACCENT_GRADIENT_END.r,
        ),
        f32::midpoint(
            colors::PRIMARY_GRADIENT_START.g,
            colors::ACCENT_GRADIENT_END.g,
        ),
        f32::midpoint(
            colors::PRIMARY_GRADIENT_START.b,
            colors::ACCENT_GRADIENT_END.b,
        ),
    );

    container(
        text(pos)
            .size(typography::SIZE_SM)
            .style(move |_theme: &iced::Theme| iced::widget::text::Style {
                color: Some(with_alpha(colors::TEXT_ON_PRIMARY, opacity)),
            }),
    )
    .padding(iced::Padding::from([spacing::XS, spacing::SM]))
    .style(move |_theme: &iced::Theme| container::Style {
        background: Some(with_alpha(timestamp_badge_color, opacity).into()),
        text_color: Some(with_alpha(colors::TEXT_ON_PRIMARY, opacity)),
        border: Border {
            color: with_alpha(colors::ACCENT_GRADIENT_END, opacity),
            width: border_width::THIN,
            radius: border_radius::SM.into(),
        },
        shadow: shadows::aurora_shadow_tinted(0.15),
        ..Default::default()
    })
    .into()
}

/// Builds the note badge for a bookmark row
fn build_note_badge(bm: &Bookmark, opacity: f32) -> Element<'static, Message> {
    use crate::ui::styles::{border_radius, colors};
    use iced::Border;

    let note_badge_color = Color::from_rgb(
        f32::midpoint(
            colors::PRIMARY_GRADIENT_START.r,
            colors::CELESTIAL_GRADIENT_END.r,
        ),
        f32::midpoint(
            colors::PRIMARY_GRADIENT_START.g,
            colors::CELESTIAL_GRADIENT_END.g,
        ),
        f32::midpoint(
            colors::PRIMARY_GRADIENT_START.b,
            colors::CELESTIAL_GRADIENT_END.b,
        ),
    );

    if bm.note.as_ref().is_some_and(|n| !n.is_empty()) {
        container(
            text("NOTE")
                .size(typography::SIZE_XS)
                .style(move |_theme: &iced::Theme| iced::widget::text::Style {
                    color: Some(with_alpha(colors::TEXT_ON_PRIMARY, opacity)),
                }),
        )
        .padding(iced::Padding::from([spacing::XS, spacing::SM]))
        .style(move |_theme: &iced::Theme| container::Style {
            background: Some(with_alpha(note_badge_color, opacity).into()),
            text_color: Some(with_alpha(colors::TEXT_ON_PRIMARY, opacity)),
            border: Border {
                color: with_alpha(colors::CELESTIAL_GRADIENT_END, opacity),
                width: border_width::XS,
                radius: border_radius::SM.into(),
            },
            shadow: shadows::celestial_shadow(),
            ..Default::default()
        })
        .into()
    } else {
        container(text("")).into()
    }
}

/// Builds the jump-to-position button for a bookmark row
fn build_jump_button(
    label_text: String,
    id: Option<i64>,
    opacity: f32,
) -> Element<'static, Message> {
    button(
        text(label_text)
            .size(typography::SIZE_SM)
            .style(move |_theme: &iced::Theme| iced::widget::text::Style {
                color: Some(with_alpha(colors::TEXT_PRIMARY, opacity)),
            }),
    )
    .on_press_maybe(id.map(Message::BookmarkJump))
    .padding(spacing::SM)
    .into()
}

/// Builds the edit button for a bookmark row
fn build_edit_button(id: Option<i64>, opacity: f32) -> Element<'static, Message> {
    button(
        text("Edit (E)")
            .size(typography::SIZE_XS)
            .style(move |_theme: &iced::Theme| iced::widget::text::Style {
                color: Some(with_alpha(colors::TEXT_PRIMARY, opacity)),
            }),
    )
    .on_press_maybe(id.map(Message::BookmarkEdit))
    .padding(spacing::SM)
    .style(button_styles::secondary)
    .into()
}

/// Builds the delete button for a bookmark row
fn build_delete_button(id: Option<i64>, opacity: f32) -> Element<'static, Message> {
    button(
        text("Delete")
            .size(typography::SIZE_XS)
            .style(move |_theme: &iced::Theme| iced::widget::text::Style {
                color: Some(with_alpha(colors::TEXT_PRIMARY, opacity)),
            }),
    )
    .on_press_maybe(id.map(Message::BookmarkDelete))
    .padding(spacing::SM)
    .style(button_styles::danger)
    .into()
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

/// Formats position in milliseconds to human-readable time string
fn format_position(position_ms: i64) -> String {
    let ms = position_ms.max(0);
    let seconds = ms / 1000;
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

    const TEST_ANIMATION_ELAPSED_MS: u64 = 1000;
    const TEST_PREFERS_REDUCED_MOTION: bool = false;

    fn create_test_bookmark(
        id: i64,
        label: &str,
        position_ms: i64,
        note: &str,
        file_path: &str,
    ) -> Bookmark {
        Bookmark {
            id: Some(id),
            audiobook_id: 1,
            file_path: file_path.to_string(),
            label: label.to_string(),
            position_ms,
            note: if note.is_empty() {
                None
            } else {
                Some(note.to_string())
            },
            created_at: chrono::Utc::now(),
        }
    }

    fn create_test_file(full_path: &str, exists: bool) -> AudiobookFile {
        AudiobookFile {
            audiobook_id: 1,
            name: "chapter1.mp3".to_string(),
            full_path: full_path.to_string(),
            length_of_file: Some(3_600_000),
            seek_position: None,
            checksum: None,
            position: 0,
            completeness: 0,
            file_exists: exists,
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
        let bookmarks: Vec<Bookmark> = vec![];
        let files: Vec<AudiobookFile> = vec![];

        let element = view(
            &bookmarks,
            &files,
            TEST_ANIMATION_ELAPSED_MS,
            TEST_PREFERS_REDUCED_MOTION,
        );
        drop(element);
    }

    #[test]
    fn test_view_renders_single_bookmark() {
        let bookmarks = vec![create_test_bookmark(1, "Start", 1000, "", "/path/file.mp3")];
        let files = vec![create_test_file("/path/file.mp3", true)];

        let element = view(
            &bookmarks,
            &files,
            TEST_ANIMATION_ELAPSED_MS,
            TEST_PREFERS_REDUCED_MOTION,
        );
        drop(element);
    }

    #[test]
    fn test_view_renders_multiple_bookmarks() {
        let bookmarks = vec![
            create_test_bookmark(1, "Chapter 1", 1000, "", "/path/file1.mp3"),
            create_test_bookmark(2, "Chapter 2", 60000, "Important note", "/path/file2.mp3"),
            create_test_bookmark(3, "Chapter 3", 3_600_000, "", "/path/file3.mp3"),
        ];
        let files = vec![
            create_test_file("/path/file1.mp3", true),
            create_test_file("/path/file2.mp3", true),
            create_test_file("/path/file3.mp3", true),
        ];

        let element = view(
            &bookmarks,
            &files,
            TEST_ANIMATION_ELAPSED_MS,
            TEST_PREFERS_REDUCED_MOTION,
        );
        drop(element);
    }

    #[test]
    fn test_editor_renders_empty_fields() {
        let editor_state = BookmarkEditor {
            id: None,
            audiobook_id: 1,
            file_path: "/path/file.mp3".to_string(),
            position_ms: 1000,
            label: String::new(),
            note: String::new(),
        };

        let element = editor(&editor_state, FocusedElement::BookmarkEditorSaveButton, 1.0);
        drop(element);
    }

    #[test]
    fn test_editor_renders_with_data() {
        let editor_state = BookmarkEditor {
            id: Some(1),
            audiobook_id: 1,
            file_path: "/path/file.mp3".to_string(),
            position_ms: 120_000,
            label: "Important Section".to_string(),
            note: "This is a key moment".to_string(),
        };

        let element = editor(&editor_state, FocusedElement::BookmarkEditorSaveButton, 1.0);
        drop(element);
    }

    #[test]
    fn test_bookmark_row_normal() {
        let bookmark = create_test_bookmark(1, "Test Bookmark", 60000, "", "/path/file.mp3");
        let files = vec![create_test_file("/path/file.mp3", true)];

        let element = bookmark_row(&bookmark, &files, test_animation());
        drop(element);
    }

    #[test]
    fn test_bookmark_row_with_note() {
        let bookmark = create_test_bookmark(
            1,
            "Test Bookmark",
            60000,
            "This is a note",
            "/path/file.mp3",
        );
        let files = vec![create_test_file("/path/file.mp3", true)];

        let element = bookmark_row(&bookmark, &files, test_animation());
        drop(element);
    }

    #[test]
    fn test_bookmark_row_missing_file() {
        let bookmark = create_test_bookmark(1, "Test Bookmark", 60000, "", "/path/file.mp3");
        let files = vec![
            create_test_file("/path/file.mp3", false), // File does not exist
        ];

        let element = bookmark_row(&bookmark, &files, test_animation());
        drop(element);
    }

    #[test]
    fn test_bookmark_row_file_not_in_list() {
        let bookmark = create_test_bookmark(1, "Test Bookmark", 60000, "", "/path/file.mp3");
        let files: Vec<AudiobookFile> = vec![];

        let element = bookmark_row(&bookmark, &files, test_animation());
        drop(element);
    }

    #[test]
    fn test_format_position_with_hours() {
        assert_eq!(format_position(3_661_000), "1:01:01");
        assert_eq!(format_position(7_200_000), "2:00:00");
    }

    #[test]
    fn test_format_position_with_minutes_only() {
        assert_eq!(format_position(125_000), "2:05");
        assert_eq!(format_position(60000), "1:00");
    }

    #[test]
    fn test_format_position_with_zero() {
        assert_eq!(format_position(0), "0:00");
    }

    #[test]
    fn test_format_position_handles_negative() {
        // Negative values should be clamped to 0
        assert_eq!(format_position(-1000), "0:00");
    }

    #[test]
    fn test_bookmark_without_id() {
        let bookmark = Bookmark {
            id: None,
            audiobook_id: 1,
            file_path: "/path/file.mp3".to_string(),
            position_ms: 1000,
            label: "Test".to_string(),
            note: Some(String::new()),
            created_at: chrono::Utc::now(),
        };
        let files = vec![create_test_file("/path/file.mp3", true)];

        let element = bookmark_row(&bookmark, &files, test_animation());
        drop(element);
    }

    #[test]
    fn test_editor_with_zero_position() {
        let editor_state = BookmarkEditor {
            id: None,
            audiobook_id: 1,
            file_path: "/path/file.mp3".to_string(),
            position_ms: 0,
            label: "Start".to_string(),
            note: String::new(),
        };

        let element = editor(&editor_state, FocusedElement::BookmarkEditorSaveButton, 1.0);
        drop(element);
    }

    #[test]
    fn test_editor_with_large_position() {
        let editor_state = BookmarkEditor {
            id: None,
            audiobook_id: 1,
            file_path: "/path/file.mp3".to_string(),
            position_ms: 36_000_000, // 10 hours
            label: "End".to_string(),
            note: String::new(),
        };

        let element = editor(&editor_state, FocusedElement::BookmarkEditorSaveButton, 1.0);
        drop(element);
    }

    #[test]
    fn test_bookmark_row_with_long_label() {
        let long_label = "This is a very long bookmark label that should still render correctly without causing any layout issues in the UI";
        let bookmark = create_test_bookmark(1, long_label, 60000, "", "/path/file.mp3");
        let files = vec![create_test_file("/path/file.mp3", true)];

        let element = bookmark_row(&bookmark, &files, test_animation());
        drop(element);
    }

    #[test]
    fn test_view_handles_large_bookmark_list() {
        let bookmarks: Vec<Bookmark> = (1..=50)
            .map(|i| {
                create_test_bookmark(i, &format!("Bookmark {i}"), i * 60000, "", "/path/file.mp3")
            })
            .collect();
        let files = vec![create_test_file("/path/file.mp3", true)];

        let element = view(
            &bookmarks,
            &files,
            TEST_ANIMATION_ELAPSED_MS,
            TEST_PREFERS_REDUCED_MOTION,
        );
        drop(element);
    }
}
