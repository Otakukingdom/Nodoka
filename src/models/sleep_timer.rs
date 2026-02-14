//! Sleep timer model for automatic playback pause
//!
//! Sleep timers allow users to automatically pause playback after a specified
//! duration or at the end of the current chapter/file.

use chrono::{DateTime, Duration, Utc};

/// Sleep timer mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// Pause after a specific duration in seconds
    Duration(i64),
    /// Pause at the end of the current file/chapter
    EndOfChapter,
}

/// Represents an active sleep timer
#[derive(Debug, Clone)]
pub struct SleepTimer {
    /// Timer mode
    pub mode: Mode,
    /// When the timer was started
    pub started_at: DateTime<Utc>,
    /// Fade out duration in seconds before pausing
    pub fade_duration_secs: u32,
}

impl SleepTimer {
    /// Creates a new sleep timer
    ///
    /// # Arguments
    ///
    /// * `mode` - Timer mode (duration or end of chapter)
    /// * `fade_duration_secs` - How many seconds to fade out before pausing
    #[must_use]
    pub fn new(mode: Mode, fade_duration_secs: u32) -> Self {
        Self {
            mode,
            started_at: Utc::now(),
            fade_duration_secs,
        }
    }

    /// Checks if the timer has expired
    #[must_use]
    pub fn is_expired(&self) -> bool {
        match self.mode {
            Mode::Duration(secs) => {
                let elapsed = Utc::now().signed_duration_since(self.started_at);
                elapsed >= Duration::seconds(secs)
            }
            Mode::EndOfChapter => false, // Handled by playback logic
        }
    }

    /// Gets remaining time in seconds (None for end-of-chapter mode)
    #[must_use]
    pub fn remaining_seconds(&self) -> Option<i64> {
        match self.mode {
            Mode::Duration(secs) => {
                let elapsed = Utc::now().signed_duration_since(self.started_at);
                Some((secs - elapsed.num_seconds()).max(0))
            }
            Mode::EndOfChapter => None,
        }
    }
}
