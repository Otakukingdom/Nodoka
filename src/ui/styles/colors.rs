//! Color palettes for Nodoka audiobook player design system.
//!
//! This module provides two color palettes optimized for audiobook players:
//! - **Vibrant Rose**: Energetic primary palette with high engagement
//! - **Calm Cyan**: Alternative palette for reduced eye strain during extended sessions
//!
//! All colors are tested for WCAG AA accessibility compliance.

use iced::Color;

/// Primary brand color – vibrant rose `#E11D48`.
pub const PRIMARY: Color = Color::from_rgb(0.882, 0.114, 0.282);
/// Primary brand color on hover – slightly darker rose.
pub const PRIMARY_HOVER: Color = Color::from_rgb(0.753, 0.098, 0.239);
/// Primary brand color when pressed/active – darkest rose.
pub const PRIMARY_ACTIVE: Color = Color::from_rgb(0.639, 0.082, 0.200);
/// Secondary accent – soft rose `#FB7185`.
pub const SECONDARY: Color = Color::from_rgb(0.984, 0.443, 0.522);
/// Call-to-action accent – engagement blue `#2563EB`.
pub const ACCENT: Color = Color::from_rgb(0.149, 0.388, 0.922);

/// Semantic success color – forest green `#228B22`.
pub const SUCCESS: Color = Color::from_rgb(0.133, 0.545, 0.133);
/// Semantic warning color – warm sandy orange `#F4A460`.
pub const WARNING: Color = Color::from_rgb(0.957, 0.643, 0.376);
/// Semantic error color – vivid red `#DC2626`.
pub const ERROR: Color = Color::from_rgb(0.863, 0.149, 0.149);
/// Error color on hover – darker red `#B91C1C`.
pub const ERROR_HOVER: Color = Color::from_rgb(0.725, 0.110, 0.110);
/// Error color when pressed – darkest red `#991B1B`.
pub const ERROR_ACTIVE: Color = Color::from_rgb(0.600, 0.106, 0.106);
/// Semantic informational color – blue, same hue as accent.
pub const INFO: Color = Color::from_rgb(0.149, 0.388, 0.922);

/// Primary page background – very light rose tint `#FFF1F2`.
pub const BG_PRIMARY: Color = Color::from_rgb(1.0, 0.945, 0.949);
/// Secondary content background – pure white.
pub const BG_SECONDARY: Color = Color::from_rgb(1.0, 1.0, 1.0);
/// Elevated surface background – slight gray lift.
pub const BG_ELEVATED: Color = Color::from_rgb(0.98, 0.98, 0.98);
/// Background color on hover.
pub const BG_HOVER: Color = Color::from_rgb(0.97, 0.97, 0.97);

/// Primary text – dark rose-gray `#730F2E` for WCAG AA contrast.
pub const TEXT_PRIMARY: Color = Color::from_rgb(0.45, 0.06, 0.18);
/// Secondary text – medium gray `#666666`.
pub const TEXT_SECONDARY: Color = Color::from_rgb(0.4, 0.4, 0.4);
/// Disabled text – light gray `#999999`.
pub const TEXT_DISABLED: Color = Color::from_rgb(0.6, 0.6, 0.6);
/// White text for use on primary-color backgrounds (WCAG AA contrast).
pub const TEXT_ON_PRIMARY: Color = Color::from_rgb(1.0, 1.0, 1.0);

/// Semi-transparent dark overlay applied on hover (5 % opacity).
pub const HOVER_OVERLAY: Color = Color::from_rgba(0.0, 0.0, 0.0, 0.05);
/// Semi-transparent dark overlay applied when active/pressed (10 % opacity).
pub const ACTIVE_OVERLAY: Color = Color::from_rgba(0.0, 0.0, 0.0, 0.1);
/// Keyboard-focus ring color – accessible blue `#2563EB`.
pub const FOCUS_RING: Color = Color::from_rgb(0.149, 0.388, 0.922);
/// Background used for selected list items.
pub const SELECTION_BG: Color = Color::from_rgb(0.882, 0.114, 0.282);
/// Text color on selected list items – white for WCAG AA contrast.
pub const SELECTION_TEXT: Color = TEXT_ON_PRIMARY;

/// Default border color – light gray `#E5E5E5`.
pub const BORDER_DEFAULT: Color = Color::from_rgb(0.9, 0.9, 0.9);
/// Border color when an element has keyboard focus.
pub const BORDER_FOCUS: Color = Color::from_rgb(0.149, 0.388, 0.922);
/// Border color when an element is in an error state.
pub const BORDER_ERROR: Color = ERROR;

/// Legacy top-bar color – golden yellow `#FEDB53` (superseded by design system colors).
pub const TOP_BAR_COLOR: Color = Color::from_rgb(0.996, 0.855, 0.325);
/// Legacy player background (alias for [`BG_ELEVATED`]).
pub const PLAYER_BG_COLOR: Color = BG_ELEVATED;
/// Legacy player text color (alias for [`TEXT_PRIMARY`]).
pub const PLAYER_TEXT_COLOR: Color = TEXT_PRIMARY;
/// Legacy audiobook list background (alias for [`BG_PRIMARY`]).
pub const AUDIOBOOK_LIST_BG: Color = BG_PRIMARY;
/// Legacy file list background (alias for [`BG_SECONDARY`]).
pub const FILE_LIST_BG: Color = BG_SECONDARY;

/// Audiobook-optimized color palette (calm cyan + health green)
///
/// Based on ui-ux-pro-max recommendations for extended reading sessions.
/// This alternative palette uses cooler tones to reduce eye strain during
/// long audiobook listening sessions.
pub mod colors_audiobook {
    use iced::Color;

    // Primary colors (calm cyan palette)
    /// Primary brand color - calm cyan (#0E7490) - darker for better contrast
    pub const PRIMARY: Color = Color::from_rgb(0.055, 0.455, 0.565);
    /// Primary hover state - darker cyan for feedback
    pub const PRIMARY_HOVER: Color = Color::from_rgb(0.045, 0.380, 0.475);
    /// Primary active state - darkest cyan for pressed
    pub const PRIMARY_ACTIVE: Color = Color::from_rgb(0.035, 0.310, 0.390);
    /// Secondary accent - soft cyan (#22D3EE)
    pub const SECONDARY: Color = Color::from_rgb(0.133, 0.827, 0.933);
    /// Call-to-action accent - health green (#059669)
    pub const ACCENT: Color = Color::from_rgb(0.020, 0.588, 0.412);

    /// Semantic success color – forest green `#228B22`.
    pub const SUCCESS: Color = Color::from_rgb(0.133, 0.545, 0.133);
    /// Semantic warning color – warm sandy orange `#F4A460`.
    pub const WARNING: Color = Color::from_rgb(0.957, 0.643, 0.376);
    /// Semantic error color – vivid red `#DC2626`.
    pub const ERROR: Color = Color::from_rgb(0.863, 0.149, 0.149);
    /// Error color on hover – darker red `#B91C1C`.
    pub const ERROR_HOVER: Color = Color::from_rgb(0.725, 0.110, 0.110);
    /// Error color when pressed – darkest red `#991B1B`.
    pub const ERROR_ACTIVE: Color = Color::from_rgb(0.600, 0.106, 0.106);
    /// Informational color – soft cyan, same hue as secondary.
    pub const INFO: Color = Color::from_rgb(0.133, 0.827, 0.933);

    // Background colors with cyan tint for reduced eye strain
    /// Primary background - soft cyan tint (#ECFEFF)
    pub const BG_PRIMARY: Color = Color::from_rgb(0.925, 0.996, 1.0);
    /// Secondary background - pure white for contrast
    pub const BG_SECONDARY: Color = Color::from_rgb(1.0, 1.0, 1.0);
    /// Elevated surface background - slight cyan tint
    pub const BG_ELEVATED: Color = Color::from_rgb(0.95, 0.99, 1.0);
    /// Hover background state
    pub const BG_HOVER: Color = Color::from_rgb(0.94, 0.98, 0.99);

    // Text colors (proper contrast for accessibility)
    /// Primary text - dark cyan-gray for readability
    pub const TEXT_PRIMARY: Color = Color::from_rgb(0.06, 0.35, 0.42); // Dark cyan
    /// Secondary text - medium gray
    pub const TEXT_SECONDARY: Color = Color::from_rgb(0.4, 0.4, 0.4);
    /// Disabled text - light gray
    pub const TEXT_DISABLED: Color = Color::from_rgb(0.6, 0.6, 0.6);
    /// Text on primary color buttons - white for contrast
    pub const TEXT_ON_PRIMARY: Color = Color::from_rgb(1.0, 1.0, 1.0);

    /// Semi-transparent dark overlay on hover (5 % opacity).
    pub const HOVER_OVERLAY: Color = Color::from_rgba(0.0, 0.0, 0.0, 0.05);
    /// Semi-transparent dark overlay when active/pressed (10 % opacity).
    pub const ACTIVE_OVERLAY: Color = Color::from_rgba(0.0, 0.0, 0.0, 0.1);
    /// Focus ring - health green for accessibility
    pub const FOCUS_RING: Color = Color::from_rgb(0.020, 0.588, 0.412);
    /// Selection background - calm cyan primary
    pub const SELECTION_BG: Color = PRIMARY;
    /// Selection text - white for WCAG AA contrast
    pub const SELECTION_TEXT: Color = TEXT_ON_PRIMARY;

    /// Default border color – light gray `#E5E5E5`.
    pub const BORDER_DEFAULT: Color = Color::from_rgb(0.9, 0.9, 0.9);
    /// Border color for focused elements.
    pub const BORDER_FOCUS: Color = FOCUS_RING;
    /// Border color for elements in an error state.
    pub const BORDER_ERROR: Color = ERROR;
}
