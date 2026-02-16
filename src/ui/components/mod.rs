//! UI Components Module
//!
//! This module contains all reusable UI components for the Nodoka audiobook player,
//! implementing consistent design patterns from the ui-ux-pro-max design system.
//!
//! # Component Overview
//!
//! - `audiobook_list`: Displays library of audiobooks with progress indicators
//! - `file_list`: Shows chapters/files within selected audiobook
//! - `bookmarks`: Manages and displays bookmarks with position information
//! - `player_controls`: Playback controls with speed, volume, and sleep timer
//!
//! # Common UX Patterns
//!
//! All components follow these standard patterns:
//!
//! ## Interactive Feedback
//!
//! - Cursor changes to pointer on hover for clickable items
//! - Background color transitions on hover (200ms)
//! - Active state shows pressed overlay
//! - Selection uses PRIMARY color with high contrast text
//!
//! ## Empty States
//!
//! Components show helpful empty states when no data is available:
//! - Clear title explaining the empty state
//! - Descriptive guidance on how to populate
//! - Optional CTA button for primary action
//!
//! ## Loading States
//!
//! Components show skeleton UI during async operations:
//! - Appears for operations longer than 200ms
//! - Matches structure of actual content
//! - Uses grayscale colors from loading module
//!
//! ## Accessibility
//!
//! - All components support keyboard navigation
//! - Focus indicators meet WCAG 2.1 requirements (3px blue border)
//! - Text labels for all interactive elements
//! - Color is not the sole indicator of state

pub mod audiobook_list;
pub mod bookmarks;
pub mod file_list;
pub mod player_controls;

/// Empty state component for when lists have no items
///
/// Provides helpful guidance following ui-ux-pro-max patterns:
/// - Clear title explaining why the list is empty (`SIZE_XXL` for prominence)
/// - Descriptive text providing context (`SIZE_BASE` with proper line spacing)
/// - Optional call-to-action button for resolution
/// - Centered layout for visual balance
/// - Generous padding (LG) for visual breathing room
///
/// # Design Guidelines
///
/// Per ui-ux-pro-max empty state patterns:
/// - Use secondary text color to differentiate from main content
/// - Provide actionable guidance on how to resolve the empty state
/// - Keep messaging concise but helpful (1-2 sentences)
/// - Always include a CTA when there's a clear next action
///
/// # Examples
///
/// ```ignore
/// use nodoka::ui::components::empty_state;
/// use nodoka::ui::Message;
///
/// // Empty state with CTA button and icon
/// let state = empty_state(
///     Some("▦"), // Unicode icon
///     "No Audiobooks Found",
///     "Add audiobook directories in Settings to get started. Nodoka will automatically scan for audiobook files.",
///     Some("Open Settings"),
///     Some(Message::OpenSettings),
/// );
///
/// // Empty state without CTA (informational)
/// let state = empty_state(
///     None, // No icon
///     "No Files",
///     "Select an audiobook from the list to view its files.",
///     None,
///     None,
/// );
/// ```
pub fn empty_state<'a>(
    icon: Option<&'static str>,
    title: &'static str,
    description: &'static str,
    action_label: Option<&'static str>,
    action_message: Option<crate::ui::Message>,
) -> iced::Element<'a, crate::ui::Message> {
    use crate::ui::styles::{border_radius, button_styles, colors, spacing, typography};
    use iced::widget::{button, column, container, text, Space};
    use iced::{Border, Length};

    let mut content = column![]
        .align_x(iced::Alignment::Center)
        .spacing(spacing::MD); // Reduced from LG for tighter hierarchy

    // Optional icon at top (Unicode symbol)
    if let Some(icon_text) = icon {
        content = content.push(
            text(icon_text)
                .size(typography::SIZE_XXL * 1.5) // 48px for visual prominence
                .style(|_theme: &iced::Theme| iced::widget::text::Style {
                    color: Some(colors::TEXT_SECONDARY),
                }),
        );
        content = content.push(Space::new().height(Length::Fixed(spacing::LG)));
    }

    // Title with enhanced typography hierarchy
    content = content.push(
        text(title)
            .size(typography::SIZE_XL) // 24px per specification
            .style(|_theme: &iced::Theme| iced::widget::text::Style {
                color: Some(colors::TEXT_PRIMARY),
            }),
    );

    content = content.push(Space::new().height(Length::Fixed(spacing::MD)));

    // Description with proper spacing
    content = content.push(
        container(
            text(description)
                .size(typography::SIZE_BASE) // 16px
                .style(|_theme: &iced::Theme| iced::widget::text::Style {
                    color: Some(colors::TEXT_SECONDARY),
                }),
        )
        .width(Length::Fixed(400.0)) // Max width for readability
        .center_x(Length::Fill),
    );

    if let (Some(label), Some(message)) = (action_label, action_message) {
        content = content.push(Space::new().height(Length::Fixed(spacing::LG)));
        content = content.push(
            button(
                text(label)
                    .size(typography::SIZE_BASE)
                    .style(|_theme: &iced::Theme| iced::widget::text::Style {
                        color: Some(colors::TEXT_ON_PRIMARY),
                    }),
            )
            .on_press(message)
            .padding(iced::Padding::from([
                spacing::SM + spacing::XS,
                spacing::LG,
            ]))
            .style(button_styles::primary),
        );
    }

    // Wrap content in container with subtle background tint
    let inner_container = container(content)
        .width(Length::Shrink)
        .padding(spacing::XXL)
        .style(|_theme: &iced::Theme| container::Style {
            background: Some(colors::BG_SECONDARY.into()),
            border: Border {
                color: colors::BORDER_DEFAULT,
                width: 2.0,
                radius: border_radius::LG.into(),
            },
            ..Default::default()
        });

    container(inner_container)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .padding(spacing::XXL)
        .into()
}
