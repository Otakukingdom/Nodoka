//! Notification toast view helpers.
//!
//! These functions render the notification overlay and its individual toasts.

use crate::ui::styles::{border_radius, spacing, typography};
use crate::ui::{Message, State};
use iced::widget::{button, column, container, row, text, Space};
use iced::{Border, Color, Element, Length};

/// Renders notification toasts in the top-right corner.
///
/// Shows up to three concurrent notifications with semantic colors.
pub(super) fn notification_area(state: &State) -> Element<'_, Message> {
    // Take up to 3 most recent notifications.
    let notifications_to_show = state
        .notification_queue
        .iter()
        .take(3)
        .fold(column![].spacing(spacing::SM), |col, notification| {
            col.push(notification_toast(notification))
        });

    // Position in top-right corner with padding.
    let notifications = container(notifications_to_show)
        .width(Length::Fixed(320.0))
        .padding(spacing::MD);

    container(row![Space::new().width(Length::Fill), notifications])
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(iced::Padding {
            top: spacing::XL,
            right: spacing::XL,
            bottom: 0.0,
            left: 0.0,
        })
        .style(|_theme: &iced::Theme| iced::widget::container::Style {
            background: None, // Transparent background for overlay.
            ..Default::default()
        })
        .into()
}

/// Renders a single notification toast with semantic colors.
fn notification_toast(notification: &crate::ui::state::Notification) -> Element<'_, Message> {
    use crate::ui::styles::components::notification_styles;
    use iced::Shadow;

    let bg_color = notification_styles::background_color(&notification.kind);
    let icon_text = notification.icon();

    let content_col = notification_content_column(notification);
    let mut main_content = notification_main_content(icon_text, content_col);

    if let (Some(action_label), Some(action_message)) = (
        &notification.action_label,
        notification.action_message.clone(),
    ) {
        main_content = main_content.push(notification_button(action_label, action_message));
    }

    main_content = main_content.push(notification_button(
        "Dismiss",
        Message::DismissNotification(notification.id),
    ));

    let (progress_portion, remaining_portion) = notification.dismiss_progress_portions();
    if progress_portion < 100 {
        main_content = main_content.push(notification_progress_bar(
            progress_portion,
            remaining_portion,
        ));
    }

    container(main_content)
        .padding(spacing::MD)
        .width(Length::Fill)
        .style(move |_theme: &iced::Theme| iced::widget::container::Style {
            background: Some(bg_color.into()),
            text_color: Some(notification_styles::TEXT_COLOR),
            border: Border {
                radius: border_radius::MD.into(),
                ..Default::default()
            },
            shadow: Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.15),
                offset: iced::Vector::new(0.0, 2.0),
                blur_radius: 8.0,
            },
            ..Default::default()
        })
        .into()
}

/// Builds notification content column with optional title and message.
fn notification_content_column(
    notification: &crate::ui::state::Notification,
) -> iced::widget::Column<'_, Message> {
    let mut content_col = column![].spacing(spacing::XS);

    if let Some(title) = &notification.title {
        content_col = content_col.push(text(title).size(typography::SIZE_BASE).style(
            |_theme: &iced::Theme| iced::widget::text::Style {
                color: Some(crate::ui::styles::components::notification_styles::TEXT_COLOR),
            },
        ));
    }

    content_col = content_col.push(text(&notification.message).size(typography::SIZE_SM).style(
        |_theme: &iced::Theme| iced::widget::text::Style {
            color: Some(crate::ui::styles::components::notification_styles::TEXT_COLOR),
        },
    ));

    content_col
}

/// Builds main notification content with icon and text.
fn notification_main_content<'a>(
    icon_text: &'static str,
    content_col: iced::widget::Column<'a, Message>,
) -> iced::widget::Column<'a, Message> {
    column![row![
        container(text(icon_text).size(typography::SIZE_LG))
            .width(Length::Fixed(32.0))
            .center_y(Length::Fill)
            .center_x(Length::Fill),
        container(content_col).width(Length::Fill),
    ]
    .spacing(spacing::SM)
    .align_y(iced::Alignment::Start),]
    .spacing(spacing::XS)
}

/// Creates a styled notification button with the given `label` and `on_press` message.
///
/// All notification buttons share the same hover highlight, padding, border-radius,
/// and right-aligned container layout.
fn notification_button(label: &str, on_press: Message) -> Element<'_, Message> {
    use crate::ui::styles::components::notification_styles;

    let btn = button(
        text(label)
            .size(typography::SIZE_SM)
            .style(|_theme: &iced::Theme| iced::widget::text::Style {
                color: Some(notification_styles::TEXT_COLOR),
            }),
    )
    .on_press(on_press)
    .padding(iced::Padding {
        top: spacing::XS,
        bottom: spacing::XS,
        left: spacing::SM,
        right: spacing::SM,
    })
    .style(move |_theme: &iced::Theme, status| {
        let bg = if matches!(status, iced::widget::button::Status::Hovered) {
            Color::from_rgba(1.0, 1.0, 1.0, 0.2)
        } else {
            Color::from_rgba(1.0, 1.0, 1.0, 0.1)
        };

        iced::widget::button::Style {
            background: Some(bg.into()),
            text_color: notification_styles::TEXT_COLOR,
            border: Border {
                radius: border_radius::SM.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    });

    container(btn)
        .width(Length::Fill)
        .align_x(iced::alignment::Horizontal::Right)
        .into()
}

/// Creates notification progress bar for auto-dismiss countdown.
fn notification_progress_bar(
    progress_portion: u16,
    remaining_portion: u16,
) -> Element<'static, Message> {
    use crate::ui::styles::components::notification_styles;

    let progress_bar = container(Space::new())
        .width(Length::FillPortion(progress_portion))
        .height(Length::Fixed(3.0))
        .style(move |_theme: &iced::Theme| iced::widget::container::Style {
            background: Some(notification_styles::PROGRESS_COLOR.into()),
            ..Default::default()
        });

    container(row![
        progress_bar,
        Space::new().width(Length::FillPortion(remaining_portion)),
    ])
    .width(Length::Fill)
    .height(Length::Fixed(3.0))
    .style(|_theme: &iced::Theme| iced::widget::container::Style {
        background: Some(notification_styles::PROGRESS_BG.into()),
        ..Default::default()
    })
    .into()
}
