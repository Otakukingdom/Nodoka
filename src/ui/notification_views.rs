//! Notification toast view helpers.
//!
//! These functions render the notification overlay and its individual toasts.

use crate::ui::styles::{border_radius, colors, shadows, spacing, typography};
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

/// Renders a single notification toast with premium styling
///
/// Creates refined notification toasts with:
/// - Enhanced shadow (md) for elevation
/// - Gradient-tinted backgrounds based on notification kind
/// - Smooth progress bar with gradient treatment
fn notification_toast(notification: &crate::ui::state::Notification) -> Element<'_, Message> {
    use crate::ui::styles::components::notification_styles;

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
            &notification.kind,
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
            shadow: shadows::shadow_md(),
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

/// Creates a premium styled notification button with enhanced hover states
///
/// Button styling features:
/// - Smooth opacity transition on hover
/// - Subtle border for better affordance
/// - Consistent padding and border-radius
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
            Color::from_rgba(1.0, 1.0, 1.0, 0.25)
        } else {
            Color::from_rgba(1.0, 1.0, 1.0, 0.1)
        };

        iced::widget::button::Style {
            background: Some(bg.into()),
            text_color: notification_styles::TEXT_COLOR,
            border: Border {
                color: Color::from_rgba(1.0, 1.0, 1.0, 0.3),
                width: 1.0,
                radius: border_radius::SM.into(),
            },
            ..Default::default()
        }
    });

    container(btn)
        .width(Length::Fill)
        .align_x(iced::alignment::Horizontal::Right)
        .into()
}

/// Creates premium notification progress bar with gradient treatment
///
/// Progress bar features:
/// - Gradient fill based on notification kind
/// - Subtle background track
/// - Smooth 3px height for refinement
fn notification_progress_bar(
    progress_portion: u16,
    remaining_portion: u16,
    kind: &crate::ui::state::NotificationType,
) -> Element<'static, Message> {
    use crate::ui::styles::components::notification_styles;

    // Use gradient colors based on notification kind
    let progress_color = match kind {
        crate::ui::state::NotificationType::Info => colors::PRIMARY_GRADIENT_START,
        crate::ui::state::NotificationType::Success => colors::SUCCESS_LIGHT,
        crate::ui::state::NotificationType::Warning => colors::WARNING_LIGHT,
        crate::ui::state::NotificationType::Error => colors::ERROR_HOVER,
    };

    let progress_bar = container(Space::new())
        .width(Length::FillPortion(progress_portion))
        .height(Length::Fixed(3.0))
        .style(move |_theme: &iced::Theme| iced::widget::container::Style {
            background: Some(progress_color.into()),
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
