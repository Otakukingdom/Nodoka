//! UI Components Module
//!
//! This module contains all reusable UI components for the Nodoka audiobook player,
//! implementing consistent project design patterns.
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
pub mod continue_listening;
pub mod empty_state;
pub mod file_list;
pub mod library_toolbar;
pub mod player_controls;
#[cfg(test)]
mod visual_tests;

/// Empty state component for when lists have no items
///
/// Provides helpful guidance with consistent layout and hierarchy:
/// - Clear title explaining why the list is empty (`SIZE_XXL` for prominence)
/// - Descriptive text providing context (`SIZE_BASE` with proper line spacing)
/// - Optional call-to-action button for resolution
/// - Centered layout for visual balance
/// - Generous padding (LG) for visual breathing room
///
/// # Design Guidelines
///
/// Empty state guidelines:
/// - Use secondary text color to differentiate from main content
/// - Provide actionable guidance on how to resolve the empty state
/// - Keep messaging concise but helpful (1-2 sentences)
/// - Always include a CTA when there's a clear next action
///
/// # Examples
///
/// ```ignore
/// use nodoka::ui::components::{empty_state, empty_state::EmptyStateContext};
/// use nodoka::ui::Message;
///
/// // Premium empty state with context
/// let state = empty_state::view(
///     EmptyStateContext::Library,
///     "No Audiobooks Found",
///     "Add audiobook directories in Settings to get started. Nodoka will automatically scan for audiobook files.",
///     Some("Open Settings"),
///     Some(Message::OpenSettings),
/// );
///
/// // Simplified empty state (backward compatible)
/// let state = empty_state(
///     Some("♪"),
///     "No Files",
///     "Select an audiobook from the list to view its files.",
///     None,
///     None,
/// );
/// ```
#[must_use]
pub fn empty_state<'a>(
    icon: Option<&'static str>,
    title: &'static str,
    description: &'static str,
    action_label: Option<&'static str>,
    action_message: Option<crate::ui::Message>,
) -> iced::Element<'a, crate::ui::Message> {
    // Use the new premium empty state module
    empty_state::simple(icon, title, description, action_label, action_message)
}
