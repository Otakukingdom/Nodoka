//! Settings panel view for configuring directories, volume, and speed.
//!
//! Renders the settings form shown inside the settings modal overlay.

use crate::ui::styles::components::border_width;
use crate::ui::styles::{border_radius, button_styles, shadows, spacing, typography};
use crate::ui::{Message, State};
use iced::widget::{button, column, container, row, scrollable, text, Space};
use iced::{Border, Color, Element, Length};

fn build_directory_list(state: &State) -> Element<'static, Message> {
    use crate::ui::styles::colors;

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

                col.push(
                    container(
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
                    )
                    .padding(spacing::SM)
                    .style(move |_theme: &iced::Theme| container::Style {
                        background: Some(colors::BG_RAISED.into()),
                        border: iced::Border {
                            color: colors::BORDER_SUBTLE,
                            width: border_width::XS,
                            radius: border_radius::MD.into(),
                        },
                        // Enhanced shadow for better depth
                        shadow: shadows::shadow_md(),
                        ..Default::default()
                    }),
                )
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

fn build_appearance_section(state: &State) -> Element<'static, Message> {
    use crate::ui::state::ThemeMode;

    column![
        section_header("Appearance"),
        container(
            row![
                text("Theme:").size(typography::SIZE_SM),
                button(text("Light").size(typography::SIZE_SM))
                    .on_press(Message::ThemeChanged(ThemeMode::Light))
                    .padding(spacing::SM)
                    .style(if state.theme == ThemeMode::Light {
                        button_styles::primary
                    } else {
                        button_styles::secondary
                    }),
                button(text("Dark").size(typography::SIZE_SM))
                    .on_press(Message::ThemeChanged(ThemeMode::Dark))
                    .padding(spacing::SM)
                    .style(if state.theme == ThemeMode::Dark {
                        button_styles::primary
                    } else {
                        button_styles::secondary
                    }),
            ]
            .spacing(spacing::MD)
        )
        .padding(iced::Padding {
            top: spacing::SM,
            right: 0.0,
            bottom: spacing::SM,
            left: 0.0,
        }),
    ]
    .into()
}

fn build_playback_section(state: &State) -> Element<'static, Message> {
    use crate::ui::styles::colors;

    column![
        section_header("Playback"),
        container(
            column![
                row![
                    text("Current Speed:").size(typography::SIZE_SM),
                    Space::new().width(Length::Fill),
                    text(format!("{:.2}x", state.speed))
                        .size(typography::SIZE_SM)
                        .style(|_theme: &iced::Theme| iced::widget::text::Style {
                            color: Some(colors::TEXT_SECONDARY),
                        }),
                ]
                .spacing(spacing::MD),
                row![
                    text("Current Volume:").size(typography::SIZE_SM),
                    Space::new().width(Length::Fill),
                    text(format!("{}%", state.volume))
                        .size(typography::SIZE_SM)
                        .style(|_theme: &iced::Theme| iced::widget::text::Style {
                            color: Some(colors::TEXT_SECONDARY),
                        }),
                ]
                .spacing(spacing::MD),
            ]
            .spacing(spacing::SM)
        )
        .padding(iced::Padding {
            top: spacing::SM,
            right: 0.0,
            bottom: spacing::SM,
            left: 0.0,
        }),
    ]
    .into()
}

fn build_keyboard_shortcuts_section(scale: f32) -> Element<'static, Message> {
    let badge_width = (120.0 * scale).max(80.0);

    column![
        section_header("Keyboard Shortcuts"),
        container(
            column![
                shortcut_row("Space".to_string(), "Play/Pause", badge_width),
                shortcut_row(
                    "← →".to_string(),
                    "Seek backward/forward 5 seconds",
                    badge_width
                ),
                shortcut_row("↑ ↓".to_string(), "Previous/Next file", badge_width),
                shortcut_row(
                    crate::ui::shortcuts::format_command_shortcut("F"),
                    "Focus search bar",
                    badge_width,
                ),
                shortcut_row(
                    crate::ui::shortcuts::format_command_shortcut("L"),
                    "Toggle focus mode",
                    badge_width,
                ),
                shortcut_row(
                    crate::ui::shortcuts::format_command_shortcut("T"),
                    "Toggle theme",
                    badge_width,
                ),
                shortcut_row(
                    crate::ui::shortcuts::format_command_shortcut("R"),
                    "Play last listened audiobook",
                    badge_width,
                ),
                shortcut_row(
                    crate::ui::shortcuts::format_command_shortcut("["),
                    "Decrease panel ratio",
                    badge_width,
                ),
                shortcut_row(
                    crate::ui::shortcuts::format_command_shortcut("]"),
                    "Increase panel ratio",
                    badge_width,
                ),
                shortcut_row(
                    crate::ui::shortcuts::format_command_shortcut("B"),
                    "Create bookmark",
                    badge_width,
                ),
                shortcut_row("F1".to_string(), "Show keyboard help", badge_width),
                shortcut_row("Esc".to_string(), "Close modal", badge_width),
                shortcut_row("Tab".to_string(), "Cycle focus forward", badge_width),
                shortcut_row("Shift+Tab".to_string(), "Cycle focus backward", badge_width),
            ]
            .spacing(spacing::SM)
        )
        .padding(iced::Padding {
            top: spacing::SM,
            right: 0.0,
            bottom: spacing::SM,
            left: 0.0,
        }),
    ]
    .into()
}

fn section_header(title: &'static str) -> Element<'static, Message> {
    use crate::ui::styles::{border_radius, colors, shadows};

    // Simplified 2-color gradient for section divider
    let gradient_start = colors::PRIMARY_GRADIENT_START;
    let gradient_end = colors::ACCENT_GRADIENT_END;

    column![
        container(text(title).size(typography::SIZE_LG))
            .padding(iced::Padding {
                top: spacing::LG,
                right: 0.0,
                bottom: spacing::SM,
                left: 0.0,
            })
            .style(|_theme: &iced::Theme| container::Style {
                text_color: Some(colors::TEXT_PRIMARY),
                ..Default::default()
            }),
        container(Space::new())
            .width(Length::Fill)
            .height(Length::Fixed(3.0))
            .style(move |_theme: &iced::Theme| container::Style {
                // Simplified 2-color gradient for section divider
                background: Some(
                    Color::from_rgb(
                        f32::midpoint(gradient_start.r, gradient_end.r),
                        f32::midpoint(gradient_start.g, gradient_end.g),
                        f32::midpoint(gradient_start.b, gradient_end.b),
                    )
                    .into()
                ),
                border: Border {
                    radius: border_radius::FULL.into(),
                    ..Default::default()
                },
                shadow: shadows::shadow_sm(),
                ..Default::default()
            }),
    ]
    .into()
}

fn shortcut_row(
    keys: String,
    description: &'static str,
    badge_width: f32,
) -> Element<'static, Message> {
    use crate::ui::styles::{border_radius, colors};

    // Simplified 2-color gradient for shortcut badge
    let badge_gradient = {
        let primary = colors::PRIMARY_GRADIENT_START;
        let accent = colors::ACCENT_GRADIENT_END;
        Color::from_rgb(
            f32::midpoint(primary.r, accent.r),
            f32::midpoint(primary.g, accent.g),
            f32::midpoint(primary.b, accent.b),
        )
    };

    row![
        container(text(keys).size(typography::SIZE_SM))
            .padding(iced::Padding::from([spacing::XS, spacing::SM]))
            .style(move |_theme: &iced::Theme| container::Style {
                background: Some(badge_gradient.into()),
                border: Border {
                    color: colors::ACCENT_GRADIENT_END,
                    width: border_width::THIN,
                    radius: border_radius::SM.into(),
                },
                shadow: shadows::shadow_sm(),
                text_color: Some(colors::TEXT_ON_PRIMARY),
                ..Default::default()
            })
            .width(Length::Fixed(badge_width)),
        Space::new().width(Length::Fixed(spacing::MD)),
        text(description).size(typography::SIZE_SM),
    ]
    .align_y(iced::Alignment::Center)
    .into()
}

/// Builds the settings dialog modal with improved layout and visual hierarchy
///
/// Features:
/// - Clear modal title with proper typography
/// - Section headers with visual separators (horizontal rules)
/// - Card-styled directory list items
/// - Scrollable directory list with action buttons
/// - Improved spacing and padding for better scannability
/// - Visual grouping of primary/secondary actions
#[must_use]
pub fn build_settings_dialog(state: &State, scale: f32) -> Element<'static, Message> {
    use crate::ui::styles::colors;

    let scale = scale.max(0.01);
    let padding = spacing::XL * scale;
    let directory_list_height = 200.0 * scale;

    let directory_list = build_directory_list(state);
    let action_buttons = build_action_buttons(state);

    container(
        scrollable(column![
            text("Settings").size(typography::SIZE_XXL),
            section_header("Audiobook Directories"),
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
                text_color: Some(colors::TEXT_SECONDARY),
                ..Default::default()
            }),
            container(scrollable(directory_list).height(Length::Fixed(directory_list_height))),
            build_playback_section(state),
            build_appearance_section(state),
            build_keyboard_shortcuts_section(scale),
            action_buttons,
        ]
        .padding(padding)
        .spacing(spacing::SM))
    )
    .width(Length::Fill)
    .height(Length::Shrink)
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
