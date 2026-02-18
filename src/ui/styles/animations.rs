//! Animation state machines and helpers.
//!
//! This module contains animation primitives (fade/scale transitions, stagger helpers,
//! pulse animation, and modal animation state) used by the UI.

pub use crate::ui::styles::effects::easing;
pub use crate::ui::styles::effects::{animations_enabled, transition_duration};

use crate::ui::styles::effects::u64_to_f32;

/// Fade transition utility for opacity animations.
///
/// Manages smooth opacity transitions from one value to another over time.
#[derive(Debug, Clone)]
pub struct FadeTransition {
    /// Current elapsed time in milliseconds.
    elapsed_ms: u64,
    /// Total duration of transition in milliseconds.
    duration_ms: u64,
    /// Starting opacity value (0.0 to 1.0).
    from: f32,
    /// Target opacity value (0.0 to 1.0).
    to: f32,
}

impl FadeTransition {
    /// Creates a new fade transition.
    #[must_use]
    pub const fn new(duration_ms: u64, from: f32, to: f32) -> Self {
        Self {
            elapsed_ms: 0,
            duration_ms,
            from,
            to,
        }
    }

    /// Updates transition progress.
    pub fn update(&mut self, delta_ms: u64) {
        self.elapsed_ms = (self.elapsed_ms + delta_ms).min(self.duration_ms);
    }

    /// Returns current opacity value (0.0 to 1.0).
    #[must_use]
    pub fn opacity(&self) -> f32 {
        if self.duration_ms == 0 {
            return self.to;
        }
        let progress = u64_to_f32(self.elapsed_ms) / u64_to_f32(self.duration_ms);
        let eased = easing::ease_out(progress);
        self.to.mul_add(eased, self.from * (1.0 - eased))
    }

    /// Returns whether transition is complete.
    #[must_use]
    pub const fn is_complete(&self) -> bool {
        self.elapsed_ms >= self.duration_ms
    }

    /// Resets transition to start.
    pub const fn reset(&mut self) {
        self.elapsed_ms = 0;
    }
}

/// Scale transition utility for size animations.
///
/// Manages smooth scale transitions for microinteractions.
#[derive(Debug, Clone)]
pub struct ScaleTransition {
    /// Current elapsed time in milliseconds.
    elapsed_ms: u64,
    /// Total duration of transition in milliseconds.
    duration_ms: u64,
    /// Starting scale value (typically 0.9 to 1.1).
    from: f32,
    /// Target scale value (typically 1.0).
    to: f32,
}

impl ScaleTransition {
    /// Creates a new scale transition.
    #[must_use]
    pub const fn new(duration_ms: u64, from: f32, to: f32) -> Self {
        Self {
            elapsed_ms: 0,
            duration_ms,
            from,
            to,
        }
    }

    /// Updates transition progress.
    pub fn update(&mut self, delta_ms: u64) {
        self.elapsed_ms = (self.elapsed_ms + delta_ms).min(self.duration_ms);
    }

    /// Returns current scale value.
    #[must_use]
    pub fn scale(&self) -> f32 {
        if self.duration_ms == 0 {
            return self.to;
        }
        let progress = u64_to_f32(self.elapsed_ms) / u64_to_f32(self.duration_ms);
        let eased = easing::ease_out(progress);
        self.to.mul_add(eased, self.from * (1.0 - eased))
    }

    /// Returns whether transition is complete.
    #[must_use]
    pub const fn is_complete(&self) -> bool {
        self.elapsed_ms >= self.duration_ms
    }

    /// Resets transition to start.
    pub const fn reset(&mut self) {
        self.elapsed_ms = 0;
    }
}

/// Calculates stagger delay for list item animations.
#[must_use]
pub const fn stagger_delay(index: usize, stagger_ms: u64) -> u64 {
    index as u64 * stagger_ms
}

/// Calculates stagger opacity for list item animations.
#[must_use]
pub fn stagger_opacity(index: usize, delay_ms: u64, elapsed_ms: u64) -> f32 {
    let capped_index = index.min(15);
    let item_delay = stagger_delay(capped_index, delay_ms);

    if elapsed_ms < item_delay {
        0.0
    } else {
        let progress = u64_to_f32(elapsed_ms - item_delay) / 200.0;
        easing::ease_out(progress.min(1.0))
    }
}

/// Vertical translation distance for stagger animations (pixels).
pub const STAGGER_TRANSLATE_Y_PX: f32 = 20.0;

/// Calculates stagger vertical translation for list item animations.
#[must_use]
pub fn stagger_translate_y(index: usize, delay_ms: u64, elapsed_ms: u64) -> f32 {
    let capped_index = index.min(15);
    let item_delay = stagger_delay(capped_index, delay_ms);

    if elapsed_ms < item_delay {
        STAGGER_TRANSLATE_Y_PX
    } else {
        let progress = u64_to_f32(elapsed_ms - item_delay) / 200.0;
        STAGGER_TRANSLATE_Y_PX * (1.0 - easing::ease_out(progress.min(1.0)))
    }
}

/// Pulse animation for indicating active/playing state.
#[derive(Debug, Clone)]
pub struct PulseAnimation {
    /// Current phase (0.0 to 1.0).
    phase: f32,
    /// Animation duration in milliseconds.
    duration_ms: u64,
}

impl PulseAnimation {
    /// Creates a new pulse animation with specified duration.
    #[must_use]
    pub const fn new(duration_ms: u64) -> Self {
        Self {
            phase: 0.0,
            duration_ms,
        }
    }

    /// Updates animation phase based on elapsed time.
    pub fn update(&mut self, delta_ms: u64) {
        if self.duration_ms == 0 {
            return;
        }
        let progress = u64_to_f32(delta_ms) / u64_to_f32(self.duration_ms);
        self.phase = (self.phase + progress).rem_euclid(1.0);
    }

    /// Returns current opacity for pulse effect (0.0 to 1.0).
    #[must_use]
    pub fn opacity(&self) -> f32 {
        let phase = self.phase.rem_euclid(1.0);
        (phase * std::f32::consts::PI * 2.0).sin().mul_add(0.5, 0.5)
    }

    /// Resets animation to start.
    pub const fn reset(&mut self) {
        self.phase = 0.0;
    }
}

/// Modal overlay animation system.
pub mod modal {
    use super::easing;
    use super::u64_to_f32;

    /// Modal animation state machine.
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum ModalState {
        /// Modal is not visible.
        Closed,
        /// Modal is animating in.
        Opening {
            /// Milliseconds elapsed since the opening animation began.
            elapsed_ms: u64,
        },
        /// Modal is fully visible.
        Open,
        /// Modal is animating out.
        Closing {
            /// Milliseconds elapsed since the closing animation began.
            elapsed_ms: u64,
        },
    }

    impl ModalState {
        /// Duration of modal opening animation in milliseconds.
        pub const OPENING_DURATION_MS: u64 = 300;

        /// Duration of modal closing animation in milliseconds.
        pub const CLOSING_DURATION_MS: u64 = 200;

        /// Maximum backdrop opacity (semi-transparent black).
        pub const MAX_BACKDROP_OPACITY: f32 = 0.5;

        /// Updates modal animation state.
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

        /// Returns current backdrop opacity (0.0 to 0.5).
        #[must_use]
        pub fn backdrop_opacity(&self) -> f32 {
            match self {
                Self::Closed => 0.0,
                Self::Opening { elapsed_ms } => {
                    let progress =
                        (u64_to_f32(*elapsed_ms) / u64_to_f32(Self::OPENING_DURATION_MS)).min(1.0);
                    Self::MAX_BACKDROP_OPACITY * easing::ease_out(progress)
                }
                Self::Open => Self::MAX_BACKDROP_OPACITY,
                Self::Closing { elapsed_ms } => {
                    let progress =
                        (u64_to_f32(*elapsed_ms) / u64_to_f32(Self::CLOSING_DURATION_MS)).min(1.0);
                    Self::MAX_BACKDROP_OPACITY * (1.0 - easing::ease_in(progress))
                }
            }
        }

        /// Returns current content scale factor (0.9 to 1.0).
        #[must_use]
        pub fn content_scale(&self) -> f32 {
            match self {
                Self::Closed => 0.9,
                Self::Opening { elapsed_ms } => {
                    let progress =
                        (u64_to_f32(*elapsed_ms) / u64_to_f32(Self::OPENING_DURATION_MS)).min(1.0);
                    0.1_f32.mul_add(easing::ease_out(progress), 0.9)
                }
                Self::Open => 1.0,
                Self::Closing { elapsed_ms } => {
                    let progress =
                        (u64_to_f32(*elapsed_ms) / u64_to_f32(Self::CLOSING_DURATION_MS)).min(1.0);
                    0.1f32.mul_add(-easing::ease_in(progress), 1.0)
                }
            }
        }

        /// Returns current content opacity (0.0 to 1.0).
        #[must_use]
        pub fn content_opacity(&self) -> f32 {
            match self {
                Self::Closed => 0.0,
                Self::Opening { elapsed_ms } => {
                    let progress =
                        (u64_to_f32(*elapsed_ms) / u64_to_f32(Self::OPENING_DURATION_MS)).min(1.0);
                    easing::ease_out(progress)
                }
                Self::Open => 1.0,
                Self::Closing { elapsed_ms } => {
                    let progress =
                        (u64_to_f32(*elapsed_ms) / u64_to_f32(Self::CLOSING_DURATION_MS)).min(1.0);
                    1.0 - easing::ease_in(progress)
                }
            }
        }

        /// Returns whether modal is currently visible.
        #[must_use]
        pub const fn is_visible(&self) -> bool {
            !matches!(self, Self::Closed)
        }

        /// Starts opening animation.
        pub const fn open(&mut self) {
            if matches!(self, Self::Closed) {
                *self = Self::Opening { elapsed_ms: 0 };
            }
        }

        /// Starts closing animation.
        pub const fn close(&mut self) {
            if matches!(self, Self::Open | Self::Opening { .. }) {
                *self = Self::Closing { elapsed_ms: 0 };
            }
        }
    }
}
