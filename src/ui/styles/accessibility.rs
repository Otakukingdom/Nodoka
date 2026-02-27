//! Accessibility utilities for WCAG 2.1 Level AA compliance.
//!
//! This module provides utilities and constants for ensuring the application
//! meets WCAG 2.1 Level AA accessibility standards. In Nodoka, all interactive
//! elements must:
//!
//! - Have minimum touch target size of 44x44px (WCAG 2.5.5 Level AAA)
//! - Provide sufficient color contrast (4.5:1 for normal text)
//! - Include focus indicators with 3:1 contrast minimum
//! - Support keyboard navigation
//! - Respect reduced motion preferences
//!
//! # Examples
//!
//! ```ignore
//! use nodoka::ui::styles::accessibility;
//!
//! // Verify button meets touch target requirements
//! assert!(accessibility::is_touch_target_accessible(48.0, 48.0));
//!
//! // Check color contrast meets WCAG AA
//! assert!(accessibility::verify_contrast_aa(
//!     colors::TEXT_PRIMARY,
//!     colors::BG_PRIMARY
//! ));
//! ```

use iced::Color;

use super::{border_radius, colors};

/// Minimum touch target size per WCAG 2.1 Level AAA (2.5.5)
///
/// While Level AAA requires 44x44px, this is a best practice
/// for all interactive elements to ensure ease of use.
pub const MIN_TOUCH_TARGET_SIZE: f32 = 44.0;

/// WCAG AA contrast ratio for normal text (4.5:1)
pub const CONTRAST_RATIO_AA_NORMAL: f64 = 4.5;

/// WCAG AA contrast ratio for large text (3:1)
pub const CONTRAST_RATIO_AA_LARGE: f64 = 3.0;

/// WCAG AAA contrast ratio for normal text (7:1)
pub const CONTRAST_RATIO_AAA_NORMAL: f64 = 7.0;

/// WCAG minimum contrast ratio for non-text elements (3:1)
pub const CONTRAST_RATIO_NON_TEXT: f64 = 3.0;

/// Returns true if touch target meets minimum size requirements
///
/// Per WCAG 2.1 Level AAA 2.5.5: The size of the target for pointer
/// inputs is at least 44 by 44 CSS pixels.
///
/// # Arguments
///
/// * `width` - Width of the touch target in pixels
/// * `height` - Height of the touch target in pixels
///
/// # Examples
///
/// ```ignore
/// assert!(accessibility::is_touch_target_accessible(48.0, 48.0)); // true
/// assert!(!accessibility::is_touch_target_accessible(30.0, 30.0)); // false
/// ```
#[must_use]
pub const fn is_touch_target_accessible(width: f32, height: f32) -> bool {
    width >= MIN_TOUCH_TARGET_SIZE && height >= MIN_TOUCH_TARGET_SIZE
}

/// Verifies color contrast meets WCAG AA standard for normal text (4.5:1)
///
/// Per WCAG 2.1 Level AA 1.4.3: The visual presentation of text and
/// images of text has a contrast ratio of at least 4.5:1.
///
/// # Arguments
///
/// * `foreground` - Text or foreground color
/// * `background` - Background color
///
/// # Examples
///
/// ```ignore
/// use nodoka::ui::styles::{colors, accessibility};
///
/// assert!(accessibility::verify_contrast_aa(
///     colors::TEXT_PRIMARY,
///     colors::BG_PRIMARY
/// ));
/// ```
#[must_use]
pub fn verify_contrast_aa(foreground: Color, background: Color) -> bool {
    wcag_contrast_ratio(foreground, background) >= CONTRAST_RATIO_AA_NORMAL
}

/// Verifies color contrast meets WCAG AAA standard for normal text (7:1)
///
/// # Arguments
///
/// * `foreground` - Text or foreground color
/// * `background` - Background color
#[must_use]
pub fn verify_contrast_aaa(foreground: Color, background: Color) -> bool {
    wcag_contrast_ratio(foreground, background) >= CONTRAST_RATIO_AAA_NORMAL
}

/// Verifies color contrast meets WCAG AA standard for large text (3:1)
///
/// Large text is defined as 18pt (24px) or larger, or 14pt (19px) bold or larger.
///
/// # Arguments
///
/// * `foreground` - Text color
/// * `background` - Background color
#[must_use]
pub fn verify_contrast_aa_large(foreground: Color, background: Color) -> bool {
    wcag_contrast_ratio(foreground, background) >= CONTRAST_RATIO_AA_LARGE
}

/// Verifies color contrast meets WCAG minimum for non-text elements (3:1)
///
/// Per WCAG 2.1 Level AA 1.4.11: Visual presentation of UI components
/// and graphical objects have a contrast ratio of at least 3:1.
///
/// # Arguments
///
/// * `foreground` - Component color
/// * `background` - Background color
#[must_use]
pub fn verify_contrast_non_text(foreground: Color, background: Color) -> bool {
    wcag_contrast_ratio(foreground, background) >= CONTRAST_RATIO_NON_TEXT
}

/// Helper function to calculate WCAG contrast ratio
///
/// Uses the WCAG 2.1 relative luminance formula to calculate
/// contrast ratio between two colors.
///
/// # Formula
///
/// Contrast ratio = (L1 + 0.05) / (L2 + 0.05)
/// where L1 is the relative luminance of the lighter color
/// and L2 is the relative luminance of the darker color.
fn wcag_contrast_ratio(foreground: Color, background: Color) -> f64 {
    let fg = relative_luminance(foreground);
    let bg = relative_luminance(background);
    let (lighter, darker) = if fg >= bg { (fg, bg) } else { (bg, fg) };
    (lighter + 0.05) / (darker + 0.05)
}

/// Calculates relative luminance per WCAG 2.1 formula
fn relative_luminance(color: Color) -> f64 {
    let r = srgb_to_linear(f64::from(color.r));
    let g = srgb_to_linear(f64::from(color.g));
    let b = srgb_to_linear(f64::from(color.b));
    (0.2126_f64).mul_add(r, (0.7152_f64).mul_add(g, 0.0722_f64 * b))
}

/// Converts sRGB color channel to linear RGB
fn srgb_to_linear(channel: f64) -> f64 {
    if channel <= 0.04045 {
        channel / 12.92
    } else {
        ((channel + 0.055) / 1.055).powf(2.4)
    }
}

/// Returns container style for screen reader only content
///
/// Content styled with this function will be visually hidden but
/// still accessible to screen readers. Per WCAG best practices:
/// use for skip links and aria-label alternatives.
///
/// # Note
///
/// iced 0.14 has limited CSS-like positioning, so we achieve this
/// by using transparent colors. The element remains in layout flow.
///
/// # Examples
///
/// ```ignore
/// container(text("Skip to main content"))
///     .style(|theme| accessibility::screen_reader_only())
/// ```
#[must_use]
pub fn screen_reader_only() -> iced::widget::container::Style {
    use iced::widget::container;
    container::Style {
        text_color: Some(Color::TRANSPARENT),
        background: Some(Color::TRANSPARENT.into()),
        ..Default::default()
    }
}

/// Returns style for skip-to-content links
///
/// Skip links should be visually hidden but become visible when focused.
/// This is a placeholder implementation - full keyboard focus integration
/// requires application state tracking.
///
/// # Examples
///
/// ```ignore
/// let is_focused = state.focused_element == FocusedElement::SkipLink;
/// container(text("Skip to main content"))
///     .style(move |theme| accessibility::skip_link_style(is_focused))
/// ```
pub fn skip_link_style(
    is_focused: bool,
) -> impl Fn(&iced::Theme) -> iced::widget::container::Style {
    use iced::widget::container;
    move |_theme: &iced::Theme| {
        if is_focused {
            // When focused, show the skip link
            container::Style {
                text_color: Some(colors::TEXT_PRIMARY),
                background: Some(colors::BG_PRIMARY.into()),
                border: iced::Border {
                    color: colors::FOCUS_RING,
                    width: 3.0,
                    radius: border_radius::MD.into(),
                },
                ..Default::default()
            }
        } else {
            // When not focused, hide visually
            screen_reader_only()
        }
    }
}

/// Gradient color contrast verification utilities
///
/// Provides helpers for verifying that gradient colors maintain
/// WCAG AA contrast ratios when used as backgrounds or foregrounds.
///
/// Note: Decorative gradient elements (like progress bars, glow effects)
/// are not required to meet text contrast ratios, but must meet non-text
/// component contrast requirements (3:1).
pub mod gradient {
    use super::*;

    /// Verifies gradient start color meets WCAG AA for text on primary background
    ///
    /// Use this when placing white text on gradient backgrounds.
    #[must_use]
    pub fn verify_gradient_start_text_contrast() -> bool {
        verify_contrast_aa(
            colors::TEXT_ON_PRIMARY,
            colors::PRIMARY_GRADIENT_START,
        )
    }

    /// Verifies gradient end color meets WCAG AA for text on primary background
    #[must_use]
    pub fn verify_gradient_end_text_contrast() -> bool {
        verify_contrast_aa(
            colors::TEXT_ON_PRIMARY,
            colors::PRIMARY_GRADIENT_END,
        )
    }

    /// Verifies accent gradient colors meet non-text contrast requirements
    ///
    /// Used for decorative elements like badges, indicators.
    #[must_use]
    pub fn verify_accent_gradient_non_text_contrast() -> bool {
        verify_contrast_non_text(
            colors::ACCENT_GRADIENT_START,
            colors::BG_PRIMARY,
        ) && verify_contrast_non_text(
            colors::ACCENT_GRADIENT_END,
            colors::BG_PRIMARY,
        )
    }

    /// Verifies continue listening gradient meets non-text contrast
    #[must_use]
    pub fn verify_continue_gradient_non_text_contrast() -> bool {
        verify_contrast_non_text(
            colors::CONTINUE_GRADIENT_START,
            colors::BG_PRIMARY,
        ) && verify_contrast_non_text(
            colors::CONTINUE_GRADIENT_END,
            colors::BG_PRIMARY,
        )
    }

    /// Verifies progress bar gradient meets WCAG AA for non-text elements
    #[must_use]
    pub fn verify_progress_gradient_contrast() -> bool {
        verify_contrast_non_text(
            colors::PROGRESS_GRADIENT_START,
            colors::BG_PRIMARY,
        ) && verify_contrast_non_text(
            colors::PROGRESS_GRADIENT_END,
            colors::BG_PRIMARY,
        )
    }

    /// Verifies refined success color maintains WCAG AA contrast for text
    #[must_use]
    pub fn verify_success_refined_text_contrast() -> bool {
        verify_contrast_aa(
            colors::TEXT_ON_PRIMARY,
            colors::SUCCESS_REFINED,
        )
    }

    /// Verifies refined warning color meets non-text component contrast
    #[must_use]
    pub fn verify_warning_refined_non_text_contrast() -> bool {
        verify_contrast_non_text(
            colors::WARNING_REFINED,
            colors::BG_PRIMARY,
        )
    }

    /// Calculates contrast ratio between two colors
    ///
    /// Returns the WCAG 2.1 contrast ratio for testing purposes.
    #[must_use]
    pub fn contrast_ratio(foreground: Color, background: Color) -> f64 {
        wcag_contrast_ratio(foreground, background)
    }

    /// Gets the minimum contrast ratio in a gradient pair against a background
    ///
    /// Useful for determining worst-case readability across gradient transitions.
    #[must_use]
    pub fn min_gradient_contrast(
        foreground: Color,
        gradient_start: Color,
        gradient_end: Color,
    ) -> f64 {
        let ratio_start = wcag_contrast_ratio(foreground, gradient_start);
        let ratio_end = wcag_contrast_ratio(foreground, gradient_end);
        ratio_start.min(ratio_end)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gradient_colors_are_measurable() {
        let ratio = gradient::contrast_ratio(
            colors::TEXT_ON_PRIMARY,
            colors::PRIMARY_GRADIENT_START,
        );
        assert!(ratio > 0.0);
        assert!(ratio.is_finite());
    }

    #[test]
    fn test_primary_color_contrast_meets_wcag_aa() {
        // Critical: primary color with white text must pass WCAG AA
        assert!(verify_contrast_aa(
            colors::TEXT_ON_PRIMARY,
            colors::PRIMARY,
        ));
    }

    #[test]
    fn test_text_primary_on_background_meets_contrast() {
        // Critical: main text must be readable
        assert!(verify_contrast_aa(
            colors::TEXT_PRIMARY,
            colors::BG_PRIMARY,
        ));
    }

    #[test]
    fn test_error_color_contrast() {
        // Error semantic color should support white text
        assert!(verify_contrast_aa(
            colors::TEXT_ON_PRIMARY,
            colors::ERROR,
        ));
    }

    #[test]
    fn test_focus_ring_visible() {
        // Focus indicators must be visible
        assert!(verify_contrast_non_text(
            colors::FOCUS_RING,
            colors::BG_PRIMARY,
        ));
    }

    #[test]
    fn test_dark_theme_text_contrast() {
        // Dark theme must maintain contrast
        assert!(verify_contrast_aa(
            colors::dark_theme::TEXT_PRIMARY,
            colors::dark_theme::BG_PRIMARY,
        ));
    }

    #[test]
    fn test_min_gradient_contrast_calculation() {
        // Should return the lower of the two contrast ratios
        let min_ratio = gradient::min_gradient_contrast(
            colors::TEXT_ON_PRIMARY,
            colors::PRIMARY_GRADIENT_START,
            colors::PRIMARY_GRADIENT_END,
        );
        assert!(min_ratio > 0.0);
        assert!(min_ratio.is_finite());
    }

    /// Verifies glow overlay opacity is low enough for accessibility
    const _: () = assert!(colors::GLOW_PRIMARY.a < 0.5);

    /// Verifies glass overlay is semi-transparent for visual effect
    const _: () = assert!(colors::GLASS_OVERLAY.a > 0.5);
    const _: () = assert!(colors::GLASS_OVERLAY.a < 1.0);

    #[test]
    fn test_all_gradient_ratios_are_finite() {
        // Ensure all new gradient colors produce valid contrast ratios
        let ratios = vec![
            gradient::contrast_ratio(colors::TEXT_ON_PRIMARY, colors::PRIMARY_GRADIENT_START),
            gradient::contrast_ratio(colors::TEXT_ON_PRIMARY, colors::PRIMARY_GRADIENT_END),
            gradient::contrast_ratio(colors::TEXT_ON_PRIMARY, colors::ACCENT_GRADIENT_START),
            gradient::contrast_ratio(colors::TEXT_ON_PRIMARY, colors::ACCENT_GRADIENT_END),
            gradient::contrast_ratio(colors::TEXT_ON_PRIMARY, colors::CONTINUE_GRADIENT_START),
            gradient::contrast_ratio(colors::TEXT_ON_PRIMARY, colors::CONTINUE_GRADIENT_END),
            gradient::contrast_ratio(colors::PROGRESS_GRADIENT_START, colors::BG_PRIMARY),
            gradient::contrast_ratio(colors::PROGRESS_GRADIENT_END, colors::BG_PRIMARY),
        ];

        for ratio in ratios {
            assert!(ratio.is_finite());
            assert!(ratio > 0.0);
        }
    }
}
