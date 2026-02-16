# Design System Master File

> **LOGIC:** When building a specific page, first check `design-system/pages/[page-name].md`.
> If that file exists, its rules **override** this Master file.
> If not, strictly follow the rules below.

---

**Project:** Nodoka Audiobook Player
**Generated:** 2026-02-15 22:02:09
**Category:** Social Media App

---

## Global Rules

### Color Palette

| Role | Hex | CSS Variable |
|------|-----|--------------|
| Primary | `#DB2777` | `--color-primary` |
| Secondary | `#F472B6` | `--color-secondary` |
| CTA/Accent | `#CA8A04` | `--color-cta` |
| Background | `#FDF2F8` | `--color-background` |
| Text | `#831843` | `--color-text` |

**Color Notes:** Romantic pink + elegant gold

### Typography

- **Heading Font:** Righteous
- **Body Font:** Poppins
- **Mood:** music, entertainment, fun, energetic, bold, performance
- **Google Fonts:** [Righteous + Poppins](https://fonts.google.com/share?selection.family=Poppins:wght@300;400;500;600;700|Righteous)

**CSS Import:**
```css
@import url('https://fonts.googleapis.com/css2?family=Poppins:wght@300;400;500;600;700&family=Righteous&display=swap');
```

### Spacing Variables

| Token | Value | Usage |
|-------|-------|-------|
| `--space-xs` | `4px` / `0.25rem` | Tight gaps |
| `--space-sm` | `8px` / `0.5rem` | Icon gaps, inline spacing |
| `--space-md` | `16px` / `1rem` | Standard padding |
| `--space-lg` | `24px` / `1.5rem` | Section padding |
| `--space-xl` | `32px` / `2rem` | Large gaps |
| `--space-2xl` | `48px` / `3rem` | Section margins |
| `--space-3xl` | `64px` / `4rem` | Hero padding |

### Shadow Depths

| Level | Value | Usage |
|-------|-------|-------|
| `--shadow-sm` | `0 1px 2px rgba(0,0,0,0.05)` | Subtle lift |
| `--shadow-md` | `0 4px 6px rgba(0,0,0,0.1)` | Cards, buttons |
| `--shadow-lg` | `0 10px 15px rgba(0,0,0,0.1)` | Modals, dropdowns |
| `--shadow-xl` | `0 20px 25px rgba(0,0,0,0.15)` | Hero images, featured cards |

---

## Component Specs

### Buttons

```css
/* Primary Button */
.btn-primary {
  background: #CA8A04;
  color: white;
  padding: 12px 24px;
  border-radius: 8px;
  font-weight: 600;
  transition: all 200ms ease;
  cursor: pointer;
}

.btn-primary:hover {
  opacity: 0.9;
  transform: translateY(-1px);
}

/* Secondary Button */
.btn-secondary {
  background: transparent;
  color: #DB2777;
  border: 2px solid #DB2777;
  padding: 12px 24px;
  border-radius: 8px;
  font-weight: 600;
  transition: all 200ms ease;
  cursor: pointer;
}
```

### Cards

```css
.card {
  background: #FDF2F8;
  border-radius: 12px;
  padding: 24px;
  box-shadow: var(--shadow-md);
  transition: all 200ms ease;
  cursor: pointer;
}

.card:hover {
  box-shadow: var(--shadow-lg);
  transform: translateY(-2px);
}
```

### Inputs

```css
.input {
  padding: 12px 16px;
  border: 1px solid #E2E8F0;
  border-radius: 8px;
  font-size: 16px;
  transition: border-color 200ms ease;
}

.input:focus {
  border-color: #DB2777;
  outline: none;
  box-shadow: 0 0 0 3px #DB277720;
}
```

### Modals

```css
.modal-overlay {
  background: rgba(0, 0, 0, 0.5);
  backdrop-filter: blur(4px);
}

.modal {
  background: white;
  border-radius: 16px;
  padding: 32px;
  box-shadow: var(--shadow-xl);
  max-width: 500px;
  width: 90%;
}
```

---

## Style Guidelines

**Style:** Vibrant & Block-based

**Keywords:** Bold, energetic, playful, block layout, geometric shapes, high color contrast, duotone, modern, energetic

**Best For:** Startups, creative agencies, gaming, social media, youth-focused, entertainment, consumer

**Key Effects:** Large sections (48px+ gaps), animated patterns, bold hover (color shift), scroll-snap, large type (32px+), 200-300ms

### Page Pattern

**Pattern Name:** Video-First Hero

- **Conversion Strategy:** 86% higher engagement with video. Add captions for accessibility. Compress video for performance.
- **CTA Placement:** Overlay on video (center/bottom) + Bottom section
- **Section Order:** 1. Hero with video background, 2. Key features overlay, 3. Benefits section, 4. CTA

---

## Anti-Patterns (Do NOT Use)

- ❌ Heavy skeuomorphism
- ❌ Accessibility ignored

### Additional Forbidden Patterns

- ❌ **Emojis as icons** — Use SVG icons (Heroicons, Lucide, Simple Icons)
- ❌ **Missing cursor:pointer** — All clickable elements must have cursor:pointer
- ❌ **Layout-shifting hovers** — Avoid scale transforms that shift layout
- ❌ **Low contrast text** — Maintain 4.5:1 minimum contrast ratio
- ❌ **Instant state changes** — Always use transitions (150-300ms)
- ❌ **Invisible focus states** — Focus states must be visible for a11y

---

## Implementation Status (iced 0.14)

### ✅ Implemented Features

The following ui-ux-pro-max features have been successfully implemented in the Nodoka audiobook player using the iced framework:

#### Design System Core
- **Shadow depth system** (sm, md, lg, xl) - Applied to buttons, thumbnails, player controls, and modals
- **Interactive feedback patterns** (hover, active, focus) - 5% hover overlay, 10% active overlay, 3px focus rings
- **Spacing grid** (4px base) - Consistent XS/SM/MD/LG/XL/XXL spacing throughout
- **Typography scale** - SIZE_XS through SIZE_XXL with proper hierarchy
- **Border radius consistency** - SM/MD/LG for visual consistency
- **Color system** - Vibrant rose palette with WCAG AA compliance

#### Advanced Animation System (v0.2.1)
- **Easing functions** - Linear, ease-in, ease-out, ease-in-out, ease-out-bounce, ease-out-elastic
- **FadeTransition** - Opacity animations with configurable duration and easing
- **ScaleTransition** - Scale animations for microinteractions (e.g., 0.95 to 1.0)
- **PulseAnimation** - Oscillating opacity for loading indicators
- **Stagger utility** - Delayed animations for list items (50ms per item)
- **Reduced motion support** - All animations respect user preferences (duration = 0ms when enabled)

#### Enhanced Interactive States (v0.2.1)
- **InteractiveState enum** - Combined state tracking (Idle/Hover/Focus/HoverFocus/Pressed/Disabled)
- **State-aware overlays** - Consistent darkening for different interaction states
- **Border width utilities** - 3px for focus states (WCAG compliance), 1px for others
- **Pointer cursor utilities** - Automatic cursor management based on state

#### Alternative Color Palette
- **Audiobook-optimized theme** (calm cyan + health green) - Designed for reduced eye strain during extended listening sessions
- **Module:** `colors_audiobook` with full palette support
- **Usage:** Available for future theme switching capability

#### Visual Feedback
- **Volume level color indicators** - Green (70%+), Yellow (30-70%), Red (1-30%), Gray (muted)
- **Active speed preset highlighting** - Visual indication of current playback speed
- **Button shadows on hover** - Subtle elevation feedback (shadow_sm)
- **Thumbnail shadows** - Enhanced depth for cover art (shadow_sm)
- **Player controls elevation** - Raised panel with shadow_md

#### Loading & Empty States
- **Skeleton UI** with pulse animation - Shows during async operations >200ms
- **Empty states** with icons - Unicode symbols (📚, 📄, 🔖) for visual context
- **Loading color system** - Grayscale palette for skeleton backgrounds

#### Accessibility Features
- **Keyboard navigation** - Full support with Tab/Shift+Tab, Arrow keys, shortcuts
- **Focus indicators** - 3px blue border meeting WCAG 2.1 2.4.7
- **Contrast ratios** - All text combinations meet WCAG AA 4.5:1 minimum
- **Reduced motion** - Animation durations respect user preferences
- **Touch targets** - 44x44px minimum per WCAG 2.1 Level AAA

#### UI Components
- **Notification toast system** - Auto-dismiss (success: 3s, info: 3s, warning: 5s, error: manual)
- **Modal overlays** - Semi-transparent backdrop with shadow_xl elevation
- **Keyboard help overlay** - Grouped shortcuts with styled keys (bordered containers)
- **Progress indicators** - Visual bars for audiobook completion and playback position

### 📋 Documented for Future Implementation

These features are defined in the design system but require future framework capabilities:

- **Custom web fonts** (Atkinson Hyperlegible) - iced 0.14 uses platform fonts only
- **Line height control** - Not supported in current iced version
- **SVG icon system** - iced requires raster images or Unicode text
- **CSS-like shimmer animations** - Would require custom shaders
- **Circular progress indicators** - iced only supports linear progress bars
- **Multiple shadows per element** - Framework limitation
- **Letter-spacing control** - Typography feature not yet available

### ⚠️ Framework Limitations (iced 0.14)

Important constraints when working with iced framework:

- **Icons:** Use Unicode symbols (✓, ℹ, ⚠, ✕) or raster images (PNG/JPG), not SVG
- **Fonts:** Platform fonts only, cannot bundle custom web fonts like Righteous/Poppins
- **Typography:** No line-height, letter-spacing, or font-variant control
- **Animations:** Limited animation support, no CSS-like transitions
- **Shadows:** Basic shadow support (color, offset, blur), no multiple shadows
- **Layout:** No CSS Grid equivalent, use row/column combinations
- **Hover effects:** Implemented via Status enum in style functions, not CSS :hover

### Integration with Rust Ecosystem

The design system is implemented as modular Rust modules in `src/ui/styles/`:

```rust
use nodoka::ui::styles::{
    // Core Design System
    colors,              // Main vibrant rose palette
    colors_audiobook,    // Alternative calm cyan palette
    spacing,             // 4px base grid (XS through XXL)
    typography,          // Size scale for text hierarchy
    border_radius,       // Corner rounding (SM, MD, LG)
    
    // Visual Effects (v0.2.1)
    shadows,             // Elevation system (sm, md, lg, xl)
    animations,          // Easing, FadeTransition, ScaleTransition, PulseAnimation
    transitions,         // Duration constants (FAST, NORMAL, SLOW)
    
    // Interactive Feedback (v0.2.1)
    interactive,         // InteractiveState enum, state overlays, pointer utilities
    
    // Component Utilities
    loading,             // Skeleton UI and pulse animation
    button_styles,       // Pre-configured button variants
    
    // Accessibility (v0.2.1)
    accessibility,       // Touch target validation, contrast verification
};
```

#### Module Structure

- `src/ui/styles/mod.rs` - Main module with re-exports for API compatibility
- `src/ui/styles/colors.rs` - Color palettes (vibrant rose, calm cyan)
- `src/ui/styles/layout.rs` - Spacing, typography, border radius constants
- `src/ui/styles/effects.rs` - Shadows, animations, transitions
- `src/ui/styles/interactive.rs` - Interactive state management
- `src/ui/styles/components.rs` - Button styles, focus indicators
- `src/ui/styles/accessibility.rs` - WCAG compliance utilities

### Test Coverage

Comprehensive UX compliance tests in `tests/ux_compliance_tests.rs`:

#### Core Design System Tests
- ✅ Shadow depth verification (sm, md, lg, xl specifications)
- ✅ Interactive overlay color blending (hover/active darken correctly)
- ✅ Spacing grid compliance (4px base multiples)
- ✅ Typography scale hierarchy
- ✅ Alternative palette contrast ratios (WCAG AA compliance)
- ✅ Empty state icon support (Unicode symbols)

#### Animation & Transition Tests (v0.2.1)
- ✅ Animation transitions respect reduced motion preferences
- ✅ Hover overlay darkens consistently (5% overlay)
- ✅ Interactive state overlay progression (Idle < Hover < Active)
- ✅ Easing functions mathematical correctness (bounce, elastic)
- ✅ Fade transition bounds (opacity stays within 0.0-1.0)
- ✅ Scale transition bounds (scale values within valid range)
- ✅ Modal animations complete within budget (300ms)
- ✅ Stagger delay calculation (50ms per item)
- ✅ Pulse animation oscillation (loading state feedback)

#### Component Behavior Tests (v0.2.1)
- ✅ Notification timing validation (auto-dismiss durations)
- ✅ Volume color semantic thresholds (70%, 30% boundaries)
- ✅ Skeleton UI dimensions match real components
- ✅ Focus indicator visibility (3px minimum border)
- ✅ Interactive state border width (3px focus, 1px others)
- ✅ Pointer cursor display logic (disabled states)

#### Accessibility Tests
- ✅ Focus ring meets WCAG contrast (3:1 minimum)
- ✅ Touch target validation (44x44px minimum)
- ✅ Contrast verification (WCAG AA 4.5:1, AAA 7:1)
- ✅ Transition duration standards (150-300ms)

Total: 74 UX compliance tests ensuring design system adherence.

---

## Pre-Delivery Checklist

Before delivering any UI code, verify:

### General UX Requirements
- [ ] No emojis used as functional icons (decorative empty state icons are acceptable)
- [ ] All icons from consistent set (Unicode symbols for iced, SVG for web)
- [ ] `cursor-pointer` on all clickable elements (via `button::on_press` or cursor module)
- [ ] Hover states with smooth transitions (150-300ms via style Status enum)
- [ ] Light mode: text contrast 4.5:1 minimum (verified in tests)
- [ ] Focus states visible for keyboard navigation (3px minimum border)
- [ ] `prefers-reduced-motion` respected (via transition constants)
- [ ] Responsive: 375px, 768px, 1024px, 1440px (iced window resizing)
- [ ] No content hidden behind fixed navbars
- [ ] No horizontal scroll on mobile

### iced Framework Specific
- [ ] Shadow system used appropriately (sm: buttons, md: panels, lg: modals, xl: hero)
- [ ] Interactive overlays applied (5% hover, 10% active)
- [ ] Spacing follows 4px grid (use spacing:: constants, not hardcoded values)
- [ ] Typography uses SIZE_ constants (not raw pixel values)
- [ ] Border radius uses constants (SM/MD/LG, not hardcoded)
- [ ] Colors from design system modules (colors:: or colors_audiobook::)
- [ ] Empty states use `empty_state()` helper with optional icons
- [ ] Loading states show skeleton UI for operations >200ms
- [ ] Notifications use proper timing (3s success/info, 5s warning, manual error)
- [ ] All tests pass (`cargo test --tests`)
- [ ] No clippy warnings (`cargo clippy --all-targets -- -D warnings`)
- [ ] Documentation builds (`cargo doc --no-deps`)
