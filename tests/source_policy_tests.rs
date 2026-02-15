//! Source policy regression tests.
//!
//! These tests enforce repository constraints from `AGENTS.md` that are easy to
//! accidentally violate during refactors.

#[test]
fn test_ui_update_rs_under_1000_lines() {
    const UPDATE_RS: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/ui/update.rs"));

    let lines = UPDATE_RS.lines().count();
    assert!(
        lines <= 1000,
        "src/ui/update.rs must be <= 1000 lines (found {lines})"
    );
}
