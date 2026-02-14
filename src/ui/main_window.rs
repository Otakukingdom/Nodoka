use crate::ui::components::{audiobook_list, bookmarks, file_list, player_controls};
use crate::ui::{settings_form, Message, State};
use iced::widget::{button, column, container, horizontal_space, row, text};
use iced::{Element, Length};

#[must_use]
pub fn view(state: &State) -> Element<'static, Message> {
    let audiobook_list_widget = audiobook_list::view(
        &state.audiobooks,
        state.selected_audiobook,
        &state.cover_thumbnails,
    );
    let file_list_widget = file_list::view(&state.current_files, state.selected_file.as_ref());
    let bookmarks_widget = bookmarks::view(&state.bookmarks, &state.current_files);
    let player_widget = player_controls::view(state);

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
            container(column![file_list_widget, bookmarks_widget])
                .width(Length::FillPortion(3))
                .height(Length::Fill),
        ]
        .height(Length::Fill),
        // Player controls (bottom)
        container(player_widget),
    ]);

    let mut content: Element<'static, Message> = main_content.into();

    if state.settings_open {
        content = container(column![
            content,
            settings_form::build_settings_dialog(state)
        ])
        .into();
    }

    if let Some(editor) = state.bookmark_editor.as_ref() {
        content = container(column![content, bookmarks::editor(editor)]).into();
    }

    content
}
