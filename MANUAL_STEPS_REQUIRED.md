# Manual Steps Required to Complete Release

This document outlines the manual steps that cannot be automated and must be performed by a human to complete the v0.2.0 release.

## Status: Ready for Manual Testing and Release

✅ **Completed Automatically:**
- Full Rust conversion (18/18 tests passing)
- Strict linting configuration (zero warnings)
- macOS DMG installer built locally
- Linux DEB packaging script ready and tested
- Windows MSI packaging script ready
- CI/CD pipeline configured for all three platforms
- SHA256 checksum generation configured
- Documentation updated (README, CHANGELOG, RELEASE_NOTES)

⏳ **Requires Manual Action:**
- Cross-platform smoke testing on real hardware
- GitHub release creation and publishing
- Installer verification on clean systems

---

## Step 1: Trigger Automated Installer Builds

The CI/CD pipeline will build all installers automatically when you create a release or push a tag.

### Option A: Create GitHub Release (Recommended)

1. Go to: https://github.com/otakukingdom/nodoka/releases/new
2. Click "Choose a tag" and type `v0.2.0`
3. Click "Create new tag: v0.2.0 on publish"
4. Release title: `Nodoka 0.2.0 - Rust Rewrite`
5. Description: Copy the entire content from `RELEASE_NOTES_v0.2.0.md`
6. Mark as "Latest release" (check the box)
7. Click "Publish release"

This will trigger the CI/CD workflow that:
- Builds Linux DEB on Ubuntu runner
- Builds Windows MSI on Windows runner
- Builds macOS DMG on macOS runner
- Generates SHA256 checksums
- Uploads all artifacts to the release

### Option B: Push Git Tag (Alternative)

If you prefer to build first, verify, then create the release later:

```bash
git tag -a v0.2.0 -m "Nodoka 0.2.0 - Rust Rewrite Release"
git push origin v0.2.0
```

Then go to Actions tab to monitor the build progress. Once builds complete, create the release manually and download artifacts from the workflow.

---

## Step 2: Monitor CI/CD Build

1. Go to: https://github.com/otakukingdom/nodoka/actions
2. Find the "Build and Test" workflow triggered by your tag/release
3. Monitor the progress of all jobs:
   - `lint` (should pass - already verified locally)
   - `test` (should pass - already verified locally)
   - `build` for all 3 platforms
   - `package-linux` (builds DEB)
   - `package-windows` (builds MSI)
   - `package-macos` (builds DMG)
   - `generate-checksums` (creates SHA256SUMS.txt)

4. If any job fails:
   - Click on the failed job to see logs
   - Fix the issue locally
   - Delete the tag: `git tag -d v0.2.0 && git push origin :refs/tags/v0.2.0`
   - Delete the release draft (if created)
   - Make fixes and commit
   - Start over from Step 1

5. If all jobs pass:
   - All installer artifacts will be uploaded to the release (if using Option A)
   - Or available as workflow artifacts (if using Option B)

---

## Step 3: Download and Verify Installers

### If Using Release Method (Option A)

1. Go to: https://github.com/otakukingdom/nodoka/releases/tag/v0.2.0
2. Verify all 4 files are attached:
   - `nodoka_0.2.0_amd64.deb`
   - `Nodoka-0.2.0.msi`
   - `Nodoka-0.2.0.dmg`
   - `SHA256SUMS.txt`

### If Using Tag Method (Option B)

1. Go to the completed workflow run
2. Scroll to "Artifacts" section at bottom
3. Download:
   - `linux-deb` artifact
   - `windows-msi` artifact
   - `macos-dmg` artifact
   - `checksums` artifact

### Verify Checksums

```bash
# Download SHA256SUMS.txt from release or artifact
# Download all three installers

# Verify each installer
sha256sum nodoka_0.2.0_amd64.deb    # Linux
sha256sum Nodoka-0.2.0.msi          # Windows (or use certutil on Windows)
sha256sum Nodoka-0.2.0.dmg          # macOS

# Compare with SHA256SUMS.txt
cat SHA256SUMS.txt
```

All checksums must match exactly.

---

## Step 4: Cross-Platform Smoke Testing

**CRITICAL**: This step cannot be automated. You must test on real systems.

### Required Test Environments

You need access to:
- **macOS 12+** (ideally both Intel and Apple Silicon)
- **Ubuntu 22.04 LTS** or Debian 11+ (clean VM recommended)
- **Windows 10 or 11** (clean VM recommended)

### Test Procedure

For each platform:

1. **Download the installer** from the GitHub release
2. **Verify SHA256 checksum** matches `SHA256SUMS.txt`
3. **Open SMOKE_TEST_CHECKLIST.md**
4. **Complete all 7 test scenarios** for that platform:
   - Scenario 1: Installation Verification
   - Scenario 2: First Launch
   - Scenario 3: Directory Management
   - Scenario 4: Audio Playback (**verify actual sound!**)
   - Scenario 5: Progress Persistence
   - Scenario 6: Multi-File Audiobooks
   - Scenario 7: Audio Format Support
5. **Check all boxes** in the checklist as you complete each test
6. **Document any issues** found

### Test Sample Files

Prepare sample audiobook files for testing:
- 1 MP3 file
- 1 M4A file
- 1 M4B file
- 1 OGG file
- 1 FLAC file
- 1 multi-chapter audiobook directory (e.g., 3 MP3 files)

You can use public domain audiobooks from LibriVox: https://librivox.org

### Critical Issues That Block Release

If you encounter any of these during testing, **DO NOT PUBLISH** the release:

- ❌ Application crashes on launch
- ❌ No audio output despite UI showing playback
- ❌ Database corruption or data loss
- ❌ Installer fails to complete
- ❌ VLC library linking errors
- ❌ Single instance guard not working (multiple instances can run)

Fix these issues before proceeding. You may need to:
1. Fix the code
2. Delete the tag and release
3. Start over from Step 1

### Non-Critical Issues (Can Release)

These issues do not block release but should be documented:

- ⚠️ Minor UI glitches (alignment issues, etc.)
- ⚠️ Slow performance with very large libraries
- ⚠️ Missing file format support for rare formats
- ⚠️ Platform-specific cosmetic issues

Document these in GitHub Issues for future patch releases.

---

## Step 5: Update Release with Test Results

After smoke testing passes on all platforms:

1. **Edit the GitHub release**
2. **Add a "Tested Platforms" section** at the bottom:

```markdown
## Tested Platforms

This release has been manually tested and verified on:

- ✅ macOS 13 Ventura (Intel)
- ✅ macOS 14 Sonoma (Apple Silicon)
- ✅ Ubuntu 22.04 LTS
- ✅ Windows 11

All smoke tests passed. See [SMOKE_TEST_CHECKLIST.md](SMOKE_TEST_CHECKLIST.md) for details.
```

3. **Add SHA256 checksums** to the Downloads table in release notes (copy from `SHA256SUMS.txt`)

4. **Save changes**

---

## Step 6: Post-Release Verification

After publishing the release:

1. **Test download links** - click each installer link to verify downloads work
2. **Verify "Latest Release" badge** shows v0.2.0
3. **Check README download links** point to correct release
4. **Monitor GitHub Issues** for user reports
5. **Verify CI/CD badge** shows passing status

---

## Step 7: Announce Release (Optional)

If applicable:

1. **GitHub Discussions** - create announcement post
2. **Social media** - announce new Rust version
3. **Project website** - update download links
4. **Email list** - notify existing users

---

## Quick Reference Commands

### Check Current Status
```bash
# Verify version
grep '^version' Cargo.toml

# Run tests
cargo test --all

# Run linting
cargo clippy --all-targets --all-features -- -D warnings

# Check git status
git status
```

### Create Release
```bash
# Create and push tag
git tag -a v0.2.0 -m "Nodoka 0.2.0 - Rust Rewrite Release"
git push origin v0.2.0

# Or create GitHub Release via web UI (recommended)
```

### Verify Installers
```bash
# Download installers from release
curl -LO https://github.com/otakukingdom/nodoka/releases/download/v0.2.0/nodoka_0.2.0_amd64.deb
curl -LO https://github.com/otakukingdom/nodoka/releases/download/v0.2.0/Nodoka-0.2.0.msi
curl -LO https://github.com/otakukingdom/nodoka/releases/download/v0.2.0/Nodoka-0.2.0.dmg
curl -LO https://github.com/otakukingdom/nodoka/releases/download/v0.2.0/SHA256SUMS.txt

# Verify checksums
sha256sum -c SHA256SUMS.txt
```

---

## Troubleshooting

### CI/CD Build Fails

**Problem**: Linux/Windows/macOS build job fails in GitHub Actions

**Solutions**:
- Check workflow logs for specific error
- Verify packaging scripts work locally on that platform
- Common issues:
  - VLC not found - check VLC installation steps in workflow
  - WiX Toolset version mismatch - verify chocolatey install command
  - File paths incorrect - verify relative paths in packaging scripts

### Smoke Test Fails

**Problem**: Installer works but application crashes or has issues

**Solutions**:
- Document exact error message
- Check VLC is installed correctly
- Verify database directory is writable
- Test on different OS version
- If critical bug: abort release, fix code, retry

### Checksum Mismatch

**Problem**: Downloaded installer checksum doesn't match SHA256SUMS.txt

**Solutions**:
- Re-download the installer (network corruption)
- Verify workflow completed successfully
- Check if installer was rebuilt but checksums not regenerated
- Regenerate checksums if needed:
  ```bash
  sha256sum nodoka_0.2.0_amd64.deb Nodoka-0.2.0.msi Nodoka-0.2.0.dmg > SHA256SUMS.txt
  ```

---

## Contact

For issues during release process:
- **GitHub Issues**: https://github.com/otakukingdom/nodoka/issues
- **Maintainer**: Mistlight Oriroris

---

**Next Steps**: Begin with Step 1 when ready to trigger the release build process.
