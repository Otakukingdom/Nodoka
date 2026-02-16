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
/// // Empty state with CTA button
/// let state = empty_state(
///     "No Audiobooks Found",
///     "Add audiobook directories in Settings to get started. Nodoka will automatically scan for audiobook files.",
///     Some("Open Settings"),
///     Some(Message::OpenSettings),
/// );
///
/// // Empty state without CTA (informational)
/// let state = empty_state(
///     "No Files",
///     "Select an audiobook from the list to view its files.",
///     None,
///     None,
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
            // Larger, more prominent title (SIZE_XXL instead of SIZE_XL)
            text(title)
                .size(typography::SIZE_XXL)
                .style(|_theme: &iced::Theme| iced::widget::text::Style {
                    color: Some(colors::TEXT_PRIMARY), // Use primary text color for better visibility
                }),
            // Description with better spacing
            container(text(description).size(typography::SIZE_BASE).style(
                |_theme: &iced::Theme| iced::widget::text::Style {
                    color: Some(colors::TEXT_SECONDARY),
                }
            ))
            .padding(iced::Padding::from([spacing::LG, 0.0])) // Increased padding for better separation
            .width(Length::Fixed(400.0)) // Max width for better readability
        ]
        .align_x(iced::Alignment::Center)
        .spacing(spacing::SM); // Tighter spacing between title and description

    if let (Some(label), Some(message)) = (action_label, action_message) {
        content = content.push(
            // CTA button with improved styling and focus indicator support
            container(
                button(text(label).size(typography::SIZE_BASE))
                    .on_press(message)
                    .padding(iced::Padding::from([spacing::MD, spacing::LG])) // More horizontal padding for better button proportions
                    .style(button_styles::primary),
            )
            .padding(iced::Padding::from([spacing::LG, 0.0])), // Space above button
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
