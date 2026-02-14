use crate::ui::styles::{spacing, typography};
use crate::ui::{Message, State};
use iced::widget::{button, column, container, horizontal_space, row, scrollable, text};
use iced::{Element, Length};

/// Builds the settings dialog modal with improved layout and visual hierarchy
///
/// Features:
/// - Clear modal title with proper typography
/// - Section headers for organization
/// - Scrollable directory list with action buttons
/// - Improved spacing and padding
/// - Visual grouping of primary/secondary actions
#[must_use]
pub fn build_settings_dialog(state: &State) -> Element<'static, Message> {
    let directory_list: Element<_> = if state.directories.is_empty() {
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
                        text(&path).size(typography::SIZE_SM),
                        horizontal_space(),
                        button(text("Rescan").size(typography::SIZE_XS))
                            .on_press(Message::DirectoryRescan(rescan_path))
                            .padding(spacing::XS),
                        button(text("Remove").size(typography::SIZE_XS))
                            .on_press(Message::DirectoryRemove(remove_path))
                            .padding(spacing::XS),
                    ]
                    .padding(spacing::SM)
                    .spacing(spacing::SM),
                ))
            })
            .into()
    };

    container(
        column![
            // Modal header
            text("Settings").size(typography::SIZE_XXL),
            // Section header
            container(text("Audiobook Directories").size(typography::SIZE_LG)).padding([
                spacing::MD,
                0.0,
                spacing::SM,
                0.0
            ]),
            // Directory list with scrolling
            container(scrollable(directory_list).height(Length::Fixed(200.0))),
            // Action buttons with proper grouping
            container(
                row![
                    button(text("Add Directory").size(typography::SIZE_SM))
                        .on_press(Message::DirectoryAdd)
                        .padding(spacing::MD),
                    horizontal_space(),
                    button(text("Close").size(typography::SIZE_SM))
                        .on_press(Message::CloseSettings)
                        .padding(spacing::MD),
                ]
                .spacing(spacing::MD)
            )
            .padding([spacing::MD, 0.0, 0.0, 0.0]),
        ]
        .padding(spacing::XL)
        .spacing(spacing::MD),
    )
    .width(Length::Fixed(600.0))
    .height(Length::Fixed(450.0))
    .center_x()
    .center_y()
    .into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Directory;

    #[test]
    fn test_settings_dialog_renders_empty_list() {
        let state = State {
            settings_open: true,
            directories: vec![],
            ..Default::default()
        };

        let element = build_settings_dialog(&state);
        drop(element);
    }

    #[test]
    fn test_settings_dialog_renders_with_single_directory() {
        let state = State {
            settings_open: true,
            directories: vec![Directory::new("/path/to/audiobooks".to_string())],
            ..Default::default()
        };

        let element = build_settings_dialog(&state);
        drop(element);
    }

    #[test]
    fn test_settings_dialog_renders_with_multiple_directories() {
        let state = State {
            settings_open: true,
            directories: vec![
                Directory::new("/path/to/audiobooks1".to_string()),
                Directory::new("/path/to/audiobooks2".to_string()),
                Directory::new("/path/to/audiobooks3".to_string()),
            ],
            ..Default::default()
        };

        let element = build_settings_dialog(&state);
        drop(element);
    }

    #[test]
    fn test_settings_dialog_handles_long_paths() {
        let long_path =
            "/very/long/path/to/audiobooks/".to_string() + &"nested/".repeat(20) + "final";
        let state = State {
            settings_open: true,
            directories: vec![Directory::new(long_path)],
            ..Default::default()
        };

        let element = build_settings_dialog(&state);
        drop(element);
    }

    #[test]
    fn test_settings_dialog_handles_special_characters_in_paths() {
        let state = State {
            settings_open: true,
            directories: vec![
                Directory::new("/path/with spaces/and-dashes".to_string()),
                Directory::new("/path/with (parentheses) [brackets]".to_string()),
                Directory::new("/path/with_underscore/and.dots".to_string()),
            ],
            ..Default::default()
        };

        let element = build_settings_dialog(&state);
        drop(element);
    }

    #[test]
    fn test_settings_dialog_handles_unicode_paths() {
        let state = State {
            settings_open: true,
            directories: vec![
                Directory::new("/path/with/日本語".to_string()),
                Directory::new("/path/with/émojis/📚".to_string()),
            ],
            ..Default::default()
        };

        let element = build_settings_dialog(&state);
        drop(element);
    }

    #[test]
    fn test_settings_dialog_handles_many_directories() {
        let directories: Vec<Directory> = (1..=50)
            .map(|i| Directory::new(format!("/path/to/audiobooks{i}")))
            .collect();

        let state = State {
            settings_open: true,
            directories,
            ..Default::default()
        };

        let element = build_settings_dialog(&state);
        drop(element);
    }

    #[test]
    fn test_settings_dialog_closed_state() {
        let state = State {
            settings_open: false,
            directories: vec![Directory::new("/path/to/audiobooks".to_string())],
            ..Default::default()
        };

        let element = build_settings_dialog(&state);
        drop(element);
    }

    #[test]
    fn test_settings_dialog_with_root_path() {
        let state = State {
            settings_open: true,
            directories: vec![Directory::new("/".to_string())],
            ..Default::default()
        };

        let element = build_settings_dialog(&state);
        drop(element);
    }

    #[test]
    fn test_settings_dialog_with_windows_style_paths() {
        let state = State {
            settings_open: true,
            directories: vec![
                Directory::new("C:\\Users\\Test\\Audiobooks".to_string()),
                Directory::new("D:\\Media\\Books".to_string()),
            ],
            ..Default::default()
        };

        let element = build_settings_dialog(&state);
        drop(element);
    }
}
