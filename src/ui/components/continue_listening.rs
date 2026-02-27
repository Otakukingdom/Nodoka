//! Continue Listening card component for quick resume functionality.

use crate::models::Audiobook;
use crate::ui::styles::{
    border_radius, button_styles, colors, shadows, spacing, typography,
};
use crate::ui::Message;
use iced::widget::{button, column, container, image, row, text, Space};
use iced::{Border, Color, Element, Length, Shadow, Vector};
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

    let progress_text = audiobook
        .remaining_duration_ms
        .filter(|ms| *ms > 0)
        .map(format_remaining_ms)
        .or_else(|| {
            audiobook.total_duration_ms.and_then(|total_ms| {
                if total_ms <= 0 {
                    return None;
                }
                let remaining_ms = total_ms * (100 - i64::from(audiobook.completeness)) / 100;
                (remaining_ms > 0).then_some(format_remaining_ms(remaining_ms))
            })
        })
        .unwrap_or_else(|| format!("{}% complete", audiobook.completeness));

    let content = row![
        cover,
        column![
            // Enhanced "Continue Listening" label with refined styling
            container(
                text("Continue Listening")
                    .size(typography::SIZE_SM)
                    .style(|_theme: &iced::Theme| iced::widget::text::Style {
                        color: Some(colors::TEXT_ON_PRIMARY),
                    }),
            )
            .padding(iced::Padding::from([spacing::XS, spacing::SM]))
            .style(|_theme: &iced::Theme| container::Style {
                background: Some(Color::from_rgba(1.0, 1.0, 1.0, 0.3).into()),
                border: Border {
                    color: Color::from_rgba(1.0, 1.0, 1.0, 0.5),
                    width: 1.0,
                    radius: border_radius::SM.into(),
                },
                shadow: shadows::shadow_xs(),
                ..Default::default()
            }),
            text(&audiobook.name)
                .size(typography::SIZE_XL)
                .style(|_theme: &iced::Theme| iced::widget::text::Style {
                    color: Some(colors::TEXT_ON_PRIMARY),
                }),
            text(progress_text)
                .size(typography::SIZE_BASE)
                .style(|_theme: &iced::Theme| iced::widget::text::Style {
                    color: Some(Color::from_rgba(1.0, 1.0, 1.0, 0.9)),
                }),
        ]
        .spacing(spacing::SM),
        Space::new().width(Length::Fill),
        button(
            text("▶")
                .size(typography::SIZE_BASE)
                .style(|_theme: &iced::Theme| iced::widget::text::Style {
                    color: Some(colors::TEXT_ON_PRIMARY),
                })
        )
        .on_press(Message::PlayLastListened)
        .padding(spacing::LG)
        .width(Length::Fixed(56.0))
        .height(Length::Fixed(56.0))
        .style(button_styles::elevated),
    ]
    .spacing(spacing::LG)
    .padding(spacing::LG)
    .align_y(iced::Alignment::Center);

    container(content)
        .style(|_theme: &iced::Theme| container::Style {
            // Enhanced multi-layer gradient background for dramatic depth
            background: Some(continue_gradient_multi_layer().into()),
            border: Border {
                color: colors::ACCENT_GRADIENT_END,
                width: 3.0,
                radius: border_radius::LG.into(),
            },
            // Enhanced multi-layer shadow with increased blur for dramatic elevation
            shadow: Shadow {
                color: Color::from_rgba(0.95, 0.85, 0.90, 0.12),
                offset: Vector::new(0.0, 4.0),
                blur_radius: 16.0,
            },
            text_color: Some(colors::TEXT_ON_PRIMARY),
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
            let handle = image::Handle::from_path(path.clone());
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
                    color: colors::PRIMARY_GRADIENT_START,
                    width: 1.5,
                    radius: border_radius::MD.into(),
                },
                shadow: shadows::shadow_md(),
                ..Default::default()
            })
            .into()
        },
    )
}

fn format_remaining_ms(remaining_ms: i64) -> String {
    let remaining_ms = remaining_ms.max(0);
    let hours = remaining_ms / (1000 * 60 * 60);
    let minutes = (remaining_ms / (1000 * 60)) % 60;
    if hours > 0 {
        format!("{hours} hours {minutes} minutes remaining")
    } else {
        format!("{minutes} minutes remaining")
    }
}

fn build_placeholder_cover(size: f32) -> Element<'static, Message> {
    container(
        text("[AUDIO]")
            .size(size * 0.6)
            .width(Length::Fill)
            .height(Length::Fill)
            .center(),
    )
    .width(Length::Fixed(size))
    .height(Length::Fixed(size))
    .style(|_theme: &iced::Theme| container::Style {
        // Premium gradient placeholder for sophisticated appearance
        background: Some(Color::from_rgb(
            colors::SUNSET_GRADIENT_START.r.mul_add(0.1, colors::PRIMARY_GRADIENT_START.r.mul_add(0.2, colors::BG_SECONDARY.r * 0.7)),
            colors::SUNSET_GRADIENT_START.g.mul_add(0.1, colors::PRIMARY_GRADIENT_START.g.mul_add(0.2, colors::BG_SECONDARY.g * 0.7)),
            colors::SUNSET_GRADIENT_START.b.mul_add(0.1, colors::PRIMARY_GRADIENT_START.b.mul_add(0.2, colors::BG_SECONDARY.b * 0.7)),
        ).into()),
        border: Border {
            color: colors::PRIMARY_GRADIENT_START,
            width: 1.5,
            radius: border_radius::MD.into(),
        },
        shadow: shadows::shadow_md(),
        ..Default::default()
    })
    .into()
}

/// Creates multi-layer gradient color for Continue Listening card
/// Enhanced 4-color blend with increased saturation for dramatic effect
fn continue_gradient_multi_layer() -> Color {
    // Enhanced 4-color gradient with increased saturation for more vibrant appearance
    let start = colors::CONTINUE_GRADIENT_START;
    let mid1 = colors::PRIMARY_GRADIENT_START;
    let mid2 = colors::SUNSET_GRADIENT_START;
    let end = colors::CONTINUE_GRADIENT_END;
    Color {
        r: end.r.mul_add(0.30, mid2.r.mul_add(0.25, mid1.r.mul_add(0.25, start.r * 0.20))),
        g: end.g.mul_add(0.30, mid2.g.mul_add(0.25, mid1.g.mul_add(0.25, start.g * 0.20))),
        b: end.b.mul_add(0.30, mid2.b.mul_add(0.25, mid1.b.mul_add(0.25, start.b * 0.20))),
        a: 1.0,
    }
}
