//! Micro-interaction utilities for premium UI feedback.
//!
//! This module provides scale transforms, spring animations, shimmer effects,
//! and pulse/glow effects to create a premium, crafted feel.

/// Scale transform utilities for button and card micro-interactions
pub mod scale {
    /// Scale factor for pressed state (slightly smaller)
    pub const PRESSED: f32 = 0.95;
    /// Scale factor for idle state (normal size)
    pub const IDLE: f32 = 1.0;
    /// Scale factor for hover state (slightly larger)
    pub const HOVER: f32 = 1.02;
    /// Scale factor for prominent hover on featured elements
    pub const HOVER_PROMINENT: f32 = 1.05;

    /// Calculates scale factor with spring easing for smooth transitions
    ///
    /// Uses an ease-out-back curve for a bouncy, premium feel
    #[must_use]
    pub fn spring_scale(progress: f32, from: f32, to: f32) -> f32 {
        let p = progress.clamp(0.0, 1.0);
        // Ease-out-back with subtle overshoot
        let c1 = 1.70158;
        let c3 = c1 + 1.0;
        let eased = 1.0 + c3 * (p - 1.0).powi(3) + c1 * (p - 1.0).powi(2);
        (to - from).mul_add(eased, from)
    }

    /// Linear interpolation between scale values
    #[must_use]
    pub const fn lerp(from: f32, to: f32, t: f32) -> f32 {
        from + (to - from) * t.clamp(0.0, 1.0)
    }
}

/// Spring animation helpers for natural button feedback
pub mod spring {
    /// Spring stiffness for button interactions (higher = snappier)
    pub const BUTTON_STIFFNESS: f32 = 300.0;
    /// Spring damping for button interactions (lower = more bouncy)
    pub const BUTTON_DAMPING: f32 = 15.0;
    /// Spring stiffness for modal animations (smoother)
    pub const MODAL_STIFFNESS: f32 = 200.0;
    /// Spring damping for modal animations
    pub const MODAL_DAMPING: f32 = 20.0;

    /// Calculates spring progress based on elapsed time
    ///
    /// Returns a value from 0.0 to 1.0 representing animation completion
    #[must_use]
    pub fn progress(elapsed_ms: u64, duration_ms: u64) -> f32 {
        let duration_ms = duration_ms.min(u64::from(u16::MAX));
        let duration_ms = u16::try_from(duration_ms).unwrap_or(u16::MAX);
        if duration_ms == 0 {
            return 1.0;
        }

        let elapsed_ms = elapsed_ms.min(u64::from(duration_ms));
        let elapsed_ms = u16::try_from(elapsed_ms).unwrap_or(duration_ms);

        let progress = (f32::from(elapsed_ms) / f32::from(duration_ms)).clamp(0.0, 1.0);
        // Ease-out-quad for smooth spring-like motion
        (1.0 - progress).mul_add(-(1.0 - progress), 1.0)
    }
}

/// Shimmer effect helpers for loading states
pub mod shimmer {
    use crate::ui::styles::components::loading;
    use iced::Color;

    /// Base shimmer color for loading skeleton
    pub const BASE: Color = loading::SKELETON_BG;
    /// Highlight shimmer color for animation
    pub const HIGHLIGHT: Color = loading::SKELETON_SHIMMER;

    /// Shimmer animation duration in milliseconds
    pub const DURATION_MS: u64 = 1500;

    /// Calculates shimmer position based on elapsed time
    ///
    /// Returns a value from 0.0 to 1.0 representing shimmer position
    #[must_use]
    pub fn position(elapsed_ms: u64) -> f32 {
        let duration_ms = u16::try_from(DURATION_MS).unwrap_or(1);
        let elapsed = u16::try_from(elapsed_ms % DURATION_MS).unwrap_or(0);
        (f32::from(elapsed) / f32::from(duration_ms)).clamp(0.0, 1.0)
    }

    /// Shimmer gradient colors for three-stop gradient
    #[must_use]
    pub const fn gradient_stops() -> (Color, Color, Color) {
        (BASE, HIGHLIGHT, BASE)
    }
}

/// Pulse and glow effects for active state indicators
pub mod pulse {
    use crate::ui::styles::colors;
    use iced::Color;

    /// Pulse animation duration in milliseconds
    pub const DURATION_MS: u64 = 2000;
    /// Pulse opacity range (minimum to maximum)
    pub const OPACITY_MIN: f32 = 0.4;
    pub const OPACITY_MAX: f32 = 1.0;

    /// Calculates pulse opacity based on elapsed time
    ///
    /// Returns a value between `OPACITY_MIN` and `OPACITY_MAX`
    #[must_use]
    pub fn opacity(elapsed_ms: u64) -> f32 {
        let duration_ms = u16::try_from(DURATION_MS).unwrap_or(1);
        let elapsed = u16::try_from(elapsed_ms % DURATION_MS).unwrap_or(0);
        let phase = f32::from(elapsed) / f32::from(duration_ms);
        let sine = (phase * std::f32::consts::PI * 2.0).sin();
        let normalized = f32::midpoint(sine, 1.0);
        (OPACITY_MAX - OPACITY_MIN).mul_add(normalized, OPACITY_MIN)
    }

    /// Glow effect color with adjustable intensity
    #[must_use]
    pub fn glow_color(intensity: f32) -> Color {
        let alpha = (intensity.clamp(0.0, 1.0) * 0.4).clamp(0.0, 0.5);
        Color::from_rgba(
            colors::PRIMARY.r,
            colors::PRIMARY.g,
            colors::PRIMARY.b,
            alpha,
        )
    }

    /// Inner glow for active elements (progress bars, sliders)
    #[must_use]
    pub fn inner_glow_color(intensity: f32) -> Color {
        let alpha = (intensity.clamp(0.0, 1.0) * 0.2).clamp(0.0, 0.3);
        Color::from_rgba(
            colors::PRIMARY.r,
            colors::PRIMARY.g,
            colors::PRIMARY.b,
            alpha,
        )
    }
}

/// Ripple effect parameters for button clicks
pub mod ripple {
    /// Maximum ripple radius in pixels
    pub const MAX_RADIUS: f32 = 100.0;
    /// Ripple animation duration in milliseconds
    pub const DURATION_MS: u64 = 400;
    /// Ripple opacity
    pub const OPACITY: f32 = 0.3;

    /// Calculates ripple radius based on animation progress
    #[must_use]
    pub fn radius(elapsed_ms: u64) -> f32 {
        let duration_ms = u16::try_from(DURATION_MS).unwrap_or(1);
        let elapsed_ms = u16::try_from(elapsed_ms.min(DURATION_MS)).unwrap_or(duration_ms);
        let progress = (f32::from(elapsed_ms) / f32::from(duration_ms)).clamp(0.0, 1.0);
        let eased = ease_out(progress);
        MAX_RADIUS * eased
    }

    /// Calculates ripple opacity based on animation progress
    #[must_use]
    pub fn opacity(elapsed_ms: u64) -> f32 {
        let duration_ms = u16::try_from(DURATION_MS).unwrap_or(1);
        let elapsed_ms = u16::try_from(elapsed_ms.min(DURATION_MS)).unwrap_or(duration_ms);
        let progress = (f32::from(elapsed_ms) / f32::from(duration_ms)).clamp(0.0, 1.0);
        OPACITY * (1.0 - progress)
    }

    fn ease_out(t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        t * (2.0 - t)
    }
}

/// Transition timing utilities for consistent animation feel
pub mod timing {
    /// Fast transition for immediate feedback
    pub const FAST_MS: u64 = 150;
    /// Normal transition for standard interactions
    pub const NORMAL_MS: u64 = 200;
    /// Slow transition for deliberate motion
    pub const SLOW_MS: u64 = 300;
    /// Extra slow transition for emphasis
    pub const EXTRA_SLOW_MS: u64 = 500;
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies scale constants ascend monotonically for proper interaction feedback.
    const _: () = assert!(scale::PRESSED < scale::IDLE);
    const _: () = assert!(scale::IDLE < scale::HOVER);
    const _: () = assert!(scale::HOVER < scale::HOVER_PROMINENT);

    /// Verifies timing constants are positive and ascend monotonically.
    const _: () = assert!(timing::FAST_MS > 0);
    const _: () = assert!(timing::NORMAL_MS > 0);
    const _: () = assert!(timing::SLOW_MS > 0);
    const _: () = assert!(timing::EXTRA_SLOW_MS > 0);
    const _: () = assert!(timing::FAST_MS < timing::NORMAL_MS);
    const _: () = assert!(timing::NORMAL_MS < timing::SLOW_MS);
    const _: () = assert!(timing::SLOW_MS < timing::EXTRA_SLOW_MS);

    #[test]
    fn test_scale_lerp_clamps_correctly() {
        assert!((scale::lerp(0.95, 1.05, -0.5) - 0.95).abs() < f32::EPSILON);
        assert!((scale::lerp(0.95, 1.05, 1.5) - 1.05).abs() < f32::EPSILON);
        assert!((scale::lerp(0.95, 1.05, 0.5) - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_scale_spring_scale_returns_valid_range() {
        // spring_scale uses ease-out-back which intentionally overshoots slightly
        let result_mid = scale::spring_scale(0.5, 0.95, 1.05);
        // Allow slight overshoot due to ease-out-back curve (c1 = 1.70158)
        assert!((0.95..=1.10).contains(&result_mid));

        let result_start = scale::spring_scale(0.0, 0.95, 1.05);
        assert!((result_start - 0.95).abs() < 0.01);

        let result_end = scale::spring_scale(1.0, 0.95, 1.05);
        // At progress=1.0, ease-out-back returns exactly 1.0
        assert!((result_end - 1.05).abs() < 0.01);
    }

    #[test]
    fn test_spring_progress_clamps_to_zero_one() {
        assert!((spring::progress(0, 100) - 0.0).abs() < f32::EPSILON);
        // spring::progress uses ease-out-quad: 1.0 - (1.0 - t)²
        // At t=0.5, this gives 1.0 - 0.25 = 0.75, not 0.5
        let p50 = spring::progress(50, 100);
        assert!((p50 - 0.75).abs() < 0.01);
        assert!((spring::progress(100, 100) - 1.0).abs() < f32::EPSILON);
        assert!((spring::progress(150, 100) - 1.0).abs() < f32::EPSILON);
        assert!((spring::progress(0, 0) - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_shimmer_position_returns_valid_range() {
        let pos = shimmer::position(0);
        assert!((0.0..=1.0).contains(&pos));

        let pos = shimmer::position(750);
        assert!((0.0..=1.0).contains(&pos));

        let pos = shimmer::position(1500);
        assert!((0.0..=1.0).contains(&pos));
    }

    #[test]
    fn test_pulse_opacity_returns_valid_range() {
        let opacity = pulse::opacity(0);
        assert!((pulse::OPACITY_MIN..=pulse::OPACITY_MAX).contains(&opacity));

        let opacity = pulse::opacity(1000);
        assert!((pulse::OPACITY_MIN..=pulse::OPACITY_MAX).contains(&opacity));
    }

    #[test]
    fn test_pulse_glow_color_clamps_intensity() {
        let color = pulse::glow_color(-0.5);
        assert!((0.0..=1.0).contains(&color.a));

        let color = pulse::glow_color(0.5);
        assert!((0.0..=1.0).contains(&color.a));

        let color = pulse::glow_color(1.5);
        assert!((0.0..=1.0).contains(&color.a));
    }

    #[test]
    fn test_ripple_radius_increases_with_time() {
        let r1 = ripple::radius(0);
        let r2 = ripple::radius(100);
        let r3 = ripple::radius(400);

        assert!(r1 <= r2);
        assert!(r2 <= r3);
        assert!((0.0..=ripple::MAX_RADIUS).contains(&r3));
    }

    #[test]
    fn test_ripple_opacity_decreases_with_time() {
        let o1 = ripple::opacity(0);
        let o2 = ripple::opacity(200);
        let o3 = ripple::opacity(400);

        assert!(o1 >= o2);
        assert!(o2 >= o3);
        assert!((0.0..=1.0).contains(&o3));
    }
}
