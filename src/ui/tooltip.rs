//! Tooltip state and targeting for hover-based discoverability.
//!
//! Tooltips use delayed show, instant hide, and reduced-motion support.

/// Tooltip placement relative to its anchor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TooltipPosition {
    /// Show above the anchor.
    Top,
    /// Show below the anchor.
    Bottom,
    /// Show to the left of the anchor.
    Left,
    /// Show to the right of the anchor.
    Right,
}

/// Approximate anchor location for tooltip layout.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TooltipAnchor {
    /// Top bar region (settings button, title).
    TopBar,
    /// Player controls region (bottom).
    PlayerControls,
    /// Left library panel (audiobooks).
    LibraryLeft,
}

/// Tooltip payload.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tooltip {
    /// Text content to display.
    pub text: String,
    /// Preferred placement around anchor.
    pub position: TooltipPosition,
    /// Layout anchor for overlay placement.
    pub anchor: TooltipAnchor,
}

/// Targets that can produce tooltips.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TooltipTarget {
    /// The play/pause transport button.
    PlayPause,
    /// The stop transport button.
    Stop,
    /// The progress/seek slider.
    Progress,
    /// The playback speed control.
    Speed,
    /// The volume control.
    Volume,
    /// The settings open button.
    Settings,
    /// An audiobook item in the library list.
    AudiobookItem,
    /// The search bar for filtering audiobooks.
    SearchBar,
    /// Sort order button.
    SortButton,
    /// Filter chip for "All" status.
    FilterChipAll,
    /// Filter chip for "In Progress" status.
    FilterChipInProgress,
    /// Filter chip for "Complete" status.
    FilterChipComplete,
    /// Filter chip for "Not Started" status.
    FilterChipNotStarted,
    /// Clear filters button.
    ClearFilters,
    /// Continue Listening play button.
    ContinueListeningPlay,
    /// Focus mode toggle button.
    FocusModeToggle,
    /// Panel resize divider.
    PanelResizeDivider,
    /// Theme toggle button.
    ThemeToggle,
}

impl TooltipTarget {
    /// Builds tooltip payload for the target.
    #[must_use]
    pub fn tooltip(self) -> Tooltip {
        match self {
            Self::PlayPause | Self::Stop | Self::Progress | Self::Speed | Self::Volume => {
                tooltip_player_controls(self)
            }
            Self::Settings | Self::FocusModeToggle | Self::ThemeToggle => tooltip_top_bar(self),
            Self::AudiobookItem
            | Self::SearchBar
            | Self::SortButton
            | Self::FilterChipAll
            | Self::FilterChipInProgress
            | Self::FilterChipComplete
            | Self::FilterChipNotStarted
            | Self::ClearFilters
            | Self::ContinueListeningPlay
            | Self::PanelResizeDivider => tooltip_library_left(self),
        }
    }
}

fn tooltip_player_controls(target: TooltipTarget) -> Tooltip {
    match target {
        TooltipTarget::PlayPause => Tooltip {
            text: "Play/Pause (Space)".to_string(),
            position: TooltipPosition::Top,
            anchor: TooltipAnchor::PlayerControls,
        },
        TooltipTarget::Stop => Tooltip {
            text: "Stop".to_string(),
            position: TooltipPosition::Top,
            anchor: TooltipAnchor::PlayerControls,
        },
        TooltipTarget::Progress => Tooltip {
            text: "Seek (← →)".to_string(),
            position: TooltipPosition::Top,
            anchor: TooltipAnchor::PlayerControls,
        },
        TooltipTarget::Speed => Tooltip {
            text: "Playback Speed".to_string(),
            position: TooltipPosition::Top,
            anchor: TooltipAnchor::PlayerControls,
        },
        TooltipTarget::Volume => Tooltip {
            text: "Volume".to_string(),
            position: TooltipPosition::Left,
            anchor: TooltipAnchor::PlayerControls,
        },
        _ => unreachable!("non-player-controls tooltip target"),
    }
}

fn tooltip_top_bar(target: TooltipTarget) -> Tooltip {
    match target {
        TooltipTarget::Settings => Tooltip {
            text: "Settings".to_string(),
            position: TooltipPosition::Bottom,
            anchor: TooltipAnchor::TopBar,
        },
        TooltipTarget::FocusModeToggle => Tooltip {
            text: format!(
                "Hide library panel for distraction-free listening ({})",
                crate::ui::shortcuts::format_command_shortcut("L")
            ),
            position: TooltipPosition::Bottom,
            anchor: TooltipAnchor::TopBar,
        },
        TooltipTarget::ThemeToggle => Tooltip {
            text: format!(
                "Switch between light and dark themes ({})",
                crate::ui::shortcuts::format_command_shortcut("T")
            ),
            position: TooltipPosition::Bottom,
            anchor: TooltipAnchor::TopBar,
        },
        _ => unreachable!("non-top-bar tooltip target"),
    }
}

fn tooltip_library_left(target: TooltipTarget) -> Tooltip {
    match target {
        TooltipTarget::AudiobookItem => Tooltip {
            text: "Select Audiobook".to_string(),
            position: TooltipPosition::Right,
            anchor: TooltipAnchor::LibraryLeft,
        },
        TooltipTarget::SearchBar => Tooltip {
            text: format!(
                "Search audiobooks by title or author ({})",
                crate::ui::shortcuts::format_command_shortcut("F")
            ),
            position: TooltipPosition::Bottom,
            anchor: TooltipAnchor::LibraryLeft,
        },
        TooltipTarget::SortButton => Tooltip {
            text: "Change sort order for library".to_string(),
            position: TooltipPosition::Bottom,
            anchor: TooltipAnchor::LibraryLeft,
        },
        TooltipTarget::FilterChipAll => Tooltip {
            text: "Show all audiobooks".to_string(),
            position: TooltipPosition::Bottom,
            anchor: TooltipAnchor::LibraryLeft,
        },
        TooltipTarget::FilterChipInProgress => Tooltip {
            text: "Show only partially listened audiobooks".to_string(),
            position: TooltipPosition::Bottom,
            anchor: TooltipAnchor::LibraryLeft,
        },
        TooltipTarget::FilterChipComplete => Tooltip {
            text: "Show only completed audiobooks".to_string(),
            position: TooltipPosition::Bottom,
            anchor: TooltipAnchor::LibraryLeft,
        },
        TooltipTarget::FilterChipNotStarted => Tooltip {
            text: "Show only not-started audiobooks".to_string(),
            position: TooltipPosition::Bottom,
            anchor: TooltipAnchor::LibraryLeft,
        },
        TooltipTarget::ClearFilters => Tooltip {
            text: "Reset search, sort, and filters to defaults".to_string(),
            position: TooltipPosition::Bottom,
            anchor: TooltipAnchor::LibraryLeft,
        },
        TooltipTarget::ContinueListeningPlay => Tooltip {
            text: format!(
                "Resume playback where you left off ({})",
                crate::ui::shortcuts::format_command_shortcut("R")
            ),
            position: TooltipPosition::Bottom,
            anchor: TooltipAnchor::LibraryLeft,
        },
        TooltipTarget::PanelResizeDivider => Tooltip {
            text: {
                let dec = crate::ui::shortcuts::format_command_shortcut("[");
                let inc = crate::ui::shortcuts::format_command_shortcut("]");
                format!("Adjust panel sizes with {dec} and {inc}")
            },
            position: TooltipPosition::Bottom,
            anchor: TooltipAnchor::LibraryLeft,
        },
        _ => unreachable!("non-library-left tooltip target"),
    }
}

/// Tooltip visibility state with delayed show.
#[derive(Debug, Clone)]
pub struct TooltipState {
    pending: Option<Tooltip>,
    visible: Option<Tooltip>,
    hover_started_ms: Option<u64>,
    delay_ms: u64,
}

impl TooltipState {
    /// Default tooltip delay in milliseconds.
    pub const DEFAULT_DELAY_MS: u64 = 500;

    /// Creates a new tooltip state.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            pending: None,
            visible: None,
            hover_started_ms: None,
            delay_ms: Self::DEFAULT_DELAY_MS,
        }
    }

    /// Begins hover tracking for a tooltip.
    pub fn start_hover(&mut self, tooltip: Tooltip, now_ms: u64) {
        self.pending = Some(tooltip);
        self.visible = None;
        self.hover_started_ms = Some(now_ms);
    }

    /// Ends hover tracking and hides any visible tooltip.
    pub fn end_hover(&mut self) {
        self.pending = None;
        self.visible = None;
        self.hover_started_ms = None;
    }

    /// Updates tooltip visibility based on hover timing.
    pub fn update(&mut self, now_ms: u64, reduced_motion: bool) {
        let delay = if reduced_motion { 0 } else { self.delay_ms };
        let Some(start_ms) = self.hover_started_ms else {
            return;
        };
        if let Some(pending) = self.pending.clone() {
            let elapsed = now_ms.saturating_sub(start_ms);
            if elapsed >= delay {
                self.visible = Some(pending);
            }
        }
    }

    /// Returns the currently visible tooltip, if any.
    #[must_use]
    pub const fn visible(&self) -> Option<&Tooltip> {
        self.visible.as_ref()
    }
}

impl Default for TooltipState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tooltip_target_payloads() {
        let tooltip = TooltipTarget::PlayPause.tooltip();
        assert!(tooltip.text.contains("Play"));
        assert_eq!(tooltip.position, TooltipPosition::Top);
    }

    #[test]
    fn test_tooltip_state_delays_visibility() {
        let mut state = TooltipState::new();
        let now = 1000;
        state.start_hover(TooltipTarget::Settings.tooltip(), now);
        state.update(now + 200, false);
        assert!(state.visible().is_none());
        state.update(now + TooltipState::DEFAULT_DELAY_MS, false);
        assert!(state.visible().is_some());
    }

    #[test]
    fn test_tooltip_state_reduced_motion_immediate() {
        let mut state = TooltipState::new();
        let now = 2000;
        state.start_hover(TooltipTarget::Volume.tooltip(), now);
        state.update(now, true);
        assert!(state.visible().is_some());
    }

    #[test]
    fn test_tooltip_state_end_hover_clears() {
        let mut state = TooltipState::new();
        let now = 3000;
        state.start_hover(TooltipTarget::Speed.tooltip(), now);
        state.update(now + TooltipState::DEFAULT_DELAY_MS, false);
        assert!(state.visible().is_some());
        state.end_hover();
        assert!(state.visible().is_none());
    }
}
