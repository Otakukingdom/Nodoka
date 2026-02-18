//! Layout constants for spacing, typography, and border radius.
//!
//! This module provides consistent design tokens for layout and typography
//! following a 4px base grid system.

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
/// iced 0.14 requires f32 for Pixels type
pub mod typography {
    /// Extra small text: 11px
    pub const SIZE_XS: f32 = 11.0;
    /// Small text: 13px
    pub const SIZE_SM: f32 = 13.0;
    /// Base text size: 14px
    pub const SIZE_BASE: f32 = 14.0;
    /// Large text: 16px
    pub const SIZE_LG: f32 = 16.0;
    /// Extra large text: 20px
    pub const SIZE_XL: f32 = 20.0;
    /// Extra extra large text: 24px
    pub const SIZE_XXL: f32 = 24.0;
    /// Heading text: 32px
    pub const SIZE_HEADING: f32 = 32.0;
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
