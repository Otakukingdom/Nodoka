//! Visual effects for transitions, animations, and shadows.
//!
//! This module provides utilities for creating smooth transitions,
//! microinteractions, and elevation through shadows.

use iced::{Color, Shadow, Vector};

/// Transition duration constants for smooth animations
pub mod transitions {
    /// Fast transition: 150ms
    pub const FAST: u64 = 150;
    /// Normal transition: 200ms
    pub const NORMAL: u64 = 200;
    /// Slow transition: 300ms
    pub const SLOW: u64 = 300;
}

/// Cursor styles for interactive elements
///
/// Provides cursor type constants for different interaction states.
/// All clickable elements must use cursor-pointer for clear affordance.
pub mod cursor {
    use iced::mouse;

    /// Returns cursor style for interactive elements
    ///
    /// All clickable elements use cursor-pointer to provide clear visual
    /// feedback that the element is interactive.
    #[must_use]
    pub const fn clickable() -> mouse::Interaction {
        mouse::Interaction::Pointer
    }

    /// Returns cursor for disabled elements
    ///
    /// Provides visual feedback that an element is not currently interactive.
    #[must_use]
    pub const fn disabled() -> mouse::Interaction {
        mouse::Interaction::NotAllowed
    }

    /// Returns default cursor
    #[must_use]
    pub fn default() -> mouse::Interaction {
        mouse::Interaction::default()
    }

    /// Returns text cursor for text input areas
    #[must_use]
    pub const fn text() -> mouse::Interaction {
        mouse::Interaction::Text
    }
}

/// Converts a `u64` millisecond value to `f32` for animation calculations.
///
/// Clamps to `u16::MAX` (65535 ms) before converting to `f32` to ensure a
/// lossless widening conversion. All animation durations in this codebase
/// are well below this bound; transitions range from 150-300 ms.
pub(crate) fn u64_to_f32(value: u64) -> f32 {
    f32::from(u16::try_from(value).unwrap_or(u16::MAX))
}

/// Returns appropriate transition duration based on reduced motion preference.
#[must_use]
pub const fn transition_duration(reduced_motion: bool) -> u64 {
    if reduced_motion {
        0
    } else {
        transitions::NORMAL
    }
}

/// Returns whether animations should be enabled.
#[must_use]
pub const fn animations_enabled(reduced_motion: bool) -> bool {
    !reduced_motion
}

/// Easing function utilities for smooth transitions.
pub mod easing {
    /// Linear easing (no acceleration) - use sparingly
    ///
    /// Returns input value unchanged. Generally not recommended for UI transitions
    /// as it feels mechanical. Use for progress indicators or mechanical effects.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let t = 0.5;
    /// assert_eq!(easing::linear(t), 0.5);
    /// ```
    #[must_use]
    pub const fn linear(t: f32) -> f32 {
        // Manual clamp implementation for const fn
        if t < 0.0 {
            0.0
        } else if t > 1.0 {
            1.0
        } else {
            t
        }
    }

    /// Ease-in-out (smooth start and end)
    ///
    /// Starts slowly, accelerates in the middle, then decelerates at the end.
    /// Best for most UI transitions as it feels natural and responsive.
    ///
    /// Uses quadratic easing formula for smooth acceleration/deceleration.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let t = 0.5;
    /// let eased = easing::ease_in_out(t); // Returns ~0.5 but with smooth curve
    /// ```
    #[must_use]
    pub fn ease_in_out(t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        if t < 0.5 {
            2.0 * t * t
        } else {
            t.mul_add(2.0_f32.mul_add(-t, 4.0), -1.0)
        }
    }

    /// Ease-out (slow finish)
    ///
    /// Starts quickly and decelerates towards the end.
    /// Best for entering elements (modals, dropdowns) to feel responsive.
    ///
    /// Uses quadratic easing for smooth deceleration.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let t = 0.5;
    /// let eased = easing::ease_out(t); // Decelerates towards end
    /// ```
    #[must_use]
    pub fn ease_out(t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        t * (2.0 - t)
    }

    /// Ease-in (slow start)
    ///
    /// Starts slowly and accelerates towards the end.
    /// Best for exiting elements to feel natural.
    ///
    /// Uses quadratic easing for smooth acceleration.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let t = 0.5;
    /// let eased = easing::ease_in(t); // Accelerates towards end
    /// ```
    #[must_use]
    pub fn ease_in(t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        t * t
    }

    /// Ease-out-bounce (playful overshoot effect)
    ///
    /// Creates a bouncing effect at the end of the animation.
    /// Best for playful feedback on button clicks or confirmations.
    /// Use sparingly to avoid overwhelming the UI.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let t = 0.8;
    /// let eased = easing::ease_out_bounce(t); // Bounces at end
    /// ```
    #[must_use]
    pub fn ease_out_bounce(t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        if t < 1.0 / 2.75 {
            let t2 = t * t;
            7.5625 * t2
        } else if t < 2.0 / 2.75 {
            let t = t - 1.5 / 2.75;
            7.5625_f32.mul_add(t * t, 0.75)
        } else if t < 2.5 / 2.75 {
            let t = t - 2.25 / 2.75;
            7.5625_f32.mul_add(t * t, 0.9375)
        } else {
            let t = t - 2.625 / 2.75;
            7.5625_f32.mul_add(t * t, 0.984_375)
        }
    }

    /// Ease-out-elastic (spring-like effect)
    ///
    /// Creates a spring-like oscillation at the end of the animation.
    /// Best for modal entrances or important notifications.
    /// Use for attention-grabbing moments only.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let t = 0.9;
    /// let eased = easing::ease_out_elastic(t); // Spring effect at end
    /// ```
    #[must_use]
    pub fn ease_out_elastic(t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        if t <= f32::EPSILON {
            0.0
        } else if (1.0 - t).abs() <= f32::EPSILON {
            1.0
        } else {
            let p = 0.3;
            let s = p / 4.0;
            let t1 = t - 1.0;
            (-10.0 * t1)
                .exp2()
                .mul_add(((t1 - s) * (2.0 * std::f32::consts::PI) / p).sin(), 1.0)
        }
    }
}

/// Shadow depth utilities for elevation hierarchy
///
/// Shadow depth utilities for elevation hierarchy.
/// Shadows create visual depth and establish hierarchy between UI layers.
///
/// # Shadow Levels
///
/// - **Extra Small (xs)**: Minimal elevation (0.5px offset, 1px blur) - for very subtle depth
/// - **Small (sm)**: Subtle lift (1px offset, 2px blur) - for buttons and input fields
/// - **Medium (md)**: Moderate elevation (2px offset, 4px blur) - for cards and panels
/// - **Large (lg)**: High elevation (4px offset, 8px blur) - for modals and dropdowns
/// - **Extra Large (xl)**: Featured elevation (8px offset, 16px blur) - for hero content
/// - **Extra Extra Large (xxl)**: Maximum elevation (12px offset, 24px blur) - for prominent elements
///
/// # Examples
///
/// ```ignore
/// use nodoka::ui::styles::shadows;
/// use iced::widget::container;
///
/// // Apply shadow to a modal card
/// container(content)
///     .style(|_theme| container::Style {
///         shadow: shadows::shadow_xl(),
///         ..Default::default()
///     })
/// ```
pub mod shadows {
    use super::{Color, Shadow, Vector};

    /// Extra subtle shadow for minimal elevation
    ///
    /// Equivalent to: 0 0.5px 1px rgba(0,0,0,0.04)
    ///
    /// Use for very subtle depth on small UI elements.
    #[must_use]
    pub const fn shadow_xs() -> Shadow {
        Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.04),
            offset: Vector::new(0.0, 0.5),
            blur_radius: 1.0,
        }
    }

    /// Subtle lift shadow for minor elevation (buttons, input fields)
    ///
    /// Equivalent to: 0 1px 2px rgba(0,0,0,0.06)
    ///
    /// Use for interactive elements that need minimal elevation like buttons
    /// and form inputs.
    #[must_use]
    pub const fn shadow_sm() -> Shadow {
        Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.06),
            offset: Vector::new(0.0, 1.0),
            blur_radius: 2.0,
        }
    }

    /// Card/button shadow for moderate elevation
    ///
    /// Equivalent to: 0 2px 4px rgba(0,0,0,0.10)
    ///
    /// Use for cards, raised panels, and floating buttons that need clear
    /// separation from the background.
    #[must_use]
    pub const fn shadow_md() -> Shadow {
        Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.10),
            offset: Vector::new(0.0, 2.0),
            blur_radius: 4.0,
        }
    }

    /// Modal/dropdown shadow for high elevation
    ///
    /// Equivalent to: 0 4px 8px rgba(0,0,0,0.14)
    ///
    /// Use for modals, dropdowns, and popovers that float above the main
    /// content layer.
    #[must_use]
    pub const fn shadow_lg() -> Shadow {
        Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.14),
            offset: Vector::new(0.0, 4.0),
            blur_radius: 8.0,
        }
    }

    /// Hero/featured shadow for maximum elevation
    ///
    /// Equivalent to: 0 8px 16px rgba(0,0,0,0.18)
    ///
    /// Use for hero images, featured content, or elements that need maximum
    /// visual prominence.
    #[must_use]
    pub const fn shadow_xl() -> Shadow {
        Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.18),
            offset: Vector::new(0.0, 8.0),
            blur_radius: 16.0,
        }
    }

    /// Extra large shadow for dramatic elevation
    ///
    /// Equivalent to: 0 12px 24px rgba(0,0,0,0.26)
    ///
    /// Use for very prominent elements like the continue listening card
    /// or featured modals.
    #[must_use]
    pub const fn shadow_xxl() -> Shadow {
        Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.26),
            offset: Vector::new(0.0, 12.0),
            blur_radius: 24.0,
        }
    }

    /// Inner shadow for inset depth effect
    ///
    /// Creates an inset shadow for elements like progress bars to give
    /// them a carved/recessed appearance.
    ///
    /// Equivalent to: inset 0 1px 2px rgba(0,0,0,0.1)
    #[must_use]
    pub const fn inner_shadow() -> Shadow {
        Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.1),
            offset: Vector::new(0.0, -1.0),
            blur_radius: 2.0,
        }
    }

    /// Glow effect for active state indicators
    ///
    /// Creates a colored glow using the primary color with reduced opacity.
    /// Use for active playback indicators, focused elements, or highlights.
    #[must_use]
    pub fn glow_primary(intensity: f32) -> Shadow {
        let alpha = (intensity.clamp(0.0, 1.0) * 0.35).clamp(0.0, 0.5);
        Shadow {
            color: Color::from_rgba(0.882, 0.114, 0.282, alpha),
            offset: Vector::new(0.0, 0.0),
            blur_radius: 8.0 * intensity,
        }
    }

    /// Success glow for positive feedback
    ///
    /// Creates a green glow for success states, completed actions, and
    /// positive feedback indicators.
    #[must_use]
    pub fn glow_success(intensity: f32) -> Shadow {
        let alpha = (intensity.clamp(0.0, 1.0) * 0.3).clamp(0.0, 0.45);
        Shadow {
            color: Color::from_rgba(0.13, 0.77, 0.38, alpha), // #22c762 green
            offset: Vector::new(0.0, 0.0),
            blur_radius: 8.0 * intensity,
        }
    }

    /// Warning glow for caution states
    ///
    /// Creates an amber glow for warning states, pending actions, and
    /// caution indicators.
    #[must_use]
    pub fn glow_warning(intensity: f32) -> Shadow {
        let alpha = (intensity.clamp(0.0, 1.0) * 0.3).clamp(0.0, 0.45);
        Shadow {
            color: Color::from_rgba(0.95, 0.71, 0.13, alpha), // #f2b521 amber
            offset: Vector::new(0.0, 0.0),
            blur_radius: 8.0 * intensity,
        }
    }

    /// Error glow for negative feedback
    ///
    /// Creates a red glow for error states, failed actions, and
    /// negative feedback indicators.
    #[must_use]
    pub fn glow_error(intensity: f32) -> Shadow {
        let alpha = (intensity.clamp(0.0, 1.0) * 0.35).clamp(0.0, 0.5);
        Shadow {
            color: Color::from_rgba(0.92, 0.11, 0.34, alpha), // #eb1c57 red
            offset: Vector::new(0.0, 0.0),
            blur_radius: 10.0 * intensity,
        }
    }

    // Border color fallbacks for iced elements that don't support shadows
    /// Small shadow equivalent as border color for compatibility
    pub const SM_BORDER: Color = Color::from_rgb(0.85, 0.85, 0.85); // #D9D9D9
    /// Medium shadow equivalent as border color for compatibility
    pub const MD_BORDER: Color = Color::from_rgb(0.8, 0.8, 0.8); // #CCCCCC
    /// Large shadow equivalent as border color for compatibility
    pub const LG_BORDER: Color = Color::from_rgb(0.75, 0.75, 0.75); // #BFBFBF
}
