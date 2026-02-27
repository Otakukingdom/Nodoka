//! UI Styles Module - Nodoka Design System
//!
//! This module provides a comprehensive design system for the Nodoka audiobook player.
//! It implements a premium rose palette with refined gradients, shadows, and animations
//! optimized for audiobook player applications with WCAG AA accessibility compliance.
//!
//! # Design System Overview
//!
//! The design system is organized into several modules:
//!
//! - **Colors**: Brand colors, semantic colors, text colors with WCAG AA contrast ratios
//! - **Spacing**: 4px base grid system for consistent layouts
//! - **Typography**: Size scale for text hierarchy
//! - **Border Radius**: Corner rounding constants for visual consistency
//! - **Transitions**: Animation durations with reduced motion support
//! - **Cursor**: Interaction affordances for different UI states
//! - **Interactive**: Hover and active state overlays
//! - **Loading**: Skeleton UI patterns for async operations
//! - **Button Styles**: Pre-configured button variants with focus states
//! - **Shadows**: Refined shadow system with 6 elevation levels (XS to XXL)
//! - **Animations**: Smooth transitions, stagger effects, and micro-interactions
//! - **Gradients**: Premium gradient colors for accent and brand elements
//!
//! # Premium Visual Language
//!
//! The design system uses gradients and refined shadows to create depth and visual interest:
//!
//! ## Gradient Colors
//!
//! Gradient colors provide visual richness while maintaining accessibility:
//!
//! - `PRIMARY_GRADIENT_START/END`: Rose gradient for primary actions and branding
//! - `ACCENT_GRADIENT_START/END`: Teal gradient for accent elements and badges
//! - `GLOW_*`: Emissive glow colors for elevated elements
//!
//! Use gradients on badges, buttons, and highlighted content:
//!
//! ```ignore
//! container(text("Featured").size(SIZE_SM))
//!     .padding([XS, SM])
//!     .style(|_theme| container::Style {
//!         background: Some(ACCENT_GRADIENT_START.into()),
//!         border: Border {
//!             color: ACCENT_GRADIENT_END,
//!             width: 1.0,
//!             radius: SM.into(),
//!         },
//!         shadow: shadow_xs(),
//!         ..Default::default()
//!     })
//! ```
//!
//! ## Shadow System
//!
//! Six shadow levels provide clear elevation hierarchy:
//!
//! - `shadow_xs()`: Subtle lift for badges and small elements
//! - `shadow_sm()`: Cards and list items
//! - `shadow_md()`: Floating panels and modals
//! - `shadow_lg()`: Dropdowns and popovers
//! - `shadow_xl()`: Large overlays and notifications
//! - `shadow_xxl()`: Maximum elevation for key modals
//!
//! Shadows use colored shadows matching the design palette rather than pure black:
//!
//! ```ignore
//! container(content)
//!     .style(|_theme| container::Style {
//!         shadow: shadow_md(),
//!         ..Default::default()
//!     })
//! ```
//!
//! # UX Patterns
//!
//! ## Interactive Feedback Patterns
//!
//! All clickable elements must provide clear visual feedback:
//!
//! - **Cursor**: Use `cursor::clickable()` for pointer cursor on interactive elements
//! - **Hover**: Apply `BG_HOVER` background color with 200ms transition
//! - **Pressed**: Apply `interactive::apply_active_overlay()` for darker feedback
//! - **Focus**: Use 3px `FOCUS_RING` border for keyboard navigation visibility
//!
//! ## Empty State Pattern
//!
//! Use `empty_state()` component when lists are empty to guide users:
//!
//! ```ignore
//! if items.is_empty() {
//!     return empty_state(
//!         "No Items Found",
//!         "Helpful description explaining why empty and what to do next.",
//!         Some("Primary Action"),
//!         Some(Message::OpenSettings),
//!     );
//! }
//! ```
//!
//! ## Loading State Pattern
//!
//! Show skeleton UI for operations taking longer than 200ms to provide clear feedback:
//!
//! ```ignore
//! match scan_state {
//!     ScanState::Scanning { .. } => skeleton_list(),
//!     ScanState::Idle => actual_content(),
//! }
//! ```
//!
//! ## Animation Patterns
//!
//! The animation system provides smooth transitions with reduced motion support:
//!
//! - **Stagger animations**: List items animate in with 50ms delays per item
//! - **Fade transitions**: Opacity changes for smooth appear/disappear
//! - **Scale transitions**: Subtle size changes for button presses
//! - **Pulse animations**: Gentle breathing effects for loading states
//!
//! Always respect `prefers_reduced_motion` to disable animations:
//!
//! ```ignore
//! let animation = animations_enabled(prefers_reduced_motion);
//! let opacity = if animation {
//!     animations::stagger_opacity(index, 50, elapsed_ms)
//! } else {
//!     1.0
//! };
//! ```
//!
//! ## Accessibility Requirements
//!
//! - All components support keyboard navigation (Tab/Shift+Tab)
//! - Focus indicators meet WCAG 2.1 2.4.7 (3px minimum, high contrast)
//! - Text contrast ratios meet WCAG AA 4.5:1 minimum
//! - Reduced motion preferences respected for animations
//! - All interactive elements have accessible labels
//! - Gradient backgrounds maintain sufficient contrast with foreground text
//!
//! # Theme Support
//!
//! The design system supports both light and dark themes via `ThemeMode`:
//!
//! - Light theme: Rose primary with warm gray backgrounds
//! - Dark theme: Desaturated rose with cool gray backgrounds
//! - Automatic theme colors via `get_color(mode, color_type)`
//!
//! # References
//!
//! - Design System: [`colors`], [`layout`], [`components`], [`interactive`]
//! - Animations: [`animations`], [`micro_interactions`]
//! - Effects: [`shadows`], [`cursor`], [`transitions`]
//! - WCAG 2.1: <https://www.w3.org/WAI/WCAG21/quickref/>

use iced::theme::Palette;
use iced::widget::container;
use iced::{Border, Color, Theme};

// Module declarations
pub mod accessibility;
pub mod animations;
pub mod badge_styles;
pub mod card_styles;
pub mod colors;
pub mod component_styles;
pub mod components;
pub mod design_system_tests;
pub mod effects;
pub mod gradients;
pub mod interactive;
pub mod layout;
pub mod micro_interactions;
pub mod slider;
pub mod typography;
pub mod typography_helpers;
pub mod typography_presets;

// Re-export submodules for backward compatibility
pub use accessibility::*;
pub use badge_styles::*;
pub use card_styles::*;
pub use colors::*;
pub use component_styles::*;
pub use components::*;
pub use effects::{cursor, shadows, transitions};
pub use gradients::*;
pub use interactive::*;
pub use layout::*;

/// Focus indicator styling for keyboard navigation accessibility
///
/// Provides visual focus indicators compliant with WCAG 2.1 AA 2.4.7.
/// Use with application state tracking since iced 0.14 doesn't expose focus state directly.
///
/// # Usage
///
/// ```ignore
/// let is_focused = state.focused_element == FocusedElement::PlayPauseButton;
/// container(button(text("Play/Pause")).on_press(Message::PlayPause))
///     .style(move |theme| focus_indicator(is_focused)(theme))
/// ```
///
/// # Focus Ring Specifications
///
/// - **Color**: Blue #2563EB (`FOCUS_RING` constant)
/// - **Width**: 3px (WCAG recommends minimum 3px for visibility)
/// - **Radius**: Medium border radius (8px)
/// - **Contrast**: High contrast against all backgrounds
pub fn focus_indicator(is_focused: bool) -> impl Fn(&iced::Theme) -> container::Style {
    move |_theme: &iced::Theme| {
        if is_focused {
            container::Style {
                border: Border {
                    color: colors::FOCUS_RING,
                    width: 3.0,
                    radius: border_radius::MD.into(),
                },
                ..Default::default()
            }
        } else {
            container::Style::default()
        }
    }
}

/// Legacy top-bar color re-export (alias for [`colors::TOP_BAR_COLOR`]).
pub const TOP_BAR_COLOR: Color = colors::TOP_BAR_COLOR;
/// Legacy player background re-export (alias for [`colors::PLAYER_BG_COLOR`]).
pub const PLAYER_BG_COLOR: Color = colors::PLAYER_BG_COLOR;
/// Legacy player text color re-export (alias for [`colors::PLAYER_TEXT_COLOR`]).
pub const PLAYER_TEXT_COLOR: Color = colors::PLAYER_TEXT_COLOR;
/// Legacy audiobook list background re-export (alias for [`colors::AUDIOBOOK_LIST_BG`]).
pub const AUDIOBOOK_LIST_BG: Color = colors::AUDIOBOOK_LIST_BG;
/// Legacy file list background re-export (alias for [`colors::FILE_LIST_BG`]).
pub const FILE_LIST_BG: Color = colors::FILE_LIST_BG;
/// Legacy selected item background re-export (alias for [`colors::SELECTION_BG`]).
pub const SELECTED_ITEM_BG: Color = colors::SELECTION_BG;
/// Legacy text color re-export (alias for [`colors::TEXT_PRIMARY`]).
pub const TEXT_COLOR: Color = colors::TEXT_PRIMARY;

/// Creates the custom Nodoka theme based on the design system
#[must_use]
pub fn nodoka_theme() -> Theme {
    nodoka_theme_for_mode(crate::ui::state::ThemeMode::Light)
}

/// Creates the custom Nodoka theme for the given theme mode.
#[must_use]
pub fn nodoka_theme_for_mode(mode: crate::ui::state::ThemeMode) -> Theme {
    use crate::ui::styles::colors::ColorType;

    let palette = Palette {
        background: colors::get_color(mode, ColorType::BgPrimary),
        text: colors::get_color(mode, ColorType::TextPrimary),
        primary: colors::get_color(mode, ColorType::Primary),
        success: colors::get_color(mode, ColorType::Success),
        danger: colors::get_color(mode, ColorType::Error),
        warning: colors::get_color(mode, ColorType::Warning),
    };

    let name = match mode {
        crate::ui::state::ThemeMode::Light => "Nodoka Light",
        crate::ui::state::ThemeMode::Dark => "Nodoka Dark",
    };

    Theme::custom(name.to_string(), palette)
}

/// Formats duration in milliseconds to human-readable time string (H:MM:SS or M:SS)
#[must_use]
pub fn format_duration(ms: Option<i64>) -> String {
    match ms {
        Some(duration) if duration > 0 => {
            let total_seconds = duration / 1000;
            let hours = total_seconds / 3600;
            let minutes = (total_seconds % 3600) / 60;
            let seconds = total_seconds % 60;

            if hours > 0 {
                format!("{hours}:{minutes:02}:{seconds:02}")
            } else {
                format!("{minutes}:{seconds:02}")
            }
        }
        _ => String::from("--:--"),
    }
}

/// Formats time in milliseconds to human-readable time string (H:MM:SS or M:SS)
#[must_use]
pub fn format_time_ms(ms: i64) -> String {
    let ms = ms.max(0);
    let total_seconds = ms / 1000;
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

    if hours > 0 {
        format!("{hours}:{minutes:02}:{seconds:02}")
    } else {
        format!("{minutes}:{seconds:02}")
    }
}
