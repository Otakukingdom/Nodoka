//! Modal and tooltip view helpers.
//!
//! This module keeps modal rendering concerns (backdrops, modal cards, keyboard help)
//! out of the main window view orchestration.

use crate::ui::styles::{border_radius, button_styles, colors, shadows, spacing, typography};
use crate::ui::tooltip::{TooltipAnchor, TooltipPosition};
use crate::ui::{FocusedElement, Message};
use iced::widget::{button, column, container, row, text, Space};
use iced::{Border, Color, Element, Length};

pub(super) fn modal_backdrop(opacity: f32, on_press: Message) -> Element<'static, Message> {
    button(container(text("")).width(Length::Fill).height(Length::Fill))
        .style(
            move |_theme: &iced::Theme, _status| iced::widget::button::Style {
                background: Some(Color::from_rgba(0.0, 0.0, 0.0, opacity).into()),
                ..Default::default()
            },
        )
        .on_press(on_press)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

pub(super) fn modal_backdrop_without_click(opacity: f32) -> Element<'static, Message> {
    container(text(""))
        .style(move |_theme: &iced::Theme| iced::widget::container::Style {
            background: Some(Color::from_rgba(0.0, 0.0, 0.0, opacity).into()),
            ..Default::default()
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

/// Renders the keyboard shortcuts help overlay.
pub(super) fn keyboard_help_view(
    focused_element: FocusedElement,
    scale: f32,
) -> Element<'static, Message> {
    // Platform-specific bookmark shortcut.
    #[cfg(target_os = "macos")]
    let bookmark_shortcut = "Cmd+B";
    #[cfg(not(target_os = "macos"))]
    let bookmark_shortcut = "Ctrl+B";

    let shortcuts = column![
        text("Keyboard Shortcuts")
            .size(typography::SIZE_XXL)
            .style(|_theme: &iced::Theme| iced::widget::text::Style {
                color: Some(colors::TEXT_PRIMARY),
            }),
        container(text("")).height(Length::Fixed(spacing::LG)),
        shortcut_group(
            "Playback",
            vec![
                ("Space", "Play / Pause"),
                ("← →", "Seek backward / forward 5s")
            ],
        ),
        shortcut_group(
            "Navigation",
            vec![
                ("↑ ↓", "Previous / Next file"),
                ("Tab", "Cycle through UI elements")
            ],
        ),
        shortcut_group(
            "Library",
            vec![
                (bookmark_shortcut, "Create bookmark"),
                ("Escape", "Close modal"),
                ("F1", "Show this help"),
            ],
        ),
        container(text("")).height(Length::Fixed(spacing::LG)),
        button(text("Close").size(typography::SIZE_BASE))
            .on_press(Message::HideKeyboardHelp)
            .padding(spacing::MD)
            .style(
                if focused_element == FocusedElement::KeyboardHelpCloseButton {
                    button_styles::secondary_focused
                } else {
                    button_styles::secondary
                },
            )
            .width(Length::Fill),
    ]
    .spacing(spacing::MD)
    .width(Length::Fixed(500.0 * scale));

    container(shortcuts).padding(spacing::XL).into()
}

/// Renders a grouped section of keyboard shortcuts with styled keys.
fn shortcut_group(
    title: &'static str,
    shortcuts: Vec<(&'static str, &'static str)>,
) -> Element<'static, Message> {
    let mut group = column![text(title)
        .size(typography::SIZE_LG)
        .style(|_theme: &iced::Theme| iced::widget::text::Style {
            color: Some(colors::TEXT_PRIMARY),
        })]
    .spacing(spacing::SM);

    for (key, description) in shortcuts {
        group = group.push(
            row![
                container(
                    text(key)
                        .size(typography::SIZE_SM)
                        .style(|_theme: &iced::Theme| iced::widget::text::Style {
                            color: Some(colors::TEXT_PRIMARY),
                        }),
                )
                .padding(iced::Padding::from([spacing::XS, spacing::SM]))
                .style(|_theme: &iced::Theme| iced::widget::container::Style {
                    background: Some(colors::BG_ELEVATED.into()),
                    border: Border {
                        color: colors::BORDER_DEFAULT,
                        width: 1.0,
                        radius: border_radius::SM.into(),
                    },
                    ..Default::default()
                }),
                Space::new().width(Length::Fixed(spacing::MD)),
                text(description)
                    .size(typography::SIZE_SM)
                    .style(|_theme: &iced::Theme| iced::widget::text::Style {
                        color: Some(colors::TEXT_SECONDARY),
                    }),
            ]
            .spacing(spacing::SM)
            .align_y(iced::alignment::Vertical::Center),
        );
    }

    container(group)
        .padding(iced::Padding::from([spacing::SM, 0.0]))
        .into()
}

/// Applies design system styling for modal overlays.
pub(super) fn modal_card<'a, E>(content: E, opacity: f32) -> Element<'a, Message>
where
    E: Into<Element<'a, Message>>,
{
    use iced::Shadow;

    let background = with_alpha(colors::BG_SECONDARY, opacity);
    let border_color = with_alpha(shadows::MD_BORDER, opacity);
    container(content)
        .style(move |_theme: &iced::Theme| iced::widget::container::Style {
            background: Some(background.into()),
            border: Border {
                color: border_color,
                width: 2.0,
                radius: border_radius::LG.into(),
            },
            shadow: Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.2 * opacity),
                offset: iced::Vector::new(0.0, 4.0),
                blur_radius: 16.0,
            },
            ..Default::default()
        })
        .padding(spacing::MD)
        .max_width(600.0) // Maximum width for readability
        .width(Length::Fill) // Fill available space on narrow screens
        .max_height(800.0) // Maximum height
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
}

pub(super) fn tooltip_overlay(tooltip: &crate::ui::tooltip::Tooltip) -> Element<'static, Message> {
    use crate::ui::styles::components::tooltip_styles;
    use iced::widget::text::Wrapping;

    let text_element = text(tooltip.text.clone())
        .size(typography::SIZE_SM)
        .width(Length::Fill)
        .wrapping(Wrapping::Word)
        .style(|_theme: &iced::Theme| iced::widget::text::Style {
            color: Some(tooltip_styles::TEXT_COLOR),
        });

    let tooltip_body = container(text_element)
        .padding(iced::Padding {
            top: tooltip_styles::PADDING_VERTICAL,
            bottom: tooltip_styles::PADDING_VERTICAL,
            left: tooltip_styles::PADDING_HORIZONTAL,
            right: tooltip_styles::PADDING_HORIZONTAL,
        })
        .style(tooltip_styles::container_style)
        .width(Length::Shrink)
        .max_width(tooltip_styles::MAX_WIDTH);

    match tooltip.anchor {
        TooltipAnchor::TopBar => {
            let row = match tooltip.position {
                TooltipPosition::Left => row![tooltip_body, Space::new().width(Length::Fill)],
                _ => row![Space::new().width(Length::Fill), tooltip_body],
            };

            container(row)
                .padding(iced::Padding {
                    top: if tooltip.position == TooltipPosition::Bottom {
                        spacing::LG
                    } else {
                        spacing::SM
                    },
                    right: spacing::XL,
                    bottom: 0.0,
                    left: spacing::XL,
                })
                .width(Length::Fill)
                .height(Length::Fill)
                .into()
        }
        TooltipAnchor::PlayerControls => {
            let row = match tooltip.position {
                TooltipPosition::Left => row![tooltip_body, Space::new().width(Length::Fill)],
                TooltipPosition::Right => row![Space::new().width(Length::Fill), tooltip_body],
                _ => row![
                    Space::new().width(Length::Fill),
                    tooltip_body,
                    Space::new().width(Length::Fill)
                ],
            };

            container(column![Space::new().height(Length::Fill), row])
                .padding(iced::Padding {
                    top: spacing::XL,
                    right: spacing::XL,
                    bottom: if matches!(
                        tooltip.position,
                        TooltipPosition::Top | TooltipPosition::Left | TooltipPosition::Right
                    ) {
                        spacing::XL + spacing::LG
                    } else {
                        spacing::SM
                    },
                    left: spacing::XL,
                })
                .width(Length::Fill)
                .height(Length::Fill)
                .into()
        }
        TooltipAnchor::LibraryLeft => {
            let row = match tooltip.position {
                TooltipPosition::Right => row![Space::new().width(Length::Fill), tooltip_body],
                _ => row![tooltip_body, Space::new().width(Length::Fill)],
            };

            container(row)
                .padding(iced::Padding {
                    top: spacing::XL,
                    right: spacing::XL,
                    bottom: spacing::XL,
                    left: spacing::XL,
                })
                .width(Length::Fill)
                .height(Length::Fill)
                .into()
        }
    }
}

fn with_alpha(color: Color, alpha: f32) -> Color {
    Color {
        r: color.r,
        g: color.g,
        b: color.b,
        a: (color.a * alpha).clamp(0.0, 1.0),
    }
}
