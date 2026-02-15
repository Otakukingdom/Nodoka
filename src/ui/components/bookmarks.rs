use crate::models::{AudiobookFile, Bookmark};
use crate::ui::state::BookmarkEditor;
use crate::ui::styles::{button_containers, spacing, typography};
use crate::ui::Message;
use iced::widget::{
    button, column, container, horizontal_space, row, scrollable, text, text_input,
};
use iced::{Element, Length};

/// Renders the bookmark list view with improved visual design
///
/// Features:
/// - Clear header with add button
/// - Scrollable list of bookmarks
/// - Visual indicators for missing files
/// - Improved spacing and typography
#[must_use]
pub fn view(bookmarks: &[Bookmark], files: &[AudiobookFile]) -> Element<'static, Message> {
    let header = row![
        text("Bookmarks").size(typography::SIZE_LG),
        horizontal_space(),
        container(
            button(text("Add Bookmark").size(typography::SIZE_SM))
                .on_press(Message::CreateBookmark)
                .padding(spacing::SM)
        )
        .style(button_containers::primary())
    ]
    .padding(spacing::MD);

    let items: Element<_> = bookmarks
        .iter()
        .fold(column![].spacing(spacing::XS), |col, bm| {
            col.push(bookmark_row(bm, files))
        })
        .into();

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
pub fn editor(editor: &BookmarkEditor) -> Element<'static, Message> {
    let label_input = text_input("Label", &editor.label)
        .on_input(Message::BookmarkEditorLabelChanged)
        .padding(spacing::SM)
        .width(Length::Fill);

    let note_input = text_input("Note (optional)", &editor.note)
        .on_input(Message::BookmarkEditorNoteChanged)
        .padding(spacing::SM)
        .width(Length::Fill);

    let pos = format_position(editor.position_ms);

    container(
        column![
            text("Edit Bookmark").size(typography::SIZE_XL),
            text(format!("Position: {pos}")).size(typography::SIZE_SM),
            container(label_input).padding([spacing::SM, 0.0, 0.0, 0.0]),
            container(note_input).padding([spacing::SM, 0.0, 0.0, 0.0]),
            row![
                container(
                    button(text("Cancel").size(typography::SIZE_SM))
                        .on_press(Message::BookmarkEditorCancel)
                        .padding(spacing::SM)
                )
                .style(button_containers::secondary()),
                horizontal_space(),
                container(
                    button(text("Save").size(typography::SIZE_SM))
                        .on_press(Message::BookmarkEditorSave)
                        .padding(spacing::SM)
                )
                .style(button_containers::primary())
            ]
            .spacing(spacing::MD)
            .padding([spacing::MD, 0.0, 0.0, 0.0])
        ]
        .spacing(spacing::SM)
        .padding(spacing::MD),
    )
    .width(Length::Fill)
    .into()
}

/// Builds a single bookmark row with improved action button layout
fn bookmark_row(bm: &Bookmark, files: &[AudiobookFile]) -> Element<'static, Message> {
    let is_missing = files
        .iter()
        .find(|f| f.full_path == bm.file_path)
        .is_some_and(|f| !f.file_exists);

    let label_text = if is_missing {
        format!("⚠ {} (missing)", bm.label)
    } else if bm.note.as_ref().is_some_and(|n| !n.is_empty()) {
        format!("📝 {}", bm.label) // Icon indicator for bookmarks with notes
    } else {
        bm.label.clone()
    };

    let pos = format_position(bm.position_ms);
    let id = bm.id;

    row![
        button(text(label_text).size(typography::SIZE_SM))
            .on_press_maybe(id.map(Message::BookmarkJump))
            .padding(spacing::SM),
        horizontal_space(),
        text(pos).size(typography::SIZE_XS),
        container(
            button(text("Edit").size(typography::SIZE_XS))
                .on_press_maybe(id.map(Message::BookmarkEdit))
                .padding(spacing::XS)
        )
        .style(button_containers::secondary()),
        container(
            button(text("Delete").size(typography::SIZE_XS))
                .on_press_maybe(id.map(Message::BookmarkDelete))
                .padding(spacing::XS)
        )
        .style(button_containers::danger()),
    ]
    .spacing(spacing::SM)
    .padding(spacing::SM)
    .into()
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

    #[test]
    fn test_view_renders_empty_list() {
        let bookmarks: Vec<Bookmark> = vec![];
        let files: Vec<AudiobookFile> = vec![];

        let element = view(&bookmarks, &files);
        drop(element);
    }

    #[test]
    fn test_view_renders_single_bookmark() {
        let bookmarks = vec![create_test_bookmark(1, "Start", 1000, "", "/path/file.mp3")];
        let files = vec![create_test_file("/path/file.mp3", true)];

        let element = view(&bookmarks, &files);
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

        let element = view(&bookmarks, &files);
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

        let element = editor(&editor_state);
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

        let element = editor(&editor_state);
        drop(element);
    }

    #[test]
    fn test_bookmark_row_normal() {
        let bookmark = create_test_bookmark(1, "Test Bookmark", 60000, "", "/path/file.mp3");
        let files = vec![create_test_file("/path/file.mp3", true)];

        let element = bookmark_row(&bookmark, &files);
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

        let element = bookmark_row(&bookmark, &files);
        drop(element);
    }

    #[test]
    fn test_bookmark_row_missing_file() {
        let bookmark = create_test_bookmark(1, "Test Bookmark", 60000, "", "/path/file.mp3");
        let files = vec![
            create_test_file("/path/file.mp3", false), // File does not exist
        ];

        let element = bookmark_row(&bookmark, &files);
        drop(element);
    }

    #[test]
    fn test_bookmark_row_file_not_in_list() {
        let bookmark = create_test_bookmark(1, "Test Bookmark", 60000, "", "/path/file.mp3");
        let files: Vec<AudiobookFile> = vec![];

        let element = bookmark_row(&bookmark, &files);
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

        let element = bookmark_row(&bookmark, &files);
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

        let element = editor(&editor_state);
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

        let element = editor(&editor_state);
        drop(element);
    }

    #[test]
    fn test_bookmark_row_with_long_label() {
        let long_label = "This is a very long bookmark label that should still render correctly without causing any layout issues in the UI";
        let bookmark = create_test_bookmark(1, long_label, 60000, "", "/path/file.mp3");
        let files = vec![create_test_file("/path/file.mp3", true)];

        let element = bookmark_row(&bookmark, &files);
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

        let element = view(&bookmarks, &files);
        drop(element);
    }
}
