# Nodoka v0.2.0 Release Guide

This guide provides complete instructions for creating and verifying the Nodoka v0.2.0 release.

## Overview

The release process leverages GitHub Actions CI/CD to automatically build installers for all three platforms (Linux, Windows, macOS) when a GitHub release is created. Manual smoke testing is required after installers are built.

## Prerequisites

1. **All code complete**: Rust conversion finished, all features implemented
2. **Tests passing**: `cargo test --all` shows 18/18 tests passing
3. **Linting clean**: `cargo clippy --all-targets --all-features -- -D warnings` passes
4. **No forbidden patterns**: No `.unwrap()`, `.expect()`, or `#[allow]` in `src/`
5. **macOS DMG built**: `packaging/macos/Nodoka-0.2.0.dmg` exists (4.2 MB)
6. **Version updated**: `Cargo.toml` version set to `0.2.0`
7. **Documentation ready**: `CHANGELOG.md`, `README.md`, `USER_GUIDE.md` updated
8. **GitHub CLI installed**: `gh` command available and authenticated
9. **Clean working directory**: All changes committed to git

## Release Process

### Step 1: Pre-Release Verification (Automated)

Run the automated pre-release check script:

```bash
./scripts/create-release.sh
```

This script verifies:
- Cargo.toml version matches 0.2.0
- All tests pass
- Zero clippy warnings
- No forbidden patterns in source code
- Git working directory is clean
- macOS DMG exists
- Packaging scripts are ready
- GitHub CLI is installed

If any checks fail, fix the issues before proceeding.

### Step 2: Create Git Tag

```bash
git tag -a v0.2.0 -m "Nodoka 0.2.0 - Complete Rust Rewrite"
git push origin v0.2.0
```

This creates the version tag in the repository.

### Step 3: Create GitHub Release

#### Option A: Using the Release Script (Recommended)

The `create-release.sh` script can create the release for you. When prompted, answer `y`:

```bash
./scripts/create-release.sh
```

#### Option B: Using GitHub CLI Manually

```bash
gh release create v0.2.0 \
  --title "Nodoka 0.2.0 - Rust Rewrite Release" \
  --notes-file CHANGELOG.md \
  packaging/macos/Nodoka-0.2.0.dmg
```

#### Option C: Using GitHub Web Interface

1. Navigate to: `https://github.com/YOUR_USERNAME/nodoka/releases/new`
2. **Tag**: Select `v0.2.0` (or create new tag)
3. **Release title**: `Nodoka 0.2.0 - Rust Rewrite Release`
4. **Description**: Copy content from `CHANGELOG.md`
5. **Attach files**: Drag and drop `packaging/macos/Nodoka-0.2.0.dmg`
6. Click "Publish release"

### Step 4: Monitor CI/CD Build Process

Creating the GitHub release triggers the CI/CD pipeline which will:

1. **Build jobs** (~5 minutes):
   - Lint code on Ubuntu
   - Run tests on Ubuntu, Windows, macOS
   - Build release binaries for all three platforms

2. **Package jobs** (~10 minutes):
   - `package-linux`: Build DEB package on Ubuntu
   - `package-windows`: Build MSI installer on Windows
   - `package-macos`: Build DMG on macOS (rebuilds from artifact)

3. **Checksum job** (~2 minutes):
   - Download all three installers
   - Generate SHA256SUMS.txt
   - Upload to release

Monitor progress at:
```
https://github.com/YOUR_USERNAME/nodoka/actions
```

### Step 5: Verify CI/CD Completed Successfully

After ~15-20 minutes, verify the GitHub release has all artifacts:

1. Navigate to: `https://github.com/YOUR_USERNAME/nodoka/releases/tag/v0.2.0`
2. Verify **4 assets attached**:
   - `Nodoka-0.2.0.dmg` (~4 MB) - macOS installer
   - `nodoka_0.2.0_amd64.deb` (~8 MB) - Linux installer
   - `Nodoka-0.2.0.msi` (~9 MB) - Windows installer
   - `SHA256SUMS.txt` - Checksums for all three

3. Verify release is marked as "Latest release"

4. Test download links:
   ```bash
   # Download checksums file
   wget https://github.com/YOUR_USERNAME/nodoka/releases/download/v0.2.0/SHA256SUMS.txt
   cat SHA256SUMS.txt
   ```

### Step 6: Perform Cross-Platform Smoke Tests

**CRITICAL**: Manual testing required on all three platforms.

Use the comprehensive checklist in `SMOKE_TEST_CHECKLIST.md`:

1. **macOS Testing** (macOS 12+):
   - Download `Nodoka-0.2.0.dmg`
   - Verify checksum: `shasum -a 256 Nodoka-0.2.0.dmg`
   - Install and test all 7 scenarios
   - Complete macOS section of checklist

2. **Linux Testing** (Ubuntu 22.04+):
   - Download `nodoka_0.2.0_amd64.deb`
   - Verify checksum: `sha256sum nodoka_0.2.0_amd64.deb`
   - Install and test all 7 scenarios
   - Complete Linux section of checklist

3. **Windows Testing** (Windows 10/11):
   - Download `Nodoka-0.2.0.msi`
   - Verify checksum: `certutil -hashfile Nodoka-0.2.0.msi SHA256`
   - Install and test all 7 scenarios
   - Complete Windows section of checklist

**Each platform must test**:
- Installation verification
- First launch
- Directory management
- Audio playback (with actual sound output)
- Progress persistence
- Multi-file audiobooks
- All 5 audio formats (MP3, M4A, M4B, OGG, FLAC)

### Step 7: Document Test Results

After completing smoke tests, update `SMOKE_TEST_CHECKLIST.md` with results:
- Mark all passed scenarios
- Document any issues found
- Provide overall assessment (PASS/CONDITIONAL PASS/FAIL)

### Step 8: Release Decision

Based on smoke test results:

#### If All Tests Pass:
- Mark release as final
- Update social media/announcement channels
- Close any related GitHub issues
- Celebrate! 🎉

#### If Minor Issues Found:
- Document issues in GitHub Issues
- Add to release notes as "Known Issues"
- Plan fixes for v0.2.1 patch release
- Release proceeds

#### If Critical Issues Found:
- Delete the release: `gh release delete v0.2.0`
- Delete the tag: `git push origin :refs/tags/v0.2.0`
- Fix critical issues
- Increment version to v0.2.1
- Restart release process

## Post-Release Tasks

### Update Documentation

1. Update `README.md` badges:
   - Release version badge
   - Build status badge

2. Update download links in `README.md`:
   ```markdown
   ## Installation

   ### macOS (12+)
   Download [Nodoka-0.2.0.dmg](https://github.com/YOUR_USERNAME/nodoka/releases/download/v0.2.0/Nodoka-0.2.0.dmg)

   ### Linux (Ubuntu 22.04+)
   Download [nodoka_0.2.0_amd64.deb](https://github.com/YOUR_USERNAME/nodoka/releases/download/v0.2.0/nodoka_0.2.0_amd64.deb)

   ### Windows (10/11)
   Download [Nodoka-0.2.0.msi](https://github.com/YOUR_USERNAME/nodoka/releases/download/v0.2.0/Nodoka-0.2.0.msi)
   ```

3. Add checksums to README:
   ```markdown
   Verify downloads with SHA256:
   ```
   Copy checksums from `SHA256SUMS.txt`

### Create Announcement

Draft release announcement with:
- Key features of the Rust rewrite
- Breaking changes from C++ version
- Installation instructions
- Link to release page
- Call for feedback/bug reports

### GitHub Repository Settings

Update repository metadata:
- **Description**: "A cross-platform audiobook reader built with Rust and iced. Features automatic progress tracking, VLC-powered playback, and a clean UI."
- **Topics**: rust, audiobook, iced, vlc, cross-platform, desktop-app, audiobook-player
- **Website**: Link to documentation or GitHub Pages

### Community Engagement

- Post announcement to relevant communities (r/rust, r/audiobooks, etc.)
- Update project documentation with v0.2.0 as stable baseline
- Thank contributors (if any)
- Solicit feedback and bug reports

## Rollback Procedure

If release needs to be rolled back:

```bash
# Delete GitHub release
gh release delete v0.2.0 --yes

# Delete git tag locally
git tag -d v0.2.0

# Delete git tag remotely
git push origin :refs/tags/v0.2.0
```

Fix issues, increment version, and restart release process.

## Troubleshooting

### CI/CD Build Fails

**Symptom**: Package jobs fail in GitHub Actions

**Solutions**:
1. Check workflow logs for specific error
2. Verify VLC dependencies install correctly
3. Verify WiX Toolset installs on Windows
4. Verify packaging scripts are executable
5. Test scripts locally if possible

### Checksums Don't Match

**Symptom**: Downloaded installer checksum differs from SHA256SUMS.txt

**Solutions**:
1. Re-download installer (download may have been corrupted)
2. Verify CI/CD checksum generation job completed successfully
3. Verify no manual changes to installers after CI/CD build
4. Regenerate checksums if needed

### Installer Fails to Install

**Symptom**: DEB/MSI/DMG fails during installation

**Solutions**:
1. Check system requirements (OS version, dependencies)
2. Verify VLC 3.x is installed
3. Check installer logs (dpkg, MSI log, Console.app)
4. Verify installer was built for correct architecture (x64)
5. Test on clean VM to rule out system-specific issues

### Audio Playback Doesn't Work

**Symptom**: UI works but no audio output

**Solutions**:
1. Verify VLC is installed and working
2. Test VLC directly with sample audio file
3. Check VLC library linking: `ldd nodoka | grep vlc` (Linux)
4. Check audio system (PulseAudio, PipeWire, CoreAudio, Windows Audio)
5. Verify audio file format is supported

## Files Modified During Release

- `Cargo.toml` - Version number
- `CHANGELOG.md` - Release notes
- `README.md` - Download links and checksums
- Git tags - v0.2.0 tag created
- GitHub release - Created with artifacts

## Files Generated by CI/CD

- `Nodoka-0.2.0.dmg` - macOS installer (rebuilt by CI/CD)
- `nodoka_0.2.0_amd64.deb` - Linux installer
- `Nodoka-0.2.0.msi` - Windows installer
- `SHA256SUMS.txt` - All checksums

## Success Criteria

Release is considered successful when:

1. ✅ All CI/CD jobs pass (lint, test, build, package, checksums)
2. ✅ All three installers attached to GitHub release
3. ✅ SHA256SUMS.txt attached with all three checksums
4. ✅ All download links functional
5. ✅ macOS smoke tests pass (7/7 scenarios)
6. ✅ Linux smoke tests pass (7/7 scenarios)
7. ✅ Windows smoke tests pass (7/7 scenarios)
8. ✅ All 5 audio formats play correctly on all platforms
9. ✅ No critical bugs found during testing
10. ✅ Release marked as "Latest release" on GitHub

## Timeline

- **Pre-release checks**: 15 minutes
- **Create release**: 5 minutes
- **CI/CD builds**: 15-20 minutes
- **Smoke testing**: 2-4 hours (all platforms)
- **Documentation updates**: 30 minutes
- **Total**: ~4-5 hours for complete release

## Contact

For questions about the release process:
- Create GitHub issue: https://github.com/YOUR_USERNAME/nodoka/issues
- Check documentation: docs/USER_GUIDE.md
