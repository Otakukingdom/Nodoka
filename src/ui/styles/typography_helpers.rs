//! Typography helper functions for consistent text styling application.
//!
//! This module provides convenience wrapper functions that combine typography
//! presets with theme colors for easy application throughout the UI, ensuring
//! consistent visual hierarchy.

use iced::widget::text;
use crate::ui::styles::colors;
use crate::ui::styles::typography_presets as preset;

/// Creates a styled text element with distinctive heading preset.
///
/// Applies tighter letter-spacing for premium brand appearance.
/// Uses primary text color for high contrast and readability.
#[must_use]
pub fn heading_distinctive<Message>(content: &str) -> iced::Element<'_, Message> {
    let preset_ = preset::DistinctiveHeading::new();
    text(content)
        .size(preset_.size)
        .style(move |_theme: &iced::Theme| iced::widget::text::Style {
            color: Some(colors::TEXT_PRIMARY),
        })
        .into()
}

/// Creates a styled text element with body preset and secondary color.
///
/// Uses standard body text sizing with semantic secondary coloring
/// for metadata and supporting content.
#[must_use]
pub fn body_secondary<Message>(content: &str) -> iced::Element<'_, Message> {
    let preset_ = preset::Body::new();
    text(content)
        .size(preset_.size)
        .style(move |_theme: &iced::Theme| iced::widget::text::Style {
            color: Some(colors::TEXT_SECONDARY),
        })
        .into()
}

/// Creates a styled text element with heading preset and primary color.
///
/// Standard heading styling with proper visual hierarchy.
#[must_use]
pub fn heading_primary<Message>(content: &str) -> iced::Element<'_, Message> {
    let preset_ = preset::Heading::new();
    text(content)
        .size(preset_.size)
        .style(move |_theme: &iced::Theme| iced::widget::text::Style {
            color: Some(colors::TEXT_PRIMARY),
        })
        .into()
}

/// Creates a styled text element with caption preset and secondary color.
///
/// Small, subtle text for metadata, timestamps, and hints.
#[must_use]
pub fn caption_metadata<Message>(content: &str) -> iced::Element<'_, Message> {
    let preset_ = preset::Caption::new();
    text(content)
        .size(preset_.size)
        .style(move |_theme: &iced::Theme| iced::widget::text::Style {
            color: Some(colors::TEXT_SECONDARY),
        })
        .into()
}
