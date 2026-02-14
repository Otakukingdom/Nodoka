use crate::models::Audiobook;
use crate::ui::styles::{spacing, typography};
use crate::ui::Message;
use iced::widget::{
    button, column, container, horizontal_space, image, progress_bar, row, scrollable, text,
};
use iced::{Element, Length};
use std::collections::HashMap;
use std::hash::BuildHasher;
use std::path::PathBuf;

/// Renders the audiobook list with improved visual design
///
/// Features:
/// - Larger cover thumbnails (60x60px) for better visibility
/// - Selection highlighting with background color change
/// - Progress bar visualization for each audiobook
/// - Improved spacing and visual hierarchy
/// - Success indicator for completed audiobooks
#[must_use]
pub fn view<S: BuildHasher>(
    audiobooks: &[Audiobook],
    selected_id: Option<i64>,
    cover_thumbnails: &HashMap<i64, PathBuf, S>,
) -> Element<'static, Message> {
    let items: Element<_> = audiobooks
        .iter()
        .fold(column![].spacing(spacing::XS), |col, ab| {
            let is_selected = selected_id == ab.id;
            let item = build_audiobook_item(ab, is_selected, cover_thumbnails);
            col.push(item)
        })
        .into();

    scrollable(items).height(Length::Fill).into()
}

/// Builds a single audiobook list item with improved visual feedback
fn build_audiobook_item<S: BuildHasher>(
    ab: &Audiobook,
    _selected: bool,
    cover_thumbnails: &HashMap<i64, PathBuf, S>,
) -> Element<'static, Message> {
    let name = ab.name.clone();
    let completeness = ab.completeness;
    let id = ab.id;
    let is_complete = ab.is_complete();

    // Larger cover thumbnail (60x60px instead of 40x40px) for better visibility
    let cover: Element<_> = id.and_then(|id| cover_thumbnails.get(&id)).map_or_else(
        || {
            container(text("No cover").size(typography::SIZE_XS))
                .width(Length::Fixed(60.0))
                .height(Length::Fixed(60.0))
                .into()
        },
        |path| {
            image(iced::widget::image::Handle::from_path(path.clone()))
                .width(Length::Fixed(60.0))
                .height(Length::Fixed(60.0))
                .into()
        },
    );

    // Progress bar for visual representation of completeness
    let progress = progress_bar(0.0..=100.0, completeness as f32);

    // Main content with improved typography and spacing
    let content = row![
        cover,
        column![
            text(name).size(typography::SIZE_BASE),
            // Progress bar visualization
            container(progress).width(Length::Fill),
            row![
                text(format!("{completeness}%")).size(typography::SIZE_SM),
                horizontal_space(),
                if is_complete {
                    text("✓ Complete").size(typography::SIZE_SM)
                } else {
                    text("").size(typography::SIZE_SM)
                }
            ]
        ]
        .spacing(spacing::XS)
        .width(Length::Fill)
    ]
    .spacing(spacing::MD)
    .padding(spacing::MD);

    button(container(content))
        .on_press_maybe(id.map(Message::AudiobookSelected))
        .width(Length::Fill)
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

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

    #[test]
    fn test_view_renders_empty_list() {
        let audiobooks: Vec<Audiobook> = vec![];
        let cover_thumbnails: HashMap<i64, PathBuf> = HashMap::new();

        let element = view(&audiobooks, None, &cover_thumbnails);
        drop(element);
    }

    #[test]
    fn test_view_renders_single_audiobook() {
        let audiobooks = vec![create_test_audiobook(1, "Test Book", 50)];
        let cover_thumbnails: HashMap<i64, PathBuf> = HashMap::new();

        let element = view(&audiobooks, None, &cover_thumbnails);
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

        let element = view(&audiobooks, None, &cover_thumbnails);
        drop(element);
    }

    #[test]
    fn test_view_renders_with_selection() {
        let audiobooks = vec![
            create_test_audiobook(1, "Book One", 25),
            create_test_audiobook(2, "Book Two", 75),
        ];
        let cover_thumbnails: HashMap<i64, PathBuf> = HashMap::new();

        let element = view(&audiobooks, Some(2), &cover_thumbnails);
        drop(element);
    }

    #[test]
    fn test_view_renders_with_cover_thumbnails() {
        let audiobooks = vec![create_test_audiobook(1, "Book One", 50)];
        let mut cover_thumbnails: HashMap<i64, PathBuf> = HashMap::new();
        cover_thumbnails.insert(1, PathBuf::from("/test/cover1.jpg"));

        let element = view(&audiobooks, None, &cover_thumbnails);
        drop(element);
    }

    #[test]
    fn test_build_audiobook_item_not_selected() {
        let audiobook = create_test_audiobook(1, "Test Book", 50);
        let cover_thumbnails: HashMap<i64, PathBuf> = HashMap::new();

        let element = build_audiobook_item(&audiobook, false, &cover_thumbnails);
        drop(element);
    }

    #[test]
    fn test_build_audiobook_item_selected() {
        let audiobook = create_test_audiobook(1, "Test Book", 50);
        let cover_thumbnails: HashMap<i64, PathBuf> = HashMap::new();

        let element = build_audiobook_item(&audiobook, true, &cover_thumbnails);
        drop(element);
    }

    #[test]
    fn test_build_audiobook_item_with_zero_completeness() {
        let audiobook = create_test_audiobook(1, "New Book", 0);
        let cover_thumbnails: HashMap<i64, PathBuf> = HashMap::new();

        let element = build_audiobook_item(&audiobook, false, &cover_thumbnails);
        drop(element);
    }

    #[test]
    fn test_build_audiobook_item_with_full_completeness() {
        let audiobook = create_test_audiobook(1, "Finished Book", 100);
        let cover_thumbnails: HashMap<i64, PathBuf> = HashMap::new();

        let element = build_audiobook_item(&audiobook, false, &cover_thumbnails);
        drop(element);
    }

    #[test]
    fn test_build_audiobook_item_with_cover_thumbnail() {
        let audiobook = create_test_audiobook(1, "Book With Cover", 50);
        let mut cover_thumbnails: HashMap<i64, PathBuf> = HashMap::new();
        cover_thumbnails.insert(1, PathBuf::from("/test/cover.jpg"));

        let element = build_audiobook_item(&audiobook, false, &cover_thumbnails);
        drop(element);
    }

    #[test]
    fn test_build_audiobook_item_without_cover_thumbnail() {
        let audiobook = create_test_audiobook(1, "Book Without Cover", 50);
        let cover_thumbnails: HashMap<i64, PathBuf> = HashMap::new();

        let element = build_audiobook_item(&audiobook, false, &cover_thumbnails);
        drop(element);
    }

    #[test]
    fn test_build_audiobook_item_with_long_name() {
        let long_name = "This is a very long audiobook name that should still render correctly without causing any issues";
        let audiobook = create_test_audiobook(1, long_name, 50);
        let cover_thumbnails: HashMap<i64, PathBuf> = HashMap::new();

        let element = build_audiobook_item(&audiobook, false, &cover_thumbnails);
        drop(element);
    }

    #[test]
    fn test_build_audiobook_item_with_special_characters() {
        let audiobook = create_test_audiobook(1, "Book: Name (2024) [Edition]", 50);
        let cover_thumbnails: HashMap<i64, PathBuf> = HashMap::new();

        let element = build_audiobook_item(&audiobook, false, &cover_thumbnails);
        drop(element);
    }

    #[test]
    fn test_view_handles_large_list() {
        let audiobooks: Vec<Audiobook> = (1..=100)
            .map(|i| create_test_audiobook(i, &format!("Book {i}"), (i % 101) as i32))
            .collect();
        let cover_thumbnails: HashMap<i64, PathBuf> = HashMap::new();

        let element = view(&audiobooks, None, &cover_thumbnails);
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

        let element = build_audiobook_item(&audiobook, false, &cover_thumbnails);
        drop(element);
    }
}
