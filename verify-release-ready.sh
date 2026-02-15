#!/bin/bash
# Release Readiness Verification Script
# This script verifies that the Nodoka project is ready for v0.2.0 release

set -e

echo "========================================"
echo "Nodoka v0.2.0 Release Readiness Check"
echo "========================================"
echo ""

ERRORS=0
WARNINGS=0

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

function pass() {
    echo -e "${GREEN}✓${NC} $1"
}

function fail() {
    echo -e "${RED}✗${NC} $1"
    ((ERRORS++))
}

function warn() {
    echo -e "${YELLOW}⚠${NC} $1"
    ((WARNINGS++))
}

# 1. Verify no C++ source files remain
echo "1. Checking for C++ source files..."
CPP_FILES=$(find . -type f \( -name "*.cpp" -o -name "*.h" -o -name "*.hpp" \) -not -path "./target/*" -not -path "./.git/*" | wc -l)
if [ "$CPP_FILES" -eq 0 ]; then
    pass "No C++ source files found"
else
    fail "Found $CPP_FILES C++ source files - conversion incomplete"
fi

# 2. Count Rust source files
echo "2. Checking Rust source files..."
RUST_FILES=$(find src -type f -name "*.rs" | wc -l)
if [ "$RUST_FILES" -eq 38 ]; then
    pass "All 38 Rust source files present"
else
    warn "Expected 38 Rust files, found $RUST_FILES"
fi

# 3. Run tests
echo "3. Running test suite..."
if cargo test --all --quiet 2>&1 | grep -q "test result: ok"; then
    pass "All tests passing"
else
    fail "Test failures detected"
fi

# 4. Run clippy with strict linting
echo "4. Running clippy with strict linting..."
if cargo clippy --all-targets --all-features -- -D warnings 2>&1 | grep -q "Finished"; then
    pass "Clippy passes with -D warnings"
else
    fail "Clippy warnings/errors detected"
fi

# 5. Check for forbidden patterns in source
echo "5. Checking for forbidden patterns (unwrap/expect/panic)..."
FORBIDDEN=$(rg '\.unwrap\(|\.expect\(|panic!' src/ --count-matches 2>/dev/null | awk -F: '{sum+=$2} END {print sum}')
if [ -z "$FORBIDDEN" ] || [ "$FORBIDDEN" -eq 0 ]; then
    pass "No forbidden patterns in src/"
else
    fail "Found $FORBIDDEN instances of unwrap/expect/panic in src/"
fi

# 6. Verify dependencies
echo "6. Verifying key dependencies..."
if grep -q 'iced = { version = "0.14.0"' Cargo.toml; then
    pass "iced 0.14.0 dependency confirmed"
else
    fail "iced 0.14.0 not found in Cargo.toml"
fi

if grep -q 'vlc-rs = "0.3"' Cargo.toml; then
    pass "vlc-rs 0.3 dependency confirmed"
else
    fail "vlc-rs 0.3 not found in Cargo.toml"
fi

# 7. Build release binary
echo "7. Building release binary..."
if cargo build --release --quiet; then
    BINARY_SIZE=$(ls -lh target/release/nodoka | awk '{print $5}')
    pass "Release binary built successfully ($BINARY_SIZE)"
else
    fail "Release build failed"
fi

# 8. Verify VLC linking (macOS only)
if [[ "$OSTYPE" == "darwin"* ]]; then
    echo "8. Verifying VLC linking..."
    if otool -L target/release/nodoka 2>/dev/null | grep -q "libvlc.dylib"; then
        pass "VLC libraries correctly linked"
    else
        fail "VLC libraries not linked"
    fi
else
    warn "Skipping VLC linking check (not on macOS)"
fi

# 9. Check packaging scripts exist and are executable
echo "9. Checking packaging scripts..."
if [ -x "packaging/macos/create-dmg.sh" ]; then
    pass "macOS packaging script ready"
else
    fail "macOS packaging script missing or not executable"
fi

if [ -x "packaging/linux/build-deb.sh" ]; then
    pass "Linux packaging script ready"
else
    fail "Linux packaging script missing or not executable"
fi

if [ -f "packaging/windows/nodoka.wxs" ]; then
    pass "Windows WiX configuration exists"
else
    fail "Windows WiX configuration missing"
fi

# 10. Verify macOS DMG exists
echo "10. Checking macOS DMG..."
if [ -f "packaging/macos/Nodoka-0.2.0.dmg" ]; then
    DMG_SIZE=$(ls -lh packaging/macos/Nodoka-0.2.0.dmg | awk '{print $5}')
    pass "macOS DMG exists ($DMG_SIZE)"
else
    warn "macOS DMG not built yet"
fi

# 11. Verify SHA256SUMS.txt
echo "11. Checking SHA256SUMS.txt..."
if [ -f "SHA256SUMS.txt" ]; then
    CHECKSUM_COUNT=$(wc -l < SHA256SUMS.txt | tr -d ' ')
    if [ "$CHECKSUM_COUNT" -ge 1 ]; then
        pass "SHA256SUMS.txt exists with $CHECKSUM_COUNT checksum(s)"
    else
        warn "SHA256SUMS.txt is empty"
    fi
else
    warn "SHA256SUMS.txt not created yet"
fi

# 12. Verify documentation files
echo "12. Checking documentation..."
REQUIRED_DOCS=(
    "README.md"
    "CHANGELOG.md"
    "RELEASE_NOTES_v0.2.0.md"
    "CONTRIBUTING.md"
    "docs/USER_GUIDE.md"
    "docs/TROUBLESHOOTING.md"
    "docs/LESSONS_LEARNED.md"
)

for doc in "${REQUIRED_DOCS[@]}"; do
    if [ -f "$doc" ]; then
        pass "$doc exists"
    else
        fail "$doc missing"
    fi
done

# 13. Verify CI/CD configuration
echo "13. Checking CI/CD configuration..."
if [ -f ".github/workflows/build.yml" ]; then
    if grep -q "generate-checksums" .github/workflows/build.yml; then
        pass "CI/CD pipeline configured with checksum generation"
    else
        warn "CI/CD pipeline missing checksum generation job"
    fi
else
    fail "GitHub Actions workflow missing"
fi

# 14. Verify version consistency
echo "14. Checking version consistency..."
VERSION_CARGO=$(grep '^version = "' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')
if [ "$VERSION_CARGO" = "0.2.0" ]; then
    pass "Cargo.toml version is 0.2.0"
else
    fail "Cargo.toml version is $VERSION_CARGO (expected 0.2.0)"
fi

# 15. Check for git uncommitted changes
echo "15. Checking git status..."
if [ -z "$(git status --porcelain)" ]; then
    pass "No uncommitted changes"
else
    warn "Uncommitted changes detected - commit before release"
    git status --short
fi

# Summary
echo ""
echo "========================================"
echo "Verification Summary"
echo "========================================"
echo -e "Checks passed: ${GREEN}$((15 - ERRORS - WARNINGS))${NC}"
echo -e "Warnings: ${YELLOW}$WARNINGS${NC}"
echo -e "Errors: ${RED}$ERRORS${NC}"
echo ""

if [ $ERRORS -eq 0 ] && [ $WARNINGS -eq 0 ]; then
    echo -e "${GREEN}✓ Project is ready for v0.2.0 release${NC}"
    exit 0
elif [ $ERRORS -eq 0 ]; then
    echo -e "${YELLOW}⚠ Project is mostly ready, but has warnings${NC}"
    exit 0
else
    echo -e "${RED}✗ Project is NOT ready for release - fix errors first${NC}"
    exit 1
fi
