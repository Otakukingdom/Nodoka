use crate::ui::styles::{button_styles, spacing, typography};
use crate::ui::{Message, State};
use iced::widget::{button, column, container, row, scrollable, text, Space};
use iced::{Element, Length};

fn build_directory_list(state: &State) -> Element<'static, Message> {
    if state.directories.is_empty() {
        column![text("No audiobook directories configured").size(typography::SIZE_SM)]
            .padding(spacing::MD)
            .into()
    } else {
        state
            .directories
            .iter()
            .fold(column![].spacing(spacing::SM), |col, dir| {
                let path = dir.full_path.clone();
                let rescan_path = path.clone();
                let remove_path = path.clone();

                col.push(container(
                    row![
                        text(path).size(typography::SIZE_SM),
                        Space::new().width(Length::Fill),
                        button(text("Rescan").size(typography::SIZE_XS))
                            .on_press(Message::DirectoryRescan(rescan_path))
                            .padding(spacing::XS)
                            .style(button_styles::secondary),
                        button(text("Remove").size(typography::SIZE_XS))
                            .on_press(Message::DirectoryRemove(remove_path))
                            .padding(spacing::XS)
                            .style(button_styles::danger),
                    ]
                    .padding(spacing::SM)
                    .spacing(spacing::SM),
                ))
            })
            .into()
    }
}

fn build_action_buttons(state: &State) -> Element<'static, Message> {
    container(
        row![
            button(text("Add Directory").size(typography::SIZE_SM))
                .on_press(Message::DirectoryAdd)
                .padding(spacing::MD)
                .style(
                    if state.focused_element
                        == crate::ui::FocusedElement::SettingsAddDirectoryButton
                    {
                        button_styles::primary_focused
                    } else {
                        button_styles::primary
                    }
                ),
            Space::new().width(Length::Fill),
            button(text("Close").size(typography::SIZE_SM))
                .on_press(Message::CloseSettings)
                .padding(spacing::MD)
                .style(
                    if state.focused_element == crate::ui::FocusedElement::SettingsCloseButton {
                        button_styles::secondary_focused
                    } else {
                        button_styles::secondary
                    }
                ),
        ]
        .spacing(spacing::MD),
    )
    .padding(iced::Padding {
        top: spacing::LG,
        right: 0.0,
        bottom: 0.0,
        left: 0.0,
    })
    .into()
}

/// Builds the settings dialog modal with improved layout and visual hierarchy
///
/// Features:
/// - Clear modal title with proper typography
/// - Section headers with visual separators (horizontal rules)
/// - Help text tooltips explaining each setting's purpose
/// - Scrollable directory list with action buttons
/// - Improved spacing and padding for better scannability
/// - Visual grouping of primary/secondary actions
#[must_use]
pub fn build_settings_dialog(state: &State, scale: f32) -> Element<'static, Message> {
    let directory_list = build_directory_list(state);
    let action_buttons = build_action_buttons(state);

    container(
        column![
            text("Settings").size(typography::SIZE_XXL),
            container(text("Audiobook Directories").size(typography::SIZE_LG))
                .padding(iced::Padding {
                    top: spacing::LG,
                    right: 0.0,
                    bottom: spacing::SM,
                    left: 0.0,
                })
                .style(|_theme: &iced::Theme| container::Style {
                    text_color: Some(crate::ui::styles::colors::PRIMARY),
                    ..Default::default()
                }),
            container(Space::new())
                .width(Length::Fill)
                .height(Length::Fixed(2.0))
                .style(|_theme: &iced::Theme| container::Style {
                    background: Some(crate::ui::styles::colors::BORDER_DEFAULT.into()),
                    ..Default::default()
                }),
            container(
                text("Add directories containing audiobooks. Nodoka will scan for supported audio files.")
                    .size(typography::SIZE_SM)
            )
            .padding(iced::Padding {
                top: spacing::SM,
                right: 0.0,
                bottom: spacing::SM,
                left: 0.0,
            })
            .style(|_theme: &iced::Theme| container::Style {
                text_color: Some(crate::ui::styles::colors::TEXT_SECONDARY),
                ..Default::default()
            }),
            container(scrollable(directory_list).height(Length::Fixed(200.0))),
            action_buttons,
        ]
        .padding(spacing::XL)
        .spacing(spacing::SM),
    )
    .width(Length::Fixed(600.0 * scale))
    .height(Length::Fixed(500.0 * scale))
    .center_x(Length::Fill)
    .center_y(Length::Fill)
    .into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Directory;

    #[test]
    fn test_settings_dialog_renders_empty_list() {
        let state = State {
            active_modal: crate::ui::ActiveModal::Settings,
            directories: vec![],
            ..Default::default()
        };

        let element = build_settings_dialog(&state, 1.0);
        drop(element);
    }

    #[test]
    fn test_settings_dialog_renders_with_single_directory() {
        let state = State {
            active_modal: crate::ui::ActiveModal::Settings,
            directories: vec![Directory::new("/path/to/audiobooks".to_string())],
            ..Default::default()
        };

        let element = build_settings_dialog(&state, 1.0);
        drop(element);
    }

    #[test]
    fn test_settings_dialog_renders_with_multiple_directories() {
        let state = State {
            active_modal: crate::ui::ActiveModal::Settings,
            directories: vec![
                Directory::new("/path/to/audiobooks1".to_string()),
                Directory::new("/path/to/audiobooks2".to_string()),
                Directory::new("/path/to/audiobooks3".to_string()),
            ],
            ..Default::default()
        };

        let element = build_settings_dialog(&state, 1.0);
        drop(element);
    }

    #[test]
    fn test_settings_dialog_handles_long_paths() {
        let long_path =
            "/very/long/path/to/audiobooks/".to_string() + &"nested/".repeat(20) + "final";
        let state = State {
            active_modal: crate::ui::ActiveModal::Settings,
            directories: vec![Directory::new(long_path)],
            ..Default::default()
        };

        let element = build_settings_dialog(&state, 1.0);
        drop(element);
    }

    #[test]
    fn test_settings_dialog_handles_special_characters_in_paths() {
        let state = State {
            active_modal: crate::ui::ActiveModal::Settings,
            directories: vec![
                Directory::new("/path/with spaces/and-dashes".to_string()),
                Directory::new("/path/with (parentheses) [brackets]".to_string()),
                Directory::new("/path/with_underscore/and.dots".to_string()),
            ],
            ..Default::default()
        };

        let element = build_settings_dialog(&state, 1.0);
        drop(element);
    }

    #[test]
    fn test_settings_dialog_handles_unusual_paths() {
        let state = State {
            active_modal: crate::ui::ActiveModal::Settings,
            directories: vec![
                Directory::new("/path/with/percent-encoding/%E2%9C%93".to_string()),
                Directory::new("/path/with/symbols/@#$%^&*()".to_string()),
            ],
            ..Default::default()
        };

        let element = build_settings_dialog(&state, 1.0);
        drop(element);
    }

    #[test]
    fn test_settings_dialog_handles_many_directories() {
        let directories: Vec<Directory> = (1..=50)
            .map(|i| Directory::new(format!("/path/to/audiobooks{i}")))
            .collect();

        let state = State {
            active_modal: crate::ui::ActiveModal::Settings,
            directories,
            ..Default::default()
        };

        let element = build_settings_dialog(&state, 1.0);
        drop(element);
    }

    #[test]
    fn test_settings_dialog_closed_state() {
        let state = State {
            active_modal: crate::ui::ActiveModal::None,
            directories: vec![Directory::new("/path/to/audiobooks".to_string())],
            ..Default::default()
        };

        let element = build_settings_dialog(&state, 1.0);
        drop(element);
    }

    #[test]
    fn test_settings_dialog_with_root_path() {
        let state = State {
            active_modal: crate::ui::ActiveModal::Settings,
            directories: vec![Directory::new("/".to_string())],
            ..Default::default()
        };

        let element = build_settings_dialog(&state, 1.0);
        drop(element);
    }

    #[test]
    fn test_settings_dialog_with_windows_style_paths() {
        let state = State {
            active_modal: crate::ui::ActiveModal::Settings,
            directories: vec![
                Directory::new("C:\\Users\\Test\\Audiobooks".to_string()),
                Directory::new("D:\\Media\\Books".to_string()),
            ],
            ..Default::default()
        };

        let element = build_settings_dialog(&state, 1.0);
        drop(element);
    }
}
