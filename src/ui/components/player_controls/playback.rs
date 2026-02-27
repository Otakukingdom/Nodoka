//! Playback controls: play/pause and stop buttons with ripple effects.

use crate::ui::state::RippleTarget;
use crate::ui::styles::{button_styles, spacing, typography};
use crate::ui::{Message, State};
use iced::widget::{button, container, mouse_area, row, text, Space};
use iced::{Color, Element, Length, Point};

/// Renders play/pause and stop buttons with ripple effects
#[must_use]
pub fn playback_controls(
    state: &State,
    play_pause_label: &'static str,
    play_pause_focused: bool,
    stop_focused: bool,
) -> Element<'static, Message> {
    let play_button = button(text(play_pause_label).size(typography::SIZE_BASE))
        .on_press(Message::PlayPause)
        .padding(spacing::MD)
        .width(Length::Fixed(100.0))
        .style(if play_pause_focused {
            button_styles::primary_focused
        } else {
            button_styles::primary
        });

    let stop_button = button(text("Stop").size(typography::SIZE_BASE))
        .on_press(Message::Stop)
        .padding(spacing::MD)
        .width(Length::Fixed(100.0))
        .style(if stop_focused {
            button_styles::secondary_focused
        } else {
            button_styles::secondary
        });

    let play_button = mouse_area(ripple_stack(
        play_button.into(),
        state.ripple_animations.get(&RippleTarget::PlayPauseButton),
    ))
    .on_move(|point| Message::RipplePointerMoved(RippleTarget::PlayPauseButton, point))
    .on_enter(Message::TooltipHoverStart(
        crate::ui::tooltip::TooltipTarget::PlayPause,
    ))
    .on_exit(Message::TooltipHoverEnd);

    let stop_button = mouse_area(ripple_stack(
        stop_button.into(),
        state.ripple_animations.get(&RippleTarget::StopButton),
    ))
    .on_move(|point| Message::RipplePointerMoved(RippleTarget::StopButton, point))
    .on_enter(Message::TooltipHoverStart(
        crate::ui::tooltip::TooltipTarget::Stop,
    ))
    .on_exit(Message::TooltipHoverEnd);

    container(row![play_button, stop_button].spacing(spacing::SM)).into()
}

/// Adds ripple animation overlay to a button
///
/// Creates a circular white ripple that expands from the click point
/// and fades out over time, providing tactile feedback.
pub fn ripple_stack(
    base: Element<'static, Message>,
    ripple: Option<&crate::ui::styles::interactive::RippleAnimation>,
) -> Element<'static, Message> {
    let Some(ripple) = ripple else {
        return base;
    };

    let radius = ripple.radius();
    if radius <= 0.0 {
        return base;
    }

    let diameter = radius * 2.0;
    let opacity = ripple.opacity();
    let center = ripple.center();
    let origin = Point::new(center.x - radius, center.y - radius);
    let circle = container(Space::new())
        .width(Length::Fixed(diameter))
        .height(Length::Fixed(diameter))
        .style(move |_theme: &iced::Theme| container::Style {
            background: Some(Color::from_rgba(1.0, 1.0, 1.0, opacity).into()),
            border: iced::Border {
                radius: (diameter / 2.0).into(),
                ..Default::default()
            },
            ..Default::default()
        });

    let overlay = container(circle)
        .padding(iced::Padding {
            top: origin.y,
            bottom: 0.0,
            left: origin.x,
            right: 0.0,
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill);

    iced::widget::stack![base, overlay].into()
}
