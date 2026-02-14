#!/bin/bash
# Create and push v0.2.0 release tag
# This script safely creates the release tag after verification

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

cd "$PROJECT_ROOT"

VERSION="v0.2.0"
TAG_MESSAGE="Nodoka 0.2.0 - Rust Rewrite Release

Complete rewrite from C++/Qt to Rust/iced
- Full Rust implementation with vlc-rs 0.3 bindings
- iced 0.12 UI framework with Elm architecture
- Strict linting enforced (zero unwrap/expect/panic)
- 18/18 tests passing with comprehensive coverage
- Cross-platform installers (Windows MSI, macOS DMG, Linux DEB)
- CI/CD pipeline with GitHub Actions
- Improved performance and memory efficiency"

echo "=================================================="
echo "Nodoka 0.2.0 Release Tag Creation"
echo "=================================================="
echo ""

# Color codes
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

# Check if we're in a git repository
if ! git rev-parse --git-dir > /dev/null 2>&1; then
    echo -e "${RED}Error: Not in a git repository${NC}"
    exit 1
fi

# Check for uncommitted changes
if ! git diff-index --quiet HEAD --; then
    echo -e "${YELLOW}Warning: You have uncommitted changes${NC}"
    echo ""
    git status --short
    echo ""
    read -p "Continue anyway? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "Aborted."
        exit 1
    fi
fi

# Check if tag already exists
if git tag -l | grep -q "^${VERSION}$"; then
    echo -e "${RED}Error: Tag ${VERSION} already exists${NC}"
    echo ""
    echo "To delete and recreate:"
    echo "  git tag -d ${VERSION}"
    echo "  git push origin :refs/tags/${VERSION}"
    echo ""
    exit 1
fi

# Run verification script
echo "Running release verification..."
echo ""
if ! "$SCRIPT_DIR/verify-release-ready.sh"; then
    echo ""
    echo -e "${RED}Verification failed. Fix issues before creating tag.${NC}"
    exit 1
fi

echo ""
echo "=================================================="
echo "Creating Release Tag"
echo "=================================================="
echo ""
echo "Tag: ${VERSION}"
echo "Message:"
echo "$TAG_MESSAGE"
echo ""
read -p "Create this tag? (y/N): " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "Aborted."
    exit 1
fi

# Create annotated tag
git tag -a "$VERSION" -m "$TAG_MESSAGE"
echo -e "${GREEN}✓ Tag created locally${NC}"

# Show tag info
echo ""
git show "$VERSION" --quiet
echo ""

# Ask to push
read -p "Push tag to origin? (y/N): " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    git push origin "$VERSION"
    echo -e "${GREEN}✓ Tag pushed to origin${NC}"
    echo ""
    echo "Tag ${VERSION} has been pushed!"
    echo ""
    echo "Next steps:"
    echo "1. Go to GitHub: https://github.com/otakukingdom/nodoka/releases/new"
    echo "2. Select tag: ${VERSION}"
    echo "3. Title: Nodoka 0.2.0 - Rust Rewrite"
    echo "4. Description: Copy from RELEASE_NOTES_v0.2.0.md"
    echo "5. Save as DRAFT (don't publish yet)"
    echo "6. Wait for CI/CD to build and upload installers (~10-15 minutes)"
    echo "7. Verify all 3 installers uploaded: .msi, .dmg, .deb"
    echo "8. Download installers and verify checksums"
    echo "9. Verify acceptance suite passes on all supported platforms (CI)"
    echo "10. Publish release when all CI checks pass"
    echo ""
else
    echo ""
    echo "Tag created locally but not pushed."
    echo "To push later: git push origin ${VERSION}"
    echo "To delete: git tag -d ${VERSION}"
    echo ""
fi
