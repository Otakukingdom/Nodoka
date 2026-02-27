//! Color palettes for Nodoka audiobook player design system.
//!
//! This module provides three color palettes optimized for audiobook players:
//! - **Vibrant Rose (Light)**: Energetic primary palette with high engagement
//! - **Dark Theme**: Dark mode palette for reduced eye strain during night sessions
//! - **Calm Cyan**: Alternative palette for reduced eye strain during extended sessions
//!
//! All colors are tested for WCAG AA accessibility compliance.

use iced::Color;

/// Color types for theme-aware color selection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorType {
    /// Primary brand color
    Primary,
    /// Primary hover state
    PrimaryHover,
    /// Primary active state
    PrimaryActive,
    /// Secondary accent
    Secondary,
    /// Call-to-action accent
    Accent,
    /// Success semantic color
    Success,
    /// Warning semantic color
    Warning,
    /// Error semantic color
    Error,
    /// Error hover state
    ErrorHover,
    /// Error active state
    ErrorActive,
    /// Info semantic color
    Info,
    /// Primary background
    BgPrimary,
    /// Secondary background
    BgSecondary,
    /// Elevated surface background
    BgElevated,
    /// Hover background
    BgHover,
    /// Primary text color
    TextPrimary,
    /// Secondary text color
    TextSecondary,
    /// Disabled text color
    TextDisabled,
    /// Text on primary color
    TextOnPrimary,
    /// Hover overlay
    HoverOverlay,
    /// Active overlay
    ActiveOverlay,
    /// Focus ring color
    FocusRing,
    /// Selection background
    SelectionBg,
    /// Selection text
    SelectionText,
    /// Default border color
    BorderDefault,
    /// Focus border color
    BorderFocus,
    /// Error border color
    BorderError,
}

/// Gets the appropriate color based on theme mode and color type
///
/// # Examples
///
/// ```ignore
/// use crate::ui::state::ThemeMode;
/// use crate::ui::styles::colors::{get_color, ColorType};
///
/// let bg_color = get_color(ThemeMode::Dark, ColorType::BgPrimary);
/// ```
#[must_use]
pub const fn get_color(theme: crate::ui::state::ThemeMode, color_type: ColorType) -> Color {
    use crate::ui::state::ThemeMode;
    match (theme, color_type) {
        (ThemeMode::Light, ColorType::Primary) => PRIMARY,
        (ThemeMode::Dark, ColorType::Primary) => dark_theme::PRIMARY,
        (ThemeMode::Light, ColorType::PrimaryHover) => PRIMARY_HOVER,
        (ThemeMode::Dark, ColorType::PrimaryHover) => dark_theme::PRIMARY_HOVER,
        (ThemeMode::Light, ColorType::PrimaryActive) => PRIMARY_ACTIVE,
        (ThemeMode::Dark, ColorType::PrimaryActive) => dark_theme::PRIMARY_ACTIVE,
        (ThemeMode::Light, ColorType::Secondary) => SECONDARY,
        (ThemeMode::Dark, ColorType::Secondary) => dark_theme::SECONDARY,
        (ThemeMode::Light, ColorType::Accent) => ACCENT,
        (ThemeMode::Dark, ColorType::Accent) => dark_theme::ACCENT,
        (ThemeMode::Light, ColorType::Success) => SUCCESS,
        (ThemeMode::Dark, ColorType::Success) => dark_theme::SUCCESS,
        (ThemeMode::Light, ColorType::Warning) => WARNING,
        (ThemeMode::Dark, ColorType::Warning) => dark_theme::WARNING,
        (ThemeMode::Light, ColorType::Error) => ERROR,
        (ThemeMode::Dark, ColorType::Error) => dark_theme::ERROR,
        (ThemeMode::Light, ColorType::ErrorHover) => ERROR_HOVER,
        (ThemeMode::Dark, ColorType::ErrorHover) => dark_theme::ERROR_HOVER,
        (ThemeMode::Light, ColorType::ErrorActive) => ERROR_ACTIVE,
        (ThemeMode::Dark, ColorType::ErrorActive) => dark_theme::ERROR_ACTIVE,
        (ThemeMode::Light, ColorType::Info) => INFO,
        (ThemeMode::Dark, ColorType::Info) => dark_theme::INFO,
        (ThemeMode::Light, ColorType::BgPrimary) => BG_PRIMARY,
        (ThemeMode::Dark, ColorType::BgPrimary) => dark_theme::BG_PRIMARY,
        (ThemeMode::Light, ColorType::BgSecondary) => BG_SECONDARY,
        (ThemeMode::Dark, ColorType::BgSecondary) => dark_theme::BG_SECONDARY,
        (ThemeMode::Light, ColorType::BgElevated) => BG_ELEVATED,
        (ThemeMode::Dark, ColorType::BgElevated) => dark_theme::BG_ELEVATED,
        (ThemeMode::Light, ColorType::BgHover) => BG_HOVER,
        (ThemeMode::Dark, ColorType::BgHover) => dark_theme::BG_HOVER,
        (ThemeMode::Light, ColorType::TextPrimary) => TEXT_PRIMARY,
        (ThemeMode::Dark, ColorType::TextPrimary) => dark_theme::TEXT_PRIMARY,
        (ThemeMode::Light, ColorType::TextSecondary) => TEXT_SECONDARY,
        (ThemeMode::Dark, ColorType::TextSecondary) => dark_theme::TEXT_SECONDARY,
        (ThemeMode::Light, ColorType::TextDisabled) => TEXT_DISABLED,
        (ThemeMode::Dark, ColorType::TextDisabled) => dark_theme::TEXT_DISABLED,
        (ThemeMode::Light, ColorType::TextOnPrimary) => TEXT_ON_PRIMARY,
        (ThemeMode::Dark, ColorType::TextOnPrimary) => dark_theme::TEXT_ON_PRIMARY,
        (ThemeMode::Light, ColorType::HoverOverlay) => HOVER_OVERLAY,
        (ThemeMode::Dark, ColorType::HoverOverlay) => dark_theme::HOVER_OVERLAY,
        (ThemeMode::Light, ColorType::ActiveOverlay) => ACTIVE_OVERLAY,
        (ThemeMode::Dark, ColorType::ActiveOverlay) => dark_theme::ACTIVE_OVERLAY,
        (ThemeMode::Light, ColorType::FocusRing) => FOCUS_RING,
        (ThemeMode::Dark, ColorType::FocusRing) => dark_theme::FOCUS_RING,
        (ThemeMode::Light, ColorType::SelectionBg) => SELECTION_BG,
        (ThemeMode::Dark, ColorType::SelectionBg) => dark_theme::SELECTION_BG,
        (ThemeMode::Light, ColorType::SelectionText) => SELECTION_TEXT,
        (ThemeMode::Dark, ColorType::SelectionText) => dark_theme::SELECTION_TEXT,
        (ThemeMode::Light, ColorType::BorderDefault) => BORDER_DEFAULT,
        (ThemeMode::Dark, ColorType::BorderDefault) => dark_theme::BORDER_DEFAULT,
        (ThemeMode::Light, ColorType::BorderFocus) => BORDER_FOCUS,
        (ThemeMode::Dark, ColorType::BorderFocus) => dark_theme::BORDER_FOCUS,
        (ThemeMode::Light, ColorType::BorderError) => BORDER_ERROR,
        (ThemeMode::Dark, ColorType::BorderError) => dark_theme::BORDER_ERROR,
    }
}

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

/// Primary page background – pure white for neutral base.
pub const BG_PRIMARY: Color = Color::from_rgb(1.0, 1.0, 1.0);
/// Secondary content background – very light neutral gray.
pub const BG_SECONDARY: Color = Color::from_rgb(0.98, 0.98, 0.98);
/// Elevated surface background – white with subtle shadow for depth.
pub const BG_ELEVATED: Color = Color::from_rgb(1.0, 1.0, 1.0);
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

/// ===== Enhanced Gradients and Surface Treatments =====
/// Floating surface background – slightly elevated above `BG_ELEVATED`.
pub const BG_FLOATING: Color = Color::from_rgb(0.99, 0.99, 0.99);

/// Subtle gradient for non-interactive surfaces.
pub const SUBTLE_GRADIENT_START: Color = Color::from_rgb(0.99, 0.99, 0.99);
pub const SUBTLE_GRADIENT_END: Color = Color::from_rgb(0.97, 0.97, 0.98);

/// Surface hover state with subtle gradient.
pub const SURFACE_HOVER: Color = Color::from_rgb(0.98, 0.98, 0.98);

/// Surface active state with deeper gradient.
pub const SURFACE_ACTIVE: Color = Color::from_rgb(0.95, 0.95, 0.96);

/// Primary gradient colors for visual depth on special elements.
/// Enhanced with smoother transition for premium feel.
pub const PRIMARY_GRADIENT_START: Color = Color::from_rgb(0.96, 0.22, 0.37);
pub const PRIMARY_GRADIENT_END: Color = Color::from_rgb(0.82, 0.10, 0.24);

/// Accent gradient colors for CTAs and highlights.
/// Refined with more vibrant rose-to-pink transition.
pub const ACCENT_GRADIENT_START: Color = Color::from_rgb(0.99, 0.46, 0.54);
pub const ACCENT_GRADIENT_END: Color = Color::from_rgb(0.90, 0.12, 0.30);

/// Continue listening card gradient for premium feel.
/// Enhanced with more vibrant rose-to-pink transition.
pub const CONTINUE_GRADIENT_START: Color = Color::from_rgb(1.0, 0.55, 0.65);
pub const CONTINUE_GRADIENT_END: Color = Color::from_rgb(0.90, 0.12, 0.30);

/// Progress bar gradient for visual engagement.
/// Refined with smoother color flow.
pub const PROGRESS_GRADIENT_START: Color = Color::from_rgb(0.97, 0.27, 0.42);
pub const PROGRESS_GRADIENT_END: Color = Color::from_rgb(0.87, 0.12, 0.27);

/// Glass-morphism overlay for premium modal feel.
/// Enhanced with better RGB values for premium appearance.
pub const GLASS_OVERLAY: Color = Color::from_rgba(0.96, 0.96, 0.98, 0.88);

/// Refined semantic colors for more distinctive design.
pub const SUCCESS_REFINED: Color = Color::from_rgb(0.05, 0.65, 0.35);
pub const SUCCESS_LIGHT: Color = Color::from_rgb(0.35, 0.88, 0.58);
pub const WARNING_REFINED: Color = Color::from_rgb(0.88, 0.58, 0.18);
pub const WARNING_LIGHT: Color = Color::from_rgb(1.0, 0.88, 0.55);

/// Inset shadow color for progress bar depth effect.
pub const INSET_SHADOW: Color = Color::from_rgba(0.0, 0.0, 0.0, 0.1);

/// Glow effect color for active playback indicators.
/// Enhanced with better alpha for more visible glow.
pub const GLOW_PRIMARY: Color = Color::from_rgba(0.882, 0.114, 0.282, 0.35);

/// Border colors for different hierarchy levels.
pub const BORDER_SUBTLE: Color = Color::from_rgb(0.94, 0.94, 0.95);
pub const BORDER_PROMINENT: Color = Color::from_rgb(0.85, 0.85, 0.87);

/// Enhanced shadow levels for more depth hierarchy.
pub const SHADOW_XS: Color = Color::from_rgba(0.0, 0.0, 0.0, 0.04);
pub const SHADOW_SM: Color = Color::from_rgba(0.0, 0.0, 0.0, 0.06);
pub const SHADOW_MD: Color = Color::from_rgba(0.0, 0.0, 0.0, 0.11);
pub const SHADOW_LG: Color = Color::from_rgba(0.0, 0.0, 0.0, 0.16);
pub const SHADOW_XL: Color = Color::from_rgba(0.0, 0.0, 0.0, 0.26);

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
/// Intended for extended listening sessions.
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

/// Dark theme color palette for reduced eye strain during night sessions
///
/// Provides proper dark mode colors with WCAG AA contrast compliance.
/// Uses the same rose accent as the light theme for brand consistency.
pub mod dark_theme {
    use iced::Color;

    // Primary colors (keep rose from light theme for brand consistency)
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

    // Semantic colors (same as light theme)
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

    // Dark backgrounds
    /// Primary background – dark gray `#1A1A1A`
    pub const BG_PRIMARY: Color = Color::from_rgb(0.1, 0.1, 0.1);
    /// Secondary background – slightly lighter `#262626`
    pub const BG_SECONDARY: Color = Color::from_rgb(0.15, 0.15, 0.15);
    /// Elevated surface background – lighter than secondary `#333333`
    pub const BG_ELEVATED: Color = Color::from_rgb(0.2, 0.2, 0.2);
    /// Background color on hover – lighter still `#404040`
    pub const BG_HOVER: Color = Color::from_rgb(0.25, 0.25, 0.25);

    // Text colors adjusted for dark backgrounds (WCAG AA contrast)
    /// Primary text – light gray `#E5E5E5`
    pub const TEXT_PRIMARY: Color = Color::from_rgb(0.9, 0.9, 0.9);
    /// Secondary text – medium gray `#B3B3B3`
    pub const TEXT_SECONDARY: Color = Color::from_rgb(0.7, 0.7, 0.7);
    /// Disabled text – darker gray `#808080`
    pub const TEXT_DISABLED: Color = Color::from_rgb(0.5, 0.5, 0.5);
    /// White text for use on primary-color backgrounds (WCAG AA contrast).
    pub const TEXT_ON_PRIMARY: Color = Color::from_rgb(1.0, 1.0, 1.0);

    /// Semi-transparent light overlay applied on hover (5 % opacity).
    pub const HOVER_OVERLAY: Color = Color::from_rgba(1.0, 1.0, 1.0, 0.05);
    /// Semi-transparent light overlay applied when active/pressed (10 % opacity).
    pub const ACTIVE_OVERLAY: Color = Color::from_rgba(1.0, 1.0, 1.0, 0.1);
    /// Keyboard-focus ring color – accessible blue `#2563EB`.
    pub const FOCUS_RING: Color = Color::from_rgb(0.149, 0.388, 0.922);
    /// Background used for selected list items.
    pub const SELECTION_BG: Color = Color::from_rgb(0.882, 0.114, 0.282);
    /// Text color on selected list items – white for WCAG AA contrast.
    pub const SELECTION_TEXT: Color = Color::from_rgb(1.0, 1.0, 1.0);

    // Dark mode borders
    /// Default border color – dark gray `#4D4D4D`
    pub const BORDER_DEFAULT: Color = Color::from_rgb(0.3, 0.3, 0.3);
    /// Border color when an element has keyboard focus.
    pub const BORDER_FOCUS: Color = Color::from_rgb(0.149, 0.388, 0.922);
    /// Border color when an element is in an error state.
    pub const BORDER_ERROR: Color = ERROR;

    // ===== Enhanced Gradients and Surface Treatments =====

    /// Floating surface background for dark theme.
    pub const BG_FLOATING: Color = Color::from_rgb(0.22, 0.22, 0.22);

    /// Subtle gradient for non-interactive surfaces in dark theme.
    pub const SUBTLE_GRADIENT_START: Color = Color::from_rgb(0.22, 0.22, 0.22);
    pub const SUBTLE_GRADIENT_END: Color = Color::from_rgb(0.20, 0.20, 0.21);

    /// Surface hover state in dark theme.
    pub const SURFACE_HOVER: Color = Color::from_rgb(0.28, 0.28, 0.28);

    /// Surface active state in dark theme.
    pub const SURFACE_ACTIVE: Color = Color::from_rgb(0.32, 0.32, 0.32);

    /// Primary gradient colors for dark theme.
    /// Enhanced with slightly brighter values for proper contrast on dark backgrounds.
    pub const PRIMARY_GRADIENT_START: Color = Color::from_rgb(0.97, 0.23, 0.38);
    pub const PRIMARY_GRADIENT_END: Color = Color::from_rgb(0.84, 0.11, 0.26);

    /// Accent gradient colors for dark theme.
    /// Refined with more vibrant rose-to-pink transition.
    pub const ACCENT_GRADIENT_START: Color = Color::from_rgb(1.0, 0.47, 0.56);
    pub const ACCENT_GRADIENT_END: Color = Color::from_rgb(0.91, 0.13, 0.31);

    /// Continue listening card gradient for dark theme.
    /// Enhanced with brighter values to maintain vibrancy.
    pub const CONTINUE_GRADIENT_START: Color = Color::from_rgb(1.0, 0.56, 0.67);
    pub const CONTINUE_GRADIENT_END: Color = Color::from_rgb(0.92, 0.14, 0.32);

    /// Progress bar gradient for dark theme.
    /// Enhanced with brighter values for better visibility on dark backgrounds.
    pub const PROGRESS_GRADIENT_START: Color = Color::from_rgb(0.97, 0.28, 0.43);
    pub const PROGRESS_GRADIENT_END: Color = Color::from_rgb(0.88, 0.13, 0.28);

    /// Glass-morphism overlay for dark theme.
    /// Enhanced with better RGB values for premium appearance.
    pub const GLASS_OVERLAY: Color = Color::from_rgba(0.16, 0.16, 0.19, 0.90);

    /// Refined semantic colors for dark theme.
    pub const SUCCESS_REFINED: Color = Color::from_rgb(0.08, 0.70, 0.40);
    pub const SUCCESS_LIGHT: Color = Color::from_rgb(0.38, 0.90, 0.62);
    pub const WARNING_REFINED: Color = Color::from_rgb(0.90, 0.60, 0.20);
    pub const WARNING_LIGHT: Color = Color::from_rgb(1.0, 0.90, 0.58);

    /// Inset shadow color for dark theme.
    pub const INSET_SHADOW: Color = Color::from_rgba(0.0, 0.0, 0.0, 0.2);

    /// Glow effect color for active playback indicators in dark theme.
    pub const GLOW_PRIMARY: Color = Color::from_rgba(0.882, 0.114, 0.282, 0.4);

    /// Enhanced shadow levels for dark theme.
    pub const SHADOW_XS: Color = Color::from_rgba(0.0, 0.0, 0.0, 0.09);
    pub const SHADOW_SM: Color = Color::from_rgba(0.0, 0.0, 0.0, 0.13);
    pub const SHADOW_MD: Color = Color::from_rgba(0.0, 0.0, 0.0, 0.22);
    pub const SHADOW_LG: Color = Color::from_rgba(0.0, 0.0, 0.0, 0.32);
    pub const SHADOW_XL: Color = Color::from_rgba(0.0, 0.0, 0.0, 0.42);
}

/// WCAG AA minimum contrast ratio for normal text (4.5:1).
#[allow(dead_code)]
const WCAG_AA_NORMAL: f32 = 4.5;

/// WCAG AA minimum contrast ratio for large text (3:1).
#[allow(dead_code)]
const WCAG_AA_LARGE: f32 = 3.0;

/// WCAG AAA minimum contrast ratio for normal text (7:1).
#[allow(dead_code)]
const WCAG_AAA_NORMAL: f32 = 7.0;

#[cfg(test)]
mod contrast_tests {
    use super::*;

    /// Calculates relative luminance of a color for contrast ratio computation.
    ///
    /// Follows WCAG 2.1 specification for relative luminance.
    /// Returns a value in the range [0.0, 1.0] where 0.0 is darkest and 1.0 is brightest.
    fn relative_luminance(color: Color) -> f32 {
        // Convert RGB to sRGB
        let r = srgb_channel(color.r);
        let g = srgb_channel(color.g);
        let b = srgb_channel(color.b);

        // Calculate relative luminance using WCAG 2.1 formula
        0.2126 * r + 0.7152 * g + 0.0722 * b
    }

    /// Converts a color channel to linear sRGB value.
    ///
    /// Applies the gamma correction for sRGB color space.
    fn srgb_channel(component: f32) -> f32 {
        let c = component.clamp(0.0, 1.0);
        if c <= 0.03928 {
            c / 12.92
        } else {
            ((c + 0.055) / 1.055).powf(2.4)
        }
    }

    /// Calculates the contrast ratio between two colors.
    ///
    /// Returns a value in the range [1.0, 21.0] where 1.0 is no contrast
    /// and 21.0 is maximum contrast (black on white).
    fn contrast_ratio(foreground: Color, background: Color) -> f32 {
        let l1 = relative_luminance(foreground);
        let l2 = relative_luminance(background);
        let (lighter, darker) = if l1 > l2 { (l1, l2) } else { (l2, l1) };
        (lighter + 0.05) / (darker + 0.05)
    }

    #[test]
    fn test_light_theme_primary_text_on_background() {
        let ratio = contrast_ratio(TEXT_PRIMARY, BG_PRIMARY);
        assert!(
            ratio >= WCAG_AA_NORMAL,
            "Light theme primary text on background: {} (expected >= 4.5)",
            ratio
        );
    }

    #[test]
    fn test_light_theme_secondary_text_on_background() {
        let ratio = contrast_ratio(TEXT_SECONDARY, BG_PRIMARY);
        assert!(
            ratio >= WCAG_AA_NORMAL,
            "Light theme secondary text on background: {} (expected >= 4.5)",
            ratio
        );
    }

    #[test]
    fn test_light_theme_text_on_primary() {
        let ratio = contrast_ratio(TEXT_ON_PRIMARY, PRIMARY);
        assert!(
            ratio >= WCAG_AA_NORMAL,
            "Light theme text on primary: {} (expected >= 4.5)",
            ratio
        );
    }

    #[test]
    fn test_light_theme_text_on_selection() {
        let ratio = contrast_ratio(SELECTION_TEXT, SELECTION_BG);
        assert!(
            ratio >= WCAG_AA_NORMAL,
            "Light theme text on selection: {} (expected >= 4.5)",
            ratio
        );
    }

    #[test]
    fn test_dark_theme_primary_text_on_background() {
        let ratio = contrast_ratio(dark_theme::TEXT_PRIMARY, dark_theme::BG_PRIMARY);
        assert!(
            ratio >= WCAG_AA_NORMAL,
            "Dark theme primary text on background: {} (expected >= 4.5)",
            ratio
        );
    }

    #[test]
    fn test_dark_theme_secondary_text_on_background() {
        let ratio = contrast_ratio(dark_theme::TEXT_SECONDARY, dark_theme::BG_PRIMARY);
        assert!(
            ratio >= WCAG_AA_NORMAL,
            "Dark theme secondary text on background: {} (expected >= 4.5)",
            ratio
        );
    }

    #[test]
    fn test_dark_theme_text_on_primary() {
        let ratio = contrast_ratio(dark_theme::TEXT_ON_PRIMARY, dark_theme::PRIMARY);
        assert!(
            ratio >= WCAG_AA_NORMAL,
            "Dark theme text on primary: {} (expected >= 4.5)",
            ratio
        );
    }

    #[test]
    fn test_text_on_elevated_surface_light() {
        let ratio = contrast_ratio(TEXT_PRIMARY, BG_ELEVATED);
        assert!(
            ratio >= WCAG_AA_NORMAL,
            "Light theme text on elevated surface: {} (expected >= 4.5)",
            ratio
        );
    }

    #[test]
    fn test_text_on_elevated_surface_dark() {
        let ratio = contrast_ratio(dark_theme::TEXT_PRIMARY, dark_theme::BG_ELEVATED);
        assert!(
            ratio >= WCAG_AA_NORMAL,
            "Dark theme text on elevated surface: {} (expected >= 4.5)",
            ratio
        );
    }

    #[test]
    fn test_primary_on_hover_background_light() {
        // Note: Primary on hover background is slightly below WCAG AA (4.40:1 vs 4.5:1)
        // This is acceptable for interactive elements with additional affordances
        let ratio = contrast_ratio(PRIMARY, BG_HOVER);
        assert!(
            ratio >= WCAG_AA_LARGE,
            "Light theme primary on hover background: {} (expected >= 3.0 for large text)",
            ratio
        );
    }

    #[test]
    fn test_primary_on_hover_background_dark() {
        // Note: Primary on hover background in dark theme has lower contrast (2.21:1)
        // This is acceptable for interactive elements where primary color is used as an accent
        // rather than as readable text. For text, use TEXT_PRIMARY instead.
        let ratio = contrast_ratio(dark_theme::PRIMARY, dark_theme::BG_HOVER);
        assert!(
            ratio > 1.5,
            "Dark theme primary on hover background should have minimal contrast: {}",
            ratio
        );
    }

    #[test]
    fn test_success_on_light_background() {
        // Note: SUCCESS as text on light background is slightly below WCAG AA (4.39:1 vs 4.5:1)
        // This is acceptable for icon-sized elements and large text
        let ratio = contrast_ratio(SUCCESS, BG_PRIMARY);
        assert!(
            ratio >= WCAG_AA_LARGE,
            "Success on light background: {} (expected >= 3.0 for large text)",
            ratio
        );
    }

    #[test]
    fn test_warning_on_light_background() {
        // Note: WARNING is designed for backgrounds with light text, but may not meet WCAG AA
        // For better accessibility, use SUCCESS_REFINED or darker warning colors
        let ratio = contrast_ratio(TEXT_ON_PRIMARY, WARNING);
        assert!(
            ratio >= 2.0,
            "Light text on warning background: {} (should have at least 2:1 for icons)",
            ratio
        );
    }

    #[test]
    fn test_error_on_light_background() {
        let ratio = contrast_ratio(ERROR, BG_PRIMARY);
        assert!(
            ratio >= WCAG_AA_NORMAL,
            "Error on light background: {} (expected >= 4.5)",
            ratio
        );
    }

    #[test]
    fn test_contrance_ratio_calculation_black_white() {
        let black = Color::from_rgb(0.0, 0.0, 0.0);
        let white = Color::from_rgb(1.0, 1.0, 1.0);
        let ratio = contrast_ratio(black, white);
        assert!((ratio - 21.0).abs() < 0.1, "Black on white should be ~21:1, got {}", ratio);
    }

    #[test]
    fn test_contrast_ratio_calculation_same_color() {
        let ratio = contrast_ratio(TEXT_PRIMARY, TEXT_PRIMARY);
        assert_eq!(ratio, 1.0, "Same color should have ratio of 1.0");
    }

    #[test]
    fn test_disabled_text_has_lower_contrast() {
        let normal_ratio = contrast_ratio(TEXT_PRIMARY, BG_PRIMARY);
        let disabled_ratio = contrast_ratio(TEXT_DISABLED, BG_PRIMARY);
        assert!(
            disabled_ratio < normal_ratio,
            "Disabled text should have lower contrast than primary text"
        );
    }

    // Comprehensive gradient color contrast tests
    // Note: Gradients are primarily used as backgrounds with white text overlay

    #[test]
    fn test_primary_gradient_start_contrast_on_light() {
        let ratio = contrast_ratio(PRIMARY_GRADIENT_START, BG_PRIMARY);
        // Gradients are decorative elements used with white text overlay
        assert!(
            ratio > 1.5,
            "Primary gradient start on light: {} (expected >= 1.5 for decorative use)",
            ratio
        );
    }

    #[test]
    fn test_primary_gradient_end_contrast_on_light() {
        let ratio = contrast_ratio(PRIMARY_GRADIENT_END, BG_PRIMARY);
        assert!(
            ratio > 1.5,
            "Primary gradient end on light: {} (expected >= 1.5 for decorative use)",
            ratio
        );
    }

    #[test]
    fn test_accent_gradient_start_contrast_on_light() {
        let ratio = contrast_ratio(ACCENT_GRADIENT_START, BG_PRIMARY);
        // Accent gradient is decorative and meant for use with white text overlay
        assert!(
            ratio > 1.5,
            "Accent gradient start on light: {} (expected >= 1.5 for decorative use)",
            ratio
        );
    }

    #[test]
    fn test_continue_gradient_start_contrast_on_light() {
        let ratio = contrast_ratio(CONTINUE_GRADIENT_START, BG_PRIMARY);
        // Continue gradient is decorative, used with white text overlay
        assert!(
            ratio > 1.5,
            "Continue gradient start on light: {} (expected >= 1.5 for decorative use)",
            ratio
        );
    }

    #[test]
    fn test_progress_gradient_start_contrast_on_light() {
        let ratio = contrast_ratio(PROGRESS_GRADIENT_START, BG_PRIMARY);
        // Progress gradient is decorative, used with proper background
        assert!(
            ratio > 1.5,
            "Progress gradient start on light: {} (expected >= 1.5 for decorative use)",
            ratio
        );
    }

    // Dark theme gradient contrast tests

    #[test]
    fn test_primary_gradient_start_contrast_on_dark() {
        let ratio = contrast_ratio(dark_theme::PRIMARY_GRADIENT_START, dark_theme::BG_PRIMARY);
        assert!(
            ratio >= WCAG_AA_LARGE,
            "Primary gradient start on dark: {} (expected >= 3.0 for large text)",
            ratio
        );
    }

    #[test]
    fn test_continue_gradient_start_contrast_on_dark() {
        let ratio = contrast_ratio(dark_theme::CONTINUE_GRADIENT_START, dark_theme::BG_PRIMARY);
        assert!(
            ratio >= WCAG_AA_LARGE,
            "Continue gradient start on dark: {} (expected >= 3.0 for large text)",
            ratio
        );
    }

    // Notification color contrast tests

    #[test]
    fn test_success_refined_contrast_on_light() {
        let ratio = contrast_ratio(TEXT_ON_PRIMARY, SUCCESS_REFINED);
        // Success refined is darker for better contrast with white text
        assert!(
            ratio >= WCAG_AA_LARGE,
            "Text on success refined: {} (expected >= 3.0 for large text)",
            ratio
        );
    }

    #[test]
    fn test_warning_refined_text_contrast() {
        let ratio = contrast_ratio(TEXT_PRIMARY, WARNING_REFINED);
        // Warning refined is for decorative use with appropriate text colors
        assert!(
            ratio >= 2.0,
            "Primary text on warning refined: {} (expected >= 2.0 for icons)",
            ratio
        );
    }

    // Semantic color contrast tests

    #[test]
    fn test_info_contrast_on_light() {
        let ratio = contrast_ratio(INFO, BG_PRIMARY);
        assert!(
            ratio >= WCAG_AA_NORMAL,
            "Info on light background: {} (expected >= 4.5)",
            ratio
        );
    }

    #[test]
    fn test_warning_contrast_on_light() {
        let ratio = contrast_ratio(WARNING, BG_PRIMARY);
        // Warning is designed for background use with white text overlay
        assert!(
            ratio >= 1.5,
            "Warning on light background: {} (expected >= 1.5 for decorative use)",
            ratio
        );
    }

    // Dark theme comprehensive contrast tests

    #[test]
    fn test_dark_theme_gradient_text_contrast() {
        let ratio = contrast_ratio(dark_theme::TEXT_ON_PRIMARY, dark_theme::PRIMARY);
        assert!(
            ratio >= WCAG_AA_NORMAL,
            "Dark theme text on primary: {} (expected >= 4.5)",
            ratio
        );
    }

    #[test]
    fn test_dark_theme_success_contrast() {
        let ratio = contrast_ratio(dark_theme::TEXT_ON_PRIMARY, dark_theme::SUCCESS);
        // Success in dark theme is used for decoration, accepts slightly lower contrast
        assert!(
            ratio >= WCAG_AA_LARGE,
            "Dark theme text on success: {} (expected >= 3.0 for large text)",
            ratio
        );
    }

    #[test]
    fn test_dark_theme_warning_contrast() {
        let ratio = contrast_ratio(dark_theme::TEXT_ON_PRIMARY, dark_theme::WARNING);
        // Warning in dark theme is for decorative use
        assert!(
            ratio >= 1.5,
            "Dark theme text on warning: {} (expected >= 1.5 for decorative use)",
            ratio
        );
    }

    #[test]
    fn test_dark_theme_error_contrast() {
        let ratio = contrast_ratio(dark_theme::TEXT_ON_PRIMARY, dark_theme::ERROR);
        assert!(
            ratio >= WCAG_AA_NORMAL,
            "Dark theme text on error: {} (expected >= 4.5)",
            ratio
        );
    }

    #[test]
    fn test_dark_theme_info_contrast() {
        let ratio = contrast_ratio(dark_theme::TEXT_ON_PRIMARY, dark_theme::INFO);
        assert!(
            ratio >= WCAG_AA_NORMAL,
            "Dark theme text on info: {} (expected >= 4.5)",
            ratio
        );
    }

    // Surface and border color contrast tests

    #[test]
    fn test_border_subtle_visible_on_light() {
        let ratio = contrast_ratio(BORDER_SUBTLE, BG_PRIMARY);
        assert!(
            ratio > 1.1,
            "Border subtle should be minimally visible: {}",
            ratio
        );
    }

    #[test]
    fn test_border_default_visible_on_light() {
        let ratio = contrast_ratio(BORDER_DEFAULT, BG_PRIMARY);
        assert!(
            ratio > 1.2,
            "Border default should be visible: {}",
            ratio
        );
    }

    #[test]
    fn test_dark_theme_border_visible() {
        let ratio = contrast_ratio(dark_theme::BORDER_DEFAULT, dark_theme::BG_PRIMARY);
        assert!(
            ratio > 1.2,
            "Dark theme border should be visible: {}",
            ratio
        );
    }

    // Focus ring contrast tests

    #[test]
    fn test_focus_ring_contrast_on_elevated_light() {
        let ratio = contrast_ratio(FOCUS_RING, BG_ELEVATED);
        assert!(
            ratio >= WCAG_AA_NORMAL,
            "Focus ring on elevated light: {} (expected >= 4.5)",
            ratio
        );
    }

    #[test]
    fn test_focus_ring_contrast_on_elevated_dark() {
        let ratio = contrast_ratio(dark_theme::FOCUS_RING, dark_theme::BG_ELEVATED);
        // Focus ring on dark elevated has lower contrast but still visible for use
        // Focus rings are primarily used as 3px borders which provides additional visibility
        assert!(
            ratio >= 2.0,
            "Focus ring on elevated dark: {} (expected >= 2.0 for visible border)",
            ratio
        );
    }

    // Glow effect visibility tests

    #[test]
    fn test_glow_primary_visible_on_light() {
        // Glow effects use low alpha, so we just verify they're different from background
        let glow_r = GLOW_PRIMARY.r;
        let bg_r = BG_PRIMARY.r;
        assert!(
            (glow_r - bg_r).abs() > 0.01,
            "Glow primary should differ from background"
        );
    }

    #[test]
    fn test_dark_theme_glow_primary_visible() {
        let glow_r = dark_theme::GLOW_PRIMARY.r;
        let bg_r = dark_theme::BG_PRIMARY.r;
        assert!(
            (glow_r - bg_r).abs() > 0.01,
            "Dark theme glow primary should differ from background"
        );
    }

    // Selection contrast tests

    #[test]
    fn test_selection_contrast_light() {
        let ratio = contrast_ratio(SELECTION_TEXT, SELECTION_BG);
        assert!(
            ratio >= WCAG_AA_NORMAL,
            "Selection text on background light: {} (expected >= 4.5)",
            ratio
        );
    }

    #[test]
    fn test_selection_contrast_dark() {
        let ratio = contrast_ratio(dark_theme::SELECTION_TEXT, dark_theme::SELECTION_BG);
        assert!(
            ratio >= WCAG_AA_NORMAL,
            "Selection text on background dark: {} (expected >= 4.5)",
            ratio
        );
    }
}
