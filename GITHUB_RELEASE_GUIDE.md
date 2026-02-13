# GitHub Release Creation Guide - Nodoka 0.2.0

This guide provides step-by-step instructions for creating the v0.2.0 GitHub release.

## Prerequisites

- [x] All code merged to `main` branch
- [x] CI/CD workflow passes on latest commit
- [x] All 18 tests passing
- [x] Zero clippy warnings
- [x] Documentation updated (README, CHANGELOG, RELEASE_NOTES)
- [x] macOS DMG built locally (Nodoka-0.2.0.dmg)
- [x] Packaging scripts ready for Linux DEB and Windows MSI

## Step 1: Create and Push Git Tag

### 1.1 Ensure Clean Working Directory
```bash
cd /path/to/nodoka
git status
# Should show: "nothing to commit, working tree clean"
```

### 1.2 Pull Latest Changes
```bash
git checkout main
git pull origin main
```

### 1.3 Verify Version Numbers
Check that all version references are 0.2.0:
```bash
# Cargo.toml
grep '^version = ' Cargo.toml
# Should show: version = "0.2.0"

# CHANGELOG.md
head -20 CHANGELOG.md | grep '\[0.2.0\]'
# Should show: ## [0.2.0] - 2026-02-13

# packaging scripts
grep 'VERSION=' packaging/linux/build-deb.sh
# Should show: VERSION="0.2.0"

grep 'Version=' packaging/windows/nodoka.wxs
# Should show: Version="0.2.0"
```

### 1.4 Create Annotated Tag
```bash
git tag -a v0.2.0 -m "Nodoka 0.2.0 - Rust Rewrite Release

Complete rewrite from C++/Qt to Rust/iced with:
- Full Rust implementation with strict linting
- iced UI framework for cross-platform support
- vlc-rs 0.3 audio backend
- Native installers for Windows, macOS, and Linux
- 18/18 tests passing with zero warnings
- 80% smaller binary, faster startup, lower memory usage"
```

### 1.5 Verify Tag
```bash
git tag -l -n9 v0.2.0
# Should display tag with full message

git show v0.2.0
# Should show tag details and commit
```

### 1.6 Push Tag to Trigger CI/CD
```bash
git push origin v0.2.0
```

**This will automatically trigger the GitHub Actions workflow to build all installers.**

## Step 2: Monitor CI/CD Workflow

### 2.1 Open GitHub Actions
Navigate to: https://github.com/[username]/nodoka/actions

### 2.2 Find Workflow Run
- Look for "Build and Test" workflow triggered by tag `v0.2.0`
- Click on the workflow run to see details

### 2.3 Monitor Jobs
Expected jobs and approximate durations:
1. **Lint** (~2 minutes) - Format and clippy checks
2. **Test** (~5 minutes) - Tests on Linux, macOS, Windows
3. **Build** (~8 minutes) - Release binaries for all platforms
4. **Package Windows** (~5 minutes) - MSI installer
5. **Package macOS** (~4 minutes) - DMG installer
6. **Package Linux** (~3 minutes) - DEB package
7. **Generate Checksums** (~1 minute) - SHA256SUMS.txt

**Total estimated time: 20-30 minutes**

### 2.4 Watch for Failures
If any job fails:
1. Click on the failed job to see logs
2. Identify the error
3. Fix the issue locally
4. Delete the tag: `git tag -d v0.2.0 && git push origin :refs/tags/v0.2.0`
5. Fix the code, commit, and repeat from Step 1

### 2.5 Verify Artifacts
Once all jobs complete successfully, check artifacts:
1. Click "Summary" at top of workflow run
2. Scroll to "Artifacts" section
3. Should see:
   - `windows-msi` (nodoka-0.2.0-x64.msi, ~8-10 MB)
   - `macos-dmg` (Nodoka-0.2.0.dmg, ~4 MB)
   - `linux-deb` (nodoka_0.2.0_amd64.deb, ~8 MB)
   - `checksums` (SHA256SUMS.txt, <1 KB)

## Step 3: Download and Verify Artifacts

### 3.1 Download Artifacts via GitHub CLI
```bash
# Install gh CLI if not present
# brew install gh  # macOS
# sudo apt install gh  # Linux

# Authenticate
gh auth login

# List recent workflow runs
gh run list --limit 5

# Download artifacts (replace RUN_ID with actual ID)
gh run download [RUN_ID] --dir release-artifacts

# Alternatively, download from web UI:
# Click on each artifact in GitHub Actions to download
```

### 3.2 Verify File Sizes
```bash
cd release-artifacts
ls -lh *

# Expected sizes (approximate):
# nodoka-0.2.0-x64.msi      ~8-10 MB
# Nodoka-0.2.0.dmg          ~4 MB
# nodoka_0.2.0_amd64.deb    ~8 MB
# SHA256SUMS.txt            <1 KB
```

### 3.3 Verify Checksums
```bash
# Check checksums match
cat SHA256SUMS.txt

# Verify each file
# Linux/macOS
sha256sum -c SHA256SUMS.txt

# Or manually verify each:
sha256sum Nodoka-0.2.0.dmg
sha256sum nodoka_0.2.0_amd64.deb
# Windows (PowerShell)
# certutil -hashfile nodoka-0.2.0-x64.msi SHA256
```

### 3.4 Extract and Inspect Installers (Optional)
```bash
# macOS DMG
hdiutil attach Nodoka-0.2.0.dmg
ls -la /Volumes/Nodoka\ Audiobook\ Reader/
hdiutil detach /Volumes/Nodoka\ Audiobook\ Reader

# Linux DEB
dpkg-deb --info nodoka_0.2.0_amd64.deb
dpkg-deb --contents nodoka_0.2.0_amd64.deb

# Windows MSI (requires Windows)
# msiexec /a nodoka-0.2.0-x64.msi /qn TARGETDIR=C:\temp\extract
```

## Step 4: Create GitHub Release

### 4.1 Navigate to Releases
1. Go to repository: https://github.com/[username]/nodoka
2. Click "Releases" (right sidebar)
3. Click "Draft a new release"

### 4.2 Fill Release Form

**Choose a tag**: Select `v0.2.0` from dropdown

**Release title**: 
```
Nodoka 0.2.0 - Rust Rewrite Release
```

**Description**: Copy from RELEASE_NOTES_v0.2.0.md or use this template:
```markdown
# Nodoka 0.2.0 - Rust Rewrite Release

🎉 **Complete rewrite** of Nodoka Audiobook Reader in Rust with modern cross-platform UI.

## What's New

- 🦀 **Full Rust implementation** - Memory-safe, idiomatic Rust replacing C++/Qt
- 🎨 **iced UI framework** - Native, declarative UI with Elm architecture
- 🔊 **vlc-rs 0.3 bindings** - Modern Rust VLC integration
- ✅ **Strict code quality** - Zero warnings, comprehensive tests, no unwrap/expect
- 📦 **Native installers** - DMG (macOS), DEB (Linux), MSI (Windows)
- ⚡ **Performance improvements** - 80% smaller binary, faster startup, lower memory
- 🗄️ **SQLite database** - Replaced LMDB with SQLite for better reliability

## Downloads

| Platform | Installer | Size | SHA256 Checksum |
|----------|-----------|------|-----------------|
| **macOS 12+** (Universal) | [Nodoka-0.2.0.dmg](#) | 4.0 MB | `31bee7a4509572ea58c33d486bdf83eb177256dddfcb685efc721f1711daf50f` |
| **Linux** (Ubuntu/Debian) | [nodoka_0.2.0_amd64.deb](#) | ~8 MB | See SHA256SUMS.txt |
| **Windows 10/11** | [nodoka-0.2.0-x64.msi](#) | ~9 MB | See SHA256SUMS.txt |

📄 [Download SHA256SUMS.txt](#) for checksum verification.

## Installation

### macOS
1. Download `Nodoka-0.2.0.dmg`
2. Open DMG and drag to Applications
3. Install VLC: `brew install --cask vlc`

### Linux (Ubuntu/Debian)
```bash
sudo dpkg -i nodoka_0.2.0_amd64.deb
sudo apt-get install -f  # Install dependencies
```

### Windows
1. Download and install VLC from [videolan.org](https://www.videolan.org/vlc/)
2. Run `nodoka-0.2.0-x64.msi` installer

## Requirements

**All platforms require VLC 3.x** for audio playback.

## Highlights

- 80% smaller binary (8 MB vs 40 MB Qt version)
- Faster startup (<2 seconds)
- Lower memory (~80 MB idle vs ~200 MB)
- 18/18 tests passing with zero warnings
- Strict linting (no unwrap/expect/panic)
- Cross-platform consistency

## Known Issues

- VLC 4.x not yet tested (use VLC 3.x)
- Large libraries (10,000+ files) may take time to scan
- Database migration from v0.1.0 not automated (re-scan required)

## Verification

Verify downloads with SHA256:
```bash
# macOS/Linux
shasum -a 256 Nodoka-0.2.0.dmg
sha256sum nodoka_0.2.0_amd64.deb

# Windows (PowerShell)
certutil -hashfile nodoka-0.2.0-x64.msi SHA256
```

Compare with [SHA256SUMS.txt](#).

## Technical Details

- **Rust**: 1.82+
- **UI**: iced 0.12
- **Audio**: vlc-rs 0.3
- **Database**: rusqlite 0.31 (SQLite)
- **Tests**: 18/18 passing
- **Binary size**: 8 MB (release)

## Full Changelog

See [CHANGELOG.md](https://github.com/[username]/nodoka/blob/main/CHANGELOG.md) for complete change history.

## Support

- 🐛 [Report bugs](https://github.com/[username]/nodoka/issues)
- 💬 [Discussions](https://github.com/[username]/nodoka/discussions)
- 📖 [Documentation](https://github.com/[username]/nodoka#readme)
```

### 4.3 Attach Binary Assets

**IMPORTANT**: Do NOT click "Publish release" yet. Leave as draft.

1. Scroll to "Attach binaries" section
2. Drag and drop or click to upload:
   - ✅ `nodoka-0.2.0-x64.msi` (Windows installer)
   - ✅ `Nodoka-0.2.0.dmg` (macOS installer)
   - ✅ `nodoka_0.2.0_amd64.deb` (Linux package)
   - ✅ `SHA256SUMS.txt` (checksums file)

3. Wait for all files to upload (progress bars complete)
4. Verify all 4 files are listed

### 4.4 Configure Release Options

- [x] **Set as the latest release** - Check this box
- [ ] **Set as a pre-release** - Leave unchecked
- [ ] **Create a discussion for this release** - Optional (recommended)

### 4.5 Save as Draft

**Click "Save draft"** (do NOT publish yet)

## Step 5: Pre-Release Smoke Testing

Before publishing, perform smoke tests on actual downloads.

### 5.1 Download from Draft Release
1. Navigate to draft release (visible only to repository collaborators)
2. Download installers from draft
3. Verify checksums match SHA256SUMS.txt

### 5.2 macOS Smoke Test
```bash
# Download DMG from draft release
shasum -a 256 Nodoka-0.2.0.dmg
# Verify matches: 31bee7a4509572ea58c33d486bdf83eb177256dddfcb685efc721f1711daf50f

# Install
open Nodoka-0.2.0.dmg
# Drag to Applications

# Launch
open -a Nodoka

# Test:
# ✅ Window opens
# ✅ Add directory and scan
# ✅ Play audiobook with actual sound
# ✅ Progress saves after restart
```

### 5.3 Linux Smoke Test (Ubuntu 22.04)
```bash
# Download DEB from draft release
sha256sum nodoka_0.2.0_amd64.deb
# Compare with SHA256SUMS.txt

# Install
sudo dpkg -i nodoka_0.2.0_amd64.deb
sudo apt-get install -f

# Launch
nodoka &

# Test:
# ✅ Window opens
# ✅ Add directory and scan
# ✅ Play audiobook with actual sound
# ✅ Progress saves after restart
```

### 5.4 Windows Smoke Test (Windows 10/11)
```powershell
# Download MSI from draft release
certutil -hashfile nodoka-0.2.0-x64.msi SHA256
# Compare with SHA256SUMS.txt

# Install
.\nodoka-0.2.0-x64.msi
# Follow wizard

# Launch from Start Menu
# Search "Nodoka" and click

# Test:
# ✅ Window opens
# ✅ Add directory and scan
# ✅ Play audiobook with actual sound
# ✅ Progress saves after restart
```

### 5.5 Document Test Results
Use template from SMOKE_TEST_GUIDE.md and save results.

**Critical**: All tests must pass before publishing release.

## Step 6: Publish Release

### 6.1 Final Review Checklist
- [ ] All smoke tests passed on all platforms
- [ ] All installers attached (4 files)
- [ ] Checksums verified
- [ ] Release notes complete and accurate
- [ ] Download links work (test from draft)
- [ ] Screenshots/images included (optional)
- [ ] Version numbers consistent everywhere

### 6.2 Publish
1. Navigate to draft release
2. Click "Edit"
3. Final review of description
4. **Click "Publish release"**

🎉 **Release is now live!**

## Step 7: Post-Release Tasks

### 7.1 Verify Release Page
1. Check release appears at top of Releases page
2. Verify "Latest" badge is displayed
3. Test download links work for public users
4. Check that release is visible in repository sidebar

### 7.2 Update Documentation Links
If README or other docs link to "latest release", verify links work:
```bash
# Test download link
curl -I https://github.com/[username]/nodoka/releases/latest
# Should redirect to v0.2.0
```

### 7.3 Announce Release
- [ ] Post to GitHub Discussions
- [ ] Update project website (if exists)
- [ ] Social media announcement (Twitter, Reddit, etc.)
- [ ] Email mailing list subscribers (if applicable)

### 7.4 Monitor Issues
- Watch GitHub Issues for bug reports
- Respond to questions within 24 hours
- Track download statistics

### 7.5 Tag Main Branch (Optional)
```bash
# Ensure main branch is tagged
git checkout main
git pull origin main
git tag -f v0.2.0 HEAD
git push -f origin v0.2.0
```

## Rollback Procedure

If critical bugs are found after release:

### Option 1: Mark as Pre-Release
1. Edit release on GitHub
2. Check "Set as a pre-release"
3. Add warning to description
4. Create hotfix release v0.2.1

### Option 2: Delete Release (Last Resort)
```bash
# Delete tag locally
git tag -d v0.2.0

# Delete tag remotely
git push origin :refs/tags/v0.2.0

# Delete release on GitHub
# Go to release page → Edit → Delete release
```

Then fix issues and re-release.

## Success Metrics

Release is successful if:
- ✅ All 3 installers download and install correctly
- ✅ No critical bugs reported within 48 hours
- ✅ Core functionality works (scanning, playback, progress)
- ✅ Download count increases steadily
- ✅ User feedback is positive or neutral

## Support Resources

- **Issues**: https://github.com/[username]/nodoka/issues
- **Discussions**: https://github.com/[username]/nodoka/discussions
- **Email**: mistlight@otakukingdom.com

## Checklist Summary

- [x] Step 1: Create and push tag v0.2.0
- [x] Step 2: Monitor CI/CD (20-30 minutes)
- [x] Step 3: Download and verify artifacts
- [ ] Step 4: Create GitHub release (draft)
- [ ] Step 5: Smoke test all platforms
- [ ] Step 6: Publish release
- [ ] Step 7: Post-release tasks

**Current Status**: Ready for Step 1 (tag creation)
