//! Continue Listening card component for quick resume functionality.

use crate::models::Audiobook;
use crate::ui::styles::{border_radius, button_styles, colors, shadows, spacing, typography};
use crate::ui::Message;
use iced::widget::{button, column, container, image, row, text, Space};
use iced::{Border, Element, Length};
use std::collections::HashMap;
use std::hash::BuildHasher;
use std::path::PathBuf;

/// Renders the Continue Listening card with the last played audiobook
///
/// Features:
/// - Large cover art thumbnail (120x120px)
/// - Prominent "Continue Listening" heading
/// - Audiobook title and progress information
/// - Large play button for immediate resume
/// - Elevated card design with shadow for prominence
#[must_use]
pub fn view<'a, S: BuildHasher>(
    last_audiobook: Option<&'a Audiobook>,
    cover_thumbnails: &'a HashMap<i64, PathBuf, S>,
) -> Element<'a, Message> {
    let Some(audiobook) = last_audiobook else {
        return container(text("")).into();
    };

    let id = audiobook.id;
    let cover = build_large_cover(id, cover_thumbnails);

    // Show remaining time if available, otherwise show percentage
    let progress_text = audiobook.total_duration_ms.map_or_else(
        || format!("{}% complete", audiobook.completeness),
        |total_ms| {
            let remaining_ms = total_ms * (100 - i64::from(audiobook.completeness)) / 100;
            let hours = remaining_ms / (1000 * 60 * 60);
            let minutes = (remaining_ms / (1000 * 60)) % 60;
            if hours > 0 {
                format!("{hours} hours {minutes} minutes remaining")
            } else if minutes > 0 {
                format!("{minutes} minutes remaining")
            } else {
                format!("{}% complete", audiobook.completeness)
            }
        },
    );

    let content = row![
        cover,
        column![
            text("Continue Listening")
                .size(typography::SIZE_SM)
                .style(|_theme: &iced::Theme| iced::widget::text::Style {
                    color: Some(colors::TEXT_SECONDARY),
                }),
            text(&audiobook.name)
                .size(typography::SIZE_XL)
                .style(|_theme: &iced::Theme| iced::widget::text::Style {
                    color: Some(colors::TEXT_PRIMARY),
                }),
            text(progress_text)
                .size(typography::SIZE_BASE)
                .style(|_theme: &iced::Theme| iced::widget::text::Style {
                    color: Some(colors::TEXT_SECONDARY),
                }),
        ]
        .spacing(spacing::SM),
        Space::new().width(Length::Fill),
        button(
            text("Play")
                .size(typography::SIZE_LG)
                .style(|_theme: &iced::Theme| iced::widget::text::Style {
                    color: Some(colors::TEXT_ON_PRIMARY),
                })
        )
        .on_press(Message::PlayLastListened)
        .padding(spacing::LG)
        .style(button_styles::primary),
    ]
    .spacing(spacing::LG)
    .padding(spacing::LG)
    .align_y(iced::Alignment::Center);

    container(content)
        .style(|_theme: &iced::Theme| container::Style {
            background: Some(colors::BG_ELEVATED.into()),
            border: Border {
                radius: border_radius::LG.into(),
                ..Default::default()
            },
            shadow: shadows::shadow_lg(),
            ..Default::default()
        })
        .width(Length::Fill)
        .into()
}

fn build_large_cover<S: BuildHasher>(
    id: Option<i64>,
    cover_thumbnails: &HashMap<i64, PathBuf, S>,
) -> Element<'static, Message> {
    const COVER_SIZE: f32 = 120.0;

    let Some(ab_id) = id else {
        return build_placeholder_cover(COVER_SIZE);
    };

    cover_thumbnails.get(&ab_id).map_or_else(
        || build_placeholder_cover(COVER_SIZE),
        |path| {
            let handle = image::Handle::from_path(path);
            container(
                image(handle)
                    .width(Length::Fixed(COVER_SIZE))
                    .height(Length::Fixed(COVER_SIZE))
                    .content_fit(iced::ContentFit::Cover),
            )
            .width(Length::Fixed(COVER_SIZE))
            .height(Length::Fixed(COVER_SIZE))
            .style(|_theme: &iced::Theme| container::Style {
                background: Some(colors::BG_SECONDARY.into()),
                border: Border {
                    color: colors::BORDER_DEFAULT,
                    width: 1.0,
                    radius: border_radius::MD.into(),
                },
                ..Default::default()
            })
            .into()
        },
    )
}

fn build_placeholder_cover(size: f32) -> Element<'static, Message> {
    container(
        text("🎧")
            .size(size * 0.6)
            .width(Length::Fill)
            .height(Length::Fill)
            .center(),
    )
    .width(Length::Fixed(size))
    .height(Length::Fixed(size))
    .style(|_theme: &iced::Theme| container::Style {
        background: Some(colors::BG_SECONDARY.into()),
        border: Border {
            color: colors::BORDER_DEFAULT,
            width: 1.0,
            radius: border_radius::MD.into(),
        },
        ..Default::default()
    })
    .into()
}
