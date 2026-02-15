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
