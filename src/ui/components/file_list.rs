use crate::models::AudiobookFile;
use crate::ui::Message;
use iced::widget::{button, column, container, horizontal_space, row, scrollable, text};
use iced::{Element, Length};

#[must_use]
pub fn build_file_list(
    files: &[AudiobookFile],
    selected_path: Option<&String>,
) -> Element<'static, Message> {
    let items: Element<_> = files
        .iter()
        .fold(column![].spacing(0), |col, file| {
            let is_selected = selected_path == Some(&file.full_path);
            let item = build_file_item(file, is_selected);
            col.push(item)
        })
        .into();

    scrollable(items).height(Length::Fill).into()
}

fn build_file_item(file: &AudiobookFile, _selected: bool) -> Element<'static, Message> {
    let name = file.name.clone();
    let duration = format_duration(file.length_of_file);
    let completeness = file.completeness;
    let has_progress = file.seek_position.is_some();
    let is_complete = completeness >= 100;
    let path = file.full_path.clone();

    button(container(
        column![
            text(name).size(13),
            row![
                text(duration).size(11),
                horizontal_space(),
                if is_complete {
                    text("✓").size(11)
                } else if has_progress {
                    text(format!("{completeness}%")).size(11)
                } else {
                    text("").size(11)
                }
            ]
        ]
        .padding(5),
    ))
    .on_press(Message::FileSelected(path))
    .width(Length::Fill)
    .into()
}

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
