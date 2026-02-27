//! Modal and tooltip view helpers.
//!
//! This module keeps modal rendering concerns (backdrops, modal cards, keyboard help)
//! out of the main window view orchestration.

use crate::ui::styles::components::border_width;
use crate::ui::styles::{border_radius, button_styles, colors, shadows, spacing, typography};
use crate::ui::tooltip::{TooltipAnchor, TooltipPosition};
use crate::ui::{FocusedElement, Message};
use iced::widget::{button, column, container, row, text, Space};
use iced::{Border, Color, Element, Length, Shadow, Vector};

/// Modal backdrop with glass-morphism effect and click-to-dismiss
///
/// Creates a premium semi-transparent overlay using glass-morphism design.
/// The backdrop dims the background content while maintaining visual context.
pub(super) fn modal_backdrop(opacity: f32, on_press: Message) -> Element<'static, Message> {
    let glass_opacity = (opacity * 0.92).clamp(0.0, 1.0);
    button(container(text("")).width(Length::Fill).height(Length::Fill))
        .style(
            move |_theme: &iced::Theme, _status| iced::widget::button::Style {
                background: Some(improved_glass_backdrop(glass_opacity).into()),
                ..Default::default()
            },
        )
        .on_press(on_press)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

/// Modal backdrop without click interaction for loading states
///
/// Uses the same glass-morphism effect as `modal_backdrop` but without
/// click-to-dismiss functionality for use with loading indicators.
pub(super) fn modal_backdrop_without_click(opacity: f32) -> Element<'static, Message> {
    let glass_opacity = (opacity * 0.88).clamp(0.0, 1.0);
    container(text(""))
        .style(move |_theme: &iced::Theme| iced::widget::container::Style {
            background: Some(improved_glass_backdrop(glass_opacity).into()),
            ..Default::default()
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

/// Simplified glass-morphism backdrop with 2-color tint
/// Creates premium backdrop with cleaner color layering
fn improved_glass_backdrop(opacity: f32) -> Color {
    // Simplified 2-color glass-morphism for cleaner modal feel
    let base = Color::from_rgb(0.12, 0.08, 0.15); // Deep rose base
    let overlay = Color::from_rgb(0.08, 0.12, 0.25); // Cool overlay

    Color {
        r: f32::midpoint(base.r, overlay.r),
        g: f32::midpoint(base.g, overlay.g),
        b: f32::midpoint(base.b, overlay.b),
        a: (0.88 * opacity).clamp(0.0, 0.94),
    }
}

/// Renders the keyboard shortcuts help overlay.
pub(super) fn keyboard_help_view(
    focused_element: FocusedElement,
    scale: f32,
) -> Element<'static, Message> {
    let bookmark_shortcut = crate::ui::shortcuts::format_command_shortcut("B");

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
                ("Space".to_string(), "Play / Pause".to_string()),
                ("← →".to_string(), "Seek backward / forward 5s".to_string()),
                ("↑ ↓".to_string(), "Previous / Next file".to_string()),
            ],
        ),
        shortcut_group(
            "Navigation",
            vec![
                ("Tab".to_string(), "Cycle focus forward".to_string()),
                ("Shift+Tab".to_string(), "Cycle focus backward".to_string()),
                (
                    crate::ui::shortcuts::format_command_shortcut("["),
                    "Decrease panel ratio".to_string(),
                ),
                (
                    crate::ui::shortcuts::format_command_shortcut("]"),
                    "Increase panel ratio".to_string(),
                ),
            ],
        ),
        shortcut_group(
            "Library",
            vec![
                (bookmark_shortcut, "Create bookmark".to_string()),
                (
                    crate::ui::shortcuts::format_command_shortcut("F"),
                    "Focus search".to_string(),
                ),
                (
                    crate::ui::shortcuts::format_command_shortcut("R"),
                    "Quick resume".to_string(),
                ),
                (
                    crate::ui::shortcuts::format_command_shortcut("T"),
                    "Toggle theme".to_string(),
                ),
                (
                    crate::ui::shortcuts::format_command_shortcut("L"),
                    "Toggle focus mode".to_string(),
                ),
                ("Escape".to_string(), "Close modal".to_string()),
                ("F1".to_string(), "Show this help".to_string()),
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

/// Renders a grouped section of keyboard shortcuts with premium styled keys
///
/// Creates refined key badges with:
/// - Premium multi-gradient background with distinctive character
/// - Enhanced border with Aurora Rose gradient for brand identity
/// - Multi-layer shadow for depth
fn shortcut_group(
    title: &'static str,
    shortcuts: Vec<(String, String)>,
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
                            color: Some(colors::TEXT_ON_PRIMARY),
                        }),
                )
                .padding(iced::Padding::from([spacing::XS, spacing::SM]))
                .style(|_theme: &iced::Theme| iced::widget::container::Style {
                    // Simplified 2-color Aurora Rose gradient
                    background: Some({
                        let start = colors::AURORA_ROSE_GRADIENT_START;
                        let end = colors::AURORA_ROSE_GRADIENT_END;
                        Color::from_rgb(
                            f32::midpoint(start.r, end.r),
                            f32::midpoint(start.g, end.g),
                            f32::midpoint(start.b, end.b),
                        )
                        .into()
                    }),
                    border: Border {
                        color: colors::AURORA_ROSE_GRADIENT_END,
                        width: border_width::THIN,
                        radius: border_radius::SM.into(),
                    },
                    shadow: shadows::shadow_sm(),
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

/// Premium modal card with glass-morphism backdrop and elevated shadow
///
/// Creates a refined modal overlay with:
/// - Simplified shadow with rose tint for elevation
/// - Consistent border width for clean appearance
/// - XL border radius for softer appearance
/// - Smooth opacity transitions for animation
pub(super) fn modal_card<'a, E>(content: E, opacity: f32) -> Element<'a, Message>
where
    E: Into<Element<'a, Message>>,
{
    let background = with_alpha(colors::BG_SECONDARY, opacity);
    container(content)
        .style(move |_theme: &iced::Theme| iced::widget::container::Style {
            background: Some(background.into()),
            border: Border {
                color: colors::AURORA_ROSE_GRADIENT_END,
                width: border_width::REGULAR,
                radius: border_radius::XL.into(),
            },
            // Simplified rose-tinted shadow for elevation
            shadow: Shadow {
                color: Color::from_rgba(0.96, 0.81, 1.0, 0.22),
                offset: Vector::new(0.0, 12.0),
                blur_radius: 32.0,
            },
            ..Default::default()
        })
        .padding(spacing::XL)
        .max_width(700.0)
        .width(Length::Fill)
        .max_height(880.0)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
}

pub(super) fn tooltip_overlay(tooltip: &crate::ui::tooltip::Tooltip) -> Element<'static, Message> {
    use crate::ui::styles::component_styles::tooltip_styles;
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
