# Testing Strategy

## Overview

Nodoka uses a three-layer testing approach combining automated tests with manual verification. This document explains the testing philosophy, test organization, and best practices.

## Testing Philosophy

### Test Behavior, Not Implementation

Tests should verify **what** the code does, not **how** it does it. This ensures tests survive refactoring.

❌ **Bad** - Tests implementation details:
```rust
#[test]
fn test_volume_field_exists() {
    let state = State::default();
    let _ = state.volume;  // Testing field existence
}
```

✅ **Good** - Tests behavior:
```rust
#[test]
fn test_volume_changes_affect_playback() {
    let mut state = State::default();
    update(&mut state, Message::VolumeChanged(75));
    assert_eq!(state.volume, 75);
}
```

### Comprehensive Coverage

Target: **300+ automated tests** covering all UI interactions.

Current breakdown:
- Unit tests: ~200 (library + UI components)
- Integration tests: ~100 (workflows, database, player)
- Manual tests: 18 test cases (visual verification)

### Fail Fast

Tests should fail immediately when requirements are violated:

```rust
#[test]
fn test_volume_boundary_clamping() {
    let mut state = State::default();
    update(&mut state, Message::VolumeChanged(-10));
    assert_eq!(state.volume, 0, "Volume should clamp to 0");
    
    update(&mut state, Message::VolumeChanged(300));
    assert_eq!(state.volume, 200, "Volume should clamp to 200");
}
```

### Performance Baselines

Performance tests establish baselines to detect regressions:

```rust
#[test]
fn test_load_100_audiobooks_performance() {
    let start = Instant::now();
    let audiobooks = load_100_audiobooks();
    let duration = start.elapsed();
    
    assert!(
        duration.as_millis() < 100,
        "Loading 100 audiobooks took {:?}, should be < 100ms",
        duration
    );
}
```

## Three-Layer Testing Approach

### Layer 1: Unit Tests

**Purpose**: Test individual functions and components in isolation.

**Location**: 
- Library tests: `src/**/*_tests.rs` or `#[cfg(test)]` modules
- Component tests: `src/ui/components/*/tests.rs`

**What to test**:
- Pure functions
- Component rendering (smoke tests)
- Boundary conditions
- Error handling
- Edge cases

**Example** - Player controls unit test:
```rust
#[test]
fn test_volume_slider_boundary_values() {
    let mut state = State::default();
    
    // Test minimum
    state.volume = 0;
    let view = player_controls::view(&state);
    // Rendering doesn't panic
    
    // Test maximum
    state.volume = 200;
    let view = player_controls::view(&state);
    
    // Test mid-range
    state.volume = 100;
    let view = player_controls::view(&state);
}
```

**Naming convention**: `test_<component>_<scenario>`

### Layer 2: Integration Tests

**Purpose**: Test multiple components working together.

**Location**: `tests/*.rs` files

**Categories**:

1. **Acceptance Tests** (`acceptance_*.rs`)
   - Database operations
   - File scanning
   - Metadata extraction
   - Progress tracking
   
2. **UI Workflow Tests** (`*_workflow_tests.rs`)
   - Complete user journeys
   - State transitions
   - Modal interactions
   
3. **Keyboard Navigation** (`keyboard_navigation_tests.rs`)
   - All keyboard shortcuts
   - Modifier key combinations
   - Focus management
   
4. **State Transitions** (`ui_state_transitions_tests.rs`)
   - Complex multi-step workflows
   - Error recovery
   - Concurrent operations
   
5. **UX Compliance** (`ux_compliance_tests.rs`)
   - Design system adherence
   - Spacing/typography
   - Color contrast
   
6. **Accessibility** (`accessibility_tests.rs`)
   - Keyboard access
   - Screen reader support
   - Focus indicators
   
7. **Error Handling** (`ui_error_handling_tests.rs`)
   - Invalid input
   - Missing data
   - Race conditions
   
8. **Performance** (`ui_performance_tests.rs`)
   - Large datasets (100+ audiobooks, 1000+ files)
   - Query speed
   - State update performance

**Example** - Integration test:
```rust
#[test]
fn test_bookmark_complete_workflow() -> Result<(), Box<dyn Error>> {
    let db = create_test_db()?;
    let audiobook_id = create_test_audiobook(&db, "/test", "Book")?;
    
    // Create bookmark
    let bookmark = Bookmark { /* ... */ };
    let id = db::queries::insert_bookmark(db.connection(), &bookmark)?;
    
    // Verify bookmark
    let bookmarks = db::queries::get_bookmarks_for_audiobook(
        db.connection(), 
        audiobook_id
    )?;
    assert_eq!(bookmarks.len(), 1);
    
    // Delete bookmark
    db::queries::delete_bookmark(db.connection(), id)?;
    
    // Verify deletion
    let bookmarks = db::queries::get_bookmarks_for_audiobook(
        db.connection(), 
        audiobook_id
    )?;
    assert!(bookmarks.is_empty());
    
    Ok(())
}
```

**Naming convention**: `test_<workflow>_<scenario>`

### Layer 3: Manual Tests

**Purpose**: Verify visual design, accessibility, and cross-platform behavior.

**Location**: `tests/manual_ui_checklist.md`

**What to test manually**:
- Visual hierarchy (button colors, spacing)
- Focus indicators (Tab navigation)
- Modal backdrops
- Loading states
- Error banner styling
- Screen reader announcements
- Platform-specific behavior

**Test protocol**:

Each test case includes:
1. Prerequisites (app state)
2. Steps to perform
3. Expected results
4. Pass checkboxes for Windows/macOS/Linux

**Example**:
```markdown
## Test Case 5: Button Visual Hierarchy

**Prerequisites**: Application running with audiobook selected

**Steps**:
1. Navigate to player controls
2. Observe button colors and borders
3. Compare primary (Play), secondary (Stop), danger buttons

**Expected**:
- Play button: Vibrant rose background (#CC3366)
- Stop button: Elevated background with border
- All buttons clearly distinguishable

**Results**:
- [ ] Windows 11
- [ ] macOS 14
- [ ] Ubuntu 22.04
```

## Test Organization

### Directory Structure

```
tests/
├── acceptance_*.rs          # Database/scanning tests
├── keyboard_navigation_tests.rs
├── ui_state_transitions_tests.rs
├── e2e_workflow_tests.rs
├── ux_compliance_tests.rs
├── accessibility_tests.rs
├── ui_error_handling_tests.rs
├── ui_performance_tests.rs
├── manual_ui_checklist.md
└── acceptance_support/      # Test utilities
    ├── mod.rs
    ├── database.rs
    └── fixtures.rs

src/
├── db/
│   └── queries.rs           # Database query tests
├── ui/
│   ├── components/
│   │   ├── player_controls.rs  # Component tests
│   │   ├── audiobook_list.rs
│   │   └── file_list.rs
│   └── update/
│       └── tests.rs         # Update handler tests
└── models/
    └── *_tests.rs           # Model tests
```

### Test Utilities

#### `acceptance_support` Module

Shared test utilities:

```rust
// Create test database
pub fn create_test_db() -> Result<TestDatabase> {
    let db = TestDatabase::new()?;
    db.run_migrations()?;
    Ok(db)
}

// Create test audiobook
pub fn create_test_audiobook(
    db: &TestDatabase,
    directory: &str,
    name: &str,
) -> Result<i64> {
    let audiobook = Audiobook::new(
        directory.to_string(),
        name.to_string(),
        format!("{}/{}", directory, name),
        0,
    );
    db::queries::insert_audiobook(db.connection(), &audiobook)
}

// Create test file
pub fn insert_test_file(
    db: &TestDatabase,
    audiobook_id: i64,
    name: &str,
) -> Result<()> {
    let file = AudiobookFile { /* ... */ };
    db::queries::insert_audiobook_file(db.connection(), &file)
}
```

#### Test Fixtures

Reusable test data:

```rust
// fixtures/sample_audiobook/
// ├── Chapter 1.mp3
// ├── Chapter 2.mp3
// └── cover.jpg

pub fn get_fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}
```

## Testing Patterns

### Pattern 1: Arrange-Act-Assert (AAA)

```rust
#[test]
fn test_play_pause_toggle() {
    // Arrange
    let mut state = State::default();
    state.is_playing = false;
    
    // Act
    update(&mut state, Message::PlayPause);
    
    // Assert
    assert!(state.is_playing);
}
```

### Pattern 2: Given-When-Then (BDD)

```rust
#[test]
fn test_bookmark_creation() {
    // Given an audiobook is playing
    let mut state = State::default();
    state.selected_file = Some("/test/chapter1.mp3".to_string());
    state.current_time = 120_000.0;
    
    // When user creates bookmark
    update(&mut state, Message::BookmarkCreate);
    
    // Then bookmark is saved
    let bookmarks = load_bookmarks(&state);
    assert_eq!(bookmarks.len(), 1);
    assert_eq!(bookmarks[0].position_ms, 120_000);
}
```

### Pattern 3: Table-Driven Tests

```rust
#[test]
fn test_speed_preset_values() {
    let presets = vec![
        (0.5, "0.5x"),
        (0.75, "0.75x"),
        (1.0, "1.0x"),
        (1.25, "1.25x"),
        (1.5, "1.5x"),
        (2.0, "2.0x"),
    ];
    
    for (speed, label) in presets {
        let mut state = State::default();
        update(&mut state, Message::SpeedPreset(speed));
        assert_eq!(state.speed, speed, "Speed preset {}", label);
    }
}
```

### Pattern 4: Property-Based Testing

```rust
#[test]
fn test_volume_always_in_range() {
    use proptest::prelude::*;
    
    proptest!(|(volume in -1000i32..1000i32)| {
        let mut state = State::default();
        update(&mut state, Message::VolumeChanged(volume));
        
        prop_assert!(state.volume >= 0);
        prop_assert!(state.volume <= 200);
    });
}
```

## Running Tests

### Run All Tests

```bash
cargo test --all-targets
```

### Run Specific Test Suite

```bash
# Unit tests only
cargo test --lib

# Integration tests only
cargo test --test '*'

# Specific test file
cargo test --test keyboard_navigation_tests

# Specific test
cargo test test_play_pause_toggle
```

### Run with Output

```bash
# Show println! output
cargo test -- --nocapture

# Show test names
cargo test -- --list

# Run single-threaded (for debugging)
cargo test -- --test-threads=1
```

### Performance Tests

```bash
# Run performance tests with timing
cargo test --test ui_performance_tests -- --nocapture

# Release mode for accurate benchmarks
cargo test --release --test ui_performance_tests
```

### Coverage

```bash
# Generate coverage report (requires tarpaulin)
cargo tarpaulin --out Html --output-dir coverage/

# View coverage
open coverage/index.html
```

## Test Quality Criteria

### Good Tests Are:

1. **Fast**: Run in milliseconds
   - Use in-memory databases
   - Mock expensive operations
   - Avoid network calls

2. **Isolated**: Independent of other tests
   - No shared mutable state
   - Clean up resources
   - Use unique temp directories

3. **Repeatable**: Same result every run
   - No random data (use seeds)
   - No time-dependent assertions
   - No external dependencies

4. **Self-Documenting**: Clear intent
   - Descriptive test names
   - Meaningful assertions
   - Comments explain "why", not "what"

5. **Maintainable**: Easy to update
   - DRY (use helpers)
   - Follow project conventions
   - Update when requirements change

### Anti-Patterns to Avoid

❌ **Testing Implementation Details**
```rust
#[test]
fn test_state_has_volume_field() {
    let state = State::default();
    let _ = state.volume;  // BAD: Tests struct internals
}
```

❌ **Brittle Assertions**
```rust
#[test]
fn test_error_message_exact() {
    let error = get_error();
    assert_eq!(error, "Error: Failed to load file at line 42");
    // BAD: Breaks if error message changes
}
```

❌ **Slow Tests**
```rust
#[test]
fn test_sleep() {
    std::thread::sleep(Duration::from_secs(5));
    // BAD: Wastes time
}
```

❌ **Flaky Tests**
```rust
#[test]
fn test_timestamp() {
    let now = Utc::now();
    let stored = load_timestamp();
    assert_eq!(now, stored);  // BAD: Timing-dependent
}
```

## Continuous Integration

### GitHub Actions

```yaml
name: Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
    
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: 1.93.1
      - run: cargo test --all-targets
      - run: cargo clippy -- -D warnings
```

### Pre-Commit Hooks

```bash
# .git/hooks/pre-commit
#!/bin/sh
cargo test --all-targets
cargo clippy -- -D warnings
```

## Accessibility Testing

### Automated Checks

`tests/accessibility_tests.rs` verifies:
- Keyboard shortcuts defined
- Error messages have text
- Focus tracking implemented
- Color not sole differentiator

### Manual Screen Reader Testing

Required for full WCAG compliance:

**macOS (VoiceOver)**:
```bash
# Enable VoiceOver
Cmd+F5

# Navigate UI
VO+Right Arrow
```

**Windows (NVDA)**:
```bash
# Start NVDA
Ctrl+Alt+N

# Navigate UI
Down Arrow
```

**Linux (Orca)**:
```bash
# Start Orca
Super+Alt+S

# Navigate UI
Down Arrow
```

Test checklist:
- [ ] All buttons announced
- [ ] Current audiobook announced
- [ ] Error messages announced
- [ ] Loading states announced
- [ ] Progress updates announced

## Performance Testing

### Baselines (Development Machine)

- Load 100 audiobooks: < 100ms
- Query 1000 files: < 200ms
- Query 1000 bookmarks: < 100ms
- 1000 state updates: < 10ms
- Sort 200 audiobooks: < 50ms

### Large Dataset Testing

Create test databases:

```rust
#[test]
fn test_large_library() {
    let db = create_test_db()?;
    
    // Create 500 audiobooks
    for i in 0..500 {
        create_test_audiobook(&db, "/test", &format!("Book {i}"))?;
    }
    
    // Measure query performance
    let start = Instant::now();
    let audiobooks = db::queries::get_all_audiobooks(db.connection())?;
    let duration = start.elapsed();
    
    assert!(duration.as_millis() < 500);
}
```

### Memory Testing

Monitor memory usage:

```bash
# Run with memory profiler
cargo build --release
valgrind --tool=massif target/release/nodoka
```

## Debugging Tests

### Print Debugging

```rust
#[test]
fn test_with_debug() {
    let state = State::default();
    eprintln!("State: {:?}", state);  // Shows even on success
    
    // Or use dbg!
    dbg!(&state.audiobooks);
}
```

### Test-Specific Logging

```rust
#[test]
fn test_with_logging() {
    env_logger::init();
    
    log::debug!("Starting test");
    let result = do_something();
    log::debug!("Result: {:?}", result);
}
```

### Isolate Failures

```bash
# Run only failing test
cargo test test_that_fails -- --exact

# Show backtrace
RUST_BACKTRACE=1 cargo test

# Run in debugger
rust-gdb --args target/debug/deps/nodoka-* test_name
```

## Best Practices

### DO:
✅ Write tests before fixing bugs (TDD)
✅ Test edge cases and boundaries
✅ Use descriptive test names
✅ Keep tests focused (one concept per test)
✅ Use test utilities for common setup
✅ Update tests when requirements change
✅ Delete obsolete tests

### DON'T:
❌ Test private functions directly
❌ Make tests depend on each other
❌ Use real files/network/database in unit tests
❌ Commit commented-out tests
❌ Ignore test failures
❌ Skip tests with #[ignore] without reason

## Test Coverage Goals

| Category | Target | Current |
|----------|--------|---------|
| Unit tests | 80% coverage | ~75% |
| Integration tests | All workflows | ~95% |
| Manual tests | All visual features | 100% |
| Total test count | 300+ | 320+ |

## Future Improvements

1. **Visual Regression Testing**
   - Screenshot comparison tests
   - Detect UI layout changes
   
2. **Mutation Testing**
   - Verify tests catch bugs
   - Use tools like `cargo-mutants`
   
3. **Fuzz Testing**
   - Random input generation
   - Crash/panic detection
   
4. **Property-Based Testing**
   - Use `proptest` more extensively
   - Generate edge cases automatically

## References

- [Rust Testing Guide](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [WCAG Testing](https://www.w3.org/WAI/test-evaluate/)
- [Performance Testing](https://doc.rust-lang.org/cargo/guide/tests.html)
- UI Architecture: `docs/ui_architecture.md`
- Manual Tests: `tests/manual_ui_checklist.md`
