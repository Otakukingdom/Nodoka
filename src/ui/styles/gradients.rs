//! Premium gradient utilities for distinctive visual design.

use iced::Color;

/// Linear gradient interpolation between two colors at a given point
///
/// Creates a smooth color transition by blending the start and end colors
/// based on the interpolation factor `t` (0.0 = start, 1.0 = end).
///
/// # Examples
///
/// ```ignore
/// let start = Color::from_rgb(1.0, 0.0, 0.0); // Red
/// let end = Color::from_rgb(0.0, 0.0, 1.0);   // Blue
/// let midpoint = lerp_color(start, end, 0.5);  // Purple
/// ```
#[must_use]
pub const fn lerp_color(start: Color, end: Color, t: f32) -> Color {
    let t = t.clamp(0.0, 1.0);
    Color {
        r: start.r + (end.r - start.r) * t,
        g: start.g + (end.g - start.g) * t,
        b: start.b + (end.b - start.b) * t,
        a: start.a + (end.a - start.a) * t,
    }
}

/// Creates a gradient color with alpha blending
///
/// Useful for creating semi-transparent gradient backgrounds that need
/// to maintain consistent opacity across gradient stops.
///
/// # Examples
///
/// ```ignore
/// let gradient = gradient_with_alpha(Color::from_rgb(0.98, 0.32, 0.45), 0.8);
/// ```
#[must_use]
pub const fn gradient_with_alpha(color: Color, alpha: f32) -> Color {
    Color {
        r: color.r,
        g: color.g,
        b: color.b,
        a: alpha.clamp(0.0, 1.0),
    }
}

/// Sunset gradient colors for primary branding elements
///
/// Creates warm rose-to-coral transition for distinctive brand identity.
pub mod sunset {
    use super::Color;

    /// Sunset gradient start color - warm rose
    pub const START: Color = Color::from_rgb(0.98, 0.32, 0.45);

    /// Sunset gradient end color - deep coral
    pub const END: Color = Color::from_rgb(0.88, 0.15, 0.30);

    /// Returns the interpolated sunset gradient color at position t
    #[must_use]
    pub const fn at(t: f32) -> Color {
        super::lerp_color(START, END, t.clamp(0.0, 1.0))
    }

    /// Returns the sunset gradient start color with custom alpha
    #[must_use]
    pub const fn with_alpha(alpha: f32) -> Color {
        super::gradient_with_alpha(START, alpha.clamp(0.0, 1.0))
    }
}

/// Cool accent gradient colors for secondary highlights
///
/// Provides calming lavender-to-indigo transition for visual variety.
pub mod cool {
    use super::Color;

    /// Cool gradient start color - lavender
    pub const START: Color = Color::from_rgb(0.70, 0.65, 0.95);

    /// Cool gradient end color - indigo
    pub const END: Color = Color::from_rgb(0.55, 0.45, 0.85);

    /// Returns the interpolated cool gradient color at position t
    #[must_use]
    pub const fn at(t: f32) -> Color {
        super::lerp_color(START, END, t.clamp(0.0, 1.0))
    }
}

/// Premium gold gradient colors for achievements and completion
///
/// Creates sophisticated amber-gold transition for celebratory moments.
pub mod gold {
    use super::Color;

    /// Gold gradient start color - bright amber
    pub const START: Color = Color::from_rgb(1.0, 0.82, 0.40);

    /// Gold gradient end color - rich gold
    pub const END: Color = Color::from_rgb(0.92, 0.68, 0.25);

    /// Returns the interpolated gold gradient color at position t
    #[must_use]
    pub const fn at(t: f32) -> Color {
        super::lerp_color(START, END, t.clamp(0.0, 1.0))
    }
}

/// Primary rose gradient for distinctive brand elevation
///
/// Creates refined rose-to-crimson transition for premium appearance.
pub mod primary {
    use crate::ui::styles::colors;
    use iced::Color;

    /// Returns the interpolated primary gradient color at position t
    #[must_use]
    pub const fn at(t: f32) -> Color {
        super::lerp_color(colors::PRIMARY_GRADIENT_START, colors::PRIMARY_GRADIENT_END, t)
    }
}

/// Accent gradient for CTAs and highlights
///
/// Provides vibrant rose-to-magenta transition for distinctive appeal.
pub mod accent {
    use crate::ui::styles::colors;
    use iced::Color;

    /// Returns the interpolated accent gradient color at position t
    #[must_use]
    pub const fn at(t: f32) -> Color {
        super::lerp_color(colors::ACCENT_GRADIENT_START, colors::ACCENT_GRADIENT_END, t)
    }
}

/// Progress gradient for visual engagement
///
/// Creates smooth rose-to-crimson flow for polished appearance.
pub mod progress {
    use crate::ui::styles::colors;
    use iced::Color;

    /// Returns the interpolated progress gradient color at position t
    #[must_use]
    pub const fn at(t: f32) -> Color {
        super::lerp_color(colors::PROGRESS_GRADIENT_START, colors::PROGRESS_GRADIENT_END, t)
    }
}

/// Continue listening gradient for premium feel
///
/// Creates luminous rose-to-crimson transition for dramatic effect.
pub mod continue_gradient {
    use crate::ui::styles::colors;
    use iced::Color;

    /// Returns the interpolated continue gradient color at position t
    #[must_use]
    pub const fn at(t: f32) -> Color {
        super::lerp_color(colors::CONTINUE_GRADIENT_START, colors::CONTINUE_GRADIENT_END, t)
    }
}

/// Subtle gradient for non-interactive surfaces
///
/// Enhanced with warm undertones for sophisticated depth.
pub mod subtle {
    use crate::ui::styles::colors;
    use iced::Color;

    /// Returns the interpolated subtle gradient color at position t
    #[must_use]
    pub const fn at(t: f32) -> Color {
        super::lerp_color(colors::SUBTLE_GRADIENT_START, colors::SUBTLE_GRADIENT_END, t)
    }
}

/// Gradient utility functions for common operations
pub mod utils {
    use super::Color;

    /// Creates a multi-step gradient color by interpolating between multiple stops
    ///
    /// Each stop is a tuple of (position, color) where position is 0.0 to 1.0.
    /// Returns the interpolated color at the given position t.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let stops = [
    ///     (0.0, Color::from_rgb(1.0, 0.0, 0.0)),
    ///     (0.5, Color::from_rgb(0.0, 1.0, 0.0)),
    ///     (1.0, Color::from_rgb(0.0, 0.0, 1.0)),
    /// ];
    /// let color = multi_stop_gradient(stops, 0.25);
    /// ```
    #[must_use]
    pub fn multi_stop_gradient(stops: &[(f32, Color)], t: f32) -> Color {
        let t = t.clamp(0.0, 1.0);

        // Find the two stops to interpolate between
        let mut lower_stop = (0.0, stops.first().map_or(Color::WHITE, |s| s.1));
        let mut upper_stop = (1.0, stops.last().map_or(Color::BLACK, |s| s.1));

        for &(pos, color) in stops {
            if pos <= t {
                lower_stop = (pos, color);
            }
            if pos >= t {
                upper_stop = (pos, color);
                break;
            }
        }

        // Interpolate between the two stops
        let range = upper_stop.0 - lower_stop.0;
        if range > 0.0 {
            let local_t = (t - lower_stop.0) / range;
            super::lerp_color(lower_stop.1, upper_stop.1, local_t)
        } else {
            lower_stop.1
        }
    }

    /// Creates a gradient overlay by blending a gradient color over a base color
    ///
    /// Uses the "screen" blend mode for vibrant, light results.
    #[must_use]
    pub fn overlay_gradient(base: Color, gradient: Color, opacity: f32) -> Color {
        let opacity = opacity.clamp(0.0, 1.0);
        Color {
            r: (gradient.r - base.r).mul_add(opacity, base.r),
            g: (gradient.g - base.g).mul_add(opacity, base.g),
            b: (gradient.b - base.b).mul_add(opacity, base.b),
            a: base.a,
        }
    }

    /// Creates a gradient with reduced opacity for backgrounds
    #[must_use]
    pub const fn background_gradient(gradient: Color, opacity: f32) -> Color {
        Color {
            r: gradient.r,
            g: gradient.g,
            b: gradient.b,
            a: opacity.clamp(0.0, 1.0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Epsilon tolerance for float comparisons in tests
    const EPSILON: f32 = 1.0e-6;

    /// Helper function for approximate float comparison
    fn approx_eq(a: f32, b: f32) -> bool {
        (a - b).abs() < EPSILON
    }

    #[test]
    fn test_lerp_color_at_start_returns_start() {
        let start = Color::from_rgb(1.0, 0.0, 0.0);
        let end = Color::from_rgb(0.0, 0.0, 1.0);
        let result = lerp_color(start, end, 0.0);
        assert!(approx_eq(result.r, start.r));
        assert!(approx_eq(result.g, start.g));
        assert!(approx_eq(result.b, start.b));
    }

    #[test]
    fn test_lerp_color_at_end_returns_end() {
        let start = Color::from_rgb(1.0, 0.0, 0.0);
        let end = Color::from_rgb(0.0, 0.0, 1.0);
        let result = lerp_color(start, end, 1.0);
        assert!(approx_eq(result.r, end.r));
        assert!(approx_eq(result.g, end.g));
        assert!(approx_eq(result.b, end.b));
    }

    #[test]
    fn test_lerp_color_at_midpoint_returns_average() {
        let start = Color::from_rgb(0.0, 0.0, 0.0);
        let end = Color::from_rgb(1.0, 1.0, 1.0);
        let result = lerp_color(start, end, 0.5);
        assert!(approx_eq(result.r, 0.5));
        assert!(approx_eq(result.g, 0.5));
        assert!(approx_eq(result.b, 0.5));
    }

    #[test]
    fn test_gradient_with_alpha_sets_alpha() {
        let color = Color::from_rgb(1.0, 0.5, 0.25);
        let result = gradient_with_alpha(color, 0.75);
        assert!(approx_eq(result.r, color.r));
        assert!(approx_eq(result.g, color.g));
        assert!(approx_eq(result.b, color.b));
        assert!(approx_eq(result.a, 0.75));
    }

    #[test]
    fn test_sunset_at_returns_valid_color() {
        let color = sunset::at(0.5);
        assert!(color.r >= sunset::END.r && color.r <= sunset::START.r);
        assert!(color.a >= 0.0 && color.a <= 1.0);
    }

    #[test]
    fn test_cool_at_returns_valid_color() {
        let color = cool::at(0.5);
        assert!(color.r >= cool::END.r && color.r <= cool::START.r);
        assert!(color.a >= 0.0 && color.a <= 1.0);
    }

    #[test]
    fn test_gold_at_returns_valid_color() {
        let color = gold::at(0.5);
        assert!(color.r >= gold::END.r && color.r <= gold::START.r);
        assert!(color.a >= 0.0 && color.a <= 1.0);
    }

    #[test]
    fn test_lerp_color_clamps_t_below_zero() {
        let start = Color::from_rgb(0.0, 0.0, 0.0);
        let end = Color::from_rgb(1.0, 1.0, 1.0);
        let result = lerp_color(start, end, -0.5);
        assert!(approx_eq(result.r, start.r));
        assert!(approx_eq(result.g, start.g));
        assert!(approx_eq(result.b, start.b));
    }

    #[test]
    fn test_lerp_color_clamps_t_above_one() {
        let start = Color::from_rgb(0.0, 0.0, 0.0);
        let end = Color::from_rgb(1.0, 1.0, 1.0);
        let result = lerp_color(start, end, 1.5);
        assert!(approx_eq(result.r, end.r));
        assert!(approx_eq(result.g, end.g));
        assert!(approx_eq(result.b, end.b));
    }

    #[test]
    fn test_utils_multi_stop_gradient_with_two_stops() {
        let stops = [
            (0.0, Color::from_rgb(1.0, 0.0, 0.0)),
            (1.0, Color::from_rgb(0.0, 0.0, 1.0)),
        ];
        let result = utils::multi_stop_gradient(&stops, 0.5);
        assert!(approx_eq(result.r, 0.5));
        assert!(approx_eq(result.b, 0.5));
    }

    #[test]
    fn test_utils_multi_stop_gradient_with_three_stops() {
        let stops = [
            (0.0, Color::from_rgb(1.0, 0.0, 0.0)),
            (0.5, Color::from_rgb(0.0, 1.0, 0.0)),
            (1.0, Color::from_rgb(0.0, 0.0, 1.0)),
        ];
        let result = utils::multi_stop_gradient(&stops, 0.25);
        // Should be between red and green
        assert!(result.r > 0.0);
        assert!(result.g > 0.0);
    }

    #[test]
    fn test_utils_overlay_gradient_blends_colors() {
        let base = Color::from_rgb(0.5, 0.5, 0.5);
        let gradient = Color::from_rgb(1.0, 0.0, 0.0);
        let result = utils::overlay_gradient(base, gradient, 0.5);
        assert!(approx_eq(result.r, 0.75)); // 0.5 + (1.0 - 0.5) * 0.5
        assert!(approx_eq(result.g, 0.25)); // 0.5 + (0.0 - 0.5) * 0.5
        assert!(approx_eq(result.b, 0.25)); // 0.5 + (0.0 - 0.5) * 0.5
    }
}
