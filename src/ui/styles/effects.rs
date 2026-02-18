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
/// Provides cursor type constants for different interaction states per ui-ux-pro-max guidelines.
/// All clickable elements must use cursor-pointer for clear affordance.
pub mod cursor {
    use iced::mouse;

    /// Returns cursor style for interactive elements
    ///
    /// Per ui-ux-pro-max: all clickable elements must have cursor-pointer
    /// to provide clear visual feedback that the element is interactive.
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

/// Animation and transition utilities with reduced motion support
///
/// Provides utilities for applying animations with respect to user's
/// reduced motion preferences per WCAG 2.1 2.3.3.
///
/// # Reduced Motion Support
///
/// When `prefers_reduced_motion` is enabled:
/// - All transition durations become 0ms (instant)
/// - Animations are disabled
///
/// This ensures compliance with accessibility guidelines for users with
/// vestibular disorders or motion sensitivity.
///
/// # Enhanced UX Features (v0.2.1+)
///
/// This module provides advanced animation patterns following ui-ux-pro-max guidelines:
///
/// ## Easing Functions
///
/// Smooth transitions using standard easing functions:
///
/// ```ignore
/// use nodoka::ui::styles::animations::easing;
///
/// let t = 0.5; // Progress from 0.0 to 1.0
/// let eased = easing::ease_in_out(t); // Smooth acceleration and deceleration
/// ```
///
/// ## Pulse Animation
///
/// Visual feedback for active states:
///
/// ```ignore
/// use nodoka::ui::styles::animations::PulseAnimation;
///
/// let mut pulse = PulseAnimation::new(1000);
/// pulse.update(16); // Update with frame delta (ms)
/// let opacity = pulse.opacity(); // Get current opacity (0.5 to 1.0)
/// ```
pub mod animations {
    use super::transitions;

    fn u64_to_f32(value: u64) -> f32 {
        // Clamp to u16::MAX (65535ms) — all animation durations in this codebase
        // are well below this bound; transitions range from 150-300ms.
        f32::from(u16::try_from(value).unwrap_or(u16::MAX))
    }

    /// Returns appropriate transition duration based on reduced motion preference
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let duration = animations::transition_duration(state.prefers_reduced_motion);
    /// // duration is 0 if reduced motion is preferred, otherwise 200ms
    /// ```
    #[must_use]
    pub const fn transition_duration(reduced_motion: bool) -> u64 {
        if reduced_motion {
            0 // Instant transitions when reduced motion is preferred
        } else {
            transitions::NORMAL // 200ms for smooth interactions
        }
    }

    /// Returns whether animations should be enabled
    ///
    /// # Examples
    ///
    /// ```ignore
    /// if animations::animations_enabled(state.prefers_reduced_motion) {
    ///     // Apply animated transitions
    /// } else {
    ///     // Apply instant state changes
    /// }
    /// ```
    #[must_use]
    pub const fn animations_enabled(reduced_motion: bool) -> bool {
        !reduced_motion
    }

    /// Easing function utilities for smooth transitions
    ///
    /// Per ui-ux-pro-max guidelines, linear motion feels robotic.
    /// Use ease-out for entering elements and ease-in for exiting elements.
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
        /// Per ui-ux-pro-max: use sparingly to avoid overwhelming UI.
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
        /// Per ui-ux-pro-max: use for attention-grabbing moments only.
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

    /// Fade transition utility for opacity animations
    ///
    /// Manages smooth opacity transitions from one value to another over time.
    /// Per ui-ux-pro-max: use for modals, tooltips, and toast notifications.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use nodoka::ui::styles::animations::FadeTransition;
    ///
    /// let mut fade = FadeTransition::new(200, 0.0, 1.0);
    /// fade.update(16); // Update with 16ms frame delta
    /// let opacity = fade.opacity(); // Get current opacity
    /// ```
    #[derive(Debug, Clone)]
    pub struct FadeTransition {
        /// Current elapsed time in milliseconds
        elapsed_ms: u64,
        /// Total duration of transition in milliseconds
        duration_ms: u64,
        /// Starting opacity value (0.0 to 1.0)
        from: f32,
        /// Target opacity value (0.0 to 1.0)
        to: f32,
    }

    impl FadeTransition {
        /// Creates a new fade transition
        ///
        /// # Arguments
        ///
        /// * `duration_ms` - Transition duration in milliseconds (0 for instant)
        /// * `from` - Starting opacity (0.0 to 1.0)
        /// * `to` - Target opacity (0.0 to 1.0)
        ///
        /// # Examples
        ///
        /// ```ignore
        /// let fade = FadeTransition::new(200, 0.0, 1.0); // Fade in over 200ms
        /// ```
        #[must_use]
        pub const fn new(duration_ms: u64, from: f32, to: f32) -> Self {
            Self {
                elapsed_ms: 0,
                duration_ms,
                from,
                to,
            }
        }

        /// Updates transition progress
        ///
        /// # Arguments
        ///
        /// * `delta_ms` - Milliseconds elapsed since last update
        ///
        /// # Examples
        ///
        /// ```ignore
        /// fade.update(16); // Advance by one frame at 60fps
        /// ```
        pub fn update(&mut self, delta_ms: u64) {
            self.elapsed_ms = (self.elapsed_ms + delta_ms).min(self.duration_ms);
        }

        /// Returns current opacity value (0.0 to 1.0)
        ///
        /// Uses ease-out easing for smooth deceleration.
        ///
        /// # Examples
        ///
        /// ```ignore
        /// let opacity = fade.opacity();
        /// // Use opacity for element alpha channel
        /// ```
        #[must_use]
        pub fn opacity(&self) -> f32 {
            if self.duration_ms == 0 {
                return self.to;
            }
            let progress = u64_to_f32(self.elapsed_ms) / u64_to_f32(self.duration_ms);
            let eased = easing::ease_out(progress);
            self.to.mul_add(eased, self.from * (1.0 - eased))
        }

        /// Returns whether transition is complete
        #[must_use]
        pub const fn is_complete(&self) -> bool {
            self.elapsed_ms >= self.duration_ms
        }

        /// Resets transition to start
        pub const fn reset(&mut self) {
            self.elapsed_ms = 0;
        }
    }

    /// Scale transition utility for size animations
    ///
    /// Manages smooth scale transitions for microinteractions.
    /// Per ui-ux-pro-max: use for button press feedback and modal entrances.
    ///
    /// Note: iced 0.14 doesn't support CSS transform scale directly,
    /// but this utility provides the calculated values for manual application
    /// (e.g., through size adjustments).
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use nodoka::ui::styles::animations::ScaleTransition;
    ///
    /// let mut scale = ScaleTransition::new(200, 0.95, 1.0);
    /// scale.update(16); // Update with 16ms frame delta
    /// let scale_value = scale.scale(); // Get current scale (0.95 to 1.0)
    /// ```
    #[derive(Debug, Clone)]
    pub struct ScaleTransition {
        /// Current elapsed time in milliseconds
        elapsed_ms: u64,
        /// Total duration of transition in milliseconds
        duration_ms: u64,
        /// Starting scale value (typically 0.9 to 1.1)
        from: f32,
        /// Target scale value (typically 1.0)
        to: f32,
    }

    impl ScaleTransition {
        /// Creates a new scale transition
        ///
        /// # Arguments
        ///
        /// * `duration_ms` - Transition duration in milliseconds (0 for instant)
        /// * `from` - Starting scale (typically 0.95 for subtle effect)
        /// * `to` - Target scale (typically 1.0 for normal size)
        ///
        /// # Examples
        ///
        /// ```ignore
        /// let scale = ScaleTransition::new(200, 0.95, 1.0); // Subtle pop-in
        /// ```
        #[must_use]
        pub const fn new(duration_ms: u64, from: f32, to: f32) -> Self {
            Self {
                elapsed_ms: 0,
                duration_ms,
                from,
                to,
            }
        }

        /// Updates transition progress
        ///
        /// # Arguments
        ///
        /// * `delta_ms` - Milliseconds elapsed since last update
        pub fn update(&mut self, delta_ms: u64) {
            self.elapsed_ms = (self.elapsed_ms + delta_ms).min(self.duration_ms);
        }

        /// Returns current scale value
        ///
        /// Uses ease-out easing for smooth deceleration.
        ///
        /// # Examples
        ///
        /// ```ignore
        /// let scale = scale_transition.scale();
        /// // Apply scale to element size or use for transform calculation
        /// ```
        #[must_use]
        pub fn scale(&self) -> f32 {
            if self.duration_ms == 0 {
                return self.to;
            }
            let progress = u64_to_f32(self.elapsed_ms) / u64_to_f32(self.duration_ms);
            let eased = easing::ease_out(progress);
            self.to.mul_add(eased, self.from * (1.0 - eased))
        }

        /// Returns whether transition is complete
        #[must_use]
        pub const fn is_complete(&self) -> bool {
            self.elapsed_ms >= self.duration_ms
        }

        /// Resets transition to start
        pub const fn reset(&mut self) {
            self.elapsed_ms = 0;
        }
    }

    /// Calculates stagger delay for list item animations
    ///
    /// Creates a cascading animation effect by returning progressively
    /// longer delays for each item index. Per ui-ux-pro-max: use 50ms
    /// stagger for smooth visual flow without excessive total duration.
    ///
    /// # Arguments
    ///
    /// * `index` - Zero-based index of the item
    /// * `stagger_ms` - Delay between each item in milliseconds (typically 50ms)
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let delay = animations::stagger_delay(0, 50); // 0ms for first item
    /// let delay = animations::stagger_delay(1, 50); // 50ms for second item
    /// let delay = animations::stagger_delay(2, 50); // 100ms for third item
    /// ```
    #[must_use]
    pub const fn stagger_delay(index: usize, stagger_ms: u64) -> u64 {
        index as u64 * stagger_ms
    }

    /// Calculates stagger opacity for list item animations
    ///
    /// Returns opacity value based on stagger timing, creating a fade-in effect.
    /// Per ui-ux-pro-max: cap stagger effect at 15 items to prevent excessive delays.
    ///
    /// # Arguments
    ///
    /// * `index` - Zero-based index of the item
    /// * `delay_ms` - Delay between items in milliseconds (typically 50ms)
    /// * `elapsed_ms` - Total elapsed time since animation start
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let opacity = animations::stagger_opacity(2, 50, 200);
    /// // Returns 1.0 if elapsed_ms >= item_delay + 200, otherwise interpolated
    /// ```
    #[must_use]
    pub fn stagger_opacity(index: usize, delay_ms: u64, elapsed_ms: u64) -> f32 {
        // Cap stagger at 15 items per ui-ux-pro-max guidelines
        let capped_index = index.min(15);
        let item_delay = stagger_delay(capped_index, delay_ms);

        if elapsed_ms < item_delay {
            0.0
        } else {
            let progress = u64_to_f32(elapsed_ms - item_delay) / 200.0;
            easing::ease_out(progress.min(1.0))
        }
    }

    /// Vertical translation distance for stagger animations (pixels)
    ///
    /// This constant is shared with layout padding calculations to prevent
    /// layout shifts during animation.
    pub const STAGGER_TRANSLATE_Y_PX: f32 = 20.0;

    /// Calculates stagger vertical translation for list item animations
    ///
    /// Returns Y-offset value for slide-up animation effect.
    /// Items start 20px below their final position and slide up.
    ///
    /// # Arguments
    ///
    /// * `index` - Zero-based index of the item
    /// * `delay_ms` - Delay between items in milliseconds (typically 50ms)
    /// * `elapsed_ms` - Total elapsed time since animation start
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let offset_y = animations::stagger_translate_y(2, 50, 200);
    /// // Returns 0.0 when fully animated, 20.0 before animation starts
    /// ```
    #[must_use]
    pub fn stagger_translate_y(index: usize, delay_ms: u64, elapsed_ms: u64) -> f32 {
        // Cap stagger at 15 items per ui-ux-pro-max guidelines
        let capped_index = index.min(15);
        let item_delay = stagger_delay(capped_index, delay_ms);

        if elapsed_ms < item_delay {
            STAGGER_TRANSLATE_Y_PX // Start below
        } else {
            let progress = u64_to_f32(elapsed_ms - item_delay) / 200.0;
            STAGGER_TRANSLATE_Y_PX * (1.0 - easing::ease_out(progress.min(1.0)))
        }
    }

    /// Pulse animation for indicating active/playing state
    ///
    /// Creates a smooth pulsing opacity effect for visual feedback.
    /// Per ui-ux-pro-max: continuous animations should only be used for
    /// loading indicators or active state feedback.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use nodoka::ui::styles::animations::PulseAnimation;
    ///
    /// let mut pulse = PulseAnimation::new(1000); // 1 second cycle
    /// pulse.update(16); // Update with 16ms frame delta
    /// let opacity = pulse.opacity(); // Get current opacity (0.5 to 1.0)
    /// ```
    #[derive(Debug, Clone)]
    pub struct PulseAnimation {
        /// Current phase (0.0 to 1.0)
        phase: f32,
        /// Animation duration in milliseconds
        duration_ms: u64,
    }

    impl PulseAnimation {
        /// Creates a new pulse animation with specified duration
        ///
        /// # Arguments
        ///
        /// * `duration_ms` - Full cycle duration in milliseconds (typically 1000-2000ms)
        ///
        /// # Examples
        ///
        /// ```ignore
        /// let pulse = PulseAnimation::new(1500); // 1.5 second pulse cycle
        /// ```
        #[must_use]
        pub const fn new(duration_ms: u64) -> Self {
            Self {
                phase: 0.0,
                duration_ms,
            }
        }

        /// Updates animation phase based on elapsed time
        ///
        /// Call this each frame with the milliseconds elapsed since last update.
        ///
        /// # Arguments
        ///
        /// * `delta_ms` - Milliseconds elapsed since last update (typically 16ms for 60fps)
        ///
        /// # Examples
        ///
        /// ```ignore
        /// pulse.update(16); // Advance by one frame at 60fps
        /// ```
        pub fn update(&mut self, delta_ms: u64) {
            if self.duration_ms == 0 {
                return; // Avoid division by zero
            }
            let progress = u64_to_f32(delta_ms) / u64_to_f32(self.duration_ms);
            self.phase = (self.phase + progress).rem_euclid(1.0);
        }

        /// Returns current opacity for pulse effect (0.0 to 1.0)
        ///
        /// Uses sine wave for smooth pulsing motion. Returns values from 0.0 to 1.0
        /// for full pulsing effect.
        ///
        /// # Examples
        ///
        /// ```ignore
        /// let opacity = pulse.opacity();
        /// // Use opacity to modulate element alpha channel
        /// ```
        #[must_use]
        pub fn opacity(&self) -> f32 {
            // Ensure phase is in valid range for sine calculation
            let phase = self.phase.rem_euclid(1.0);
            // sin ranges from -1 to 1, so 0.5 + 0.5 * sin ranges from 0 to 1
            (phase * std::f32::consts::PI * 2.0).sin().mul_add(0.5, 0.5)
        }

        /// Resets animation to start
        pub const fn reset(&mut self) {
            self.phase = 0.0;
        }
    }

    /// Modal overlay animation system
    ///
    /// Provides state machine and animation utilities for modal entrance/exit
    /// transitions following ui-ux-pro-max guidelines.
    ///
    /// # Animation Specifications
    ///
    /// - **Entrance**: 300ms ease-out with backdrop fade-in and content scale-up
    /// - **Exit**: 200ms ease-in with backdrop fade-out and content scale-down
    /// - **Reduced motion**: Instant appearance/disappearance when enabled
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use nodoka::ui::styles::animations::ModalState;
    ///
    /// let mut modal = ModalState::Opening { elapsed_ms: 0 };
    /// modal.update(16); // Update with frame delta
    /// let opacity = modal.backdrop_opacity();
    /// let scale = modal.content_scale();
    /// ```
    pub mod modal {
        use super::easing;

        /// Modal animation state machine
        ///
        /// Tracks current state of modal animation for proper rendering.
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub enum ModalState {
            /// Modal is not visible
            Closed,
            /// Modal is animating in (0-300ms)
            Opening { elapsed_ms: u64 },
            /// Modal is fully visible
            Open,
            /// Modal is animating out (0-200ms)
            Closing { elapsed_ms: u64 },
        }

        impl ModalState {
            /// Duration of modal opening animation in milliseconds
            pub const OPENING_DURATION_MS: u64 = 300;

            /// Duration of modal closing animation in milliseconds
            pub const CLOSING_DURATION_MS: u64 = 200;

            /// Maximum backdrop opacity (semi-transparent black)
            pub const MAX_BACKDROP_OPACITY: f32 = 0.5;

            /// Updates modal animation state
            ///
            /// Call this each frame with elapsed time since last update.
            /// Automatically transitions to `Open` or `Closed` when animation completes.
            ///
            /// # Arguments
            ///
            /// * `delta_ms` - Milliseconds elapsed since last update
            ///
            /// # Examples
            ///
            /// ```ignore
            /// if let ModalState::Opening { .. } = &mut modal_state {
            ///     modal_state.update(16); // Advance by one frame at 60fps
            /// }
            /// ```
            pub const fn update(&mut self, delta_ms: u64) {
                match self {
                    Self::Opening { elapsed_ms } => {
                        *elapsed_ms += delta_ms;
                        if *elapsed_ms >= Self::OPENING_DURATION_MS {
                            *self = Self::Open;
                        }
                    }
                    Self::Closing { elapsed_ms } => {
                        *elapsed_ms += delta_ms;
                        if *elapsed_ms >= Self::CLOSING_DURATION_MS {
                            *self = Self::Closed;
                        }
                    }
                    Self::Open | Self::Closed => {}
                }
            }

            /// Returns current backdrop opacity (0.0 to 0.5)
            ///
            /// Animates from 0.0 to 0.5 during opening, and 0.5 to 0.0 during closing.
            ///
            /// # Examples
            ///
            /// ```ignore
            /// let opacity = modal_state.backdrop_opacity();
            /// // Use opacity for backdrop overlay color
            /// ```
            #[must_use]
            pub fn backdrop_opacity(&self) -> f32 {
                match self {
                    Self::Closed => 0.0,
                    Self::Opening { elapsed_ms } => {
                        let progress = (super::u64_to_f32(*elapsed_ms)
                            / super::u64_to_f32(Self::OPENING_DURATION_MS))
                        .min(1.0);
                        Self::MAX_BACKDROP_OPACITY * easing::ease_out(progress)
                    }
                    Self::Open => Self::MAX_BACKDROP_OPACITY,
                    Self::Closing { elapsed_ms } => {
                        let progress = (super::u64_to_f32(*elapsed_ms)
                            / super::u64_to_f32(Self::CLOSING_DURATION_MS))
                        .min(1.0);
                        Self::MAX_BACKDROP_OPACITY * (1.0 - easing::ease_in(progress))
                    }
                }
            }

            /// Returns current content scale factor (0.9 to 1.0)
            ///
            /// Creates subtle pop-in effect during opening with ease-out easing.
            /// Content scales from 0.9 to 1.0 for gentle entrance.
            ///
            /// # Examples
            ///
            /// ```ignore
            /// let scale = modal_state.content_scale();
            /// // Apply scale to modal content container
            /// ```
            #[must_use]
            pub fn content_scale(&self) -> f32 {
                match self {
                    Self::Closed => 0.9,
                    Self::Opening { elapsed_ms } => {
                        let progress = (super::u64_to_f32(*elapsed_ms)
                            / super::u64_to_f32(Self::OPENING_DURATION_MS))
                        .min(1.0);
                        0.1_f32.mul_add(easing::ease_out(progress), 0.9)
                    }
                    Self::Open => 1.0,
                    Self::Closing { elapsed_ms } => {
                        let progress = (super::u64_to_f32(*elapsed_ms)
                            / super::u64_to_f32(Self::CLOSING_DURATION_MS))
                        .min(1.0);
                        0.1f32.mul_add(-easing::ease_in(progress), 1.0)
                    }
                }
            }

            /// Returns current content opacity (0.0 to 1.0)
            ///
            /// Fades in content concurrently with scale animation.
            ///
            /// # Examples
            ///
            /// ```ignore
            /// let opacity = modal_state.content_opacity();
            /// ```
            #[must_use]
            pub fn content_opacity(&self) -> f32 {
                match self {
                    Self::Closed => 0.0,
                    Self::Opening { elapsed_ms } => {
                        let progress = (super::u64_to_f32(*elapsed_ms)
                            / super::u64_to_f32(Self::OPENING_DURATION_MS))
                        .min(1.0);
                        easing::ease_out(progress)
                    }
                    Self::Open => 1.0,
                    Self::Closing { elapsed_ms } => {
                        let progress = (super::u64_to_f32(*elapsed_ms)
                            / super::u64_to_f32(Self::CLOSING_DURATION_MS))
                        .min(1.0);
                        1.0 - easing::ease_in(progress)
                    }
                }
            }

            /// Returns whether modal is currently visible (any state except Closed)
            #[must_use]
            pub const fn is_visible(&self) -> bool {
                !matches!(self, Self::Closed)
            }

            /// Starts opening animation
            ///
            /// Transitions from `Closed` to `Opening`.
            pub const fn open(&mut self) {
                if matches!(self, Self::Closed) {
                    *self = Self::Opening { elapsed_ms: 0 };
                }
            }

            /// Starts closing animation
            ///
            /// Transitions from `Open` or `Opening` to `Closing`.
            pub const fn close(&mut self) {
                if matches!(self, Self::Open | Self::Opening { .. }) {
                    *self = Self::Closing { elapsed_ms: 0 };
                }
            }
        }
    }
}

/// Shadow depth utilities for elevation hierarchy
///
/// Based on ui-ux-pro-max design system shadow specifications.
/// Shadows create visual depth and establish hierarchy between UI layers.
///
/// # Shadow Levels
///
/// - **Small (sm)**: Subtle lift for buttons and input fields (1px offset, 2px blur)
/// - **Medium (md)**: Cards and raised panels (4px offset, 6px blur)
/// - **Large (lg)**: Modals and dropdowns (10px offset, 15px blur)
/// - **Extra Large (xl)**: Hero images and featured content (20px offset, 25px blur)
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

    /// Subtle lift shadow for minor elevation (buttons, input fields)
    ///
    /// Equivalent to: 0 1px 2px rgba(0,0,0,0.05)
    ///
    /// Per ui-ux-pro-max: use for interactive elements that need minimal
    /// elevation like buttons and form inputs.
    #[must_use]
    pub const fn shadow_sm() -> Shadow {
        Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.05),
            offset: Vector::new(0.0, 1.0),
            blur_radius: 2.0,
        }
    }

    /// Card/button shadow for moderate elevation
    ///
    /// Equivalent to: 0 4px 6px rgba(0,0,0,0.1)
    ///
    /// Per ui-ux-pro-max: use for cards, raised panels, and floating buttons
    /// that need clear separation from the background.
    #[must_use]
    pub const fn shadow_md() -> Shadow {
        Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.1),
            offset: Vector::new(0.0, 4.0),
            blur_radius: 6.0,
        }
    }

    /// Modal/dropdown shadow for high elevation
    ///
    /// Equivalent to: 0 10px 15px rgba(0,0,0,0.1)
    ///
    /// Per ui-ux-pro-max: use for modals, dropdowns, and popovers that
    /// float above the main content layer.
    #[must_use]
    pub const fn shadow_lg() -> Shadow {
        Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.1),
            offset: Vector::new(0.0, 10.0),
            blur_radius: 15.0,
        }
    }

    /// Hero/featured shadow for maximum elevation
    ///
    /// Equivalent to: 0 20px 25px rgba(0,0,0,0.15)
    ///
    /// Per ui-ux-pro-max: use for hero images, featured content, or
    /// elements that need maximum visual prominence.
    #[must_use]
    pub const fn shadow_xl() -> Shadow {
        Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.15),
            offset: Vector::new(0.0, 20.0),
            blur_radius: 25.0,
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
