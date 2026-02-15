use iced::theme::Palette;
use iced::widget::container;
use iced::{Border, Color, Theme};

/// Design system color tokens based on ui-ux-pro-max recommendations
/// for audiobook player applications with vibrant rose palette
pub mod colors {
    use iced::Color;

    // Primary brand colors (vibrant rose palette from design system)
    pub const PRIMARY: Color = Color::from_rgb(0.882, 0.114, 0.282); // #E11D48
    pub const PRIMARY_HOVER: Color = Color::from_rgb(0.753, 0.098, 0.239); // #C0183D (darker on hover)
    pub const PRIMARY_ACTIVE: Color = Color::from_rgb(0.639, 0.082, 0.200); // #A31533 (darker on active)
    pub const SECONDARY: Color = Color::from_rgb(0.984, 0.443, 0.522); // #FB7185
    pub const ACCENT: Color = Color::from_rgb(0.149, 0.388, 0.922); // #2563EB (engagement blue for CTA)

    // Semantic colors
    pub const SUCCESS: Color = Color::from_rgb(0.133, 0.545, 0.133); // #228B22
    pub const WARNING: Color = Color::from_rgb(0.957, 0.643, 0.376); // #F4A460
    pub const ERROR: Color = Color::from_rgb(0.882, 0.114, 0.282); // Same as primary for consistency
    pub const INFO: Color = Color::from_rgb(0.149, 0.388, 0.922); // Same as accent

    // Background colors (light mode optimized)
    pub const BG_PRIMARY: Color = Color::from_rgb(1.0, 0.945, 0.949); // #FFF1F2
    pub const BG_SECONDARY: Color = Color::from_rgb(1.0, 1.0, 1.0); // White
    pub const BG_ELEVATED: Color = Color::from_rgb(0.98, 0.98, 0.98); // Slightly darker for elevation
    pub const BG_HOVER: Color = Color::from_rgb(0.97, 0.97, 0.97); // Hover background

    // Text colors (proper contrast for accessibility - WCAG AA compliant)
    pub const TEXT_PRIMARY: Color = Color::from_rgb(0.45, 0.06, 0.18); // #730F2E (darker for better contrast)
    pub const TEXT_SECONDARY: Color = Color::from_rgb(0.4, 0.4, 0.4); // #666666 (gray)
    pub const TEXT_DISABLED: Color = Color::from_rgb(0.6, 0.6, 0.6); // #999999 (light gray)
    pub const TEXT_ON_PRIMARY: Color = Color::from_rgb(1.0, 1.0, 1.0); // White text on primary color

    // Interactive element colors
    pub const HOVER_OVERLAY: Color = Color::from_rgba(0.0, 0.0, 0.0, 0.05);
    pub const ACTIVE_OVERLAY: Color = Color::from_rgba(0.0, 0.0, 0.0, 0.1);
    pub const FOCUS_RING: Color = Color::from_rgb(0.149, 0.388, 0.922); // #2563EB (blue for focus states)
    pub const SELECTION_BG: Color = Color::from_rgb(0.882, 0.114, 0.282); // Primary color for selections
    pub const SELECTION_TEXT: Color = Color::from_rgb(1.0, 0.945, 0.949); // Light text on selection

    // Border colors
    pub const BORDER_DEFAULT: Color = Color::from_rgb(0.9, 0.9, 0.9); // #E5E5E5
    pub const BORDER_FOCUS: Color = Color::from_rgb(0.149, 0.388, 0.922); // #2563EB
    pub const BORDER_ERROR: Color = Color::from_rgb(0.882, 0.114, 0.282); // #E11D48

    // Legacy colors for backwards compatibility (deprecated, use design system colors instead)
    pub const TOP_BAR_COLOR: Color = Color::from_rgb(0.996, 0.855, 0.325); // #FEDB53 (legacy)
    pub const PLAYER_BG_COLOR: Color = BG_ELEVATED;
    pub const PLAYER_TEXT_COLOR: Color = TEXT_PRIMARY;
    pub const AUDIOBOOK_LIST_BG: Color = BG_PRIMARY;
    pub const FILE_LIST_BG: Color = BG_SECONDARY;
}

/// Spacing scale based on 4px base grid system
pub mod spacing {
    /// Extra small spacing: 4px
    pub const XS: f32 = 4.0;
    /// Small spacing: 8px
    pub const SM: f32 = 8.0;
    /// Medium spacing: 16px (base unit)
    pub const MD: f32 = 16.0;
    /// Large spacing: 24px
    pub const LG: f32 = 24.0;
    /// Extra large spacing: 32px
    pub const XL: f32 = 32.0;
    /// Extra extra large spacing: 48px
    pub const XXL: f32 = 48.0;
}

/// Typography scale for consistent text sizing
pub mod typography {
    /// Extra small text: 11px
    pub const SIZE_XS: u16 = 11;
    /// Small text: 13px
    pub const SIZE_SM: u16 = 13;
    /// Base text size: 14px
    pub const SIZE_BASE: u16 = 14;
    /// Large text: 16px
    pub const SIZE_LG: u16 = 16;
    /// Extra large text: 20px
    pub const SIZE_XL: u16 = 20;
    /// Extra extra large text: 24px
    pub const SIZE_XXL: u16 = 24;
    /// Heading text: 32px
    pub const SIZE_HEADING: u16 = 32;
}

/// Border radius constants for consistent rounded corners
pub mod border_radius {
    /// Small radius: 4px
    pub const SM: f32 = 4.0;
    /// Medium radius: 8px
    pub const MD: f32 = 8.0;
    /// Large radius: 12px
    pub const LG: f32 = 12.0;
    /// Extra large radius: 16px
    pub const XL: f32 = 16.0;
}

/// Transition duration constants for smooth animations
pub mod transitions {
    /// Fast transition: 150ms
    pub const FAST: u64 = 150;
    /// Normal transition: 200ms
    pub const NORMAL: u64 = 200;
    /// Slow transition: 300ms
    pub const SLOW: u64 = 300;
}

/// Shadow definitions for depth hierarchy (represented as border colors for iced compatibility)
pub mod shadows {
    use iced::Color;

    /// Small shadow equivalent: subtle border
    pub const SM_BORDER: Color = Color::from_rgb(0.85, 0.85, 0.85); // #D9D9D9
    /// Medium shadow equivalent: medium border
    pub const MD_BORDER: Color = Color::from_rgb(0.8, 0.8, 0.8); // #CCCCCC
    /// Large shadow equivalent: strong border
    pub const LG_BORDER: Color = Color::from_rgb(0.75, 0.75, 0.75); // #BFBFBF
}

/// Button style utilities for consistent interactive element styling
/// Provides primary, secondary, and danger button theme functions
///
/// # iced 0.12 Limitation - Button Styling NOT FUNCTIONAL
///
/// **Status**: Steps 4 and 10 from the implementation plan attempted to apply button styles
/// but failed compilation. Custom button styles cannot be applied in iced 0.12.
///
/// **Issue**: The `.style()` method on buttons expects `impl Into<iced::theme::Button>`,
/// but the functions below return `impl Fn(&Theme) -> button::Appearance`. The closure
/// type cannot be converted to `iced::theme::Button` without implementing a custom theme.
///
/// **Attempted**: Button style applications were added to `player_controls.rs`, `bookmarks.rs`,
/// and `settings_form.rs` but resulted in 27 compilation errors (trait bound not satisfied).
///
/// **Resolution**: Removed all `.style(button_styles::*)` calls. Buttons use default iced
/// theme styling. The `button_styles` module remains as documentation for future upgrades.
///
/// **Recommended Fix**: Upgrade to iced 0.13+ which may have improved styling APIs, or
/// implement a full custom theme. Alternatively, use iced's built-in theme variants or
/// container-based styling workarounds (as done for selection states).
///
/// **Impact**: Moderate UX issue. All buttons look identical, violating button hierarchy
/// principles. Primary actions (Play, Save) should be visually prominent, destructive
/// actions (Delete, Remove) should use warning colors. Current UI lacks this visual hierarchy.
///
/// # Focus Indicator Limitations (iced 0.12)
///
/// **Current Status**: Focus indicators for keyboard navigation are **not fully implemented**
/// due to iced 0.12 framework limitations.
///
/// **Issue**: The iced 0.12 button styling system does not provide direct access to focus
/// state in the style function signature. Focus state would need to be passed through the
/// application state or require a custom theme implementation.
///
/// **Accessibility Impact**: Users navigating with keyboard (Tab key) may not see visual
/// focus indicators on buttons, making it difficult to track which element is currently
/// focused. This is a **WCAG 2.1 Level AA violation** (Success Criterion 2.4.7).
///
/// **Recommended Workarounds**:
/// 1. **Framework Upgrade**: Upgrade to iced 0.13+ which has improved focus handling
/// 2. **Custom Theme**: Implement full custom theme with focus state tracking
/// 3. **Visual Cues**: Use color changes on button hover as partial substitute
/// 4. **Screen Reader**: Rely on screen reader announcements for focus tracking
///
/// **Planned Fix**: When upgrading iced, add focus indicator styling:
/// ```rust,ignore
/// pub fn primary_focused() -> button::Appearance {
///     button::Appearance {
///         border: Border {
///             color: colors::FOCUS_RING, // Blue #2563EB
///             width: 3.0,
///             radius: border_radius::MD.into(),
///         },
///         ..primary()
///     }
/// }
/// ```
///
/// **Manual Testing**: Use Test Case 15 in `tests/manual_ui_checklist.md` to verify
/// focus indicators after implementing the fix.
///
/// # Workaround
/// For custom button styling in iced 0.12, consider:
/// - Wrapping buttons in styled containers (see `audiobook_list.rs` and `file_list.rs`)
/// - Using iced's built-in theme variants
/// - Upgrading to newer iced versions with better styling support
///
/// # Usage (when iced theme system supports it)
/// ```rust,ignore
/// button(text("Save"))
///     .on_press(Message::Save)
///     .style(button_styles::primary())
/// ```
pub mod button_styles {
    use iced::widget::button;
    use iced::Border;

    use super::{border_radius, colors};

    /// Primary action button style (e.g., Play, Save, Add)
    /// Uses primary brand color with white text
    /// Provides hover state with slightly darker background
    pub fn primary() -> impl Fn(&iced::Theme) -> button::Appearance {
        move |_theme: &iced::Theme| button::Appearance {
            background: Some(colors::PRIMARY.into()),
            text_color: colors::TEXT_ON_PRIMARY,
            border: Border {
                color: iced::Color::TRANSPARENT,
                width: 0.0,
                radius: border_radius::MD.into(),
            },
            ..Default::default()
        }
    }

    /// Secondary action button style (e.g., Cancel, Close, Stop)
    /// Uses elevated background with primary text color and border
    pub fn secondary() -> impl Fn(&iced::Theme) -> button::Appearance {
        move |_theme: &iced::Theme| button::Appearance {
            background: Some(colors::BG_ELEVATED.into()),
            text_color: colors::TEXT_PRIMARY,
            border: Border {
                color: colors::BORDER_DEFAULT,
                width: 1.0,
                radius: border_radius::MD.into(),
            },
            ..Default::default()
        }
    }

    /// Danger action button style (e.g., Delete, Remove)
    /// Uses error/danger color with white text
    pub fn danger() -> impl Fn(&iced::Theme) -> button::Appearance {
        move |_theme: &iced::Theme| button::Appearance {
            background: Some(colors::ERROR.into()),
            text_color: colors::TEXT_ON_PRIMARY,
            border: Border {
                color: iced::Color::TRANSPARENT,
                width: 0.0,
                radius: border_radius::MD.into(),
            },
            ..Default::default()
        }
    }
}

/// Container-based button styling workaround for iced 0.12
///
/// Since `.style()` on buttons is not functional in iced 0.12, wrap buttons in containers
/// with appropriate backgrounds and borders to achieve visual hierarchy.
///
/// # Usage
///
/// Instead of:
/// ```ignore
/// button(text("Save")).style(button_styles::primary())
/// ```
///
/// Use:
/// ```ignore
/// container(button(text("Save")).on_press(Message::Save))
///     .style(button_containers::primary())
///     .padding(spacing::SM)
/// ```
pub mod button_containers {
    use iced::widget::container;
    use iced::Border;

    use super::{border_radius, colors};

    /// Primary action button container style (vibrant rose background)
    ///
    /// Use for the most important actions in a context:
    /// - Play/Pause (main action)
    /// - Save (confirmation dialog)
    /// - Add (adding new items)
    /// - Set (sleep timer)
    pub fn primary() -> impl Fn(&iced::Theme) -> container::Appearance {
        move |_theme: &iced::Theme| container::Appearance {
            background: Some(iced::Background::Color(colors::PRIMARY)),
            border: Border {
                color: colors::PRIMARY,
                width: 2.0,
                radius: border_radius::MD.into(),
            },
            text_color: Some(colors::TEXT_ON_PRIMARY),
            ..Default::default()
        }
    }

    /// Secondary action button container style (elevated with border)
    ///
    /// Use for secondary actions:
    /// - Stop (less important than play/pause)
    /// - Cancel (dismissive action)
    /// - Close (modal dismiss)
    /// - Edit (non-primary modification)
    pub fn secondary() -> impl Fn(&iced::Theme) -> container::Appearance {
        move |_theme: &iced::Theme| container::Appearance {
            background: Some(iced::Background::Color(colors::BG_ELEVATED)),
            border: Border {
                color: colors::BORDER_DEFAULT,
                width: 1.0,
                radius: border_radius::MD.into(),
            },
            text_color: Some(colors::TEXT_PRIMARY),
            ..Default::default()
        }
    }

    /// Danger/destructive action button container style (error color)
    ///
    /// Use for destructive actions that cannot be undone:
    /// - Delete (permanent removal)
    /// - Remove (removing from list)
    /// - Cancel timer (destructive in some contexts)
    pub fn danger() -> impl Fn(&iced::Theme) -> container::Appearance {
        move |_theme: &iced::Theme| container::Appearance {
            background: Some(iced::Background::Color(colors::ERROR)),
            border: Border {
                color: colors::BORDER_ERROR,
                width: 2.0,
                radius: border_radius::MD.into(),
            },
            text_color: Some(colors::TEXT_ON_PRIMARY),
            ..Default::default()
        }
    }
}

/// Focus indicator styling for keyboard navigation accessibility
///
/// Provides visual focus indicators compliant with WCAG 2.1 AA 2.4.7.
/// Use with application state tracking since iced 0.12 doesn't expose focus.
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
pub fn focus_indicator(is_focused: bool) -> impl Fn(&iced::Theme) -> container::Appearance {
    use iced::widget::container;
    move |_theme: &iced::Theme| {
        if is_focused {
            container::Appearance {
                border: Border {
                    color: colors::FOCUS_RING,
                    width: 3.0,
                    radius: border_radius::MD.into(),
                },
                ..Default::default()
            }
        } else {
            container::Appearance::default()
        }
    }
}

// Legacy color exports for backwards compatibility
pub const TOP_BAR_COLOR: Color = colors::TOP_BAR_COLOR;
pub const PLAYER_BG_COLOR: Color = colors::PLAYER_BG_COLOR;
pub const PLAYER_TEXT_COLOR: Color = colors::PLAYER_TEXT_COLOR;
pub const AUDIOBOOK_LIST_BG: Color = colors::AUDIOBOOK_LIST_BG;
pub const FILE_LIST_BG: Color = colors::FILE_LIST_BG;
pub const SELECTED_ITEM_BG: Color = colors::SELECTION_BG;
pub const TEXT_COLOR: Color = colors::TEXT_PRIMARY;

/// Creates the custom Nodoka theme based on the design system
#[must_use]
pub fn nodoka_theme() -> Theme {
    let palette = Palette {
        background: colors::BG_PRIMARY,
        text: colors::TEXT_PRIMARY,
        primary: colors::PRIMARY,
        success: colors::SUCCESS,
        danger: colors::ERROR,
    };

    Theme::custom("Nodoka".to_string(), palette)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_duration_with_hours() {
        assert_eq!(format_duration(Some(3_661_000)), "1:01:01");
        assert_eq!(format_duration(Some(7_200_000)), "2:00:00");
    }

    #[test]
    fn test_format_duration_with_minutes_only() {
        assert_eq!(format_duration(Some(125_000)), "2:05");
        assert_eq!(format_duration(Some(60000)), "1:00");
    }

    #[test]
    fn test_format_duration_with_zero() {
        assert_eq!(format_duration(Some(0)), "--:--");
    }

    #[test]
    fn test_format_duration_with_none() {
        assert_eq!(format_duration(None), "--:--");
    }

    #[test]
    fn test_format_duration_with_negative() {
        assert_eq!(format_duration(Some(-1000)), "--:--");
    }

    #[test]
    fn test_format_time_ms_with_hours() {
        assert_eq!(format_time_ms(3_661_000), "1:01:01");
        assert_eq!(format_time_ms(7_200_000), "2:00:00");
    }

    #[test]
    fn test_format_time_ms_with_minutes_only() {
        assert_eq!(format_time_ms(125_000), "2:05");
        assert_eq!(format_time_ms(60000), "1:00");
        assert_eq!(format_time_ms(0), "0:00");
    }

    #[test]
    fn test_format_time_ms_with_seconds_only() {
        assert_eq!(format_time_ms(30000), "0:30");
        assert_eq!(format_time_ms(1000), "0:01");
    }

    #[test]
    fn test_nodoka_theme_uses_design_system_colors() {
        let theme = nodoka_theme();
        let palette = theme.palette();

        // Verify theme uses design system colors
        assert_eq!(palette.background, colors::BG_PRIMARY);
        assert_eq!(palette.text, colors::TEXT_PRIMARY);
        assert_eq!(palette.primary, colors::PRIMARY);
        assert_eq!(palette.success, colors::SUCCESS);
        assert_eq!(palette.danger, colors::ERROR);
    }

    #[test]
    fn test_color_contrast_meets_wcag_aa() {
        // Verify primary text on primary background has sufficient contrast
        // TEXT_PRIMARY (#730F2E) on BG_PRIMARY (#FFF1F2) should have > 4.5:1 ratio
        // This is a simplified check - in production, use a proper contrast ratio calculator

        let text_r = 0.45_f32;
        let text_g = 0.06_f32;
        let text_b = 0.18_f32;

        let bg_r = 1.0_f32;
        let bg_g = 0.945_f32;
        let bg_b = 0.949_f32;

        // Calculate relative luminance (simplified)
        let text_lum = (0.2126_f32).mul_add(text_r, (0.7152_f32).mul_add(text_g, 0.0722 * text_b));
        let bg_lum = (0.2126_f32).mul_add(bg_r, (0.7152_f32).mul_add(bg_g, 0.0722 * bg_b));

        // Contrast ratio
        let contrast = (bg_lum + 0.05) / (text_lum + 0.05);

        // WCAG AA requires 4.5:1 for normal text
        assert!(
            contrast >= 4.5,
            "Text contrast ratio {contrast} does not meet WCAG AA (4.5:1)"
        );
    }

    #[test]
    fn test_spacing_follows_4px_grid() {
        const EPSILON: f32 = 1e-6;

        assert!((spacing::XS - 4.0).abs() < EPSILON);
        assert!((spacing::SM - 8.0).abs() < EPSILON);
        assert!((spacing::MD - 16.0).abs() < EPSILON);
        assert!((spacing::LG - 24.0).abs() < EPSILON);
        assert!((spacing::XL - 32.0).abs() < EPSILON);
        assert!((spacing::XXL - 48.0).abs() < EPSILON);

        // Verify all spacings are multiples of 4
        assert!((spacing::XS % 4.0).abs() < EPSILON);
        assert!((spacing::SM % 4.0).abs() < EPSILON);
        assert!((spacing::MD % 4.0).abs() < EPSILON);
        assert!((spacing::LG % 4.0).abs() < EPSILON);
        assert!((spacing::XL % 4.0).abs() < EPSILON);
        assert!((spacing::XXL % 4.0).abs() < EPSILON);
    }

    #[test]
    fn test_typography_scale_is_reasonable() {
        // Verify text sizes increase monotonically
        let sizes = [
            typography::SIZE_XS,
            typography::SIZE_SM,
            typography::SIZE_BASE,
            typography::SIZE_LG,
            typography::SIZE_XL,
            typography::SIZE_XXL,
            typography::SIZE_HEADING,
        ];

        for window in sizes.windows(2) {
            if let [current, next] = window {
                assert!(
                    current < next,
                    "Typography sizes should increase monotonically"
                );
            }
        }
    }

    #[test]
    fn test_transition_durations_are_in_acceptable_range() {
        // Verify transitions are between 100ms and 500ms (best practices)
        let durations = [transitions::FAST, transitions::NORMAL, transitions::SLOW];

        for &duration in &durations {
            assert!(
                (100..=500).contains(&duration),
                "Transition duration {duration} is out of acceptable range"
            );
        }

        // Verify durations increase monotonically
        for window in durations.windows(2) {
            if let [current, next] = window {
                assert!(
                    current < next,
                    "Transition durations should increase monotonically"
                );
            }
        }
    }
}
