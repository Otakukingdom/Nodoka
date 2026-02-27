//! Typography presets combining size, weight, and color for consistent styling.
//!
//! This module provides simplified API for common text styling needs
//! while maintaining the full typography system underneath.

use crate::ui::state::ThemeMode;
use crate::ui::styles::typography::{
    letter_spacing, line_height, semantic_color, type_scale, weight, SIZE_DISPLAY_PREMIUM,
    SIZE_PREMIUM, SIZE_XXL,
};

/// Display text preset for hero sections
pub struct Display {
    pub size: f32,
    pub weight: u16,
    pub line_height_ratio: f32,
    pub letter_spacing: f32,
}

impl Default for Display {
    fn default() -> Self {
        Self::new()
    }
}

impl Display {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            size: type_scale::DISPLAY_SIZE,
            weight: type_scale::DISPLAY_WEIGHT,
            line_height_ratio: line_height::TIGHT,
            letter_spacing: letter_spacing::SLIGHT,
        }
    }

    #[must_use]
    pub const fn color(self, theme: ThemeMode) -> iced::Color {
        semantic_color::heading(theme)
    }
}

/// Heading preset for titles
pub struct Heading {
    pub size: f32,
    pub weight: u16,
    pub line_height_ratio: f32,
    pub letter_spacing: f32,
}

impl Default for Heading {
    fn default() -> Self {
        Self::new()
    }
}

impl Heading {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            size: type_scale::H1_SIZE,
            weight: type_scale::H1_WEIGHT,
            line_height_ratio: line_height::TIGHT,
            letter_spacing: letter_spacing::SLIGHT,
        }
    }

    #[must_use]
    pub const fn level_2() -> Self {
        Self {
            size: type_scale::H2_SIZE,
            weight: type_scale::H2_WEIGHT,
            line_height_ratio: line_height::TIGHT,
            letter_spacing: letter_spacing::SLIGHT,
        }
    }

    #[must_use]
    pub const fn level_3() -> Self {
        Self {
            size: type_scale::H3_SIZE,
            weight: type_scale::H3_WEIGHT,
            line_height_ratio: line_height::NORMAL,
            letter_spacing: letter_spacing::NONE,
        }
    }

    #[must_use]
    pub const fn color(self, theme: ThemeMode) -> iced::Color {
        semantic_color::heading(theme)
    }
}

/// Body text preset for content
pub struct Body {
    pub size: f32,
    pub weight: u16,
    pub line_height_ratio: f32,
    pub letter_spacing: f32,
}

impl Default for Body {
    fn default() -> Self {
        Self::new()
    }
}

impl Body {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            size: type_scale::BODY_SIZE,
            weight: type_scale::BODY_WEIGHT,
            line_height_ratio: line_height::RELAXED,
            letter_spacing: letter_spacing::NONE,
        }
    }

    #[must_use]
    pub const fn medium() -> Self {
        Self {
            size: type_scale::BODY_SIZE,
            weight: weight::MEDIUM,
            line_height_ratio: line_height::NORMAL,
            letter_spacing: letter_spacing::NONE,
        }
    }

    #[must_use]
    pub const fn color(self, theme: ThemeMode) -> iced::Color {
        semantic_color::body(theme)
    }
}

/// Small text preset for metadata
pub struct Small {
    pub size: f32,
    pub weight: u16,
    pub line_height_ratio: f32,
    pub letter_spacing: f32,
}

impl Default for Small {
    fn default() -> Self {
        Self::new()
    }
}

impl Small {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            size: type_scale::SMALL_SIZE,
            weight: type_scale::SMALL_WEIGHT,
            line_height_ratio: line_height::NORMAL,
            letter_spacing: letter_spacing::NONE,
        }
    }

    #[must_use]
    pub const fn semibold() -> Self {
        Self {
            size: type_scale::SMALL_SIZE,
            weight: weight::SEMIBOLD,
            line_height_ratio: line_height::NORMAL,
            letter_spacing: letter_spacing::NONE,
        }
    }

    #[must_use]
    pub const fn color(self, theme: ThemeMode) -> iced::Color {
        semantic_color::metadata(theme)
    }
}

/// Caption text preset for tiny labels
pub struct Caption {
    pub size: f32,
    pub weight: u16,
    pub line_height_ratio: f32,
    pub letter_spacing: f32,
}

impl Default for Caption {
    fn default() -> Self {
        Self::new()
    }
}

impl Caption {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            size: type_scale::CAPTION_SIZE,
            weight: type_scale::CAPTION_WEIGHT,
            line_height_ratio: line_height::DENSE,
            letter_spacing: letter_spacing::NONE,
        }
    }

    #[must_use]
    pub const fn uppercase() -> Self {
        Self {
            size: type_scale::CAPTION_SIZE,
            weight: weight::MEDIUM,
            line_height_ratio: line_height::DENSE,
            letter_spacing: letter_spacing::WIDE,
        }
    }

    #[must_use]
    pub const fn color(self, theme: ThemeMode) -> iced::Color {
        semantic_color::metadata(theme)
    }
}

/// ===== Premium Heading Presets =====
///
/// Premium heading preset with distinctive character
/// Creates memorable typography with tighter letter-spacing and refined line-height
pub struct PremiumHeading {
    pub size: f32,
    pub weight: u16,
    pub line_height_ratio: f32,
    pub letter_spacing: f32,
}

impl Default for PremiumHeading {
    fn default() -> Self {
        Self::new()
    }
}

impl PremiumHeading {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            size: SIZE_PREMIUM,
            weight: weight::SEMIBOLD,
            line_height_ratio: line_height::TIGHT,
            letter_spacing: -0.015, // Tighter for premium editorial look
        }
    }

    #[must_use]
    pub const fn display() -> Self {
        Self {
            size: SIZE_DISPLAY_PREMIUM,
            weight: weight::SEMIBOLD,
            line_height_ratio: line_height::TIGHT,
            letter_spacing: -0.02,
        }
    }

    #[must_use]
    pub const fn grand() -> Self {
        Self {
            size: type_scale::GRAND_SIZE,
            weight: weight::BOLD,
            line_height_ratio: line_height::TIGHT,
            letter_spacing: -0.015,
        }
    }

    /// Brand heading preset with distinctive character for brand-critical elements
    #[must_use]
    pub const fn brand() -> Self {
        Self {
            size: SIZE_XXL,
            weight: weight::SEMIBOLD,
            line_height_ratio: line_height::TIGHT,
            letter_spacing: -0.01,
        }
    }

    #[must_use]
    pub const fn color(self, theme: ThemeMode) -> iced::Color {
        semantic_color::heading(theme)
    }
}

/// ===== Editorial Heading Presets =====
///
/// Editorial heading preset for sophisticated, magazine-style typography
/// Creates distinctive character through tighter letter-spacing and refined weights
pub struct EditorialHeading {
    pub size: f32,
    pub weight: u16,
    pub line_height_ratio: f32,
    pub letter_spacing: f32,
}

impl Default for EditorialHeading {
    fn default() -> Self {
        Self::new()
    }
}

impl EditorialHeading {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            size: SIZE_PREMIUM,
            weight: weight::SEMIBOLD,
            line_height_ratio: line_height::EXTRA_TIGHT,
            letter_spacing: -0.025, // Extra tight for editorial feel
        }
    }

    #[must_use]
    pub const fn display() -> Self {
        Self {
            size: SIZE_DISPLAY_PREMIUM,
            weight: weight::BOLD,
            line_height_ratio: line_height::EXTRA_TIGHT,
            letter_spacing: -0.03,
        }
    }

    #[must_use]
    pub const fn compact() -> Self {
        Self {
            size: type_scale::H2_SIZE,
            weight: weight::SEMIBOLD,
            line_height_ratio: line_height::TIGHT,
            letter_spacing: -0.02,
        }
    }

    #[must_use]
    pub const fn color(self, theme: ThemeMode) -> iced::Color {
        semantic_color::heading(theme)
    }
}

/// ===== Distinctive Heading Presets =====
///
/// Distinctive heading preset with unique character for premium branding.
/// Creates memorable typography through tighter letter-spacing for a refined,
/// sophisticated appearance that sets the design apart from generic applications.
///
/// The negative letter-spacing creates a distinctive premium character commonly
/// found in high-end editorial design and luxury brand typography.
pub struct DistinctiveHeading {
    pub size: f32,
    pub weight: u16,
    pub line_height_ratio: f32,
    pub letter_spacing: f32,
}

impl Default for DistinctiveHeading {
    fn default() -> Self {
        Self::new()
    }
}

impl DistinctiveHeading {
    /// Creates a new distinctive heading with standard H1 size
    ///
    /// Uses tighter letter-spacing (-0.02em) for distinctive premium character.
    /// This creates a more refined, condensed appearance compared to standard headings.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            size: type_scale::H1_SIZE,
            weight: weight::SEMIBOLD,
            line_height_ratio: line_height::TIGHT,
            letter_spacing: -0.02, // Tighter for distinctive look
        }
    }

    /// Compact variant for secondary headings
    ///
    /// Slightly smaller with proportionally adjusted letter-spacing.
    #[must_use]
    pub const fn compact() -> Self {
        Self {
            size: type_scale::H2_SIZE,
            weight: weight::SEMIBOLD,
            line_height_ratio: line_height::TIGHT,
            letter_spacing: -0.015,
        }
    }

    /// Small variant for tertiary headings
    ///
    /// Maintains distinctive character at smaller sizes.
    #[must_use]
    pub const fn small() -> Self {
        Self {
            size: type_scale::H3_SIZE,
            weight: weight::MEDIUM,
            line_height_ratio: line_height::TIGHT,
            letter_spacing: -0.01,
        }
    }

    /// Extra tight variant for maximum distinctive character
    ///
    /// Uses the tightest letter-spacing for dramatic, editorial-style headings.
    /// Best reserved for hero sections and feature titles.
    #[must_use]
    pub const fn extra_tight() -> Self {
        Self {
            size: type_scale::H1_SIZE,
            weight: weight::BOLD,
            line_height_ratio: line_height::TIGHT,
            letter_spacing: -0.03,
        }
    }

    /// Returns the appropriate text color for this heading based on theme
    #[must_use]
    pub const fn color(self, theme: ThemeMode) -> iced::Color {
        semantic_color::heading(theme)
    }
}

/// Button text preset with slightly heavier weight
pub struct Button {
    pub size: f32,
    pub weight: u16,
    pub line_height_ratio: f32,
    pub letter_spacing: f32,
}

impl Default for Button {
    fn default() -> Self {
        Self::new()
    }
}

impl Button {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            size: type_scale::BUTTON_SIZE,
            weight: weight::SEMIBOLD,
            line_height_ratio: line_height::DENSE,
            letter_spacing: letter_spacing::NONE,
        }
    }

    #[must_use]
    pub const fn semibold() -> Self {
        Self {
            size: type_scale::BUTTON_SIZE,
            weight: weight::SEMIBOLD,
            line_height_ratio: line_height::DENSE,
            letter_spacing: letter_spacing::NONE,
        }
    }

    #[must_use]
    pub const fn color(self, _theme: ThemeMode) -> iced::Color {
        // Button text color is typically handled by button styles
        iced::Color::WHITE
    }
}

/// Body large text preset for emphasized content
pub struct BodyLarge {
    pub size: f32,
    pub weight: u16,
    pub line_height_ratio: f32,
    pub letter_spacing: f32,
}

impl Default for BodyLarge {
    fn default() -> Self {
        Self::new()
    }
}

impl BodyLarge {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            size: type_scale::BODY_LARGE_SIZE,
            weight: type_scale::BODY_LARGE_WEIGHT,
            line_height_ratio: line_height::RELAXED,
            letter_spacing: letter_spacing::NONE,
        }
    }

    #[must_use]
    pub const fn medium() -> Self {
        Self {
            size: type_scale::BODY_LARGE_SIZE,
            weight: weight::MEDIUM,
            line_height_ratio: line_height::RELAXED,
            letter_spacing: letter_spacing::NONE,
        }
    }

    #[must_use]
    pub const fn color(self, theme: ThemeMode) -> iced::Color {
        semantic_color::body(theme)
    }
}

/// Compact text preset for dense UI elements
pub struct Compact {
    pub size: f32,
    pub weight: u16,
    pub line_height_ratio: f32,
    pub letter_spacing: f32,
}

impl Default for Compact {
    fn default() -> Self {
        Self::new()
    }
}

impl Compact {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            size: type_scale::SMALL_SIZE,
            weight: weight::MEDIUM,
            line_height_ratio: line_height::DENSE,
            letter_spacing: letter_spacing::NONE,
        }
    }

    #[must_use]
    pub const fn color(self, theme: ThemeMode) -> iced::Color {
        semantic_color::body(theme)
    }
}

/// Label text preset for form labels and UI labels
pub struct Label {
    pub size: f32,
    pub weight: u16,
    pub line_height_ratio: f32,
    pub letter_spacing: f32,
}

impl Default for Label {
    fn default() -> Self {
        Self::new()
    }
}

impl Label {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            size: type_scale::SMALL_SIZE,
            weight: weight::SEMIBOLD,
            line_height_ratio: line_height::DENSE,
            letter_spacing: letter_spacing::SLIGHT,
        }
    }

    #[must_use]
    pub const fn uppercase() -> Self {
        Self {
            size: type_scale::CAPTION_SIZE,
            weight: weight::SEMIBOLD,
            line_height_ratio: line_height::DENSE,
            letter_spacing: letter_spacing::WIDE,
        }
    }

    #[must_use]
    pub const fn color(self, theme: ThemeMode) -> iced::Color {
        semantic_color::metadata(theme)
    }
}

/// Subheading text preset for content sections
pub struct Subheading {
    pub size: f32,
    pub weight: u16,
    pub line_height_ratio: f32,
    pub letter_spacing: f32,
}

impl Default for Subheading {
    fn default() -> Self {
        Self::new()
    }
}

impl Subheading {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            size: type_scale::H4_SIZE,
            weight: type_scale::H4_WEIGHT,
            line_height_ratio: line_height::TIGHT,
            letter_spacing: letter_spacing::SLIGHT_TIGHT,
        }
    }

    #[must_use]
    pub const fn color(self, theme: ThemeMode) -> iced::Color {
        semantic_color::heading(theme)
    }
}
