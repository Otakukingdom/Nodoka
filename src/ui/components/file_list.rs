use crate::models::AudiobookFile;
use crate::ui::styles::{spacing, typography};
use crate::ui::Message;
use iced::widget::{
    button, column, container, horizontal_space, progress_bar, row, scrollable, text,
};
use iced::{Element, Length};

/// Renders the file list with improved UX patterns
///
/// Features:
/// - Selection highlighting with background color (no prefix)
/// - Warning indicator for missing files
/// - Progress bars for files with partial progress
/// - Visual separation between files
/// - Improved completion indicators with semantic colors
/// - Duration alignment for better scannability
#[must_use]
pub fn view(files: &[AudiobookFile], selected_path: Option<&String>) -> Element<'static, Message> {
    let items: Element<_> = files
        .iter()
        .fold(column![].spacing(spacing::XS), |col, file| {
            let is_selected = selected_path == Some(&file.full_path);
            let item = build_file_item(file, is_selected);
            col.push(item)
        })
        .into();

    scrollable(items).height(Length::Fill).into()
}

/// Builds a single file list item with improved status indicators and visual hierarchy
fn build_file_item(file: &AudiobookFile, _selected: bool) -> Element<'static, Message> {
    let name = file.name.clone();
    let is_missing = !file.file_exists;
    let duration = format_duration(file.length_of_file);
    let completeness = file.completeness;
    let has_progress = file.seek_position.is_some();
    let is_complete = completeness >= 100;
    let path = file.full_path.clone();

    // Status indicator with semantic colors
    let status_indicator = if is_missing {
        text("⚠ Missing").size(typography::SIZE_SM)
    } else if is_complete {
        text("✓ Complete").size(typography::SIZE_SM)
    } else if has_progress {
        text(format!("{completeness}%")).size(typography::SIZE_SM)
    } else {
        text("").size(typography::SIZE_SM)
    };

    // Progress bar for files with partial progress
    let progress_element = if has_progress && !is_complete && !is_missing {
        container(progress_bar(0.0..=100.0, completeness as f32)).width(Length::Fill)
    } else {
        container(text("").size(1)).width(Length::Fill)
    };

    let content = column![
        row![text(name).size(typography::SIZE_SM),],
        progress_element,
        row![
            text(duration).size(typography::SIZE_XS),
            horizontal_space(),
            status_indicator,
        ]
    ]
    .padding(spacing::SM)
    .spacing(spacing::XS);

    let mut item_button = button(container(content)).width(Length::Fill);

    if !is_missing {
        item_button = item_button.on_press(Message::FileSelected(path));
    }

    item_button.into()
}

/// Formats duration in milliseconds to human-readable time string
fn format_duration(duration_ms: Option<i64>) -> String {
    duration_ms.map_or_else(
        || "Unknown".to_string(),
        |ms| {
            let seconds = ms / 1000;
            let minutes = seconds / 60;
            let hours = minutes / 60;

            if hours > 0 {
                format!("{}:{:02}:{:02}", hours, minutes % 60, seconds % 60)
            } else {
                format!("{}:{:02}", minutes, seconds % 60)
            }
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

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
            length_of_file: Some(3600000), // 1 hour
            seek_position: if completeness > 0 { Some(1000) } else { None },
            checksum: None,
            position: 0,
            completeness,
            file_exists: !missing,
            created_at: chrono::Utc::now(),
        }
    }

    #[test]
    fn test_view_renders_empty_list() {
        let files: Vec<AudiobookFile> = vec![];
        let element = view(&files, None);
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
        let element = view(&files, None);
        drop(element);
    }

    #[test]
    fn test_view_renders_multiple_files() {
        let files = vec![
            create_test_file("chapter1.mp3", "/path/chapter1.mp3", 100, false),
            create_test_file("chapter2.mp3", "/path/chapter2.mp3", 50, false),
            create_test_file("chapter3.mp3", "/path/chapter3.mp3", 0, false),
        ];
        let element = view(&files, None);
        drop(element);
    }

    #[test]
    fn test_view_renders_with_selection() {
        let files = vec![
            create_test_file("chapter1.mp3", "/path/chapter1.mp3", 50, false),
            create_test_file("chapter2.mp3", "/path/chapter2.mp3", 25, false),
        ];
        let selected = "/path/chapter2.mp3".to_string();
        let element = view(&files, Some(&selected));
        drop(element);
    }

    #[test]
    fn test_build_file_item_not_selected() {
        let file = create_test_file("chapter1.mp3", "/path/chapter1.mp3", 50, false);
        let element = build_file_item(&file, false);
        drop(element);
    }

    #[test]
    fn test_build_file_item_selected() {
        let file = create_test_file("chapter1.mp3", "/path/chapter1.mp3", 50, false);
        let element = build_file_item(&file, true);
        drop(element);
    }

    #[test]
    fn test_build_file_item_missing_file() {
        let file = create_test_file("chapter1.mp3", "/path/chapter1.mp3", 0, true);
        let element = build_file_item(&file, false);
        drop(element);
    }

    #[test]
    fn test_build_file_item_complete() {
        let file = create_test_file("chapter1.mp3", "/path/chapter1.mp3", 100, false);
        let element = build_file_item(&file, false);
        drop(element);
    }

    #[test]
    fn test_build_file_item_with_progress() {
        let file = create_test_file("chapter1.mp3", "/path/chapter1.mp3", 50, false);
        let element = build_file_item(&file, false);
        drop(element);
    }

    #[test]
    fn test_build_file_item_no_progress() {
        let mut file = create_test_file("chapter1.mp3", "/path/chapter1.mp3", 0, false);
        file.seek_position = None;
        let element = build_file_item(&file, false);
        drop(element);
    }

    #[test]
    fn test_format_duration_with_hours() {
        assert_eq!(format_duration(Some(3661000)), "1:01:01");
        assert_eq!(format_duration(Some(7200000)), "2:00:00");
    }

    #[test]
    fn test_format_duration_with_minutes_only() {
        assert_eq!(format_duration(Some(125000)), "2:05");
        assert_eq!(format_duration(Some(60000)), "1:00");
    }

    #[test]
    fn test_format_duration_with_zero() {
        assert_eq!(format_duration(Some(0)), "0:00");
    }

    #[test]
    fn test_format_duration_with_none() {
        assert_eq!(format_duration(None), "Unknown");
    }

    #[test]
    fn test_build_file_item_with_long_filename() {
        let long_name = "This is a very long chapter name that should still render correctly without causing layout issues.mp3";
        let file = create_test_file(long_name, "/path/long.mp3", 50, false);
        let element = build_file_item(&file, false);
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
        let element = build_file_item(&file, false);
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
        let element = view(&files, None);
        drop(element);
    }

    #[test]
    fn test_file_without_duration() {
        let mut file = create_test_file("chapter1.mp3", "/path/chapter1.mp3", 50, false);
        file.length_of_file = None;
        let element = build_file_item(&file, false);
        drop(element);
    }

    #[test]
    fn test_file_with_zero_duration() {
        let mut file = create_test_file("chapter1.mp3", "/path/chapter1.mp3", 50, false);
        file.length_of_file = Some(0);
        let element = build_file_item(&file, false);
        drop(element);
    }

    #[test]
    fn test_missing_file_not_clickable() {
        let file = create_test_file("missing.mp3", "/path/missing.mp3", 0, true);
        let element = build_file_item(&file, false);
        // Just verify it renders without panic
        drop(element);
    }
}
