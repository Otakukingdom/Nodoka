// Shadow depth system tests
//
// Shadow levels follow a smooth elevation progression:
// - xs: 0.5px offset, 1px blur, 0.04 opacity (minimal elevation)
// - sm: 1px offset, 2px blur, 0.06 opacity (subtle lift for buttons)
// - md: 2px offset, 4px blur, 0.10 opacity (moderate elevation for cards)
// - lg: 4px offset, 8px blur, 0.14 opacity (high elevation for modals)
// - xl: 8px offset, 16px blur, 0.18 opacity (featured elevation for hero content)
// - xxl: 12px offset, 24px blur, 0.26 opacity (maximum elevation for prominent elements)

use nodoka::ui::shadows;

#[test]
fn test_shadow_xs_follows_design_system() {
    let shadow = shadows::shadow_xs();
    crate::assert_approx_eq_f32(
        shadow.offset.y,
        2.0,
        0.001,
        "XS shadow offset should be 2px",
    );
    crate::assert_approx_eq_f32(
        shadow.blur_radius,
        3.0,
        0.001,
        "XS shadow blur should be 3px",
    );
    assert!(
        (shadow.color.a - 0.05).abs() < 1e-6,
        "XS shadow opacity should be 0.05"
    );
}

#[test]
fn test_shadow_sm_follows_design_system() {
    let shadow = shadows::shadow_sm();
    crate::assert_approx_eq_f32(
        shadow.offset.y,
        4.0,
        0.001,
        "Small shadow offset should be 4px",
    );
    crate::assert_approx_eq_f32(
        shadow.blur_radius,
        6.0,
        0.001,
        "Small shadow blur should be 6px",
    );
    assert!(
        (shadow.color.a - 0.08).abs() < 1e-6,
        "Small shadow opacity should be 0.08"
    );
}

#[test]
fn test_shadow_md_follows_design_system() {
    let shadow = shadows::shadow_md();
    crate::assert_approx_eq_f32(
        shadow.offset.y,
        8.0,
        0.001,
        "Medium shadow offset should be 8px",
    );
    crate::assert_approx_eq_f32(
        shadow.blur_radius,
        12.0,
        0.001,
        "Medium shadow blur should be 12px",
    );
    assert!(
        (shadow.color.a - 0.13).abs() < 1e-6,
        "Medium shadow opacity should be 0.13"
    );
}

#[test]
fn test_shadow_lg_follows_design_system() {
    let shadow = shadows::shadow_lg();
    crate::assert_approx_eq_f32(
        shadow.offset.y,
        16.0,
        0.001,
        "Large shadow offset should be 16px",
    );
    crate::assert_approx_eq_f32(
        shadow.blur_radius,
        24.0,
        0.001,
        "Large shadow blur should be 24px",
    );
    assert!(
        (shadow.color.a - 0.18).abs() < 1e-6,
        "Large shadow opacity should be 0.18"
    );
}

#[test]
fn test_shadow_xl_follows_design_system() {
    let shadow = shadows::shadow_xl();
    crate::assert_approx_eq_f32(
        shadow.offset.y,
        24.0,
        0.001,
        "XL shadow offset should be 24px",
    );
    crate::assert_approx_eq_f32(
        shadow.blur_radius,
        40.0,
        0.001,
        "XL shadow blur should be 40px",
    );
    assert!(
        (shadow.color.a - 0.23).abs() < 1e-6,
        "XL shadow opacity should be 0.23"
    );
}

#[test]
fn test_shadow_xxl_follows_design_system() {
    let shadow = shadows::shadow_xxl();
    crate::assert_approx_eq_f32(
        shadow.offset.y,
        48.0,
        0.001,
        "XXL shadow offset should be 48px",
    );
    crate::assert_approx_eq_f32(
        shadow.blur_radius,
        80.0,
        0.001,
        "XXL shadow blur should be 80px",
    );
    assert!(
        (shadow.color.a - 0.32).abs() < 1e-6,
        "XXL shadow opacity should be 0.32"
    );
}

#[test]
fn test_shadow_hierarchy_increases() {
    let xs = shadows::shadow_xs();
    let sm = shadows::shadow_sm();
    let md = shadows::shadow_md();
    let lg = shadows::shadow_lg();
    let xl = shadows::shadow_xl();
    let xxl = shadows::shadow_xxl();

    // Shadow elevation should increase: xs < sm < md < lg < xl < xxl
    assert!(
        xs.offset.y < sm.offset.y,
        "SM shadow should be higher than XS"
    );
    assert!(
        sm.offset.y < md.offset.y,
        "MD shadow should be higher than SM"
    );
    assert!(
        md.offset.y < lg.offset.y,
        "LG shadow should be higher than MD"
    );
    assert!(
        lg.offset.y < xl.offset.y,
        "XL shadow should be higher than LG"
    );
    assert!(
        xl.offset.y < xxl.offset.y,
        "XXL shadow should be higher than XL"
    );

    // Blur radius should also increase
    assert!(
        xs.blur_radius < sm.blur_radius,
        "SM blur should be larger than XS"
    );
    assert!(
        sm.blur_radius < md.blur_radius,
        "MD blur should be larger than SM"
    );
    assert!(
        md.blur_radius < lg.blur_radius,
        "LG blur should be larger than MD"
    );
    assert!(
        lg.blur_radius < xl.blur_radius,
        "XL blur should be larger than LG"
    );
    assert!(
        xl.blur_radius < xxl.blur_radius,
        "XXL blur should be larger than XL"
    );
}
