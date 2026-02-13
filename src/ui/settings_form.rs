use crate::ui::{Message, NodokaState};
use iced::widget::{button, column, container, horizontal_space, row, scrollable, text};
use iced::{Element, Length};

pub fn build_settings_dialog(state: &NodokaState) -> Element<'static, Message> {
    let directory_list: Element<_> = state
        .directories
        .iter()
        .fold(column![].spacing(5), |col, dir| {
            let path = dir.full_path.clone();
            let rescan_path = path.clone();
            let remove_path = path.clone();

            col.push(
                row![
                    text(&path),
                    horizontal_space(),
                    button("Rescan").on_press(Message::DirectoryRescan(rescan_path)),
                    button("Remove").on_press(Message::DirectoryRemove(remove_path)),
                ]
                .padding(5)
                .spacing(5),
            )
        })
        .into();

    container(
        column![
            text("Settings").size(20),
            text("Audiobook Directories").size(16),
            scrollable(directory_list).height(Length::Fixed(200.0)),
            button("Add Directory").on_press(Message::DirectoryAdd),
            button("Close").on_press(Message::CloseSettings),
        ]
        .padding(20)
        .spacing(10),
    )
    .width(Length::Fixed(500.0))
    .height(Length::Fixed(400.0))
    .center_x()
    .center_y()
    .into()
}
