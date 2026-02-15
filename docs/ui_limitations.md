# UI Limitations in iced 0.12

This document describes known limitations of the iced 0.12 GUI framework and provides workarounds for achieving desired UX patterns in Nodoka Audiobook Player.

## Overview

Nodoka uses iced 0.12 for its cross-platform GUI. While iced provides excellent cross-platform support and performance, version 0.12 has several limitations that affect UI/UX implementation. This document serves as a guide for developers working on the UI.

## Button Styling Limitations

### Issue

Custom button styles cannot be applied via the `.style()` method in iced 0.12. The method expects `impl Into<iced::theme::Button>`, but custom styling functions return `impl Fn(&Theme) -> button::Appearance`, which is incompatible.

### Impact

- **Severity**: Moderate UX issue
- All buttons look identical, violating visual hierarchy principles
- Primary, secondary, and danger actions are not visually distinguished
- Users may have difficulty identifying the most important action in a context

### Code Example (Non-functional)

```rust
// This does NOT work in iced 0.12
button(text("Save"))
    .on_press(Message::Save)
    .style(button_styles::primary())  // ❌ Compilation error
```

### Workaround: Container-Based Styling

Wrap buttons in styled containers to achieve visual hierarchy:

```rust
use iced::widget::{button, container, text};
use crate::ui::styles::{button_containers, spacing};

// ✅ This works in iced 0.12
container(
    button(text("Save"))
        .on_press(Message::Save)
)
.style(button_containers::primary())
.padding(spacing::SM)
```

### Available Container Styles

- `button_containers::primary()` - Vibrant rose background for primary actions
- `button_containers::secondary()` - Elevated background with border for secondary actions
- `button_containers::danger()` - Error color for destructive actions

See `src/ui/styles.rs` for complete documentation.

## Focus Indicator Limitations

### Issue

iced 0.12 does not expose focus state to custom styling functions. Applications cannot programmatically determine which widget currently has keyboard focus.

### Impact

- **Severity**: High accessibility issue (WCAG 2.1 AA 2.4.7 violation)
- Keyboard users cannot see which element is focused
- Tab navigation is functional but invisible
- Screen reader users are unaffected (focus is tracked internally)

### Workaround 1: Application State Tracking

Track focus manually in application state:

```rust
pub struct State {
    pub focused_element: FocusedElement,
    // ... other fields
}

pub enum FocusedElement {
    None,
    PlayPauseButton,
    VolumeSlider,
    AudiobookList,
    // ... other focusable elements
}
```

Apply conditional styling based on focus state:

```rust
let play_button_style = if state.focused_element == FocusedElement::PlayPauseButton {
    container::Appearance {
        border: Border {
            color: colors::FOCUS_RING,
            width: 3.0,
            radius: border_radius::MD.into(),
        },
        ..Default::default()
    }
} else {
    container::Appearance::default()
};
```

### Workaround 2: Color-Based Cues

Use hover color changes as partial substitute for focus indicators:

- Buttons change color on hover (iced built-in behavior)
- Selected items use high-contrast background colors
- Active elements use distinct text colors

### Workaround 3: Screen Reader Support

Rely on iced's internal accessibility tree for screen reader users:

- Focus state is tracked internally by iced
- Screen readers correctly announce focused elements
- Keyboard navigation works correctly (Tab, arrow keys, Space, Enter)

## Modal Backdrop Limitations

### Issue

iced 0.12 does not provide a `Stack` widget for layering UI elements. Modal dialogs cannot have semi-transparent backdrops that prevent interaction with background content.

### Impact

- **Severity**: Low UX issue
- Users can accidentally click background elements while modal is open
- Visual hierarchy is less clear (no dimming effect)
- Modal dismissal requires explicit action (button or Escape key)

### Workaround

- Use Escape key to close modals (implemented in keyboard shortcuts)
- Provide prominent "Close" or "Cancel" buttons
- Use distinct background colors for modal content
- Document modal behavior in manual testing checklist

Example:

```rust
// Settings modal
if state.settings_open {
    container(
        column![
            text("Settings").size(typography::SIZE_HEADING),
            // ... settings form ...
            button(text("Close")).on_press(Message::CloseSettings),
        ]
    )
    .style(|_| container::Appearance {
        background: Some(colors::BG_SECONDARY.into()),
        border: Border {
            color: colors::BORDER_DEFAULT,
            width: 2.0,
            radius: border_radius::LG.into(),
        },
        ..Default::default()
    })
    .padding(spacing::LG)
}
```

## Upgrade Path

Many of these limitations may be resolved in future iced versions:

### iced 0.13+ (Future)

- Expected to have improved styling APIs
- May support custom button styles via theme system
- Possible `Stack` widget for proper modals

### Migration Strategy

When upgrading iced:

1. Test button styling with `.style()` method
2. If functional, replace container workarounds with direct button styles
3. Test focus indicators with new iced APIs
4. Evaluate `Stack` widget for modal backdrops
5. Update this document with findings

### Tracking Issue

Monitor iced GitHub repository for:
- Button styling improvements
- Focus state exposure
- Stack/overlay widgets
- Accessibility enhancements

## Testing Implications

### Automated Testing

Cannot test:
- Visual appearance of buttons (container workarounds are tested instead)
- Focus ring visibility (manual testing required)
- Modal backdrop behavior

Can test:
- Message routing (keyboard shortcuts, button clicks)
- State transitions
- Component rendering (no-panic smoke tests)

### Manual Testing

Required for:
- Visual hierarchy verification (button colors distinguishable)
- Focus indicator visibility (Tab through UI)
- Modal interaction (background click-through)
- Keyboard navigation (all shortcuts work)

See `tests/manual_ui_checklist.md` for complete manual testing protocol.

## References

- iced documentation: https://docs.rs/iced/0.12/iced/
- WCAG 2.1 AA Guidelines: https://www.w3.org/WAI/WCAG21/quickref/
- Nodoka design system: `design-system/nodoka-audiobook-player/MASTER.md`
- Button styling code: `src/ui/styles.rs` (lines 197-341)
