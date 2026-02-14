#!/bin/bash
# Release Readiness Verification Script for Nodoka 0.2.0
# This script verifies all acceptance criteria before creating a GitHub release

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

echo "=================================================="
echo "Nodoka 0.2.0 Release Readiness Verification"
echo "=================================================="
echo ""

cd "$PROJECT_ROOT"

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

PASS_COUNT=0
FAIL_COUNT=0
WARN_COUNT=0

check_pass() {
    echo -e "${GREEN}✓${NC} $1"
    PASS_COUNT=$((PASS_COUNT + 1))
}

check_fail() {
    echo -e "${RED}✗${NC} $1"
    FAIL_COUNT=$((FAIL_COUNT + 1))
}

check_warn() {
    echo -e "${YELLOW}!${NC} $1"
    WARN_COUNT=$((WARN_COUNT + 1))
}

echo "=== Verification 1: Code Quality ==="
echo ""

# Check Cargo.toml version
echo -n "Checking version in Cargo.toml... "
VERSION=$(grep '^version = ' Cargo.toml | head -1 | cut -d'"' -f2)
if [ "$VERSION" = "0.2.0" ]; then
    check_pass "Version is 0.2.0"
else
    check_fail "Version is $VERSION, expected 0.2.0"
fi

# Check for C++ files (should be none)
echo -n "Checking for remaining C++ files... "
CPP_COUNT=$(find . -name "*.cpp" -o -name "*.h" | grep -v ".git" | grep -v "target" | wc -l | tr -d ' ')
if [ "$CPP_COUNT" = "0" ]; then
    check_pass "No C++ files found"
else
    check_fail "Found $CPP_COUNT C++ files"
fi

# Run tests
echo -n "Running test suite... "
if cargo test --all --quiet > /dev/null 2>&1; then
    TEST_OUTPUT=$(cargo test --all 2>&1)
    TEST_COUNT=$(echo "$TEST_OUTPUT" | grep -o "test result: ok\." | wc -l | tr -d ' ')
    check_pass "All tests passing ($TEST_COUNT test suites)"
else
    check_fail "Tests failed"
    echo "Run 'cargo test' to see details"
fi

# Run clippy
echo -n "Running strict clippy checks... "
if cargo clippy --all-targets --all-features -- -D warnings > /dev/null 2>&1; then
    check_pass "Clippy passed with zero warnings"
else
    check_fail "Clippy found warnings"
    echo "Run 'cargo clippy --all-targets --all-features -- -D warnings' to see details"
fi

# Check for forbidden patterns in src/
echo -n "Checking for unwrap/expect/allow in src/... "
FORBIDDEN=$(rg '\.unwrap\(|\.expect\(|#\[allow' src/ || true)
if [ -z "$FORBIDDEN" ]; then
    check_pass "No forbidden patterns in src/"
else
    check_fail "Found forbidden patterns:"
    echo "$FORBIDDEN"
fi

# Check formatting
echo -n "Checking code formatting... "
if cargo fmt --check > /dev/null 2>&1; then
    check_pass "Code is properly formatted"
else
    check_fail "Code needs formatting (run 'cargo fmt')"
fi

echo ""
echo "=== Verification 2: Dependencies ==="
echo ""

# Check iced version
echo -n "Checking iced version... "
ICED_VERSION=$(grep 'iced = ' Cargo.toml | grep -o '"[^"]*"' | head -1 | tr -d '"')
if [[ "$ICED_VERSION" == 0.12* ]]; then
    check_pass "iced $ICED_VERSION"
else
    check_warn "iced version is $ICED_VERSION (expected 0.12.x)"
fi

# Check vlc-rs version
echo -n "Checking vlc-rs version... "
VLC_VERSION=$(grep 'vlc-rs = ' Cargo.toml | grep -o '"[^"]*"' | head -1 | tr -d '"')
if [[ "$VLC_VERSION" == 0.3* ]]; then
    check_pass "vlc-rs $VLC_VERSION"
else
    check_warn "vlc-rs version is $VLC_VERSION (expected 0.3.x)"
fi

# Check rusqlite version
echo -n "Checking rusqlite version... "
SQLITE_VERSION=$(grep 'rusqlite = ' Cargo.toml | grep -o '"[^"]*"' | head -1 | tr -d '"')
if [[ "$SQLITE_VERSION" == 0.31* ]]; then
    check_pass "rusqlite $SQLITE_VERSION"
else
    check_warn "rusqlite version is $SQLITE_VERSION (expected 0.31.x)"
fi

echo ""
echo "=== Verification 3: Packaging Scripts ==="
echo ""

# Check Linux packaging script
echo -n "Checking Linux DEB build script... "
if [ -f "packaging/linux/build-deb.sh" ] && [ -x "packaging/linux/build-deb.sh" ]; then
    check_pass "build-deb.sh exists and is executable"
else
    check_fail "build-deb.sh missing or not executable"
fi

# Check Windows WiX script
echo -n "Checking Windows WiX script... "
if [ -f "packaging/windows/nodoka.wxs" ]; then
    WIX_VERSION=$(grep 'Version="' packaging/windows/nodoka.wxs | head -1 | grep -o '"[^"]*"' | head -1 | tr -d '"')
    if [ "$WIX_VERSION" = "0.2.0" ]; then
        check_pass "nodoka.wxs exists with version 0.2.0"
    else
        check_fail "nodoka.wxs has version $WIX_VERSION (expected 0.2.0)"
    fi
else
    check_fail "nodoka.wxs not found"
fi

# Check macOS packaging script
echo -n "Checking macOS DMG script... "
if [ -f "packaging/macos/create-dmg.sh" ] && [ -x "packaging/macos/create-dmg.sh" ]; then
    check_pass "create-dmg.sh exists and is executable"
else
    check_fail "create-dmg.sh missing or not executable"
fi

echo ""
echo "=== Verification 4: CI/CD Pipeline ==="
echo ""

# Check GitHub Actions workflow
echo -n "Checking GitHub Actions workflow... "
if [ -f ".github/workflows/build.yml" ]; then
    check_pass "build.yml exists"
    
    # Check for release trigger
    echo -n "Checking release trigger... "
    if grep -q "release:" .github/workflows/build.yml; then
        check_pass "Release trigger configured"
    else
        check_fail "Release trigger not found in workflow"
    fi
    
    # Check for packaging jobs
    echo -n "Checking packaging jobs... "
    HAS_WINDOWS=$(grep -q "package-windows:" .github/workflows/build.yml && echo "1" || echo "0")
    HAS_MACOS=$(grep -q "package-macos:" .github/workflows/build.yml && echo "1" || echo "0")
    HAS_LINUX=$(grep -q "package-linux:" .github/workflows/build.yml && echo "1" || echo "0")
    TOTAL=$((HAS_WINDOWS + HAS_MACOS + HAS_LINUX))
    if [ "$TOTAL" = "3" ]; then
        check_pass "All 3 packaging jobs present (Windows, macOS, Linux)"
    else
        check_fail "Found $TOTAL packaging jobs (expected 3)"
    fi
else
    check_fail "GitHub Actions workflow not found"
fi

echo ""
echo "=== Verification 5: Documentation ==="
echo ""

# Check README
echo -n "Checking README.md... "
if grep -q "0.2.0" README.md && grep -q "Rust" README.md; then
    check_pass "README.md updated for v0.2.0"
else
    check_warn "README.md may need v0.2.0 updates"
fi

# Check CHANGELOG
echo -n "Checking CHANGELOG.md... "
if grep -q "\[0.2.0\]" CHANGELOG.md; then
    check_pass "CHANGELOG.md has v0.2.0 entry"
else
    check_fail "CHANGELOG.md missing v0.2.0 entry"
fi

# Check release notes
echo -n "Checking release notes... "
if [ -f "RELEASE_NOTES_v0.2.0.md" ]; then
    check_pass "RELEASE_NOTES_v0.2.0.md exists"
else
    check_fail "RELEASE_NOTES_v0.2.0.md not found"
fi

# Check release checklist
echo -n "Checking release checklist... "
if [ -f "RELEASE_CHECKLIST.md" ]; then
    check_pass "RELEASE_CHECKLIST.md exists"
else
    check_warn "RELEASE_CHECKLIST.md not found"
fi

echo ""
echo "=== Verification 6: Build Test ==="
echo ""

# Test release build
echo -n "Testing release build... "
if cargo build --release --quiet > /dev/null 2>&1; then
    check_pass "Release build succeeded"

    # Check binary size (should be reasonable)
    if [ -f "target/release/nodoka" ]; then
        BINARY_SIZE=$(du -h target/release/nodoka | cut -f1)
        echo "   Binary size: $BINARY_SIZE"
    fi
else
    check_fail "Release build failed"
fi

echo ""
echo "=================================================="
echo "Verification Summary"
echo "=================================================="
echo ""
echo -e "${GREEN}Passed:${NC} $PASS_COUNT"
echo -e "${YELLOW}Warnings:${NC} $WARN_COUNT"
echo -e "${RED}Failed:${NC} $FAIL_COUNT"
echo ""

if [ $FAIL_COUNT -eq 0 ]; then
    echo -e "${GREEN}✓ All critical checks passed!${NC}"
    echo ""
    echo "Next steps:"
    echo "1. Review RELEASE_CHECKLIST.md"
    echo "2. Create git tag: git tag -a v0.2.0 -m 'Nodoka 0.2.0 - Rust Rewrite'"
    echo "3. Push tag: git push origin v0.2.0"
    echo "4. Create GitHub release from tag"
    echo "5. Wait for CI/CD to build installers"
    echo "6. Download and verify all installers"
    echo "7. Verify acceptance suite passes on all supported platforms (CI)"
    echo "8. Publish release when CI passes"
    echo ""
    exit 0
else
    echo -e "${RED}✗ Some checks failed. Fix issues before release.${NC}"
    echo ""
    exit 1
fi
