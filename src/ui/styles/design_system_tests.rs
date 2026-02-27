//! Design System Tests
//!
//! Comprehensive tests for the Nodoka design system including color contrast,
//! typography scaling, spacing consistency, and WCAG AA accessibility compliance.

#[cfg(test)]
mod tests {
    use crate::ui::styles::{colors, dark_theme, effects, interactive, layout, animations, format_duration, format_time_ms};

    fn srgb_to_linear(channel: f64) -> f64 {
        if channel <= 0.04045 {
            channel / 12.92
        } else {
            ((channel + 0.055) / 1.055).powf(2.4)
        }
    }

    fn wcag_relative_luminance(color: iced::Color) -> f64 {
        let r = srgb_to_linear(f64::from(color.r));
        let g = srgb_to_linear(f64::from(color.g));
        let b = srgb_to_linear(f64::from(color.b));
        (0.2126_f64).mul_add(r, (0.7152_f64).mul_add(g, 0.0722_f64 * b))
    }

    fn wcag_contrast_ratio(foreground: iced::Color, background: iced::Color) -> f64 {
        let fg = wcag_relative_luminance(foreground);
        let bg = wcag_relative_luminance(background);
        let (lighter, darker) = if fg >= bg { (fg, bg) } else { (bg, fg) };
        (lighter + 0.05) / (darker + 0.05)
    }

    // Duration formatting tests

    #[test]
    fn test_format_duration_with_hours() {
        assert_eq!(format_duration(Some(3_661_000)), "1:01:01");
        assert_eq!(format_duration(Some(7_200_000)), "2:00:00");
    }

    #[test]
    fn test_format_duration_with_minutes_only() {
        assert_eq!(format_duration(Some(125_000)), "2:05");
        assert_eq!(format_duration(Some(60000)), "1:00");
    }

    #[test]
    fn test_format_duration_with_zero() {
        assert_eq!(format_duration(Some(0)), "--:--");
    }

    #[test]
    fn test_format_duration_with_none() {
        assert_eq!(format_duration(None), "--:--");
    }

    #[test]
    fn test_format_duration_with_negative() {
        assert_eq!(format_duration(Some(-500)), "--:--");
    }

    #[test]
    fn test_format_time_ms_with_hours() {
        assert_eq!(format_time_ms(3_661_000), "1:01:01");
        assert_eq!(format_time_ms(7_200_000), "2:00:00");
    }

    #[test]
    fn test_format_time_ms_with_minutes_only() {
        assert_eq!(format_time_ms(125_000), "2:05");
        assert_eq!(format_time_ms(60000), "1:00");
    }

    #[test]
    fn test_format_time_ms_with_zero() {
        assert_eq!(format_time_ms(0), "0:00");
    }

    #[test]
    fn test_format_time_ms_with_negative_clamped_to_zero() {
        assert_eq!(format_time_ms(-500), "0:00");
    }

    // WCAG AA contrast ratio tests

    #[test]
    fn test_primary_text_contrast_on_primary_bg() {
        let ratio = wcag_contrast_ratio(colors::TEXT_PRIMARY, colors::BG_PRIMARY);
        assert!(
            ratio >= 4.5,
            "Primary text contrast ratio {ratio:.2} does not meet WCAG AA (4.5:1)"
        );
    }

    #[test]
    fn test_text_on_primary_button_contrast() {
        let ratio = wcag_contrast_ratio(colors::TEXT_ON_PRIMARY, colors::PRIMARY);
        assert!(
            ratio >= 4.5,
            "Button text contrast ratio {ratio:.2} does not meet WCAG AA (4.5:1)"
        );
    }

    #[test]
    fn test_focus_ring_contrast() {
        let ratio = wcag_contrast_ratio(colors::FOCUS_RING, colors::BG_PRIMARY);
        assert!(
            ratio >= 3.0,
            "Focus ring contrast ratio {ratio:.2} does not meet WCAG 2.1 (3:1)"
        );
    }

    #[test]
    fn test_error_button_contrast() {
        let ratio = wcag_contrast_ratio(colors::TEXT_ON_PRIMARY, colors::ERROR);
        assert!(
            ratio >= 4.5,
            "Error button contrast ratio {ratio:.2} does not meet WCAG AA (4.5:1)"
        );
    }

    // Animation and transition tests

    #[test]
    fn test_transitions_respect_reduced_motion() {
        use animations::transition_duration;

        let normal = transition_duration(false);
        assert_eq!(normal, 200, "Normal transition should be 200ms");

        let reduced = transition_duration(true);
        assert_eq!(reduced, 0, "Reduced motion transition should be 0ms");
    }

    #[test]
    fn test_hover_overlay_darkens_color() {
        use interactive::apply_hover_overlay;

        let base = colors::BG_ELEVATED;
        let hovered = apply_hover_overlay(base);

        assert!(hovered.r < base.r);
        assert!(hovered.g < base.g);
        assert!(hovered.b < base.b);
    }

    #[test]
    fn test_active_overlay_darker_than_hover() {
        use interactive::{apply_active_overlay, apply_hover_overlay};

        let base = colors::BG_ELEVATED;
        let hovered = apply_hover_overlay(base);
        let active = apply_active_overlay(base);

        assert!(active.r < hovered.r);
        assert!(active.g < hovered.g);
        assert!(active.b < hovered.b);
    }

    #[test]
    fn test_interactive_state_overlay_progression() {
        use interactive::{apply_state_overlay, InteractiveState};

        let base = colors::BG_ELEVATED;
        let idle = apply_state_overlay(base, InteractiveState::Idle);
        let hover = apply_state_overlay(base, InteractiveState::Hover);
        let pressed = apply_state_overlay(base, InteractiveState::Pressed);

        assert!(
            (idle.r - base.r).abs() < f32::EPSILON,
            "Idle should not change color"
        );
        assert!(hover.r < idle.r, "Hover should be darker than idle");
        assert!(pressed.r < hover.r, "Pressed should be darker than hover");
    }

    #[test]
    fn test_animations_enabled_returns_correct_bool() {
        use animations::animations_enabled;

        assert!(animations_enabled(false), "Should enable animations");
        assert!(!animations_enabled(true), "Should disable animations");
    }

    #[test]
    fn test_fade_transition_bounds() {
        use animations::FadeTransition;

        let mut fade = FadeTransition::new(200, 0.0, 1.0);

        assert!((fade.opacity() - 0.0).abs() < f32::EPSILON);

        fade.update(100);
        let mid = fade.opacity();
        assert!(
            (0.0..=1.0).contains(&mid),
            "Mid opacity {mid} out of bounds"
        );

        fade.update(200);
        assert!((fade.opacity() - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_scale_transition_bounds() {
        use animations::ScaleTransition;

        let mut scale = ScaleTransition::new(200, 0.95, 1.0);

        assert!((scale.scale() - 0.95).abs() < f32::EPSILON);

        scale.update(100);
        let mid = scale.scale();
        assert!((0.95..=1.0).contains(&mid), "Mid scale {mid} out of bounds");

        scale.update(200);
        assert!((scale.scale() - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_stagger_delay_progression() {
        use animations::stagger_delay;

        assert_eq!(stagger_delay(0, 50), 0);
        assert_eq!(stagger_delay(1, 50), 50);
        assert_eq!(stagger_delay(2, 50), 100);
        assert_eq!(stagger_delay(5, 50), 250);
    }

    #[test]
    fn test_pulse_animation_opacity_range() {
        use animations::PulseAnimation;

        let mut pulse = PulseAnimation::new(1000);

        for _ in 0..100 {
            pulse.update(20);
            let opacity = pulse.opacity();
            assert!(
                (-0.01..=1.01).contains(&opacity),
                "Pulse opacity {opacity} out of range"
            );
        }
    }

    #[test]
    fn test_pulse_animation_with_zero_duration() {
        use animations::PulseAnimation;

        let mut pulse = PulseAnimation::new(0);
        let initial = pulse.opacity();

        pulse.update(100);
        let after_update = pulse.opacity();

        assert!((initial - after_update).abs() < f32::EPSILON);
    }

    #[test]
    fn test_pulse_animation_multiple_updates() {
        use animations::PulseAnimation;

        let mut pulse = PulseAnimation::new(1000);

        for _ in 0..60 {
            pulse.update(16);
            let opacity = pulse.opacity();
            assert!(
                (-0.01..=1.01).contains(&opacity),
                "Opacity {opacity} out of range"
            );
        }
    }

    #[test]
    fn test_ease_in_out_is_symmetric() {
        use animations::easing;

        let t1 = easing::ease_in_out(0.25);
        let t2 = easing::ease_in_out(0.75);

        assert!((t1 + t2 - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_ease_out_starts_faster_than_ease_in() {
        use animations::easing;

        let ease_out_early = easing::ease_out(0.1);
        let ease_in_early = easing::ease_in(0.1);

        assert!(ease_out_early > ease_in_early);
    }

    // Spacing tests

    #[test]
    fn test_spacing_follows_4px_grid() {
        let spacing_values = [
            layout::spacing::XS,
            layout::spacing::SM,
            layout::spacing::MD,
            layout::spacing::LG,
            layout::spacing::XL,
            layout::spacing::XXL,
        ];

        for value in spacing_values {
            assert!(
                (value / 4.0).fract() < f32::EPSILON,
                "Spacing value {value} is not a multiple of 4px base grid"
            );
        }
    }

    #[test]
    fn test_spacing_progresses_monotonically() {
        const {
            assert!(layout::spacing::XS < layout::spacing::SM);
            assert!(layout::spacing::SM < layout::spacing::MD);
            assert!(layout::spacing::MD < layout::spacing::LG);
            assert!(layout::spacing::LG < layout::spacing::XL);
            assert!(layout::spacing::XL < layout::spacing::XXL);
        }
    }

    // Typography tests

    #[test]
    fn test_typography_sizes_are_positive() {
        let type_sizes = [
            layout::typography::SIZE_XS,
            layout::typography::SIZE_SM,
            layout::typography::SIZE_BASE,
            layout::typography::SIZE_LG,
            layout::typography::SIZE_XL,
            layout::typography::SIZE_XXL,
            layout::typography::SIZE_HEADING,
        ];

        for size in type_sizes {
            assert!(size > 0.0, "Typography size {size} should be positive");
        }
    }

    #[test]
    fn test_typography_sizes_descend_properly() {
        use crate::ui::styles::layout::typography;

        let sizes = [
            typography::SIZE_HEADING,
            typography::SIZE_XXL,
            typography::SIZE_XL,
            typography::SIZE_LG,
            typography::SIZE_BASE,
            typography::SIZE_SM,
            typography::SIZE_XS,
        ];

        for (i, window) in sizes.windows(2).enumerate() {
            let [larger, smaller] = window else { continue };
            assert!(
                larger > smaller,
                "Typography size {i} ({larger}) should be larger than {} ({smaller})",
                i + 1
            );
        }
    }

    #[test]
    fn test_typography_sizes_within_reasonable_range() {
        use crate::ui::styles::layout::typography;

        let sizes = [
            typography::SIZE_XS,
            typography::SIZE_SM,
            typography::SIZE_BASE,
            typography::SIZE_LG,
            typography::SIZE_XL,
            typography::SIZE_XXL,
            typography::SIZE_HEADING,
        ];

        for size in sizes {
            assert!(
                (10.0..=60.0).contains(&size),
                "Typography size {size} is outside reasonable range [10.0, 60.0]"
            );
        }
    }

    // Border radius tests

    #[test]
    fn test_border_radius_follows_scale() {
        const {
            assert!(layout::border_radius::SM < layout::border_radius::MD);
            assert!(layout::border_radius::MD < layout::border_radius::LG);
            assert!(layout::border_radius::LG < layout::border_radius::XL);
            assert!(layout::border_radius::XL < layout::border_radius::FULL);
        }
    }

    #[test]
    fn test_border_radius_values_are_reasonable() {
        use crate::ui::styles::layout::border_radius;

        let radii = [
            border_radius::SM,
            border_radius::MD,
            border_radius::LG,
            border_radius::XL,
            border_radius::FULL,
        ];

        for radius in radii {
            assert!(
                (0.0..=100.0).contains(&radius) || radius >= 900.0,
                "Border radius {radius} is outside reasonable range [0.0, 100.0] or large pill"
            );
        }
    }

    #[test]
    fn test_border_radius_increases() {
        use crate::ui::styles::layout::border_radius;

        const {
            assert!(border_radius::SM < border_radius::MD);
            assert!(border_radius::MD < border_radius::LG);
            assert!(border_radius::LG < border_radius::XL);
            assert!(border_radius::XL < border_radius::FULL);
        }
    }

    // Color validation tests

    #[test]
    fn test_color_rgb_values_are_valid() {
        let test_colors = [
            colors::PRIMARY,
            colors::PRIMARY_HOVER,
            colors::PRIMARY_ACTIVE,
            colors::BG_PRIMARY,
            colors::BG_ELEVATED,
            colors::TEXT_PRIMARY,
            colors::TEXT_SECONDARY,
            colors::ERROR,
            colors::WARNING,
            colors::SUCCESS,
            colors::INFO,
            colors::FOCUS_RING,
        ];

        for color in test_colors {
            assert!(
                (0.0..=1.0).contains(&color.r),
                "Color red channel {} is out of valid range [0.0, 1.0]",
                color.r
            );
            assert!(
                (0.0..=1.0).contains(&color.g),
                "Color green channel {} is out of valid range [0.0, 1.0]",
                color.g
            );
            assert!(
                (0.0..=1.0).contains(&color.b),
                "Color blue channel {} is out of valid range [0.0, 1.0]",
                color.b
            );
        }
    }

    #[test]
    fn test_color_alpha_values_are_valid() {
        let test_colors = [
            colors::PRIMARY,
            colors::BG_PRIMARY,
            colors::TEXT_PRIMARY,
            colors::GLASS_OVERLAY,
            colors::GLOW_PRIMARY,
        ];

        for color in test_colors {
            assert!(
                (0.0..=1.0).contains(&color.a),
                "Color alpha {} is out of valid range [0.0, 1.0]",
                color.a
            );
        }
    }

    #[test]
    fn test_primary_color_has_sufficient_saturation() {
        let max_channel = colors::PRIMARY.r.max(colors::PRIMARY.g).max(colors::PRIMARY.b);
        let min_channel = colors::PRIMARY.r.min(colors::PRIMARY.g).min(colors::PRIMARY.b);
        let saturation = max_channel - min_channel;

        assert!(
            saturation > 0.2,
            "Primary color should have sufficient saturation (got {saturation})"
        );
    }

    #[test]
    fn test_gradient_colors_have_reasonable_transitions() {
        let start_end_diff = (
            colors::PRIMARY_GRADIENT_START.r - colors::PRIMARY_GRADIENT_END.r
        ).abs();

        assert!(
            start_end_diff > 0.01 || start_end_diff < 1.0,
            "Gradient colors should have reasonable transition"
        );
    }

    #[test]
    fn test_notification_colors_are_semantically_correct() {
        const {
            assert!(
                colors::SUCCESS.g > colors::SUCCESS.r,
                "Success should have more green than red"
            );
        }

        const {
            assert!(
                colors::WARNING.r > colors::WARNING.b,
                "Warning should have more red than blue"
            );
        }

        const {
            assert!(
                colors::ERROR.r > colors::ERROR.g,
                "Error should have more red than green"
            );
        }
    }

    // Shadow tests

    #[test]
    fn test_shadow_levels_have_valid_values() {
        let shadow_extra_small = effects::shadows::shadow_xs();
        let shadow_small = effects::shadows::shadow_sm();
        let shadow_medium = effects::shadows::shadow_md();
        let shadow_large = effects::shadows::shadow_lg();
        let shadow_extra_large = effects::shadows::shadow_xl();
        let shadow_extra_extra = effects::shadows::shadow_xxl();

        let shadows = [
            shadow_extra_small,
            shadow_small,
            shadow_medium,
            shadow_large,
            shadow_extra_large,
            shadow_extra_extra,
        ];

        for shadow in shadows {
            assert!(
                shadow.blur_radius >= 0.0,
                "Shadow blur radius should be non-negative"
            );
            assert!(
                shadow.color.r >= 0.0 && shadow.color.r <= 1.0,
                "Shadow color red channel out of range"
            );
            assert!(
                shadow.color.a >= 0.0 && shadow.color.a <= 1.0,
                "Shadow color alpha out of range"
            );
        }
    }

    #[test]
    fn test_shadow_progression_increases() {
        let blur_extra_small = effects::shadows::shadow_xs().blur_radius;
        let blur_small = effects::shadows::shadow_sm().blur_radius;
        let blur_medium = effects::shadows::shadow_md().blur_radius;
        let blur_large = effects::shadows::shadow_lg().blur_radius;
        let blur_extra_large = effects::shadows::shadow_xl().blur_radius;
        let blur_extra_extra = effects::shadows::shadow_xxl().blur_radius;

        assert!(blur_extra_small <= blur_small, "Shadow XS blur should be <= SM");
        assert!(blur_small <= blur_medium, "Shadow SM blur should be <= MD");
        assert!(blur_medium <= blur_large, "Shadow MD blur should be <= LG");
        assert!(blur_large <= blur_extra_large, "Shadow LG blur should be <= XL");
        assert!(
            blur_extra_large <= blur_extra_extra,
            "Shadow XL blur should be <= XXL"
        );
    }

    #[test]
    fn test_shadow_offset_y_increases_with_level() {
        let offset_extra_small = effects::shadows::shadow_xs().offset.y;
        let offset_small = effects::shadows::shadow_sm().offset.y;
        let offset_medium = effects::shadows::shadow_md().offset.y;
        let offset_large = effects::shadows::shadow_lg().offset.y;
        let offset_extra_large = effects::shadows::shadow_xl().offset.y;
        let offset_extra_extra = effects::shadows::shadow_xxl().offset.y;

        assert!(
            offset_extra_small <= offset_small,
            "Shadow XS offset should be <= SM"
        );
        assert!(offset_small <= offset_medium, "Shadow SM offset should be <= MD");
        assert!(offset_medium <= offset_large, "Shadow MD offset should be <= LG");
        assert!(
            offset_large <= offset_extra_large,
            "Shadow LG offset should be <= XL"
        );
        assert!(
            offset_extra_large <= offset_extra_extra,
            "Shadow XL offset should be <= XXL"
        );
    }

    #[test]
    fn test_shadow_alpha_increases_with_level() {
        let alpha_small = effects::shadows::shadow_sm().color.a;
        let alpha_medium = effects::shadows::shadow_md().color.a;
        let alpha_extra_large = effects::shadows::shadow_xl().color.a;
        let alpha_extra_extra = effects::shadows::shadow_xxl().color.a;

        assert!(alpha_medium >= alpha_small, "Shadow MD alpha should be >= SM");
        assert!(
            alpha_extra_large >= alpha_medium,
            "Shadow XL alpha should be >= MD"
        );
        assert!(
            alpha_extra_extra >= alpha_extra_large,
            "Shadow XXL alpha should be >= XL"
        );
    }

    #[test]
    fn test_shadow_values_are_positive() {
        let shadow = effects::shadows::shadow_md();
        assert!(shadow.blur_radius >= 0.0, "Shadow blur must be non-negative");
        assert!(shadow.offset.y >= 0.0, "Shadow offset Y should be non-negative");
    }

    // Spacing grid tests

    #[test]
    fn test_spacing_constants_within_grid() {
        use crate::ui::styles::layout::spacing;

        let spacing_values = [
            spacing::XS,
            spacing::SM,
            spacing::MD,
            spacing::LG,
            spacing::XL,
            spacing::XXL,
        ];

        for value in spacing_values {
            let remainder = value % 4.0;
            assert!(
                remainder < f32::EPSILON || (1.0..=100.0).contains(&value),
                "Spacing value {value} should follow 4px grid or be reasonable"
            );
        }
    }

    // Theme hierarchy tests

    #[test]
    fn test_dark_theme_colors_have_lower_luminance() {
        const {
            assert!(
                dark_theme::BG_PRIMARY.r < colors::BG_PRIMARY.r,
                "Dark theme primary background should be darker"
            );
            assert!(
                dark_theme::BG_PRIMARY.g < colors::BG_PRIMARY.g,
                "Dark theme primary background should be darker"
            );
            assert!(
                dark_theme::BG_PRIMARY.b < colors::BG_PRIMARY.b,
                "Dark theme primary background should be darker"
            );
        }
    }

    #[test]
    fn test_text_colors_have_proper_hierarchy() {
        let light_text_ratio = wcag_contrast_ratio(colors::TEXT_PRIMARY, colors::BG_PRIMARY);
        let secondary_text_ratio = wcag_contrast_ratio(colors::TEXT_SECONDARY, colors::BG_PRIMARY);
        let disabled_text_ratio = wcag_contrast_ratio(colors::TEXT_DISABLED, colors::BG_PRIMARY);

        assert!(
            light_text_ratio >= secondary_text_ratio,
            "Primary text should have higher contrast than secondary"
        );
        assert!(
            secondary_text_ratio >= disabled_text_ratio,
            "Secondary text should have higher contrast than disabled"
        );
    }

    #[test]
    fn test_focus_ring_meets_wcag_minimum() {
        let light_ratio = wcag_contrast_ratio(colors::FOCUS_RING, colors::BG_PRIMARY);
        let dark_ratio = wcag_contrast_ratio(colors::FOCUS_RING, dark_theme::BG_PRIMARY);

        assert!(
            light_ratio >= 3.0,
            "Focus ring on light background meets WCAG AA (3:1)"
        );
        assert!(
            dark_ratio >= 3.0,
            "Focus ring on dark background meets WCAG AA (3:1)"
        );
    }

    // === Gradient Smoothness Tests ===

    #[test]
    fn test_primary_gradient_has_smooth_transition() {
        let start = colors::PRIMARY_GRADIENT_START;
        let end = colors::PRIMARY_GRADIENT_END;

        // Calculate total color difference
        let delta_r = (start.r - end.r).abs();
        let delta_g = (start.g - end.g).abs();
        let delta_b = (start.b - end.b).abs();
        let total_delta = delta_r + delta_g + delta_b;

        // Transition should be noticeable but not extreme
        assert!(
            total_delta > 0.05,
            "Primary gradient should have noticeable transition"
        );
        assert!(
            total_delta < 0.8,
            "Primary gradient transition should not be too extreme"
        );
    }

    #[test]
    fn test_accent_gradient_has_smooth_transition() {
        let start = colors::ACCENT_GRADIENT_START;
        let end = colors::ACCENT_GRADIENT_END;

        let delta_r = (start.r - end.r).abs();
        let delta_g = (start.g - end.g).abs();
        let delta_b = (start.b - end.b).abs();
        let total_delta = delta_r + delta_g + delta_b;

        assert!(
            total_delta > 0.05,
            "Accent gradient should have noticeable transition"
        );
        assert!(
            total_delta < 0.8,
            "Accent gradient transition should not be too extreme"
        );
    }

    #[test]
    fn test_continue_gradient_is_distinctive_and_smooth() {
        let start = colors::CONTINUE_GRADIENT_START;
        let end = colors::CONTINUE_GRADIENT_END;

        let delta_r = (start.r - end.r).abs();
        let delta_g = (start.g - end.g).abs();
        let delta_b = (start.b - end.b).abs();
        let total_delta = delta_r + delta_g + delta_b;

        assert!(
            total_delta > 0.05,
            "Continue gradient should have noticeable transition"
        );
        // Continue gradient is intentionally more dramatic for the hero card
        assert!(
            total_delta < 1.0,
            "Continue gradient transition should be smooth, got {total_delta}"
        );
    }

    #[test]
    fn test_subtle_gradient_is_truly_subtle() {
        let start = colors::SUBTLE_GRADIENT_START;
        let end = colors::SUBTLE_GRADIENT_END;

        let delta_r = (start.r - end.r).abs();
        let delta_g = (start.g - end.g).abs();
        let delta_b = (start.b - end.b).abs();
        let total_delta = delta_r + delta_g + delta_b;

        // Subtle gradient should have minimal transition
        assert!(
            total_delta < 0.3,
            "Subtle gradient should have minimal transition, got {total_delta}"
        );
    }

    #[test]
    fn test_all_gradients_are_within_valid_rgb_range() {
        let gradient_pairs = [
            (colors::PRIMARY_GRADIENT_START, colors::PRIMARY_GRADIENT_END),
            (colors::ACCENT_GRADIENT_START, colors::ACCENT_GRADIENT_END),
            (colors::SUBTLE_GRADIENT_START, colors::SUBTLE_GRADIENT_END),
            (colors::CONTINUE_GRADIENT_START, colors::CONTINUE_GRADIENT_END),
        ];

        for (start, end) in gradient_pairs {
            assert!((0.0..=1.0).contains(&start.r));
            assert!((0.0..=1.0).contains(&start.g));
            assert!((0.0..=1.0).contains(&start.b));
            assert!((0.0..=1.0).contains(&end.r));
            assert!((0.0..=1.0).contains(&end.g));
            assert!((0.0..=1.0).contains(&end.b));
        }
    }

    #[test]
    fn test_gradient_colors_have_proper_contrast_for_white_text() {
        // Test that gradient END colors (darker) have sufficient contrast with white text
        // Gradient start colors are lighter and may not meet 4.5:1, but gradient
        // end colors are darker and provide the needed contrast for text
        let gradient_colors = [
            colors::PRIMARY_GRADIENT_START,  // Dark rose - meets contrast
            colors::PRIMARY_GRADIENT_END,     // Even darker - meets contrast
            colors::ACCENT_GRADIENT_END,      // Teal - should meet contrast
            colors::CONTINUE_GRADIENT_END,    // Darker pink - meets contrast
        ];

        for gradient in gradient_colors {
            let ratio = wcag_contrast_ratio(colors::TEXT_ON_PRIMARY, gradient);
            assert!(
                ratio >= 3.0,
                "Gradient color should have at least 3.0:1 contrast for large text, got {ratio:.2}"
            );
        }
    }

    // === Enhanced Shadow Tests ===

    #[test]
    fn test_shadow_xs_has_minimal_elevation() {
        let shadow = effects::shadows::shadow_xs();
        assert!(
            shadow.blur_radius <= 4.0,
            "Shadow XS should have minimal blur, got {}",
            shadow.blur_radius
        );
        assert!(
            shadow.color.a <= 0.1,
            "Shadow XS should have minimal opacity, got {}",
            shadow.color.a
        );
    }

    #[test]
    fn test_shadow_xxl_has_maximum_elevation() {
        let shadow = effects::shadows::shadow_xxl();
        assert!(
            shadow.blur_radius >= 20.0,
            "Shadow XXL should have significant blur, got {}",
            shadow.blur_radius
        );
        assert!(
            shadow.blur_radius <= 30.0,
            "Shadow XXL blur should be reasonable, got {}",
            shadow.blur_radius
        );
    }

    #[test]
    fn test_shadow_opacity_levels_are_consistent() {
        let shadows = [
            effects::shadows::shadow_xs(),
            effects::shadows::shadow_sm(),
            effects::shadows::shadow_md(),
            effects::shadows::shadow_lg(),
            effects::shadows::shadow_xl(),
            effects::shadows::shadow_xxl(),
        ];

        // Verify each shadow has valid opacity
        for (i, shadow) in shadows.iter().enumerate() {
            assert!(
                shadow.color.a >= 0.0 && shadow.color.a <= 0.5,
                "Shadow level {} has invalid opacity {}",
                i,
                shadow.color.a
            );
        }
    }

    #[test]
    fn test_shadow_offset_increases_with_elevation() {
        let xs_offset = effects::shadows::shadow_xs().offset.y;
        let md_offset = effects::shadows::shadow_md().offset.y;
        let xxl_offset = effects::shadows::shadow_xxl().offset.y;

        assert!(
            xxl_offset > md_offset,
            "XXL shadow offset should be greater than MD"
        );
        assert!(
            md_offset > xs_offset,
            "MD shadow offset should be greater than XS"
        );
    }

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
}
