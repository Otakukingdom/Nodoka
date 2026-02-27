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
/// Floating surface background – slightly elevated above [`BG_ELEVATED`].
pub const BG_FLOATING: Color = Color::from_rgb(0.99, 0.99, 0.99);

/// Raised surface for cards and list items (slightly above [`BG_ELEVATED`]).
/// Creates subtle elevation for card elements.
pub const BG_RAISED: Color = Color::from_rgb(0.995, 0.995, 0.998);

/// Surface tier for interactive elements (between [`BG_ELEVATED`] and [`BG_FLOATING`]).
/// Provides intermediate elevation level for hover states and interactive elements.
pub const BG_TIER: Color = Color::from_rgb(0.998, 0.998, 1.0);

/// Subtle gradient for non-interactive surfaces.
/// Enhanced with warm undertones for sophisticated depth.
pub const SUBTLE_GRADIENT_START: Color = Color::from_rgb(0.99, 0.985, 0.99);
pub const SUBTLE_GRADIENT_END: Color = Color::from_rgb(0.975, 0.97, 0.985);

/// Surface hover state with subtle gradient.
/// Enhanced with warm rose tint for premium feel.
pub const SURFACE_HOVER: Color = Color::from_rgb(0.98, 0.975, 0.98);

/// Surface active state with deeper gradient.
/// Enhanced with stronger rose tint for feedback.
pub const SURFACE_ACTIVE: Color = Color::from_rgb(0.95, 0.94, 0.96);

/// Primary gradient colors for visual depth on special elements.
/// Refined with ultra-smooth rose-to-crimson transition for premium appearance.
pub const PRIMARY_GRADIENT_START: Color = Color::from_rgb(0.97, 0.24, 0.38);
pub const PRIMARY_GRADIENT_END: Color = Color::from_rgb(0.84, 0.11, 0.26);

/// Accent gradient colors for CTAs and highlights.
/// Refined with vibrant rose-to-magenta transition for distinctive appeal.
pub const ACCENT_GRADIENT_START: Color = Color::from_rgb(0.995, 0.48, 0.56);
pub const ACCENT_GRADIENT_END: Color = Color::from_rgb(0.91, 0.14, 0.32);

/// Continue listening card gradient for premium feel.
/// Refined with luminous rose-to-crimson transition for dramatic effect.
pub const CONTINUE_GRADIENT_START: Color = Color::from_rgb(1.0, 0.57, 0.67);
pub const CONTINUE_GRADIENT_END: Color = Color::from_rgb(0.91, 0.14, 0.32);

/// Progress bar gradient for visual engagement.
/// Refined with smooth rose-to-crimson flow for polished appearance.
pub const PROGRESS_GRADIENT_START: Color = Color::from_rgb(0.975, 0.28, 0.43);
pub const PROGRESS_GRADIENT_END: Color = Color::from_rgb(0.88, 0.13, 0.28);

/// Warm gradient for accent elements and badges.
/// Provides sophisticated amber-rose tint for distinctive highlights.
pub const GRADIENT_WARM_START: Color = Color::from_rgb(1.0, 0.85, 0.75);
pub const GRADIENT_WARM_END: Color = Color::from_rgb(0.98, 0.65, 0.55);

/// Cool gradient for secondary accents.
/// Provides calming teal-to-blue tint for balanced variety.
pub const GRADIENT_COOL_START: Color = Color::from_rgb(0.45, 0.85, 0.90);
pub const GRADIENT_COOL_END: Color = Color::from_rgb(0.35, 0.65, 0.80);

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

/// ===== Premium Distinctive Gradients =====
/// Premium sunset gradient for distinctive brand identity
/// Creates warm, sophisticated rose-to-coral transition for premium appearance
pub const SUNSET_GRADIENT_START: Color = Color::from_rgb(0.98, 0.32, 0.45);
pub const SUNSET_GRADIENT_END: Color = Color::from_rgb(0.88, 0.15, 0.30);

/// Cool accent gradient for secondary highlights
/// Provides calming lavender-to-indigo transition for visual variety
pub const COOL_GRADIENT_START: Color = Color::from_rgb(0.70, 0.65, 0.95);
pub const COOL_GRADIENT_END: Color = Color::from_rgb(0.55, 0.45, 0.85);

/// Premium gold gradient for achievements and completion states
/// Creates sophisticated amber-gold transition for celebratory moments
pub const GOLD_GRADIENT_START: Color = Color::from_rgb(1.0, 0.82, 0.40);
pub const GOLD_GRADIENT_END: Color = Color::from_rgb(0.92, 0.68, 0.25);

/// Premium glow for active elements with enhanced rose tint
pub const GLOW_PREMIUM: Color = Color::from_rgba(0.98, 0.32, 0.45, 0.45);

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

    /// Raised surface for cards and list items in dark theme.
    /// Creates subtle elevation for card elements.
    pub const BG_RAISED: Color = Color::from_rgb(0.21, 0.21, 0.22);

    /// Surface tier for interactive elements in dark theme.
    /// Provides intermediate elevation level for hover states and interactive elements.
    pub const BG_TIER: Color = Color::from_rgb(0.20, 0.20, 0.21);

    /// Subtle gradient for non-interactive surfaces in dark theme.
    /// Enhanced with subtle warmth for depth.
    pub const SUBTLE_GRADIENT_START: Color = Color::from_rgb(0.22, 0.215, 0.22);
    pub const SUBTLE_GRADIENT_END: Color = Color::from_rgb(0.20, 0.195, 0.21);

    /// Surface hover state in dark theme.
    /// Enhanced with warm undertone for feedback.
    pub const SURFACE_HOVER: Color = Color::from_rgb(0.28, 0.275, 0.28);

    /// Surface active state in dark theme.
    /// Enhanced with stronger warm tint for active feedback.
    pub const SURFACE_ACTIVE: Color = Color::from_rgb(0.32, 0.31, 0.32);

    /// Primary gradient colors for dark theme.
    /// Refined with enhanced brightness for excellent contrast on dark backgrounds.
    pub const PRIMARY_GRADIENT_START: Color = Color::from_rgb(0.98, 0.25, 0.40);
    pub const PRIMARY_GRADIENT_END: Color = Color::from_rgb(0.86, 0.13, 0.28);

    /// Accent gradient colors for dark theme.
    /// Refined with vibrant rose-to-magenta transition for visibility.
    pub const ACCENT_GRADIENT_START: Color = Color::from_rgb(1.0, 0.49, 0.58);
    pub const ACCENT_GRADIENT_END: Color = Color::from_rgb(0.92, 0.15, 0.33);

    /// Continue listening card gradient for dark theme.
    /// Enhanced with maximum brightness to maintain vibrancy on dark backgrounds.
    pub const CONTINUE_GRADIENT_START: Color = Color::from_rgb(1.0, 0.58, 0.69);
    pub const CONTINUE_GRADIENT_END: Color = Color::from_rgb(0.93, 0.16, 0.34);

    /// Progress bar gradient for dark theme.
    /// Enhanced with optimized brightness for visibility on dark backgrounds.
    pub const PROGRESS_GRADIENT_START: Color = Color::from_rgb(0.98, 0.30, 0.45);
    pub const PROGRESS_GRADIENT_END: Color = Color::from_rgb(0.89, 0.15, 0.30);

    /// Warm gradient for accent elements in dark theme.
    /// Provides sophisticated amber-rose tint for distinctive highlights.
    pub const GRADIENT_WARM_START: Color = Color::from_rgb(1.0, 0.88, 0.78);
    pub const GRADIENT_WARM_END: Color = Color::from_rgb(0.99, 0.68, 0.58);

    /// Cool gradient for secondary accents in dark theme.
    /// Provides calming teal-to-blue tint for balanced variety.
    pub const GRADIENT_COOL_START: Color = Color::from_rgb(0.48, 0.88, 0.93);
    pub const GRADIENT_COOL_END: Color = Color::from_rgb(0.38, 0.68, 0.83);

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

    /// ===== Premium Distinctive Gradients for Dark Theme =====

    /// Premium sunset gradient for dark theme with enhanced brightness
    /// Creates warm, sophisticated rose-to-coral transition optimized for dark backgrounds
    pub const SUNSET_GRADIENT_START: Color = Color::from_rgb(0.99, 0.35, 0.50);
    pub const SUNSET_GRADIENT_END: Color = Color::from_rgb(0.90, 0.18, 0.35);

    /// Cool accent gradient for dark theme with enhanced luminosity
    /// Provides calming lavender-to-indigo transition with better contrast
    pub const COOL_GRADIENT_START: Color = Color::from_rgb(0.75, 0.70, 0.98);
    pub const COOL_GRADIENT_END: Color = Color::from_rgb(0.60, 0.50, 0.88);

    /// Premium gold gradient for dark theme with maximum brightness
    /// Creates sophisticated amber-gold transition optimized for visibility on dark
    pub const GOLD_GRADIENT_START: Color = Color::from_rgb(1.0, 0.86, 0.48);
    pub const GOLD_GRADIENT_END: Color = Color::from_rgb(0.94, 0.72, 0.32);

    /// Premium glow for active elements in dark theme
    pub const GLOW_PREMIUM: Color = Color::from_rgba(0.98, 0.32, 0.45, 0.55);
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

// Include color contrast tests from separate file to keep main module under 1000 lines
#[cfg(test)]
include!("colors_tests.rs");
