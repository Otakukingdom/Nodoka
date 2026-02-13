#!/bin/bash
set -e

VERSION="0.2.0"
TAG="v${VERSION}"

echo "==============================================="
echo "Nodoka Release Script v${VERSION}"
echo "==============================================="
echo ""

# Pre-release checks
echo "Running pre-release checks..."
echo ""

echo "1. Checking Cargo.toml version..."
CARGO_VERSION=$(grep '^version = ' Cargo.toml | head -n1 | cut -d'"' -f2)
if [ "$CARGO_VERSION" != "$VERSION" ]; then
    echo "ERROR: Cargo.toml version ($CARGO_VERSION) does not match release version ($VERSION)"
    exit 1
fi
echo "   ✓ Cargo.toml version: $CARGO_VERSION"

echo ""
echo "2. Running test suite..."
if ! cargo test --all --quiet; then
    echo "ERROR: Tests failed"
    exit 1
fi
echo "   ✓ All tests passed"

echo ""
echo "3. Running clippy..."
if ! cargo clippy --all-targets --all-features -- -D warnings 2>&1 | grep -q "Finished"; then
    echo "ERROR: Clippy found warnings"
    exit 1
fi
echo "   ✓ Zero clippy warnings"

echo ""
echo "4. Checking for forbidden patterns..."
if rg '\.unwrap\(|\.expect\(|#\[allow' src/ > /dev/null 2>&1; then
    echo "ERROR: Found unwrap/expect/allow in src/"
    exit 1
fi
echo "   ✓ No unwrap/expect/allow in src/"

echo ""
echo "5. Checking git status..."
if ! git diff-index --quiet HEAD --; then
    echo "WARNING: You have uncommitted changes"
    echo "Please commit or stash your changes before creating a release"
    exit 1
fi
echo "   ✓ Working directory clean"

echo ""
echo "6. Verifying macOS DMG exists..."
if [ ! -f "packaging/macos/Nodoka-0.2.0.dmg" ]; then
    echo "ERROR: macOS DMG not found"
    exit 1
fi
DMG_SIZE=$(ls -lh packaging/macos/Nodoka-0.2.0.dmg | awk '{print $5}')
echo "   ✓ macOS DMG found ($DMG_SIZE)"

echo ""
echo "7. Verifying packaging scripts..."
if [ ! -x "packaging/linux/build-deb.sh" ]; then
    echo "ERROR: Linux build script not executable"
    exit 1
fi
if [ ! -f "packaging/windows/nodoka.wxs" ]; then
    echo "ERROR: Windows WiX config not found"
    exit 1
fi
echo "   ✓ Packaging scripts ready"

echo ""
echo "8. Checking GitHub CLI..."
if ! command -v gh &> /dev/null; then
    echo "ERROR: GitHub CLI (gh) not installed"
    echo "Install from: https://cli.github.com/"
    exit 1
fi
echo "   ✓ GitHub CLI available"

echo ""
echo "==============================================="
echo "All pre-release checks passed!"
echo "==============================================="
echo ""
echo "Next steps to create release:"
echo ""
echo "1. Create and push git tag:"
echo "   git tag -a $TAG -m 'Nodoka $VERSION - Complete Rust Rewrite'"
echo "   git push origin $TAG"
echo ""
echo "2. Create GitHub release (this will trigger CI/CD to build installers):"
echo "   gh release create $TAG \\"
echo "     --title 'Nodoka $VERSION - Rust Rewrite Release' \\"
echo "     --notes-file CHANGELOG.md \\"
echo "     packaging/macos/Nodoka-0.2.0.dmg"
echo ""
echo "3. CI/CD will automatically:"
echo "   - Build Linux DEB package"
echo "   - Build Windows MSI installer"
echo "   - Upload both to the release"
echo "   - Generate SHA256SUMS.txt with all three checksums"
echo ""
echo "4. After CI/CD completes (~10 minutes), verify:"
echo "   - All 3 installers attached to release"
echo "   - SHA256SUMS.txt attached to release"
echo "   - Download links work"
echo ""
echo "5. Mark release as 'Latest release' on GitHub"
echo ""
echo "Would you like to proceed with creating the release? (y/n)"
read -r response
if [[ "$response" =~ ^[Yy]$ ]]; then
    echo ""
    echo "Creating git tag $TAG..."
    git tag -a "$TAG" -m "Nodoka $VERSION - Complete Rust Rewrite"
    
    echo "Pushing tag to origin..."
    git push origin "$TAG"
    
    echo ""
    echo "Creating GitHub release..."
    gh release create "$TAG" \
      --title "Nodoka $VERSION - Rust Rewrite Release" \
      --notes-file CHANGELOG.md \
      packaging/macos/Nodoka-0.2.0.dmg
    
    echo ""
    echo "==============================================="
    echo "Release created successfully!"
    echo "==============================================="
    echo ""
    echo "Monitor CI/CD progress at:"
    echo "https://github.com/$(git config --get remote.origin.url | sed 's/.*github.com[:/]\(.*\)\.git/\1/')/actions"
    echo ""
    echo "The release page:"
    echo "https://github.com/$(git config --get remote.origin.url | sed 's/.*github.com[:/]\(.*\)\.git/\1/')/releases/tag/$TAG"
else
    echo ""
    echo "Release creation cancelled."
    echo "Run this script again when ready."
fi
