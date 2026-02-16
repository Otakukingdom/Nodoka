pub mod audiobook_list;
pub mod bookmarks;
pub mod file_list;
pub mod player_controls;

/// Empty state component for when lists have no items
///
/// Provides helpful guidance following ui-ux-pro-max patterns:
/// - Clear title explaining why the list is empty
/// - Descriptive text providing context
/// - Optional call-to-action button for resolution
/// - Centered layout for visual balance
///
/// # Examples
///
/// ```ignore
/// use nodoka::ui::components::empty_state;
/// use nodoka::ui::Message;
///
/// let state = empty_state(
///     "No Audiobooks Found",
///     "Add audiobook directories in Settings to get started.",
///     Some("Open Settings"),
///     Some(Message::OpenSettings),
/// );
/// ```
pub fn empty_state<'a>(
    title: &'static str,
    description: &'static str,
    action_label: Option<&'static str>,
    action_message: Option<crate::ui::Message>,
) -> iced::Element<'a, crate::ui::Message> {
    use crate::ui::styles::{button_styles, colors, spacing, typography};
    use iced::widget::{button, column, container, text};
    use iced::Length;

    let mut content =
        column![
            text(title)
                .size(typography::SIZE_XL)
                .style(|_theme: &iced::Theme| iced::widget::text::Style {
                    color: Some(colors::TEXT_SECONDARY),
                }),
            container(text(description).size(typography::SIZE_BASE).style(
                |_theme: &iced::Theme| iced::widget::text::Style {
                    color: Some(colors::TEXT_SECONDARY),
                }
            ))
            .padding(iced::Padding::from([spacing::MD, 0.0])),
        ]
        .align_x(iced::Alignment::Center)
        .spacing(spacing::MD);

    if let (Some(label), Some(message)) = (action_label, action_message) {
        content = content.push(
            button(text(label).size(typography::SIZE_BASE))
                .on_press(message)
                .padding(spacing::MD)
                .style(button_styles::primary),
        );
    }

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .padding(spacing::XXL)
        .into()
}
