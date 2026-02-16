# Nodoka Design System (iced Desktop UI)

> **LOGIC:** When building a specific screen, first check `design-system/pages/[screen-name].md`.
> If that file exists, its rules override this Master file.
> If not, follow the rules below.

---

Project: Nodoka Audiobook Player
Updated: 2026-02-15
Stack: Rust + iced 0.14

---

## Source Of Truth

- Tokens and component styles live in `src/ui/styles.rs` (colors, spacing, typography, shadows, button styles)
- UI composition patterns live in `src/ui/main_window.rs` (stack overlays, modal backdrop, modal card)

This file describes intent and usage. Do not treat it as a second implementation.

## Tokens

### Color Roles

Use `src/ui/styles.rs` `colors::*` constants.

- Primary action: `colors::PRIMARY` (#E11D48)
- Accent / focus: `colors::ACCENT` and `colors::FOCUS_RING` (#2563EB)
- Backgrounds: `colors::BG_PRIMARY`, `colors::BG_SECONDARY`, `colors::BG_ELEVATED`
- Text: `colors::TEXT_PRIMARY`, `colors::TEXT_SECONDARY`, `colors::TEXT_ON_PRIMARY`
- Semantics: `colors::SUCCESS`, `colors::WARNING`, `colors::ERROR`

### Spacing And Type

Use `src/ui/styles.rs` `spacing::*` and `typography::*`.

- Spacing follows a 4px base grid (XS/SM/MD/LG/XL)
- Text sizes use the shared scale (XS..XXL)

## Component Guidelines

### Buttons

- Use `button_styles::{primary,secondary,danger}`
- Prefer text labels over icon-only controls
- Keep click targets comfortable (roughly 44x44px minimum)

### Modals

- Pattern: `stack![content, backdrop, dialog]`
- Backdrop: semi-transparent, click-to-dismiss for dismissible modals
- Dialog: centered on both axes; prefer fixed widths for form-like dialogs

### Lists (Audiobooks / Files / Bookmarks)

- Selection: use a clear, high-contrast selection treatment
- Scrolling: lists should remain usable with large libraries; avoid heavy per-row work in view()

### Errors And Loading

- Errors: top banner with a dismiss action
- Loading: show a blocking overlay for global loading states

## Accessibility Baselines

- Keyboard: all core actions reachable without a mouse
- Focus: visible focus indicator (use `colors::FOCUS_RING`)
- Contrast: keep text contrast at or above WCAG AA (4.5:1 for normal text)

## Interactive Patterns (ui-ux-pro-max)

### Cursor Affordances
- All clickable elements: `cursor::clickable()`
- Disabled elements: `cursor::disabled()`
- Text input areas: `cursor::text()`

### Hover States
- Use subtle background color changes (5% darker overlay via `interactive::apply_hover_overlay()`)
- Transition duration: 150-200ms for smooth feedback (see `transitions::*`)
- Never scale elements on hover (causes layout shift)
- Ensure hover state is distinct from selection state

### Pressed/Active States
- Use `interactive::apply_active_overlay()` for 10% darker overlay
- Increase border width from 1px to 2px on pressed state
- Use `colors::BORDER_FOCUS` for pressed border color

### Touch Targets
- Minimum 44x44px per WCAG 2.1 Level AAA
- Use `spacing::MD` (16px) padding on buttons
- Increase padding on small touch areas (icons, checkboxes)

### Loading States (Implemented)

#### Skeleton UI

- Used during directory scanning when audiobook list is empty
- 3 skeleton items shown to indicate loading activity
- Skeleton covers: 60x60px with `loading::SKELETON_BG` color
- Skeleton text: 150x20px with rounded corners (SM radius)
- Consistent spacing follows 4px grid system
- Colors: `loading::SKELETON_BG` and `loading::SKELETON_SHIMMER`
- Spinner color: `loading::SPINNER_COLOR` (matches primary brand)

#### When to Use

- Directory scanning operations (can take multiple seconds)
- Initial library load if no cached data available
- Show loading indicators for operations >200ms
- Prefer skeleton UI over spinners for list-based content

### Empty States
- Include helpful title and description
- Explain why the list is empty
- Provide clear CTA when action is available
- Use `empty_state()` helper function for consistency
- Examples:
  - Audiobook list: "Add audiobook directories in Settings to get started"
  - File list: "Select an audiobook from the list to view its files"
  - Bookmarks: "Create bookmarks to mark important positions"

### Modal Overlays
- Backdrop opacity: 50% (0.5 alpha) for clear distinction
- Click-to-dismiss for user-initiated modals only
- Focus trap: return focus to triggering element on close
- Modal cards: elevated with shadow and border (see `modal_card()` in main_window.rs)

### Progress Indicators
- File progress: 6px height bars with semantic colors
- Complete files: green accent (`colors::SUCCESS`) with [COMPLETE] label
- In-progress: percentage display with progress bar
- Missing files: warning color (`colors::WARNING`) with [MISSING] label

## Accessibility Enhancements

### Visual Feedback
- Focus indicators: 3px blue ring (WCAG 2.1 2.4.7) using `colors::FOCUS_RING`
- Error states: red border + text description
- Success states: green accent + confirmation message

### Reduced Motion (Implemented)

#### WCAG 2.1 2.3.3 Compliance

- `State.prefers_reduced_motion` flag controls all animations
- When enabled: all transition durations become 0ms
- Use `animations::transition_duration()` helper in components
- Never hard-code transition durations without checking preference

#### Testing

- Verify animations disable when flag is true
- Test that UI remains functional without transitions
- Check focus indicators remain visible without animations

### Semantic Colors
- Use color to reinforce status but not as sole indicator
- Always include text labels alongside color coding
- [MISSING], [COMPLETE] labels in addition to color
- Maintains accessibility for colorblind users
