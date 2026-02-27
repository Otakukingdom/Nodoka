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
/// - **Extra Small (xs)**: Minimal elevation (2px offset, 3px blur) - for very subtle depth
/// - **Small (sm)**: Subtle lift (4px offset, 6px blur) - for buttons and input fields
/// - **Medium (md)**: Moderate elevation (8px offset, 12px blur) - for cards and panels
/// - **Large (lg)**: High elevation (16px offset, 24px blur) - for modals and dropdowns
/// - **Extra Large (xl)**: Featured elevation (24px offset, 40px blur) - for hero content
/// - **Extra Extra Large (xxl)**: Maximum elevation (48px offset, 80px blur) - for prominent elements
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
    /// Equivalent to: 0 2px 3px rgba(0,0,0,0.05)
    ///
    /// Use for very subtle depth on small UI elements.
    #[must_use]
    pub const fn shadow_xs() -> Shadow {
        Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.05),
            offset: Vector::new(0.0, 2.0),
            blur_radius: 3.0,
        }
    }

    /// Subtle lift shadow for minor elevation (buttons, input fields)
    ///
    /// Equivalent to: 0 4px 6px rgba(0,0,0,0.08)
    ///
    /// Use for interactive elements that need minimal elevation like buttons
    /// and form inputs.
    #[must_use]
    pub const fn shadow_sm() -> Shadow {
        Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.08),
            offset: Vector::new(0.0, 4.0),
            blur_radius: 6.0,
        }
    }

    /// Card/button shadow for moderate elevation
    ///
    /// Equivalent to: 0 8px 12px rgba(0,0,0,0.13)
    ///
    /// Use for cards, raised panels, and floating buttons that need clear
    /// separation from the background.
    #[must_use]
    pub const fn shadow_md() -> Shadow {
        Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.13),
            offset: Vector::new(0.0, 8.0),
            blur_radius: 12.0,
        }
    }

    /// Modal/dropdown shadow for high elevation
    ///
    /// Equivalent to: 0 16px 24px rgba(0,0,0,0.18)
    ///
    /// Use for modals, dropdowns, and popovers that float above the main
    /// content layer.
    #[must_use]
    pub const fn shadow_lg() -> Shadow {
        Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.18),
            offset: Vector::new(0.0, 16.0),
            blur_radius: 24.0,
        }
    }

    /// Hero/featured shadow for maximum elevation
    ///
    /// Equivalent to: 0 24px 40px rgba(0,0,0,0.23)
    ///
    /// Use for hero images, featured content, or elements that need maximum
    /// visual prominence.
    #[must_use]
    pub const fn shadow_xl() -> Shadow {
        Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.23),
            offset: Vector::new(0.0, 24.0),
            blur_radius: 40.0,
        }
    }

    /// Extra large shadow for dramatic elevation
    ///
    /// Equivalent to: 0 48px 80px rgba(0,0,0,0.32)
    ///
    /// Use for very prominent elements like the continue listening card
    /// or featured modals.
    #[must_use]
    pub const fn shadow_xxl() -> Shadow {
        Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.32),
            offset: Vector::new(0.0, 48.0),
            blur_radius: 80.0,
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
    /// ===== Rose-Tinted Shadows for Premium Feel =====
    /// Rose-tinted shadow for elevated elements
    /// Creates distinctive character with subtle rose color instead of pure black
    #[must_use]
    pub const fn shadow_primary() -> Shadow {
        Shadow {
            color: Color::from_rgba(0.882, 0.114, 0.282, 0.16),
            offset: Vector::new(0.0, 8.0),
            blur_radius: 12.0,
        }
    }

    /// Warm shadow for depth in light theme
    /// Provides subtle warmth for sophisticated appearance
    #[must_use]
    pub const fn shadow_warm() -> Shadow {
        Shadow {
            color: Color::from_rgba(0.95, 0.85, 0.90, 0.14),
            offset: Vector::new(0.0, 4.0),
            blur_radius: 6.0,
        }
    }

    /// Rose-tinted medium shadow for cards
    /// Enhanced with rose color for distinctive character
    #[must_use]
    pub const fn shadow_md_rose() -> Shadow {
        Shadow {
            color: Color::from_rgba(0.882, 0.114, 0.282, 0.14),
            offset: Vector::new(0.0, 8.0),
            blur_radius: 12.0,
        }
    }

    /// Rose-tinted large shadow for modals
    /// Creates premium elevation with rose tint
    #[must_use]
    pub const fn shadow_lg_rose() -> Shadow {
        Shadow {
            color: Color::from_rgba(0.882, 0.114, 0.282, 0.20),
            offset: Vector::new(0.0, 16.0),
            blur_radius: 24.0,
        }
    }

    /// ===== Premium Glow Effects =====
    ///
    /// Premium sunset glow for distinctive branding
    /// Creates warm rose glow with sunset gradient colors
    #[must_use]
    pub fn glow_sunset(intensity: f32) -> Shadow {
        let alpha = (intensity.clamp(0.0, 1.0) * 0.45).clamp(0.0, 0.55);
        Shadow {
            color: Color::from_rgba(0.98, 0.32, 0.45, alpha),
            offset: Vector::new(0.0, 0.0),
            blur_radius: 14.0 * intensity,
        }
    }

    /// Premium cool glow for secondary highlights
    /// Creates calming lavender glow with cool gradient colors
    #[must_use]
    pub fn glow_cool(intensity: f32) -> Shadow {
        let alpha = (intensity.clamp(0.0, 1.0) * 0.40).clamp(0.0, 0.50);
        Shadow {
            color: Color::from_rgba(0.70, 0.65, 0.95, alpha),
            offset: Vector::new(0.0, 0.0),
            blur_radius: 12.0 * intensity,
        }
    }

    /// Premium gold glow for achievements and completion
    /// Creates sophisticated amber glow for celebratory moments
    #[must_use]
    pub fn glow_gold(intensity: f32) -> Shadow {
        let alpha = (intensity.clamp(0.0, 1.0) * 0.42).clamp(0.0, 0.52);
        Shadow {
            color: Color::from_rgba(1.0, 0.82, 0.40, alpha),
            offset: Vector::new(0.0, 0.0),
            blur_radius: 13.0 * intensity,
        }
    }

    /// Enhanced premium glow with maximum intensity
    /// Creates strongest glow effect for key interactive elements
    #[must_use]
    pub fn glow_premium(intensity: f32) -> Shadow {
        let alpha = (intensity.clamp(0.0, 1.0) * 0.50).clamp(0.0, 0.60);
        Shadow {
            color: Color::from_rgba(0.98, 0.32, 0.45, alpha),
            offset: Vector::new(0.0, 0.0),
            blur_radius: 16.0 * intensity,
        }
    }

    /// ===== Premium Colored Shadows =====
    /// These shadows use the brand color instead of pure black for distinctive character
    /// Rose-tinted extra small shadow for minimal elevation with brand presence
    #[must_use]
    pub const fn shadow_xs_brand() -> Shadow {
        Shadow {
            color: Color::from_rgba(0.882, 0.114, 0.282, 0.07),
            offset: Vector::new(0.0, 2.0),
            blur_radius: 3.0,
        }
    }

    /// Rose-tinted small shadow for buttons and inputs
    #[must_use]
    pub const fn shadow_sm_brand() -> Shadow {
        Shadow {
            color: Color::from_rgba(0.882, 0.114, 0.282, 0.11),
            offset: Vector::new(0.0, 4.0),
            blur_radius: 6.0,
        }
    }

    /// Rose-tinted medium shadow for cards
    #[must_use]
    pub const fn shadow_md_brand() -> Shadow {
        Shadow {
            color: Color::from_rgba(0.882, 0.114, 0.282, 0.17),
            offset: Vector::new(0.0, 8.0),
            blur_radius: 12.0,
        }
    }

    /// Rose-tinted large shadow for modals
    #[must_use]
    pub const fn shadow_lg_brand() -> Shadow {
        Shadow {
            color: Color::from_rgba(0.882, 0.114, 0.282, 0.24),
            offset: Vector::new(0.0, 16.0),
            blur_radius: 24.0,
        }
    }

    /// ===== Multi-Layer Shadow Utilities =====
    ///
    /// Multi-layer shadows create sophisticated depth by combining
    /// ambient and directional shadows for premium elevation.
    /// Returns a vector of shadows for premium layered elevation
    /// Combines ambient shadow for subtle lift with directional rose-tinted shadow for definition
    #[must_use]
    pub fn shadow_premium_layered() -> Vec<Shadow> {
        vec![
            // Ambient shadow for subtle lift
            Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.05),
                offset: Vector::new(0.0, 4.0),
                blur_radius: 8.0,
            },
            // Directional rose-tinted shadow for definition
            Shadow {
                color: Color::from_rgba(0.882, 0.114, 0.282, 0.10),
                offset: Vector::new(0.0, 8.0),
                blur_radius: 16.0,
            },
        ]
    }

    /// Premium multi-layer shadow for cards
    /// Creates sophisticated elevation with warm rose-tinted shadow
    #[must_use]
    pub fn shadow_card_premium() -> Vec<Shadow> {
        vec![
            Shadow {
                color: Color::from_rgba(0.95, 0.85, 0.90, 0.10),
                offset: Vector::new(0.0, 4.0),
                blur_radius: 8.0,
            },
            Shadow {
                color: Color::from_rgba(0.882, 0.114, 0.282, 0.15),
                offset: Vector::new(0.0, 10.0),
                blur_radius: 20.0,
            },
        ]
    }

    /// Multi-layer shadow for modal cards
    /// Creates dramatic elevation with stronger rose tint
    #[must_use]
    pub fn shadow_modal_premium() -> Vec<Shadow> {
        vec![
            Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.06),
                offset: Vector::new(0.0, 8.0),
                blur_radius: 16.0,
            },
            Shadow {
                color: Color::from_rgba(0.882, 0.114, 0.282, 0.18),
                offset: Vector::new(0.0, 16.0),
                blur_radius: 32.0,
            },
            Shadow {
                color: Color::from_rgba(0.98, 0.32, 0.45, 0.10),
                offset: Vector::new(0.0, 24.0),
                blur_radius: 48.0,
            },
        ]
    }

    /// Enhanced premium glow with maximum intensity
    /// Creates strongest glow effect for key interactive elements
    #[must_use]
    pub fn glow_premium_max(intensity: f32) -> Shadow {
        let alpha = (intensity.clamp(0.0, 1.0) * 0.6).clamp(0.0, 0.7);
        Shadow {
            color: Color::from_rgba(0.98, 0.32, 0.45, alpha),
            offset: Vector::new(0.0, 0.0),
            blur_radius: 20.0 * intensity,
        }
    }

    /// Multi-layer glow for active playback indicators
    /// Combines inner glow with outer glow for dramatic effect
    #[must_use]
    pub fn glow_playback_active() -> Vec<Shadow> {
        vec![
            Shadow {
                color: Color::from_rgba(0.98, 0.32, 0.45, 0.3),
                offset: Vector::new(0.0, 0.0),
                blur_radius: 6.0,
            },
            Shadow {
                color: Color::from_rgba(0.882, 0.114, 0.282, 0.2),
                offset: Vector::new(0.0, 0.0),
                blur_radius: 12.0,
            },
        ]
    }

    /// ===== Premium Colored Shadow Functions =====
    ///
    /// These shadows use brand colors instead of pure black for distinctive character.
    /// Creates a colored shadow with sunset gradient tint for premium elevation.
    /// Uses sunset gradient colors for warm, sophisticated elevation.
    #[must_use]
    pub fn shadow_sunset_tinted(opacity: f32, offset: f32, blur: f32) -> Shadow {
        let alpha = (opacity.clamp(0.0, 1.0) * 0.18).clamp(0.0, 0.30);
        Shadow {
            color: Color {
                r: 0.98 * alpha,
                g: 0.32 * alpha,
                b: 0.45 * alpha,
                a: alpha,
            },
            offset: Vector::new(0.0, offset),
            blur_radius: blur,
        }
    }

    /// Creates a colored shadow with aurora gradient tint
    /// Provides cool blue-to-purple elevation for variety
    #[must_use]
    pub fn shadow_aurora_tinted(opacity: f32, offset: f32, blur: f32) -> Shadow {
        let alpha = (opacity.clamp(0.0, 1.0) * 0.16).clamp(0.0, 0.28);
        Shadow {
            color: Color {
                r: 0.60 * alpha,
                g: 0.78 * alpha,
                b: 0.95 * alpha,
                a: alpha,
            },
            offset: Vector::new(0.0, offset),
            blur_radius: blur,
        }
    }

    /// Creates a colored shadow with emerald gradient tint
    /// Provides fresh green elevation for success/accent contexts
    #[must_use]
    pub fn shadow_emerald_tinted(opacity: f32, offset: f32, blur: f32) -> Shadow {
        let alpha = (opacity.clamp(0.0, 1.0) * 0.15).clamp(0.0, 0.25);
        Shadow {
            color: Color {
                r: 0.15 * alpha,
                g: 0.75 * alpha,
                b: 0.55 * alpha,
                a: alpha,
            },
            offset: Vector::new(0.0, offset),
            blur_radius: blur,
        }
    }

    /// Creates a colored shadow with gold gradient tint
    /// Provides sophisticated amber elevation for premium elements
    #[must_use]
    pub fn shadow_gold_tinted(opacity: f32, offset: f32, blur: f32) -> Shadow {
        let alpha = (opacity.clamp(0.0, 1.0) * 0.17).clamp(0.0, 0.28);
        Shadow {
            color: Color {
                r: 1.0 * alpha,
                g: 0.82 * alpha,
                b: 0.40 * alpha,
                a: alpha,
            },
            offset: Vector::new(0.0, offset),
            blur_radius: blur,
        }
    }

    /// ===== Aurora Rose Brand Shadow =====
    ///
    /// Creates Aurora rose-tinted shadow for premium brand elevation
    /// Uses distinctive Aurora Rose gradient colors for brand identity
    #[must_use]
    pub const fn aurora_shadow() -> Shadow {
        Shadow {
            color: Color::from_rgba(0.88, 0.12, 0.28, 0.15),
            offset: Vector::new(0.0, 4.0),
            blur_radius: 16.0,
        }
    }

    /// Creates Aurora rose-tinted shadow with configurable intensity
    /// Provides distinctive brand shadow for cards and elevated elements
    #[must_use]
    pub fn aurora_shadow_tinted(intensity: f32) -> Shadow {
        let alpha = (intensity.clamp(0.0, 1.0) * 0.18).clamp(0.0, 0.25);
        Shadow {
            color: Color {
                r: 0.88 * alpha,
                g: 0.12 * alpha,
                b: 0.28 * alpha,
                a: alpha,
            },
            offset: Vector::new(0.0, 4.0 * intensity),
            blur_radius: 16.0 * intensity,
        }
    }

    /// Creates Celestial purple-rose shadow for dramatic elevation
    /// Provides sophisticated evening-themed shadow for premium elements
    #[must_use]
    pub const fn celestial_shadow() -> Shadow {
        Shadow {
            color: Color::from_rgba(0.50, 0.10, 0.48, 0.18),
            offset: Vector::new(0.0, 6.0),
            blur_radius: 20.0,
        }
    }

    /// Creates Velvet plum shadow for luxurious dark mode depth
    /// Provides rich burgundy-tinted shadow for premium dark theme elevation
    #[must_use]
    pub const fn velvet_shadow() -> Shadow {
        Shadow {
            color: Color::from_rgba(0.25, 0.08, 0.35, 0.20),
            offset: Vector::new(0.0, 8.0),
            blur_radius: 24.0,
        }
    }

    /// Aurora Rose glow for premium brand presence
    /// Creates soft rose glow for distinctive brand elevation
    #[must_use]
    pub fn glow_aurora(intensity: f32) -> Shadow {
        let alpha = (intensity.clamp(0.0, 1.0) * 0.40).clamp(0.0, 0.55);
        Shadow {
            color: Color::from_rgba(0.96, 0.81, 1.0, alpha),
            offset: Vector::new(0.0, 0.0),
            blur_radius: 12.0 * intensity,
        }
    }

    /// Celestial glow for dramatic evening accents
    /// Creates deep purple-rose glow for premium highlights
    #[must_use]
    pub fn glow_celestial(intensity: f32) -> Shadow {
        let alpha = (intensity.clamp(0.0, 1.0) * 0.45).clamp(0.0, 0.60);
        Shadow {
            color: Color::from_rgba(0.50, 0.10, 0.48, alpha),
            offset: Vector::new(0.0, 0.0),
            blur_radius: 14.0 * intensity,
        }
    }
}
