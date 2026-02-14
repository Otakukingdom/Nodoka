use crate::models::Audiobook;
use crate::ui::Message;
use iced::widget::{button, column, container, horizontal_space, image, row, scrollable, text};
use iced::{Element, Length};
use std::collections::HashMap;
use std::hash::BuildHasher;
use std::path::PathBuf;

#[must_use]
pub fn view<S: BuildHasher>(
    audiobooks: &[Audiobook],
    selected_id: Option<i64>,
    cover_thumbnails: &HashMap<i64, PathBuf, S>,
) -> Element<'static, Message> {
    let items: Element<_> = audiobooks
        .iter()
        .fold(column![].spacing(0), |col, ab| {
            let is_selected = selected_id == ab.id;
            let item = build_audiobook_item(ab, is_selected, cover_thumbnails);
            col.push(item)
        })
        .into();

    scrollable(items).height(Length::Fill).into()
}

fn build_audiobook_item<S: BuildHasher>(
    ab: &Audiobook,
    _selected: bool,
    cover_thumbnails: &HashMap<i64, PathBuf, S>,
) -> Element<'static, Message> {
    let name = ab.name.clone();
    let completeness = ab.completeness;
    let id = ab.id;
    let is_complete = ab.is_complete();

    let cover: Element<_> = id.and_then(|id| cover_thumbnails.get(&id)).map_or_else(
        || {
            container(text("No cover").size(10))
                .width(Length::Fixed(40.0))
                .height(Length::Fixed(40.0))
                .into()
        },
        |path| {
            image(iced::widget::image::Handle::from_path(path.clone()))
                .width(Length::Fixed(40.0))
                .height(Length::Fixed(40.0))
                .into()
        },
    );

    button(container(
        row![
            cover,
            column![
                text(name).size(14),
                row![
                    text(format!("{completeness}%")).size(12),
                    horizontal_space(),
                    if is_complete {
                        text("✓").size(12)
                    } else {
                        text("").size(12)
                    }
                ]
            ]
            .spacing(2)
        ]
        .spacing(10)
        .padding(10),
    ))
    .on_press_maybe(id.map(Message::AudiobookSelected))
    .width(Length::Fill)
    .into()
}
