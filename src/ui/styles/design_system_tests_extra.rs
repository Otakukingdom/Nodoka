// Additional design system tests split out to keep `design_system_tests.rs` under 1000 lines.

// === Warm and Cool Gradient Tests ===

#[test]
fn test_warm_gradient_has_warm_tones() {
    // Warm gradient should lean toward red/orange
    let warm = colors::GRADIENT_WARM_START;
    assert!(
        warm.r > warm.b,
        "Warm gradient should have more red than blue"
    );
    assert!(
        warm.r > 0.5,
        "Warm gradient should have significant red component"
    );
}

#[test]
fn test_cool_gradient_has_cool_tones() {
    // Cool gradient should lean toward blue/cyan
    let cool = colors::GRADIENT_COOL_START;
    assert!(
        cool.b > cool.r,
        "Cool gradient should have more blue than red"
    );
    assert!(
        cool.b > 0.5,
        "Cool gradient should have significant blue component"
    );
}

// === Hover State Utility Tests ===

#[test]
fn test_hover_subtle_returns_subtle_color() {
    use interactive::hover_subtle;

    let hover = hover_subtle(0.5);
    assert!(
        hover.a <= 0.15,
        "Subtle hover should have low opacity, got {}",
        hover.a
    );
}

#[test]
fn test_hover_elevated_returns_moderate_color() {
    use interactive::hover_elevated;

    let hover = hover_elevated(0.7);
    assert!(
        hover.a <= 0.25,
        "Elevated hover should have moderate opacity, got {}",
        hover.a
    );
}

#[test]
fn test_hover_glow_returns_premium_color() {
    use interactive::hover_glow;

    let glow = hover_glow(0.6);
    assert!(
        glow.a <= 0.35,
        "Glow should have reasonable opacity, got {}",
        glow.a
    );
    // Should have rose tint from PRIMARY_GRADIENT_START
    assert!(
        glow.r > glow.b,
        "Glow should have rose tint (more red than blue)"
    );
}
