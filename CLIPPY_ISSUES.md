# Clippy Linting Issues

## Summary
- Total Issues: 55 clippy warnings treated as errors due to `-D warnings`
- Deny-level lints: 6 configured (unwrap_used, expect_used, panic, indexing_slicing, missing_errors_doc, missing_panics_doc)
- Warn-level lints: pedantic, nursery (many stylistic issues)

## Fixed Issues

1. **indexing_slicing** in `src/tasks/checksum.rs` ✅
   - Changed `&buffer[..n]` to `buffer.get(..n)` with safe access

## Remaining Issues by Category

### Missing Documentation (13 issues)
**Impact**: Deny-level lint violation
**Files affected**:
- src/proxy/audiobook_proxy.rs (4 functions)
- src/proxy/manager.rs (3 functions)
- src/ui/components/*.rs (3 functions)
- src/ui/main_window.rs (1 function)
- src/ui/settings_form.rs (1 function)

**Fix**: Add `/// # Errors` sections to all functions returning `Result`

### Type Conversion Issues (6 issues)
**Impact**: Potential runtime bugs
**Files affected**:
- src/models/audiobook_file.rs: `i64` to `i32` cast
- src/tasks/scan_directory.rs: `usize` to `i32` cast  
- src/ui/components/player_controls.rs: `f64`/`i64` conversions
- src/ui/update.rs: `i64` to `i32` cast

**Fix**: Use `try_from()` or explicitly allow with lint annotation

### Concurrency Issues (2 issues)
**Impact**: Potential threading problems
**Files affected**:
- src/proxy/audiobook_proxy.rs: Arc<RwLock> not Send+Sync
- src/proxy/manager.rs: Arc<RwLock<HashMap>> not Send+Sync

**Fix**: Use `std::sync::Mutex` instead of `RwLock`, or use `parking_lot::RwLock`

### Code Quality (34 issues)
**Impact**: Code maintainability
**Categories**:
- `map_unwrap_or`: Use `map_or_else` instead (3)
- `mutex_integer`: Use AtomicI32 instead of Mutex<i32> (1)
- `ignored_unit_patterns`: Use `()` instead of `_` (2)
- `option_if_let_else`: Use `map_or_else` (6)
- `missing_const_for_fn`: Mark functions as const (7)
- `must_use_candidate`: Add `#[must_use]` attributes (5)
- `significant_drop_tightening`: Drop locks earlier (3)
- `redundant_closure`: Simplify closures (1)
- `approx_constant`: Use `f64::consts::FRAC_1_PI` (4)
- `uninlined_format_args`: Use `format!("{var}")` (2)

**Fix**: Apply suggested clippy fixes

### Architecture Issues (2 issues)
- `cognitive_complexity`: update() function too complex (47/30)
- `too_many_lines`: update() function too long (188/100)

**Fix**: Refactor `src/ui/update.rs::update()` into smaller functions

## Recommended Approach

### Phase 1: Critical Fixes (Required for MVP)
1. Add missing `# Errors` documentation to all Result-returning functions
2. Fix Arc<RwLock> Send+Sync issues in proxy layer
3. Fix type conversion issues with try_from()

### Phase 2: Code Quality (Post-MVP)
1. Apply all suggested clippy fixes for code quality issues
2. Refactor large update() function
3. Add #[must_use] attributes
4. Make functions const where possible

### Phase 3: Optimization (Future)
1. Use AtomicI32 for player volume
2. Optimize lock dropping
3. Simplify closures

## Temporary Workaround

Until all issues are fixed, tests and builds can run without clippy errors by:

```bash
# Build without clippy
cargo build --release

# Test without clippy  
export DYLD_LIBRARY_PATH=/Applications/VLC.app/Contents/MacOS/lib:$DYLD_LIBRARY_PATH
cargo test --all -- --nocapture

# Or use the helper script
./run_nodoka.sh test
./run_nodoka.sh build
```

To see warnings without failing:
```bash
cargo clippy --all-targets
```

## Impact on Acceptance Criteria

**Acceptance Criteria**: "Strict linting rules with no allow() or expect(), no dead code"

**Current Status**: 
- ✅ No `allow()` or `expect()` in code
- ✅ No dead code (dead_code lint is deny-level)
- ❌ 55 clippy warnings preventing `cargo clippy -- -D warnings` from passing

**Resolution**: Fix Phase 1 issues (estimated ~2-3 hours of work)
