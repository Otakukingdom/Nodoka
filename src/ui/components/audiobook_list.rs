use crate::models::Audiobook;
use crate::ui::Message;
use iced::widget::{button, column, container, horizontal_space, row, scrollable, text};
use iced::{Element, Length};

#[must_use]
pub fn build_audiobook_list(
    audiobooks: &[Audiobook],
    selected_id: Option<i64>,
) -> Element<'static, Message> {
    let items: Element<_> = audiobooks
        .iter()
        .fold(column![].spacing(0), |col, ab| {
            let is_selected = selected_id == ab.id;
            let item = build_audiobook_item(ab, is_selected);
            col.push(item)
        })
        .into();

    scrollable(items).height(Length::Fill).into()
}

fn build_audiobook_item(ab: &Audiobook, _selected: bool) -> Element<'static, Message> {
    let name = ab.name.clone();
    let completeness = ab.completeness;
    let id = ab.id;
    let is_complete = ab.is_complete();

    button(container(
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
        .padding(10),
    ))
    .on_press_maybe(id.map(Message::AudiobookSelected))
    .width(Length::Fill)
    .into()
}
