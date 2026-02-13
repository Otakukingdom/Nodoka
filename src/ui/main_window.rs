use crate::ui::components::{audiobook_list, file_list, player_controls};
use crate::ui::{settings_form, Message, State};
use iced::widget::{button, column, container, horizontal_space, row, text};
use iced::{Element, Length};

#[must_use]
pub fn view(state: &State) -> Element<'static, Message> {
    let audiobook_list_widget =
        audiobook_list::build_audiobook_list(&state.audiobooks, state.selected_audiobook);
    let file_list_widget =
        file_list::build_file_list(&state.current_files, state.selected_file.as_ref());
    let player_widget = player_controls::build_player_controls(state);

    let main_content = container(column![
        // Top bar with yellow background (#FEDB53)
        container(
            row![
                text("Nodoka Audiobook Reader").size(24),
                horizontal_space(),
                button("Settings").on_press(Message::OpenSettings),
            ]
            .padding(10)
        ),
        // Main content area
        row![
            // Audiobook list (left panel)
            container(audiobook_list_widget)
                .width(Length::FillPortion(2))
                .height(Length::Fill),
            // File list (right panel)
            container(file_list_widget)
                .width(Length::FillPortion(3))
                .height(Length::Fill),
        ]
        .height(Length::Fill),
        // Player controls (bottom)
        container(player_widget),
    ]);

    // Overlay settings dialog if open
    if state.settings_open {
        container(column![
            main_content,
            settings_form::build_settings_dialog(state)
        ])
        .into()
    } else {
        main_content.into()
    }
}
