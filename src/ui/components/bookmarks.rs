use crate::models::{AudiobookFile, Bookmark};
use crate::ui::state::BookmarkEditor;
use crate::ui::Message;
use iced::widget::{
    button, column, container, horizontal_space, row, scrollable, text, text_input,
};
use iced::{Element, Length};

#[must_use]
pub fn view(bookmarks: &[Bookmark], files: &[AudiobookFile]) -> Element<'static, Message> {
    let header = row![
        text("Bookmarks").size(14),
        horizontal_space(),
        button("Add").on_press(Message::CreateBookmark)
    ]
    .padding(10);

    let items: Element<_> = bookmarks
        .iter()
        .fold(column![].spacing(0), |col, bm| {
            col.push(bookmark_row(bm, files))
        })
        .into();

    column![header, scrollable(items).height(Length::FillPortion(1))]
        .height(Length::Fill)
        .into()
}

#[must_use]
pub fn editor(editor: &BookmarkEditor) -> Element<'static, Message> {
    let label_input = text_input("Label", &editor.label)
        .on_input(Message::BookmarkEditorLabelChanged)
        .padding(8)
        .width(Length::Fill);

    let note_input = text_input("Note (optional)", &editor.note)
        .on_input(Message::BookmarkEditorNoteChanged)
        .padding(8)
        .width(Length::Fill);

    let pos = format_position(editor.position_ms);

    container(
        column![
            text("Edit Bookmark").size(16),
            text(pos).size(12),
            label_input,
            note_input,
            row![
                button("Cancel").on_press(Message::BookmarkEditorCancel),
                horizontal_space(),
                button("Save").on_press(Message::BookmarkEditorSave)
            ]
            .spacing(10)
        ]
        .spacing(10)
        .padding(12),
    )
    .width(Length::Fill)
    .into()
}

fn bookmark_row(bm: &Bookmark, files: &[AudiobookFile]) -> Element<'static, Message> {
    let is_missing = files
        .iter()
        .find(|f| f.full_path == bm.file_path)
        .is_some_and(|f| !f.file_exists);

    let label = if is_missing {
        format!("{} (missing)", bm.label)
    } else {
        bm.label.clone()
    };

    let pos = format_position(bm.position_ms);
    let id = bm.id;

    row![
        button(text(label).size(13)).on_press_maybe(id.map(Message::BookmarkJump)),
        horizontal_space(),
        text(pos).size(11),
        button("Edit").on_press_maybe(id.map(Message::BookmarkEdit)),
        button("Delete").on_press_maybe(id.map(Message::BookmarkDelete)),
    ]
    .spacing(8)
    .padding(6)
    .into()
}

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
