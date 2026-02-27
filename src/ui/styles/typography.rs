//! Typography hierarchy system for distinctive visual design.
//!
//! This module provides a comprehensive typography system with font weights,
//! line heights, letter spacing, and type scale pairing to create clear
//! visual hierarchy and distinctive character.

// Re-export font size constants from layout module for convenience
pub use crate::ui::styles::layout::typography::{
    SIZE_BASE, SIZE_HEADING, SIZE_LG, SIZE_SM, SIZE_XL, SIZE_XS, SIZE_XXL,
};

/// Additional size constant for premium headings (28px)
pub const SIZE_PREMIUM: f32 = 28.0;
/// Extra large heading size for brand-critical text (36px)
pub const SIZE_DISPLAY_PREMIUM: f32 = 36.0;

// Re-export preset and helper modules from parent module
pub use crate::ui::styles::typography_presets::*;
pub use crate::ui::styles::typography_helpers::*;

/// Font weight constants for distinctive typography
pub mod weight {
    /// Extra light font weight (200)
    pub const EXTRA_LIGHT: u16 = 200;
    /// Light font weight (300)
    pub const LIGHT: u16 = 300;
    /// Regular font weight (400)
    pub const REGULAR: u16 = 400;
    /// Medium font weight (500)
    pub const MEDIUM: u16 = 500;
    /// Semibold font weight (600)
    pub const SEMIBOLD: u16 = 600;
    /// Bold font weight (700)
    pub const BOLD: u16 = 700;
    /// Extra bold font weight (800)
    pub const EXTRA_BOLD: u16 = 800;
}

/// Line height ratios for different text contexts
pub mod line_height {
    /// Extra tight line height for large headings (1.15)
    pub const EXTRA_TIGHT: f32 = 1.15;
    /// Tight line height for headings (1.2)
    pub const TIGHT: f32 = 1.2;
    /// Dense line height for compact UI (1.3)
    pub const DENSE: f32 = 1.3;
    /// Normal line height for body text (1.5)
    pub const NORMAL: f32 = 1.5;
    /// Relaxed line height for long-form content (1.6)
    pub const RELAXED: f32 = 1.6;
    /// Extra relaxed line height for maximum readability (1.75)
    pub const EXTRA_RELAXED: f32 = 1.75;
}

/// Letter spacing adjustments for display text
pub mod letter_spacing {
    /// Extra tight letter spacing for dramatic headings (-0.03em)
    pub const EXTRA_TIGHT: f32 = -0.03;
    /// Tight letter spacing for distinctive headings (-0.02em)
    pub const TIGHT: f32 = -0.02;
    /// Slightly tight letter spacing for premium headings (-0.01em)
    pub const SLIGHT_TIGHT: f32 = -0.01;
    /// No letter spacing adjustment
    pub const NONE: f32 = 0.0;
    /// Slight letter spacing for headings (+0.01em)
    pub const SLIGHT: f32 = 0.01;
    /// Moderate letter spacing for display text (+0.02em)
    pub const MODERATE: f32 = 0.02;
    /// Wide letter spacing for buttons/labels (+0.05em)
    pub const WIDE: f32 = 0.05;
    /// Extra wide letter spacing for emphasis (+0.08em)
    pub const EXTRA_WIDE: f32 = 0.08;
}

/// Type scale pairing system - combines sizes with weights for hierarchy
///
/// Usage:
/// - Grand: Extra large hero text for feature sections
/// - Display: Hero sections, large titles
/// - Heading 1: Page titles
/// - Heading 2: Section titles
/// - Heading 3: Subsection titles
/// - Heading 4: Minor headings
/// - Body Large: Emphasized body text
/// - Body: Primary content
/// - Small: Secondary content, metadata
/// - Caption: Tiny labels, hints
pub mod type_scale {
    use super::weight;

    /// Grand/hero text - extra large for feature sections
    pub const GRAND_SIZE: f32 = 56.0;
    pub const GRAND_WEIGHT: u16 = weight::BOLD;

    /// Display/hero text - largest, boldest
    pub const DISPLAY_SIZE: f32 = 48.0;
    pub const DISPLAY_WEIGHT: u16 = weight::BOLD;

    /// Heading 1 - page titles
    pub const H1_SIZE: f32 = 32.0;
    pub const H1_WEIGHT: u16 = weight::SEMIBOLD;

    /// Heading 2 - section titles
    pub const H2_SIZE: f32 = 24.0;
    pub const H2_WEIGHT: u16 = weight::SEMIBOLD;

    /// Heading 3 - subsection titles
    pub const H3_SIZE: f32 = 20.0;
    pub const H3_WEIGHT: u16 = weight::MEDIUM;

    /// Heading 4 - minor headings
    pub const H4_SIZE: f32 = 18.0;
    pub const H4_WEIGHT: u16 = weight::MEDIUM;

    /// Body large text - emphasized body text
    pub const BODY_LARGE_SIZE: f32 = 18.0;
    pub const BODY_LARGE_WEIGHT: u16 = weight::REGULAR;

    /// Body text - primary content
    pub const BODY_SIZE: f32 = 16.0;
    pub const BODY_WEIGHT: u16 = weight::REGULAR;

    /// Small text - secondary content, metadata
    pub const SMALL_SIZE: f32 = 14.0;
    pub const SMALL_WEIGHT: u16 = weight::REGULAR;

    /// Caption text - tiny labels, hints
    pub const CAPTION_SIZE: f32 = 12.0;
    pub const CAPTION_WEIGHT: u16 = weight::REGULAR;

    /// Button text - slightly heavier for affordance
    pub const BUTTON_SIZE: f32 = 14.0;
    pub const BUTTON_WEIGHT: u16 = weight::MEDIUM;
}

/// Theme-aware text color helpers for semantic typography
///
/// Provides consistent text color selection based on hierarchy level
/// and semantic purpose.
pub mod semantic_color {
    use crate::ui::styles::colors::{self, ColorType};
    use crate::ui::state::ThemeMode;

    /// Primary text color for headings and important content
    #[must_use]
    pub const fn heading(theme: ThemeMode) -> iced::Color {
        colors::get_color(theme, ColorType::TextPrimary)
    }

    /// Secondary text color for body content
    #[must_use]
    pub const fn body(theme: ThemeMode) -> iced::Color {
        colors::get_color(theme, ColorType::TextPrimary)
    }

    /// Muted text color for metadata and secondary information
    #[must_use]
    pub const fn metadata(theme: ThemeMode) -> iced::Color {
        colors::get_color(theme, ColorType::TextSecondary)
    }

    /// Disabled text color
    #[must_use]
    pub const fn disabled(theme: ThemeMode) -> iced::Color {
        colors::get_color(theme, ColorType::TextDisabled)
    }

    /// Accent text color for highlights and CTAs
    #[must_use]
    pub const fn accent(theme: ThemeMode) -> iced::Color {
        colors::get_color(theme, ColorType::Primary)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies font weight constants are within standard CSS weight range (100-900)
    /// using const assertions for compile-time verification.
    const _: () = assert!(weight::EXTRA_LIGHT >= 100 && weight::EXTRA_LIGHT <= 900);
    const _: () = assert!(weight::LIGHT >= 100 && weight::LIGHT <= 900);
    const _: () = assert!(weight::REGULAR >= 100 && weight::REGULAR <= 900);
    const _: () = assert!(weight::MEDIUM >= 100 && weight::MEDIUM <= 900);
    const _: () = assert!(weight::SEMIBOLD >= 100 && weight::SEMIBOLD <= 900);
    const _: () = assert!(weight::BOLD >= 100 && weight::BOLD <= 900);
    const _: () = assert!(weight::EXTRA_BOLD >= 100 && weight::EXTRA_BOLD <= 900);

    /// Verifies font weights ascend monotonically for proper hierarchy.
    const _: () = assert!(weight::EXTRA_LIGHT < weight::LIGHT);
    const _: () = assert!(weight::LIGHT < weight::REGULAR);
    const _: () = assert!(weight::REGULAR < weight::MEDIUM);
    const _: () = assert!(weight::MEDIUM < weight::SEMIBOLD);
    const _: () = assert!(weight::SEMIBOLD < weight::BOLD);
    const _: () = assert!(weight::BOLD < weight::EXTRA_BOLD);

    /// Verifies line height ratios are positive for valid layout.
    const _: () = assert!(line_height::EXTRA_TIGHT > 0.0);
    const _: () = assert!(line_height::TIGHT > 0.0);
    const _: () = assert!(line_height::DENSE > 0.0);
    const _: () = assert!(line_height::NORMAL > 0.0);
    const _: () = assert!(line_height::RELAXED > 0.0);
    const _: () = assert!(line_height::EXTRA_RELAXED > 0.0);

    /// Verifies line height ratios progress from tight to relaxed.
    const _: () = assert!(line_height::EXTRA_TIGHT < line_height::TIGHT);
    const _: () = assert!(line_height::TIGHT < line_height::DENSE);
    const _: () = assert!(line_height::DENSE < line_height::NORMAL);
    const _: () = assert!(line_height::NORMAL < line_height::RELAXED);
    const _: () = assert!(line_height::RELAXED < line_height::EXTRA_RELAXED);

    /// Verifies letter spacing values are non-negative.
    const _: () = assert!(letter_spacing::NONE >= 0.0);
    const _: () = assert!(letter_spacing::SLIGHT >= 0.0);
    const _: () = assert!(letter_spacing::MODERATE >= 0.0);
    const _: () = assert!(letter_spacing::WIDE >= 0.0);

    /// Verifies type scale sizes are positive for rendering.
    const _: () = assert!(type_scale::GRAND_SIZE > 0.0);
    const _: () = assert!(type_scale::DISPLAY_SIZE > 0.0);
    const _: () = assert!(type_scale::H1_SIZE > 0.0);
    const _: () = assert!(type_scale::H2_SIZE > 0.0);
    const _: () = assert!(type_scale::H3_SIZE > 0.0);
    const _: () = assert!(type_scale::H4_SIZE > 0.0);
    const _: () = assert!(type_scale::BODY_LARGE_SIZE > 0.0);
    const _: () = assert!(type_scale::BODY_SIZE > 0.0);
    const _: () = assert!(type_scale::SMALL_SIZE > 0.0);
    const _: () = assert!(type_scale::CAPTION_SIZE > 0.0);

    /// Verifies type scale sizes descend properly for visual hierarchy.
    const _: () = assert!(type_scale::GRAND_SIZE > type_scale::DISPLAY_SIZE);
    const _: () = assert!(type_scale::DISPLAY_SIZE > type_scale::H1_SIZE);
    const _: () = assert!(type_scale::H1_SIZE > type_scale::H2_SIZE);
    const _: () = assert!(type_scale::H2_SIZE > type_scale::H3_SIZE);
    const _: () = assert!(type_scale::H3_SIZE > type_scale::H4_SIZE);
    const _: () = assert!(type_scale::H4_SIZE >= type_scale::BODY_LARGE_SIZE);
    const _: () = assert!(type_scale::BODY_LARGE_SIZE >= type_scale::BODY_SIZE);
    const _: () = assert!(type_scale::BODY_SIZE > type_scale::SMALL_SIZE);
    const _: () = assert!(type_scale::SMALL_SIZE > type_scale::CAPTION_SIZE);

    #[test]
    fn test_preset_display_returns_valid_values() {
        let display = crate::ui::styles::typography_presets::Display::new();
        assert!(display.size > 0.0);
        assert!(display.weight >= 100 && display.weight <= 900);
        assert!(display.line_height_ratio > 0.0);
    }

    #[test]
    fn test_preset_heading_returns_valid_values() {
        let heading = crate::ui::styles::typography_presets::Heading::new();
        assert!(heading.size > 0.0);
        assert!(heading.weight >= 100 && heading.weight <= 900);
        assert!(heading.line_height_ratio > 0.0);
    }

    #[test]
    fn test_preset_heading_levels_descend() {
        let h1 = crate::ui::styles::typography_presets::Heading::new();
        let h2 = crate::ui::styles::typography_presets::Heading::level_2();
        let h3 = crate::ui::styles::typography_presets::Heading::level_3();
        assert!(h1.size > h2.size);
        assert!(h2.size > h3.size);
    }

    #[test]
    fn test_preset_body_returns_valid_values() {
        let body = crate::ui::styles::typography_presets::Body::new();
        assert!(body.size > 0.0);
        assert!(body.weight >= 100 && body.weight <= 900);
        assert!(body.line_height_ratio > 0.0);
    }

    #[test]
    fn test_preset_small_returns_valid_values() {
        let small = crate::ui::styles::typography_presets::Small::new();
        assert!(small.size > 0.0);
        assert!(small.weight >= 100 && small.weight <= 900);
        assert!(small.line_height_ratio > 0.0);
    }

    #[test]
    fn test_preset_caption_returns_valid_values() {
        let caption = crate::ui::styles::typography_presets::Caption::new();
        assert!(caption.size > 0.0);
        assert!(caption.weight >= 100 && caption.weight <= 900);
        assert!(caption.line_height_ratio > 0.0);
    }

    #[test]
    fn test_preset_button_returns_valid_values() {
        let button = crate::ui::styles::typography_presets::Button::new();
        assert!(button.size > 0.0);
        assert!(button.weight >= 100 && button.weight <= 900);
        assert!(button.line_height_ratio > 0.0);
    }

    #[test]
    fn test_preset_body_large_returns_valid_values() {
        let body_large = crate::ui::styles::typography_presets::BodyLarge::new();
        assert!(body_large.size > 0.0);
        assert!(body_large.weight >= 100 && body_large.weight <= 900);
        assert!(body_large.line_height_ratio > 0.0);
    }

    #[test]
    fn test_preset_compact_returns_valid_values() {
        let compact = crate::ui::styles::typography_presets::Compact::new();
        assert!(compact.size > 0.0);
        assert!(compact.weight >= 100 && compact.weight <= 900);
        assert!(compact.line_height_ratio > 0.0);
    }

    #[test]
    fn test_preset_label_returns_valid_values() {
        let label = crate::ui::styles::typography_presets::Label::new();
        assert!(label.size > 0.0);
        assert!(label.weight >= 100 && label.weight <= 900);
        assert!(label.line_height_ratio > 0.0);
    }

    #[test]
    fn test_preset_subheading_returns_valid_values() {
        let subheading = crate::ui::styles::typography_presets::Subheading::new();
        assert!(subheading.size > 0.0);
        assert!(subheading.weight >= 100 && subheading.weight <= 900);
        assert!(subheading.line_height_ratio > 0.0);
    }

    #[test]
    fn test_semantic_color_returns_valid_colors() {
        use crate::ui::state::ThemeMode;

        let light_heading = semantic_color::heading(ThemeMode::Light);
        let light_body = semantic_color::body(ThemeMode::Light);
        let light_metadata = semantic_color::metadata(ThemeMode::Light);

        assert!(light_heading.r >= 0.0 && light_heading.r <= 1.0);
        assert!(light_body.r >= 0.0 && light_body.r <= 1.0);
        assert!(light_metadata.r >= 0.0 && light_metadata.r <= 1.0);

        let dark_heading = semantic_color::heading(ThemeMode::Dark);
        let dark_body = semantic_color::body(ThemeMode::Dark);

        assert!(dark_heading.r >= 0.0 && dark_heading.r <= 1.0);
        assert!(dark_body.r >= 0.0 && dark_body.r <= 1.0);
    }

    #[test]
    fn test_preset_color_methods_work() {
        use crate::ui::state::ThemeMode;

        let display = crate::ui::styles::typography_presets::Display::new();
        let _color = display.color(ThemeMode::Light);

        let heading = crate::ui::styles::typography_presets::Heading::new();
        let _color = heading.color(ThemeMode::Dark);

        let body = crate::ui::styles::typography_presets::Body::new();
        let _color = body.color(ThemeMode::Light);

        let small = crate::ui::styles::typography_presets::Small::new();
        let _color = small.color(ThemeMode::Dark);

        let caption = crate::ui::styles::typography_presets::Caption::new();
        let _color = caption.color(ThemeMode::Light);

        let button = crate::ui::styles::typography_presets::Button::new();
        let _color = button.color(ThemeMode::Dark);
    }
}
