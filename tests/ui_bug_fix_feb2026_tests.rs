//! Integration-style regression tests for named Feb 2026 bug fixes.

use nodoka::ui::{format_duration, format_time_ms, State};

/// Bug FIX #003 (Feb 2026): Added `operation_in_progress` flag
///
/// Scenario: Rapid button clicks could cause duplicate operations
/// when no operation-in-progress flag exists.
#[test]
fn test_bug_fix_feb2026_003_operation_in_progress_flag_exists() {
    let state = State::default();

    assert!(
        !state.operation_in_progress,
        "operation_in_progress should be false by default"
    );
}

/// Bug FIX #003 (integration): Operation flag state management
#[test]
fn test_bug_fix_feb2026_003_operation_flag_state_management() {
    let mut state = State {
        operation_in_progress: false,
        ..Default::default()
    };

    assert!(!state.operation_in_progress);

    state.operation_in_progress = true;
    assert!(state.operation_in_progress);

    state.operation_in_progress = false;
    assert!(!state.operation_in_progress);
}

/// Bug FIX #005 (Feb 2026): Time format consistency
///
/// Scenario: `format_time` and `format_duration` handled edge cases differently,
/// especially negative values and zero duration.
#[test]
fn test_bug_fix_feb2026_005_time_format_consistency_zero() {
    let duration_zero = format_duration(Some(0));
    let time_zero = format_time_ms(0);

    assert_eq!(duration_zero, "--:--");
    assert_eq!(time_zero, "0:00");
}

/// Bug FIX #005 (variant): Negative time handling consistency
#[test]
fn test_bug_fix_feb2026_005_time_format_consistency_negative() {
    let duration_neg = format_duration(Some(-1000));
    let time_neg = format_time_ms(-1000);

    assert_eq!(duration_neg, "--:--");
    assert_eq!(time_neg, "0:00");
}

/// Bug FIX #005 (variant): Large time values consistency
#[test]
fn test_bug_fix_feb2026_005_time_format_consistency_large_values() {
    let ten_hours = 36_000_000_i64; // 10 hours in milliseconds

    let duration_large = format_duration(Some(ten_hours));
    let time_large = format_time_ms(ten_hours);

    assert_eq!(duration_large, "10:00:00");
    assert_eq!(time_large, "10:00:00");
}
